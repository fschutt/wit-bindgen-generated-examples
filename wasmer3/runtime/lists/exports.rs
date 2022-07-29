#[allow(clippy::all)]
pub mod imports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct Flag32: u32 {const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
        const B4 = 1 << 4;
        const B5 = 1 << 5;
        const B6 = 1 << 6;
        const B7 = 1 << 7;
        const B8 = 1 << 8;
        const B9 = 1 << 9;
        const B10 = 1 << 10;
        const B11 = 1 << 11;
        const B12 = 1 << 12;
        const B13 = 1 << 13;
        const B14 = 1 << 14;
        const B15 = 1 << 15;
        const B16 = 1 << 16;
        const B17 = 1 << 17;
        const B18 = 1 << 18;
        const B19 = 1 << 19;
        const B20 = 1 << 20;
        const B21 = 1 << 21;
        const B22 = 1 << 22;
        const B23 = 1 << 23;
        const B24 = 1 << 24;
        const B25 = 1 << 25;
        const B26 = 1 << 26;
        const B27 = 1 << 27;
        const B28 = 1 << 28;
        const B29 = 1 << 29;
        const B30 = 1 << 30;
        const B31 = 1 << 31;
      }
    }

    impl core::fmt::Display for Flag32 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("Flag32(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct Flag64: u64 {const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
        const B4 = 1 << 4;
        const B5 = 1 << 5;
        const B6 = 1 << 6;
        const B7 = 1 << 7;
        const B8 = 1 << 8;
        const B9 = 1 << 9;
        const B10 = 1 << 10;
        const B11 = 1 << 11;
        const B12 = 1 << 12;
        const B13 = 1 << 13;
        const B14 = 1 << 14;
        const B15 = 1 << 15;
        const B16 = 1 << 16;
        const B17 = 1 << 17;
        const B18 = 1 << 18;
        const B19 = 1 << 19;
        const B20 = 1 << 20;
        const B21 = 1 << 21;
        const B22 = 1 << 22;
        const B23 = 1 << 23;
        const B24 = 1 << 24;
        const B25 = 1 << 25;
        const B26 = 1 << 26;
        const B27 = 1 << 27;
        const B28 = 1 << 28;
        const B29 = 1 << 29;
        const B30 = 1 << 30;
        const B31 = 1 << 31;
        const B32 = 1 << 32;
        const B33 = 1 << 33;
        const B34 = 1 << 34;
        const B35 = 1 << 35;
        const B36 = 1 << 36;
        const B37 = 1 << 37;
        const B38 = 1 << 38;
        const B39 = 1 << 39;
        const B40 = 1 << 40;
        const B41 = 1 << 41;
        const B42 = 1 << 42;
        const B43 = 1 << 43;
        const B44 = 1 << 44;
        const B45 = 1 << 45;
        const B46 = 1 << 46;
        const B47 = 1 << 47;
        const B48 = 1 << 48;
        const B49 = 1 << 49;
        const B50 = 1 << 50;
        const B51 = 1 << 51;
        const B52 = 1 << 52;
        const B53 = 1 << 53;
        const B54 = 1 << 54;
        const B55 = 1 << 55;
        const B56 = 1 << 56;
        const B57 = 1 << 57;
        const B58 = 1 << 58;
        const B59 = 1 << 59;
        const B60 = 1 << 60;
        const B61 = 1 << 61;
        const B62 = 1 << 62;
        const B63 = 1 << 63;
      }
    }

    impl core::fmt::Display for Flag64 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("Flag64(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct UnalignedRecord {
        pub a: u32,
        pub b: u64,
    }
    impl core::fmt::Debug for UnalignedRecord {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("UnalignedRecord")
                .field("a", &self.a)
                .field("b", &self.b)
                .finish()
        }
    }
    impl wit_bindgen_wasmer::Endian for UnalignedRecord {
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
    unsafe impl wit_bindgen_wasmer::AllBytesValid for UnalignedRecord {}
    pub trait Imports: Sized + Send + Sync + 'static {
        fn list_param(&mut self, a: &[u8]) -> ();

        fn list_param2(&mut self, a: &str) -> ();

        fn list_param3(&mut self, a: Vec<&str>) -> ();

        fn list_param4(&mut self, a: Vec<Vec<&str>>) -> ();

        fn list_result(&mut self) -> Vec<u8>;

        fn list_result2(&mut self) -> String;

        fn list_result3(&mut self) -> Vec<String>;

        fn list_minmax8(&mut self, a: &[u8], b: &[i8]) -> (Vec<u8>, Vec<i8>);

        fn list_minmax16(&mut self, a: &[Le<u16>], b: &[Le<i16>]) -> (Vec<u16>, Vec<i16>);

        fn list_minmax32(&mut self, a: &[Le<u32>], b: &[Le<i32>]) -> (Vec<u32>, Vec<i32>);

        fn list_minmax64(&mut self, a: &[Le<u64>], b: &[Le<i64>]) -> (Vec<u64>, Vec<i64>);

        fn list_minmax_float(&mut self, a: &[Le<f32>], b: &[Le<f64>]) -> (Vec<f32>, Vec<f64>);

        fn list_roundtrip(&mut self, a: &[u8]) -> Vec<u8>;

        fn string_roundtrip(&mut self, a: &str) -> String;

        fn unaligned_roundtrip1(
            &mut self,
            a: &[Le<u16>],
            b: &[Le<u32>],
            c: &[Le<u64>],
            d: Vec<Flag32>,
            e: Vec<Flag64>,
        ) -> ();

        fn unaligned_roundtrip2(
            &mut self,
            a: &[Le<UnalignedRecord>],
            b: &[Le<f32>],
            c: &[Le<f64>],
            d: Vec<&str>,
            e: Vec<&[u8]>,
        ) -> ();
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
        T: Imports,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Imports> {
            data: T,
            lazy: std::rc::Rc<OnceCell<LazyInitialized>>,
        }
        unsafe impl<T: Imports> Send for EnvWrapper<T> {}
        unsafe impl<T: Imports> Sync for EnvWrapper<T> {}
        let lazy = std::rc::Rc::new(OnceCell::new());
        let env = EnvWrapper {
            data,
            lazy: std::rc::Rc::clone(&lazy),
        };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
            "list-param",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let param0 = _bc.slice(ptr0, len0)?;
                    let host = &mut data_mut.data;
                    let result = host.list_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-param2",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let param0 = _bc.slice_str(ptr0, len0)?;
                    let host = &mut data_mut.data;
                    let result = host.list_param2(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-param3",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let len3 = arg1;
                    let base3 = arg0;
                    let mut result3 = Vec::with_capacity(len3 as usize);
                    for i in 0..len3 {
                        let base = base3 + i * 8;
                        result3.push({
                            let load0 = _bc.load::<i32>(base + 0)?;
                            let load1 = _bc.load::<i32>(base + 4)?;
                            let ptr2 = load0;
                            let len2 = load1;
                            _bc.slice_str(ptr2, len2)?
                        });
                    }
                    let param0 = result3;
                    let host = &mut data_mut.data;
                    let result = host.list_param3(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-param4",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let len6 = arg1;
                    let base6 = arg0;
                    let mut result6 = Vec::with_capacity(len6 as usize);
                    for i in 0..len6 {
                        let base = base6 + i * 8;
                        result6.push({
                            let load0 = _bc.load::<i32>(base + 0)?;
                            let load1 = _bc.load::<i32>(base + 4)?;
                            let len5 = load1;
                            let base5 = load0;
                            let mut result5 = Vec::with_capacity(len5 as usize);
                            for i in 0..len5 {
                                let base = base5 + i * 8;
                                result5.push({
                                    let load2 = _bc.load::<i32>(base + 0)?;
                                    let load3 = _bc.load::<i32>(base + 4)?;
                                    let ptr4 = load2;
                                    let len4 = load3;
                                    _bc.slice_str(ptr4, len4)?
                                });
                            }
                            result5
                        });
                    }
                    let param0 = result6;
                    let host = &mut data_mut.data;
                    let result = host.list_param4(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-result",
            wasmer::Function::new_native(
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
                    let result = host.list_result();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        (vec0.len() as i32) * 1,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr0, &vec0)?;
                    caller_memory
                        .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32))?;
                    caller_memory.store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-result2",
            wasmer::Function::new_native(
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
                    let result = host.list_result2();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec0.len() as i32,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                    caller_memory
                        .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32))?;
                    caller_memory.store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-result3",
            wasmer::Function::new_native(
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
                    let result = host.list_result3();
                    let vec1 = result;
                    let len1 = vec1.len() as i32;
                    let result1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        len1 * 8,
                    )?;
                    for (i, e) in vec1.into_iter().enumerate() {
                        let base = result1 + (i as i32) * 8;
                        {
                            let vec0 = e;
                            let ptr0 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec0.len() as i32,
                            )?;
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory.store_many(ptr0, vec0.as_bytes())?;
                            caller_memory.store(
                                base + 4,
                                wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32),
                            )?;
                            caller_memory.store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                        }
                    }
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(len1))?;
                    caller_memory.store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(result1))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-minmax8",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let ptr1 = arg2;
                    let len1 = arg3;
                    let param0 = _bc.slice(ptr0, len0)?;
                    let param1 = _bc.slice(ptr1, len1)?;
                    let host = &mut data_mut.data;
                    let result = host.list_minmax8(param0, param1);
                    let (t2_0, t2_1) = result;
                    let vec3 = t2_0;
                    let ptr3 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        (vec3.len() as i32) * 1,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr3, &vec3)?;
                    caller_memory
                        .store(arg4 + 4, wit_bindgen_wasmer::rt::as_i32(vec3.len() as i32))?;
                    caller_memory.store(arg4 + 0, wit_bindgen_wasmer::rt::as_i32(ptr3))?;
                    let vec4 = t2_1;
                    let ptr4 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        (vec4.len() as i32) * 1,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr4, &vec4)?;
                    caller_memory
                        .store(arg4 + 12, wit_bindgen_wasmer::rt::as_i32(vec4.len() as i32))?;
                    caller_memory.store(arg4 + 8, wit_bindgen_wasmer::rt::as_i32(ptr4))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-minmax16",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let ptr1 = arg2;
                    let len1 = arg3;
                    let param0 = _bc.slice(ptr0, len0)?;
                    let param1 = _bc.slice(ptr1, len1)?;
                    let host = &mut data_mut.data;
                    let result = host.list_minmax16(param0, param1);
                    let (t2_0, t2_1) = result;
                    let vec3 = t2_0;
                    let ptr3 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        2,
                        (vec3.len() as i32) * 2,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr3, &vec3)?;
                    caller_memory
                        .store(arg4 + 4, wit_bindgen_wasmer::rt::as_i32(vec3.len() as i32))?;
                    caller_memory.store(arg4 + 0, wit_bindgen_wasmer::rt::as_i32(ptr3))?;
                    let vec4 = t2_1;
                    let ptr4 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        2,
                        (vec4.len() as i32) * 2,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr4, &vec4)?;
                    caller_memory
                        .store(arg4 + 12, wit_bindgen_wasmer::rt::as_i32(vec4.len() as i32))?;
                    caller_memory.store(arg4 + 8, wit_bindgen_wasmer::rt::as_i32(ptr4))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-minmax32",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let ptr1 = arg2;
                    let len1 = arg3;
                    let param0 = _bc.slice(ptr0, len0)?;
                    let param1 = _bc.slice(ptr1, len1)?;
                    let host = &mut data_mut.data;
                    let result = host.list_minmax32(param0, param1);
                    let (t2_0, t2_1) = result;
                    let vec3 = t2_0;
                    let ptr3 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        (vec3.len() as i32) * 4,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr3, &vec3)?;
                    caller_memory
                        .store(arg4 + 4, wit_bindgen_wasmer::rt::as_i32(vec3.len() as i32))?;
                    caller_memory.store(arg4 + 0, wit_bindgen_wasmer::rt::as_i32(ptr3))?;
                    let vec4 = t2_1;
                    let ptr4 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        (vec4.len() as i32) * 4,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr4, &vec4)?;
                    caller_memory
                        .store(arg4 + 12, wit_bindgen_wasmer::rt::as_i32(vec4.len() as i32))?;
                    caller_memory.store(arg4 + 8, wit_bindgen_wasmer::rt::as_i32(ptr4))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-minmax64",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let ptr1 = arg2;
                    let len1 = arg3;
                    let param0 = _bc.slice(ptr0, len0)?;
                    let param1 = _bc.slice(ptr1, len1)?;
                    let host = &mut data_mut.data;
                    let result = host.list_minmax64(param0, param1);
                    let (t2_0, t2_1) = result;
                    let vec3 = t2_0;
                    let ptr3 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        8,
                        (vec3.len() as i32) * 8,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr3, &vec3)?;
                    caller_memory
                        .store(arg4 + 4, wit_bindgen_wasmer::rt::as_i32(vec3.len() as i32))?;
                    caller_memory.store(arg4 + 0, wit_bindgen_wasmer::rt::as_i32(ptr3))?;
                    let vec4 = t2_1;
                    let ptr4 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        8,
                        (vec4.len() as i32) * 8,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr4, &vec4)?;
                    caller_memory
                        .store(arg4 + 12, wit_bindgen_wasmer::rt::as_i32(vec4.len() as i32))?;
                    caller_memory.store(arg4 + 8, wit_bindgen_wasmer::rt::as_i32(ptr4))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-minmax-float",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let ptr1 = arg2;
                    let len1 = arg3;
                    let param0 = _bc.slice(ptr0, len0)?;
                    let param1 = _bc.slice(ptr1, len1)?;
                    let host = &mut data_mut.data;
                    let result = host.list_minmax_float(param0, param1);
                    let (t2_0, t2_1) = result;
                    let vec3 = t2_0;
                    let ptr3 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        (vec3.len() as i32) * 4,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr3, &vec3)?;
                    caller_memory
                        .store(arg4 + 4, wit_bindgen_wasmer::rt::as_i32(vec3.len() as i32))?;
                    caller_memory.store(arg4 + 0, wit_bindgen_wasmer::rt::as_i32(ptr3))?;
                    let vec4 = t2_1;
                    let ptr4 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        8,
                        (vec4.len() as i32) * 8,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr4, &vec4)?;
                    caller_memory
                        .store(arg4 + 12, wit_bindgen_wasmer::rt::as_i32(vec4.len() as i32))?;
                    caller_memory.store(arg4 + 8, wit_bindgen_wasmer::rt::as_i32(ptr4))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-roundtrip",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let param0 = _bc.slice(ptr0, len0)?;
                    let host = &mut data_mut.data;
                    let result = host.list_roundtrip(param0);
                    let vec1 = result;
                    let ptr1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        (vec1.len() as i32) * 1,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr1, &vec1)?;
                    caller_memory
                        .store(arg2 + 4, wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32))?;
                    caller_memory.store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(ptr1))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "string-roundtrip",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let param0 = _bc.slice_str(ptr0, len0)?;
                    let host = &mut data_mut.data;
                    let result = host.string_roundtrip(param0);
                    let vec1 = result;
                    let ptr1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec1.len() as i32,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr1, vec1.as_bytes())?;
                    caller_memory
                        .store(arg2 + 4, wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32))?;
                    caller_memory.store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(ptr1))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "unaligned-roundtrip1",
            wasmer::Function::new_native(
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
                      arg9: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let ptr1 = arg2;
                    let len1 = arg3;
                    let ptr2 = arg4;
                    let len2 = arg5;
                    let len4 = arg7;
                    let base4 = arg6;
                    let mut result4 = Vec::with_capacity(len4 as usize);
                    for i in 0..len4 {
                        let base = base4 + i * 4;
                        result4.push({
                            let load3 = _bc.load::<i32>(base + 0)?;
                            validate_flags(
                                0 | ((load3 as u32) << 0),
                                Flag32::all().bits(),
                                "Flag32",
                                |bits| Flag32 { bits },
                            )?
                        });
                    }
                    let len7 = arg9;
                    let base7 = arg8;
                    let mut result7 = Vec::with_capacity(len7 as usize);
                    for i in 0..len7 {
                        let base = base7 + i * 8;
                        result7.push({
                            let load5 = _bc.load::<i32>(base + 0)?;
                            let load6 = _bc.load::<i32>(base + 4)?;
                            validate_flags(
                                0 | ((load5 as u64) << 0) | ((load6 as u64) << 32),
                                Flag64::all().bits(),
                                "Flag64",
                                |bits| Flag64 { bits },
                            )?
                        });
                    }
                    let param0 = _bc.slice(ptr0, len0)?;
                    let param1 = _bc.slice(ptr1, len1)?;
                    let param2 = _bc.slice(ptr2, len2)?;
                    let param3 = result4;
                    let param4 = result7;
                    let host = &mut data_mut.data;
                    let result = host.unaligned_roundtrip1(param0, param1, param2, param3, param4);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "unaligned-roundtrip2",
            wasmer::Function::new_native(
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
                      arg9: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let ptr1 = arg2;
                    let len1 = arg3;
                    let ptr2 = arg4;
                    let len2 = arg5;
                    let len6 = arg7;
                    let base6 = arg6;
                    let mut result6 = Vec::with_capacity(len6 as usize);
                    for i in 0..len6 {
                        let base = base6 + i * 8;
                        result6.push({
                            let load3 = _bc.load::<i32>(base + 0)?;
                            let load4 = _bc.load::<i32>(base + 4)?;
                            let ptr5 = load3;
                            let len5 = load4;
                            _bc.slice_str(ptr5, len5)?
                        });
                    }
                    let len10 = arg9;
                    let base10 = arg8;
                    let mut result10 = Vec::with_capacity(len10 as usize);
                    for i in 0..len10 {
                        let base = base10 + i * 8;
                        result10.push({
                            let load7 = _bc.load::<i32>(base + 0)?;
                            let load8 = _bc.load::<i32>(base + 4)?;
                            let ptr9 = load7;
                            let len9 = load8;
                            _bc.slice(ptr9, len9)?
                        });
                    }
                    let param0 = _bc.slice(ptr0, len0)?;
                    let param1 = _bc.slice(ptr1, len1)?;
                    let param2 = _bc.slice(ptr2, len2)?;
                    let param3 = result6;
                    let param4 = result10;
                    let host = &mut data_mut.data;
                    let result = host.unaligned_roundtrip2(param0, param1, param2, param3, param4);
                    let () = result;
                    Ok(())
                },
            ),
        );
        imports.register_namespace("imports", exports);
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
    use wit_bindgen_wasmer::rt::validate_flags;
    use wit_bindgen_wasmer::rt::RawMem;
    use wit_bindgen_wasmer::Le;
}
