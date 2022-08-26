#[allow(clippy::all)]
pub mod records {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Empty {}
    impl core::fmt::Debug for Empty {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Empty").finish()
        }
    }
    impl wit_bindgen_wasmer::Endian for Empty {
        fn into_le(self) -> Self {
            Self {}
        }
        fn from_le(self) -> Self {
            Self {}
        }
    }
    unsafe impl wit_bindgen_wasmer::AllBytesValid for Empty {}
    /// A record containing two scalar fields
    /// that both have the same type
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Scalars {
        /// The first field, named a
        pub a: u32,
        /// The second field, named b
        pub b: u32,
    }
    impl core::fmt::Debug for Scalars {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Scalars")
                .field("a", &self.a)
                .field("b", &self.b)
                .finish()
        }
    }
    impl wit_bindgen_wasmer::Endian for Scalars {
        fn into_le(self) -> Self {
            Self {
                a: self.a.into_le(),
                b: self.b.into_le(),
            }
        }
        fn from_le(self) -> Self {
            Self {
                a: self.a.from_le(),
                b: self.b.from_le(),
            }
        }
    }
    unsafe impl wit_bindgen_wasmer::AllBytesValid for Scalars {}
    /// A record that is really just flags
    /// All of the fields are bool
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ReallyFlags {
        pub a: bool,
        pub b: bool,
        pub c: bool,
        pub d: bool,
        pub e: bool,
        pub f: bool,
        pub g: bool,
        pub h: bool,
        pub i: bool,
    }
    impl core::fmt::Debug for ReallyFlags {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ReallyFlags")
                .field("a", &self.a)
                .field("b", &self.b)
                .field("c", &self.c)
                .field("d", &self.d)
                .field("e", &self.e)
                .field("f", &self.f)
                .field("g", &self.g)
                .field("h", &self.h)
                .field("i", &self.i)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct AggregatesParam<'a> {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: &'a str,
        pub e: ReallyFlags,
    }
    impl<'a> core::fmt::Debug for AggregatesParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("AggregatesParam")
                .field("a", &self.a)
                .field("b", &self.b)
                .field("c", &self.c)
                .field("d", &self.d)
                .field("e", &self.e)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct AggregatesResult {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: String,
        pub e: ReallyFlags,
    }
    impl core::fmt::Debug for AggregatesResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("AggregatesResult")
                .field("a", &self.a)
                .field("b", &self.b)
                .field("c", &self.c)
                .field("d", &self.d)
                .field("e", &self.e)
                .finish()
        }
    }
    pub type IntTypedef = i32;
    pub type TupleTypedef2 = (IntTypedef,);
    pub trait Records: Sized + Send + Sync + 'static {
        fn tuple_arg(&mut self, x: (char, u32)) -> ();

        fn tuple_result(&mut self) -> (char, u32);

        fn empty_arg(&mut self, x: Empty) -> ();

        fn empty_result(&mut self) -> Empty;

        fn scalar_arg(&mut self, x: Scalars) -> ();

        fn scalar_result(&mut self) -> Scalars;

        fn flags_arg(&mut self, x: ReallyFlags) -> ();

        fn flags_result(&mut self) -> ReallyFlags;

        fn aggregate_arg(&mut self, x: AggregatesParam<'_>) -> ();

        fn aggregate_result(&mut self) -> AggregatesResult;

        fn typedef_inout(&mut self, e: TupleTypedef2) -> i32;
    }
    pub struct LazyInitialized {
        memory: wasmer::Memory,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
    }

    #[must_use = "The returned initializer function must be called
  with the instance and the store before starting the runtime"]
    pub fn add_to_imports<T>(
        store: &mut wasmer::Store,
        imports: &mut wasmer::Imports,
        data: T,
    ) -> impl FnOnce(&wasmer::Instance, &dyn wasmer::AsStoreRef) -> Result<(), anyhow::Error>
    where
        T: Records,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Records> {
            data: T,
            lazy: std::rc::Rc<OnceCell<LazyInitialized>>,
        }
        unsafe impl<T: Records> Send for EnvWrapper<T> {}
        unsafe impl<T: Records> Sync for EnvWrapper<T> {}
        let lazy = std::rc::Rc::new(OnceCell::new());
        let env = EnvWrapper {
            data,
            lazy: std::rc::Rc::clone(&lazy),
        };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
            "tuple-arg",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = (char_from_i32(arg0)?, arg1 as u32);
                    let host = &mut data_mut.data;
                    let result = host.tuple_arg(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "tuple-result",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let host = &mut data_mut.data;
                    let result = host.tuple_result();
                    let (t0_0, t0_1) = result;
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store(
                        arg0 + 0,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_0)),
                    )?;
                    caller_memory.store(
                        arg0 + 4,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_1)),
                    )?;
                    Ok(())
                },
            ),
        );
        exports.insert(
    "empty-arg",
    wasmer::Function::new_typed_with_env(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<(), wasmer::RuntimeError> {
      let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
      let data_mut = store.data_mut();
      let param0 = Empty{};
      let host = &mut data_mut.data;
      let result = host.empty_arg(param0, );
      let () = result;
      Ok(())
    }
    ));
        exports.insert(
    "empty-result",
    wasmer::Function::new_typed_with_env(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<(), wasmer::RuntimeError> {
      let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
      let data_mut = store.data_mut();
      let host = &mut data_mut.data;
      let result = host.empty_result();
      let Empty{ } = result;
      Ok(())
    }
    ));
        exports.insert(
            "scalar-arg",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = Scalars {
                        a: arg0 as u32,
                        b: arg1 as u32,
                    };
                    let host = &mut data_mut.data;
                    let result = host.scalar_arg(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "scalar-result",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let host = &mut data_mut.data;
                    let result = host.scalar_result();
                    let Scalars { a: a0, b: b0 } = result;
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store(
                        arg0 + 0,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(a0)),
                    )?;
                    caller_memory.store(
                        arg0 + 4,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(b0)),
                    )?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "flags-arg",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32,
                      arg5: i32,
                      arg6: i32,
                      arg7: i32,
                      arg8: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = ReallyFlags {
                        a: match arg0 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                        b: match arg1 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                        c: match arg2 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                        d: match arg3 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                        e: match arg4 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                        f: match arg5 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                        g: match arg6 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                        h: match arg7 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                        i: match arg8 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                    };
                    let host = &mut data_mut.data;
                    let result = host.flags_arg(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "flags-result",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let host = &mut data_mut.data;
                    let result = host.flags_result();
                    let ReallyFlags {
                        a: a0,
                        b: b0,
                        c: c0,
                        d: d0,
                        e: e0,
                        f: f0,
                        g: g0,
                        h: h0,
                        i: i0,
                    } = result;
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store(
                        arg0 + 0,
                        wit_bindgen_wasmer::rt::as_i32(match a0 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 1,
                        wit_bindgen_wasmer::rt::as_i32(match b0 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 2,
                        wit_bindgen_wasmer::rt::as_i32(match c0 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 3,
                        wit_bindgen_wasmer::rt::as_i32(match d0 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 4,
                        wit_bindgen_wasmer::rt::as_i32(match e0 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 5,
                        wit_bindgen_wasmer::rt::as_i32(match f0 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 6,
                        wit_bindgen_wasmer::rt::as_i32(match g0 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 7,
                        wit_bindgen_wasmer::rt::as_i32(match h0 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 8,
                        wit_bindgen_wasmer::rt::as_i32(match i0 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "aggregate-arg",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32,
                      arg5: i32,
                      arg6: i32,
                      arg7: i32,
                      arg8: i32,
                      arg9: i32,
                      arg10: i32,
                      arg11: i32,
                      arg12: i32,
                      arg13: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg3;
                    let len0 = arg4;
                    let param0 = AggregatesParam {
                        a: Scalars {
                            a: arg0 as u32,
                            b: arg1 as u32,
                        },
                        b: arg2 as u32,
                        c: Empty {},
                        d: _bc.slice_str(ptr0, len0)?,
                        e: ReallyFlags {
                            a: match arg5 {
                                0 => false,
                                1 => true,
                                _ => return Err(invalid_variant("bool")),
                            },
                            b: match arg6 {
                                0 => false,
                                1 => true,
                                _ => return Err(invalid_variant("bool")),
                            },
                            c: match arg7 {
                                0 => false,
                                1 => true,
                                _ => return Err(invalid_variant("bool")),
                            },
                            d: match arg8 {
                                0 => false,
                                1 => true,
                                _ => return Err(invalid_variant("bool")),
                            },
                            e: match arg9 {
                                0 => false,
                                1 => true,
                                _ => return Err(invalid_variant("bool")),
                            },
                            f: match arg10 {
                                0 => false,
                                1 => true,
                                _ => return Err(invalid_variant("bool")),
                            },
                            g: match arg11 {
                                0 => false,
                                1 => true,
                                _ => return Err(invalid_variant("bool")),
                            },
                            h: match arg12 {
                                0 => false,
                                1 => true,
                                _ => return Err(invalid_variant("bool")),
                            },
                            i: match arg13 {
                                0 => false,
                                1 => true,
                                _ => return Err(invalid_variant("bool")),
                            },
                        },
                    };
                    let host = &mut data_mut.data;
                    let result = host.aggregate_arg(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "aggregate-result",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let host = &mut data_mut.data;
                    let result = host.aggregate_result();
                    let AggregatesResult {
                        a: a0,
                        b: b0,
                        c: c0,
                        d: d0,
                        e: e0,
                    } = result;
                    let Scalars { a: a1, b: b1 } = a0;
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store(
                        arg0 + 0,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(a1)),
                    )?;
                    caller_memory.store(
                        arg0 + 4,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(b1)),
                    )?;
                    caller_memory.store(
                        arg0 + 8,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(b0)),
                    )?;
                    let Empty {} = c0;
                    let vec3 = d0;
                    let ptr3 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec3.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store_many(ptr3, vec3.as_bytes())?;
                    caller_memory
                        .store(arg0 + 16, wit_bindgen_wasmer::rt::as_i32(vec3.len() as i32))?;
                    caller_memory.store(arg0 + 12, wit_bindgen_wasmer::rt::as_i32(ptr3))?;
                    let ReallyFlags {
                        a: a4,
                        b: b4,
                        c: c4,
                        d: d4,
                        e: e4,
                        f: f4,
                        g: g4,
                        h: h4,
                        i: i4,
                    } = e0;
                    caller_memory.store(
                        arg0 + 20,
                        wit_bindgen_wasmer::rt::as_i32(match a4 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 21,
                        wit_bindgen_wasmer::rt::as_i32(match b4 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 22,
                        wit_bindgen_wasmer::rt::as_i32(match c4 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 23,
                        wit_bindgen_wasmer::rt::as_i32(match d4 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 24,
                        wit_bindgen_wasmer::rt::as_i32(match e4 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 25,
                        wit_bindgen_wasmer::rt::as_i32(match f4 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 26,
                        wit_bindgen_wasmer::rt::as_i32(match g4 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 27,
                        wit_bindgen_wasmer::rt::as_i32(match h4 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 28,
                        wit_bindgen_wasmer::rt::as_i32(match i4 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "typedef-inout",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = (arg0,);
                    let host = &mut data_mut.data;
                    let result = host.typedef_inout(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        imports.register_namespace("records", exports);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| {
            let memory = _instance.exports.get_memory("memory")?.clone();
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(&_store.as_store_ref(), "canonical_abi_realloc")
                .unwrap()
                .clone();
            lazy.set(LazyInitialized {
                memory,
                func_canonical_abi_realloc,
            })
            .map_err(|_e| anyhow::anyhow!("Couldn't set lazy initialized data"))?;
            Ok(())
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::once_cell::unsync::OnceCell;
    use wit_bindgen_wasmer::rt::char_from_i32;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
