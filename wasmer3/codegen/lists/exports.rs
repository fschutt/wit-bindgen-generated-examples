#[allow(clippy::all)]
pub mod lists {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[derive(Clone)]
    pub struct SomeRecordParam<'a> {
        pub x: &'a str,
        pub y: OtherRecordParam<'a>,
        pub z: Vec<OtherRecordParam<'a>>,
        pub c1: u32,
        pub c2: u64,
        pub c3: i32,
        pub c4: i64,
    }
    impl<'a> core::fmt::Debug for SomeRecordParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("SomeRecordParam")
                .field("x", &self.x)
                .field("y", &self.y)
                .field("z", &self.z)
                .field("c1", &self.c1)
                .field("c2", &self.c2)
                .field("c3", &self.c3)
                .field("c4", &self.c4)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct SomeRecordResult {
        pub x: String,
        pub y: OtherRecordResult,
        pub z: Vec<OtherRecordResult>,
        pub c1: u32,
        pub c2: u64,
        pub c3: i32,
        pub c4: i64,
    }
    impl core::fmt::Debug for SomeRecordResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("SomeRecordResult")
                .field("x", &self.x)
                .field("y", &self.y)
                .field("z", &self.z)
                .field("c1", &self.c1)
                .field("c2", &self.c2)
                .field("c3", &self.c3)
                .field("c4", &self.c4)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct OtherRecordParam<'a> {
        pub a1: u32,
        pub a2: u64,
        pub a3: i32,
        pub a4: i64,
        pub b: &'a str,
        pub c: &'a [u8],
    }
    impl<'a> core::fmt::Debug for OtherRecordParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("OtherRecordParam")
                .field("a1", &self.a1)
                .field("a2", &self.a2)
                .field("a3", &self.a3)
                .field("a4", &self.a4)
                .field("b", &self.b)
                .field("c", &self.c)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct OtherRecordResult {
        pub a1: u32,
        pub a2: u64,
        pub a3: i32,
        pub a4: i64,
        pub b: String,
        pub c: Vec<u8>,
    }
    impl core::fmt::Debug for OtherRecordResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("OtherRecordResult")
                .field("a1", &self.a1)
                .field("a2", &self.a2)
                .field("a3", &self.a3)
                .field("a4", &self.a4)
                .field("b", &self.b)
                .field("c", &self.c)
                .finish()
        }
    }
    #[derive(Clone)]
    pub enum SomeVariant<'a> {
        A(&'a str),
        B,
        C(u32),
        D(Vec<OtherVariantParam<'a>>),
    }
    impl<'a> core::fmt::Debug for SomeVariant<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                SomeVariant::A(e) => f.debug_tuple("SomeVariant::A").field(e).finish(),
                SomeVariant::B => f.debug_tuple("SomeVariant::B").finish(),
                SomeVariant::C(e) => f.debug_tuple("SomeVariant::C").field(e).finish(),
                SomeVariant::D(e) => f.debug_tuple("SomeVariant::D").field(e).finish(),
            }
        }
    }
    #[derive(Clone)]
    pub enum OtherVariantParam<'a> {
        A,
        B(u32),
        C(&'a str),
    }
    impl<'a> core::fmt::Debug for OtherVariantParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                OtherVariantParam::A => f.debug_tuple("OtherVariantParam::A").finish(),
                OtherVariantParam::B(e) => f.debug_tuple("OtherVariantParam::B").field(e).finish(),
                OtherVariantParam::C(e) => f.debug_tuple("OtherVariantParam::C").field(e).finish(),
            }
        }
    }
    #[derive(Clone)]
    pub enum OtherVariantResult {
        A,
        B(u32),
        C(String),
    }
    impl core::fmt::Debug for OtherVariantResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                OtherVariantResult::A => f.debug_tuple("OtherVariantResult::A").finish(),
                OtherVariantResult::B(e) => {
                    f.debug_tuple("OtherVariantResult::B").field(e).finish()
                }
                OtherVariantResult::C(e) => {
                    f.debug_tuple("OtherVariantResult::C").field(e).finish()
                }
            }
        }
    }
    pub type LoadStoreAllSizesParam<'a> = Vec<(
        &'a str,
        u8,
        i8,
        u16,
        i16,
        u32,
        i32,
        u64,
        i64,
        f32,
        f64,
        char,
    )>;
    pub type LoadStoreAllSizesResult =
        Vec<(String, u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char)>;
    pub trait Lists: Sized + Send + Sync + 'static {
        fn list_u8_param(&mut self, x: &[u8]) -> ();

        fn list_u16_param(&mut self, x: &[Le<u16>]) -> ();

        fn list_u32_param(&mut self, x: &[Le<u32>]) -> ();

        fn list_u64_param(&mut self, x: &[Le<u64>]) -> ();

        fn list_s8_param(&mut self, x: &[i8]) -> ();

        fn list_s16_param(&mut self, x: &[Le<i16>]) -> ();

        fn list_s32_param(&mut self, x: &[Le<i32>]) -> ();

        fn list_s64_param(&mut self, x: &[Le<i64>]) -> ();

        fn list_float32_param(&mut self, x: &[Le<f32>]) -> ();

        fn list_float64_param(&mut self, x: &[Le<f64>]) -> ();

        fn list_u8_ret(&mut self) -> Vec<u8>;

        fn list_u16_ret(&mut self) -> Vec<u16>;

        fn list_u32_ret(&mut self) -> Vec<u32>;

        fn list_u64_ret(&mut self) -> Vec<u64>;

        fn list_s8_ret(&mut self) -> Vec<i8>;

        fn list_s16_ret(&mut self) -> Vec<i16>;

        fn list_s32_ret(&mut self) -> Vec<i32>;

        fn list_s64_ret(&mut self) -> Vec<i64>;

        fn list_float32_ret(&mut self) -> Vec<f32>;

        fn list_float64_ret(&mut self) -> Vec<f64>;

        fn tuple_list(&mut self, x: &[(u8, i8)]) -> Vec<(i64, u32)>;

        fn string_list_arg(&mut self, a: Vec<&str>) -> ();

        fn string_list_ret(&mut self) -> Vec<String>;

        fn tuple_string_list(&mut self, x: Vec<(u8, &str)>) -> Vec<(String, u8)>;

        fn string_list(&mut self, x: Vec<&str>) -> Vec<String>;

        fn record_list(&mut self, x: Vec<SomeRecordParam<'_>>) -> Vec<OtherRecordResult>;

        fn record_list_reverse(&mut self, x: Vec<OtherRecordParam<'_>>) -> Vec<SomeRecordResult>;

        fn variant_list(&mut self, x: Vec<SomeVariant<'_>>) -> Vec<OtherVariantResult>;

        fn load_store_everything(
            &mut self,
            a: LoadStoreAllSizesParam<'_>,
        ) -> LoadStoreAllSizesResult;
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
        T: Lists,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Lists> {
            data: T,
            lazy: std::rc::Rc<OnceCell<LazyInitialized>>,
        }
        unsafe impl<T: Lists> Send for EnvWrapper<T> {}
        unsafe impl<T: Lists> Sync for EnvWrapper<T> {}
        let lazy = std::rc::Rc::new(OnceCell::new());
        let env = EnvWrapper {
            data,
            lazy: std::rc::Rc::clone(&lazy),
        };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
            "list-u8-param",
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
                    let result = host.list_u8_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-u16-param",
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
                    let result = host.list_u16_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-u32-param",
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
                    let result = host.list_u32_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-u64-param",
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
                    let result = host.list_u64_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-s8-param",
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
                    let result = host.list_s8_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-s16-param",
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
                    let result = host.list_s16_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-s32-param",
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
                    let result = host.list_s32_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-s64-param",
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
                    let result = host.list_s64_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-float32-param",
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
                    let result = host.list_float32_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-float64-param",
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
                    let result = host.list_float64_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-u8-ret",
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
                    let result = host.list_u8_ret();
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
            "list-u16-ret",
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
                    let result = host.list_u16_ret();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        2,
                        (vec0.len() as i32) * 2,
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
            "list-u32-ret",
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
                    let result = host.list_u32_ret();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        (vec0.len() as i32) * 4,
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
            "list-u64-ret",
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
                    let result = host.list_u64_ret();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        8,
                        (vec0.len() as i32) * 8,
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
            "list-s8-ret",
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
                    let result = host.list_s8_ret();
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
            "list-s16-ret",
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
                    let result = host.list_s16_ret();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        2,
                        (vec0.len() as i32) * 2,
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
            "list-s32-ret",
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
                    let result = host.list_s32_ret();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        (vec0.len() as i32) * 4,
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
            "list-s64-ret",
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
                    let result = host.list_s64_ret();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        8,
                        (vec0.len() as i32) * 8,
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
            "list-float32-ret",
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
                    let result = host.list_float32_ret();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        (vec0.len() as i32) * 4,
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
            "list-float64-ret",
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
                    let result = host.list_float64_ret();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        8,
                        (vec0.len() as i32) * 8,
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
            "tuple-list",
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
                    let result = host.tuple_list(param0);
                    let vec1 = result;
                    let ptr1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        8,
                        (vec1.len() as i32) * 16,
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
            "string-list-arg",
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
                    let result = host.string_list_arg(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "string-list-ret",
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
                    let result = host.string_list_ret();
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
            "tuple-string-list",
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
                    let len4 = arg1;
                    let base4 = arg0;
                    let mut result4 = Vec::with_capacity(len4 as usize);
                    for i in 0..len4 {
                        let base = base4 + i * 12;
                        result4.push({
                            let load0 = _bc.load::<u8>(base + 0)?;
                            let load1 = _bc.load::<i32>(base + 4)?;
                            let load2 = _bc.load::<i32>(base + 8)?;
                            let ptr3 = load1;
                            let len3 = load2;
                            (
                                u8::try_from(i32::from(load0)).map_err(bad_int)?,
                                _bc.slice_str(ptr3, len3)?,
                            )
                        });
                    }
                    let param0 = result4;
                    let host = &mut data_mut.data;
                    let result = host.tuple_string_list(param0);
                    let vec7 = result;
                    let len7 = vec7.len() as i32;
                    let result7 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        len7 * 12,
                    )?;
                    for (i, e) in vec7.into_iter().enumerate() {
                        let base = result7 + (i as i32) * 12;
                        {
                            let (t5_0, t5_1) = e;
                            let vec6 = t5_0;
                            let ptr6 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec6.len() as i32,
                            )?;
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory.store_many(ptr6, vec6.as_bytes())?;
                            caller_memory.store(
                                base + 4,
                                wit_bindgen_wasmer::rt::as_i32(vec6.len() as i32),
                            )?;
                            caller_memory.store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr6))?;
                            caller_memory.store(
                                base + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t5_1))
                                    as u8,
                            )?;
                        }
                    }
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(arg2 + 4, wit_bindgen_wasmer::rt::as_i32(len7))?;
                    caller_memory.store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(result7))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "string-list",
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
                    let result = host.string_list(param0);
                    let vec5 = result;
                    let len5 = vec5.len() as i32;
                    let result5 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        len5 * 8,
                    )?;
                    for (i, e) in vec5.into_iter().enumerate() {
                        let base = result5 + (i as i32) * 8;
                        {
                            let vec4 = e;
                            let ptr4 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec4.len() as i32,
                            )?;
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory.store_many(ptr4, vec4.as_bytes())?;
                            caller_memory.store(
                                base + 4,
                                wit_bindgen_wasmer::rt::as_i32(vec4.len() as i32),
                            )?;
                            caller_memory.store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr4))?;
                        }
                    }
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(arg2 + 4, wit_bindgen_wasmer::rt::as_i32(len5))?;
                    caller_memory.store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(result5))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "record-list",
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
                    let len30 = arg1;
                    let base30 = arg0;
                    let mut result30 = Vec::with_capacity(len30 as usize);
                    for i in 0..len30 {
                        let base = base30 + i * 96;
                        result30.push({
                            let load0 = _bc.load::<i32>(base + 0)?;
                            let load1 = _bc.load::<i32>(base + 4)?;
                            let ptr2 = load0;
                            let len2 = load1;
                            let load3 = _bc.load::<i32>(base + 8)?;
                            let load4 = _bc.load::<i64>(base + 16)?;
                            let load5 = _bc.load::<i32>(base + 24)?;
                            let load6 = _bc.load::<i64>(base + 32)?;
                            let load7 = _bc.load::<i32>(base + 40)?;
                            let load8 = _bc.load::<i32>(base + 44)?;
                            let ptr9 = load7;
                            let len9 = load8;
                            let load10 = _bc.load::<i32>(base + 48)?;
                            let load11 = _bc.load::<i32>(base + 52)?;
                            let ptr12 = load10;
                            let len12 = load11;
                            let load13 = _bc.load::<i32>(base + 56)?;
                            let load14 = _bc.load::<i32>(base + 60)?;
                            let len25 = load14;
                            let base25 = load13;
                            let mut result25 = Vec::with_capacity(len25 as usize);
                            for i in 0..len25 {
                                let base = base25 + i * 48;
                                result25.push({
                                    let load15 = _bc.load::<i32>(base + 0)?;
                                    let load16 = _bc.load::<i64>(base + 8)?;
                                    let load17 = _bc.load::<i32>(base + 16)?;
                                    let load18 = _bc.load::<i64>(base + 24)?;
                                    let load19 = _bc.load::<i32>(base + 32)?;
                                    let load20 = _bc.load::<i32>(base + 36)?;
                                    let ptr21 = load19;
                                    let len21 = load20;
                                    let load22 = _bc.load::<i32>(base + 40)?;
                                    let load23 = _bc.load::<i32>(base + 44)?;
                                    let ptr24 = load22;
                                    let len24 = load23;
                                    OtherRecordParam {
                                        a1: load15 as u32,
                                        a2: load16 as u64,
                                        a3: load17,
                                        a4: load18,
                                        b: _bc.slice_str(ptr21, len21)?,
                                        c: _bc.slice(ptr24, len24)?,
                                    }
                                });
                            }
                            let load26 = _bc.load::<i32>(base + 64)?;
                            let load27 = _bc.load::<i64>(base + 72)?;
                            let load28 = _bc.load::<i32>(base + 80)?;
                            let load29 = _bc.load::<i64>(base + 88)?;
                            SomeRecordParam {
                                x: _bc.slice_str(ptr2, len2)?,
                                y: OtherRecordParam {
                                    a1: load3 as u32,
                                    a2: load4 as u64,
                                    a3: load5,
                                    a4: load6,
                                    b: _bc.slice_str(ptr9, len9)?,
                                    c: _bc.slice(ptr12, len12)?,
                                },
                                z: result25,
                                c1: load26 as u32,
                                c2: load27 as u64,
                                c3: load28,
                                c4: load29,
                            }
                        });
                    }
                    let param0 = result30;
                    let host = &mut data_mut.data;
                    let result = host.record_list(param0);
                    let vec34 = result;
                    let len34 = vec34.len() as i32;
                    let result34 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        8,
                        len34 * 48,
                    )?;
                    for (i, e) in vec34.into_iter().enumerate() {
                        let base = result34 + (i as i32) * 48;
                        {
                            let OtherRecordResult {
                                a1: a131,
                                a2: a231,
                                a3: a331,
                                a4: a431,
                                b: b31,
                                c: c31,
                            } = e;
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory.store(
                                base + 0,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    a131,
                                )),
                            )?;
                            caller_memory.store(
                                base + 8,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(
                                    a231,
                                )),
                            )?;
                            caller_memory.store(
                                base + 16,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    a331,
                                )),
                            )?;
                            caller_memory.store(
                                base + 24,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(
                                    a431,
                                )),
                            )?;
                            let vec32 = b31;
                            let ptr32 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec32.len() as i32,
                            )?;
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory.store_many(ptr32, vec32.as_bytes())?;
                            caller_memory.store(
                                base + 36,
                                wit_bindgen_wasmer::rt::as_i32(vec32.len() as i32),
                            )?;
                            caller_memory
                                .store(base + 32, wit_bindgen_wasmer::rt::as_i32(ptr32))?;
                            let vec33 = c31;
                            let ptr33 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                (vec33.len() as i32) * 1,
                            )?;
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory.store_many(ptr33, &vec33)?;
                            caller_memory.store(
                                base + 44,
                                wit_bindgen_wasmer::rt::as_i32(vec33.len() as i32),
                            )?;
                            caller_memory
                                .store(base + 40, wit_bindgen_wasmer::rt::as_i32(ptr33))?;
                        }
                    }
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(arg2 + 4, wit_bindgen_wasmer::rt::as_i32(len34))?;
                    caller_memory.store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(result34))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "record-list-reverse",
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
                    let len10 = arg1;
                    let base10 = arg0;
                    let mut result10 = Vec::with_capacity(len10 as usize);
                    for i in 0..len10 {
                        let base = base10 + i * 48;
                        result10.push({
                            let load0 = _bc.load::<i32>(base + 0)?;
                            let load1 = _bc.load::<i64>(base + 8)?;
                            let load2 = _bc.load::<i32>(base + 16)?;
                            let load3 = _bc.load::<i64>(base + 24)?;
                            let load4 = _bc.load::<i32>(base + 32)?;
                            let load5 = _bc.load::<i32>(base + 36)?;
                            let ptr6 = load4;
                            let len6 = load5;
                            let load7 = _bc.load::<i32>(base + 40)?;
                            let load8 = _bc.load::<i32>(base + 44)?;
                            let ptr9 = load7;
                            let len9 = load8;
                            OtherRecordParam {
                                a1: load0 as u32,
                                a2: load1 as u64,
                                a3: load2,
                                a4: load3,
                                b: _bc.slice_str(ptr6, len6)?,
                                c: _bc.slice(ptr9, len9)?,
                            }
                        });
                    }
                    let param0 = result10;
                    let host = &mut data_mut.data;
                    let result = host.record_list_reverse(param0);
                    let vec20 = result;
                    let len20 = vec20.len() as i32;
                    let result20 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        8,
                        len20 * 96,
                    )?;
                    for (i, e) in vec20.into_iter().enumerate() {
                        let base = result20 + (i as i32) * 96;
                        {
                            let SomeRecordResult {
                                x: x11,
                                y: y11,
                                z: z11,
                                c1: c111,
                                c2: c211,
                                c3: c311,
                                c4: c411,
                            } = e;
                            let vec12 = x11;
                            let ptr12 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec12.len() as i32,
                            )?;
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory.store_many(ptr12, vec12.as_bytes())?;
                            caller_memory.store(
                                base + 4,
                                wit_bindgen_wasmer::rt::as_i32(vec12.len() as i32),
                            )?;
                            caller_memory.store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr12))?;
                            let OtherRecordResult {
                                a1: a113,
                                a2: a213,
                                a3: a313,
                                a4: a413,
                                b: b13,
                                c: c13,
                            } = y11;
                            caller_memory.store(
                                base + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    a113,
                                )),
                            )?;
                            caller_memory.store(
                                base + 16,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(
                                    a213,
                                )),
                            )?;
                            caller_memory.store(
                                base + 24,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    a313,
                                )),
                            )?;
                            caller_memory.store(
                                base + 32,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(
                                    a413,
                                )),
                            )?;
                            let vec14 = b13;
                            let ptr14 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec14.len() as i32,
                            )?;
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory.store_many(ptr14, vec14.as_bytes())?;
                            caller_memory.store(
                                base + 44,
                                wit_bindgen_wasmer::rt::as_i32(vec14.len() as i32),
                            )?;
                            caller_memory
                                .store(base + 40, wit_bindgen_wasmer::rt::as_i32(ptr14))?;
                            let vec15 = c13;
                            let ptr15 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                (vec15.len() as i32) * 1,
                            )?;
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory.store_many(ptr15, &vec15)?;
                            caller_memory.store(
                                base + 52,
                                wit_bindgen_wasmer::rt::as_i32(vec15.len() as i32),
                            )?;
                            caller_memory
                                .store(base + 48, wit_bindgen_wasmer::rt::as_i32(ptr15))?;
                            let vec19 = z11;
                            let len19 = vec19.len() as i32;
                            let result19 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                8,
                                len19 * 48,
                            )?;
                            for (i, e) in vec19.into_iter().enumerate() {
                                let base = result19 + (i as i32) * 48;
                                {
                                    let OtherRecordResult {
                                        a1: a116,
                                        a2: a216,
                                        a3: a316,
                                        a4: a416,
                                        b: b16,
                                        c: c16,
                                    } = e;
                                    let caller_memory = unsafe {
                                        _memory.data_unchecked_mut(&store.as_store_ref())
                                    };
                                    caller_memory.store(
                                        base + 0,
                                        wit_bindgen_wasmer::rt::as_i32(
                                            wit_bindgen_wasmer::rt::as_i32(a116),
                                        ),
                                    )?;
                                    caller_memory.store(
                                        base + 8,
                                        wit_bindgen_wasmer::rt::as_i64(
                                            wit_bindgen_wasmer::rt::as_i64(a216),
                                        ),
                                    )?;
                                    caller_memory.store(
                                        base + 16,
                                        wit_bindgen_wasmer::rt::as_i32(
                                            wit_bindgen_wasmer::rt::as_i32(a316),
                                        ),
                                    )?;
                                    caller_memory.store(
                                        base + 24,
                                        wit_bindgen_wasmer::rt::as_i64(
                                            wit_bindgen_wasmer::rt::as_i64(a416),
                                        ),
                                    )?;
                                    let vec17 = b16;
                                    let ptr17 = func_canonical_abi_realloc.call(
                                        &mut store.as_store_mut(),
                                        0,
                                        0,
                                        1,
                                        vec17.len() as i32,
                                    )?;
                                    let caller_memory = unsafe {
                                        _memory.data_unchecked_mut(&store.as_store_ref())
                                    };
                                    caller_memory.store_many(ptr17, vec17.as_bytes())?;
                                    caller_memory.store(
                                        base + 36,
                                        wit_bindgen_wasmer::rt::as_i32(vec17.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(base + 32, wit_bindgen_wasmer::rt::as_i32(ptr17))?;
                                    let vec18 = c16;
                                    let ptr18 = func_canonical_abi_realloc.call(
                                        &mut store.as_store_mut(),
                                        0,
                                        0,
                                        1,
                                        (vec18.len() as i32) * 1,
                                    )?;
                                    let caller_memory = unsafe {
                                        _memory.data_unchecked_mut(&store.as_store_ref())
                                    };
                                    caller_memory.store_many(ptr18, &vec18)?;
                                    caller_memory.store(
                                        base + 44,
                                        wit_bindgen_wasmer::rt::as_i32(vec18.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(base + 40, wit_bindgen_wasmer::rt::as_i32(ptr18))?;
                                }
                            }
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(base + 60, wit_bindgen_wasmer::rt::as_i32(len19))?;
                            caller_memory
                                .store(base + 56, wit_bindgen_wasmer::rt::as_i32(result19))?;
                            caller_memory.store(
                                base + 64,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    c111,
                                )),
                            )?;
                            caller_memory.store(
                                base + 72,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(
                                    c211,
                                )),
                            )?;
                            caller_memory.store(
                                base + 80,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    c311,
                                )),
                            )?;
                            caller_memory.store(
                                base + 88,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(
                                    c411,
                                )),
                            )?;
                        }
                    }
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(arg2 + 4, wit_bindgen_wasmer::rt::as_i32(len20))?;
                    caller_memory.store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(result20))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "variant-list",
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
                    let len13 = arg1;
                    let base13 = arg0;
                    let mut result13 = Vec::with_capacity(len13 as usize);
                    for i in 0..len13 {
                        let base = base13 + i * 12;
                        result13.push({
                            let load0 = _bc.load::<u8>(base + 0)?;
                            match i32::from(load0) {
                                0 => SomeVariant::A({
                                    let load1 = _bc.load::<i32>(base + 4)?;
                                    let load2 = _bc.load::<i32>(base + 8)?;
                                    let ptr3 = load1;
                                    let len3 = load2;
                                    _bc.slice_str(ptr3, len3)?
                                }),
                                1 => SomeVariant::B,
                                2 => SomeVariant::C({
                                    let load4 = _bc.load::<i32>(base + 4)?;
                                    load4 as u32
                                }),
                                3 => SomeVariant::D({
                                    let load5 = _bc.load::<i32>(base + 4)?;
                                    let load6 = _bc.load::<i32>(base + 8)?;
                                    let len12 = load6;
                                    let base12 = load5;
                                    let mut result12 = Vec::with_capacity(len12 as usize);
                                    for i in 0..len12 {
                                        let base = base12 + i * 12;
                                        result12.push({
                                            let load7 = _bc.load::<u8>(base + 0)?;
                                            match i32::from(load7) {
                                                0 => OtherVariantParam::A,
                                                1 => OtherVariantParam::B({
                                                    let load8 = _bc.load::<i32>(base + 4)?;
                                                    load8 as u32
                                                }),
                                                2 => OtherVariantParam::C({
                                                    let load9 = _bc.load::<i32>(base + 4)?;
                                                    let load10 = _bc.load::<i32>(base + 8)?;
                                                    let ptr11 = load9;
                                                    let len11 = load10;
                                                    _bc.slice_str(ptr11, len11)?
                                                }),
                                                _ => {
                                                    return Err(invalid_variant(
                                                        "OtherVariantParam",
                                                    ))
                                                }
                                            }
                                        });
                                    }
                                    result12
                                }),
                                _ => return Err(invalid_variant("SomeVariant")),
                            }
                        });
                    }
                    let param0 = result13;
                    let host = &mut data_mut.data;
                    let result = host.variant_list(param0);
                    let vec15 = result;
                    let len15 = vec15.len() as i32;
                    let result15 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        len15 * 12,
                    )?;
                    for (i, e) in vec15.into_iter().enumerate() {
                        let base = result15 + (i as i32) * 12;
                        {
                            match e {
                                OtherVariantResult::A => {
                                    let e = ();
                                    {
                                        let caller_memory = unsafe {
                                            _memory.data_unchecked_mut(&store.as_store_ref())
                                        };
                                        caller_memory.store(
                                            base + 0,
                                            wit_bindgen_wasmer::rt::as_i32(0i32) as u8,
                                        )?;
                                        let () = e;
                                    }
                                }
                                OtherVariantResult::B(e) => {
                                    let caller_memory = unsafe {
                                        _memory.data_unchecked_mut(&store.as_store_ref())
                                    };
                                    caller_memory.store(
                                        base + 0,
                                        wit_bindgen_wasmer::rt::as_i32(1i32) as u8,
                                    )?;
                                    caller_memory.store(
                                        base + 4,
                                        wit_bindgen_wasmer::rt::as_i32(
                                            wit_bindgen_wasmer::rt::as_i32(e),
                                        ),
                                    )?;
                                }
                                OtherVariantResult::C(e) => {
                                    let caller_memory = unsafe {
                                        _memory.data_unchecked_mut(&store.as_store_ref())
                                    };
                                    caller_memory.store(
                                        base + 0,
                                        wit_bindgen_wasmer::rt::as_i32(2i32) as u8,
                                    )?;
                                    let vec14 = e;
                                    let ptr14 = func_canonical_abi_realloc.call(
                                        &mut store.as_store_mut(),
                                        0,
                                        0,
                                        1,
                                        vec14.len() as i32,
                                    )?;
                                    let caller_memory = unsafe {
                                        _memory.data_unchecked_mut(&store.as_store_ref())
                                    };
                                    caller_memory.store_many(ptr14, vec14.as_bytes())?;
                                    caller_memory.store(
                                        base + 8,
                                        wit_bindgen_wasmer::rt::as_i32(vec14.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(base + 4, wit_bindgen_wasmer::rt::as_i32(ptr14))?;
                                }
                            };
                        }
                    }
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(arg2 + 4, wit_bindgen_wasmer::rt::as_i32(len15))?;
                    caller_memory.store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(result15))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "load-store-everything",
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
                    let len14 = arg1;
                    let base14 = arg0;
                    let mut result14 = Vec::with_capacity(len14 as usize);
                    for i in 0..len14 {
                        let base = base14 + i * 64;
                        result14.push({
                            let load0 = _bc.load::<i32>(base + 0)?;
                            let load1 = _bc.load::<i32>(base + 4)?;
                            let ptr2 = load0;
                            let len2 = load1;
                            let load3 = _bc.load::<u8>(base + 8)?;
                            let load4 = _bc.load::<i8>(base + 9)?;
                            let load5 = _bc.load::<u16>(base + 10)?;
                            let load6 = _bc.load::<i16>(base + 12)?;
                            let load7 = _bc.load::<i32>(base + 16)?;
                            let load8 = _bc.load::<i32>(base + 20)?;
                            let load9 = _bc.load::<i64>(base + 24)?;
                            let load10 = _bc.load::<i64>(base + 32)?;
                            let load11 = _bc.load::<f32>(base + 40)?;
                            let load12 = _bc.load::<f64>(base + 48)?;
                            let load13 = _bc.load::<i32>(base + 56)?;
                            (
                                _bc.slice_str(ptr2, len2)?,
                                u8::try_from(i32::from(load3)).map_err(bad_int)?,
                                i8::try_from(i32::from(load4)).map_err(bad_int)?,
                                u16::try_from(i32::from(load5)).map_err(bad_int)?,
                                i16::try_from(i32::from(load6)).map_err(bad_int)?,
                                load7 as u32,
                                load8,
                                load9 as u64,
                                load10,
                                load11,
                                load12,
                                char_from_i32(load13)?,
                            )
                        });
                    }
                    let param0 = result14;
                    let host = &mut data_mut.data;
                    let result = host.load_store_everything(param0);
                    let vec17 = result;
                    let len17 = vec17.len() as i32;
                    let result17 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        8,
                        len17 * 64,
                    )?;
                    for (i, e) in vec17.into_iter().enumerate() {
                        let base = result17 + (i as i32) * 64;
                        {
                            let (
                                t15_0,
                                t15_1,
                                t15_2,
                                t15_3,
                                t15_4,
                                t15_5,
                                t15_6,
                                t15_7,
                                t15_8,
                                t15_9,
                                t15_10,
                                t15_11,
                            ) = e;
                            let vec16 = t15_0;
                            let ptr16 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec16.len() as i32,
                            )?;
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory.store_many(ptr16, vec16.as_bytes())?;
                            caller_memory.store(
                                base + 4,
                                wit_bindgen_wasmer::rt::as_i32(vec16.len() as i32),
                            )?;
                            caller_memory.store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr16))?;
                            caller_memory.store(
                                base + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t15_1,
                                )) as u8,
                            )?;
                            caller_memory.store(
                                base + 9,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t15_2,
                                )) as u8,
                            )?;
                            caller_memory.store(
                                base + 10,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t15_3,
                                )) as u16,
                            )?;
                            caller_memory.store(
                                base + 12,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t15_4,
                                )) as u16,
                            )?;
                            caller_memory.store(
                                base + 16,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t15_5,
                                )),
                            )?;
                            caller_memory.store(
                                base + 20,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t15_6,
                                )),
                            )?;
                            caller_memory.store(
                                base + 24,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(
                                    t15_7,
                                )),
                            )?;
                            caller_memory.store(
                                base + 32,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(
                                    t15_8,
                                )),
                            )?;
                            caller_memory
                                .store(base + 40, wit_bindgen_wasmer::rt::as_f32(t15_9))?;
                            caller_memory
                                .store(base + 48, wit_bindgen_wasmer::rt::as_f64(t15_10))?;
                            caller_memory.store(
                                base + 56,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t15_11,
                                )),
                            )?;
                        }
                    }
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(arg2 + 4, wit_bindgen_wasmer::rt::as_i32(len17))?;
                    caller_memory.store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(result17))?;
                    Ok(())
                },
            ),
        );
        imports.register_namespace("lists", exports);
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
    use core::convert::TryFrom;
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::once_cell::unsync::OnceCell;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::char_from_i32;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
    use wit_bindgen_wasmer::Le;
}
