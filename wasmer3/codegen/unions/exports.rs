#[allow(clippy::all)]
pub mod unions {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    /// A union of all of the integral types
    #[derive(Clone, Copy)]
    pub enum AllIntegers {
        /// Bool is equivalent to a 1 bit integer
        /// and is treated that way in some languages
        Bool(bool),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        I8(i8),
        I16(i16),
        I32(i32),
        I64(i64),
    }
    impl core::fmt::Debug for AllIntegers {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                AllIntegers::Bool(e) => f.debug_tuple("AllIntegers::Bool").field(e).finish(),
                AllIntegers::U8(e) => f.debug_tuple("AllIntegers::U8").field(e).finish(),
                AllIntegers::U16(e) => f.debug_tuple("AllIntegers::U16").field(e).finish(),
                AllIntegers::U32(e) => f.debug_tuple("AllIntegers::U32").field(e).finish(),
                AllIntegers::U64(e) => f.debug_tuple("AllIntegers::U64").field(e).finish(),
                AllIntegers::I8(e) => f.debug_tuple("AllIntegers::I8").field(e).finish(),
                AllIntegers::I16(e) => f.debug_tuple("AllIntegers::I16").field(e).finish(),
                AllIntegers::I32(e) => f.debug_tuple("AllIntegers::I32").field(e).finish(),
                AllIntegers::I64(e) => f.debug_tuple("AllIntegers::I64").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum AllFloats {
        F32(f32),
        F64(f64),
    }
    impl core::fmt::Debug for AllFloats {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                AllFloats::F32(e) => f.debug_tuple("AllFloats::F32").field(e).finish(),
                AllFloats::F64(e) => f.debug_tuple("AllFloats::F64").field(e).finish(),
            }
        }
    }
    #[derive(Clone)]
    pub enum AllTextParam<'a> {
        Char(char),
        String(&'a str),
    }
    impl<'a> core::fmt::Debug for AllTextParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                AllTextParam::Char(e) => f.debug_tuple("AllTextParam::Char").field(e).finish(),
                AllTextParam::String(e) => f.debug_tuple("AllTextParam::String").field(e).finish(),
            }
        }
    }
    #[derive(Clone)]
    pub enum AllTextResult {
        Char(char),
        String(String),
    }
    impl core::fmt::Debug for AllTextResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                AllTextResult::Char(e) => f.debug_tuple("AllTextResult::Char").field(e).finish(),
                AllTextResult::String(e) => {
                    f.debug_tuple("AllTextResult::String").field(e).finish()
                }
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum DuplicatedS32 {
        /// The first s32
        I320(i32),
        /// The second s32
        I321(i32),
        /// The third s32
        I322(i32),
    }
    impl core::fmt::Debug for DuplicatedS32 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                DuplicatedS32::I320(e) => f.debug_tuple("DuplicatedS32::I320").field(e).finish(),
                DuplicatedS32::I321(e) => f.debug_tuple("DuplicatedS32::I321").field(e).finish(),
                DuplicatedS32::I322(e) => f.debug_tuple("DuplicatedS32::I322").field(e).finish(),
            }
        }
    }
    /// A type containing numeric types that are distinct in most languages
    #[derive(Clone, Copy)]
    pub enum DistinguishableNum {
        /// A Floating Point Number
        F64(f64),
        /// A Signed Integer
        I64(i64),
    }
    impl core::fmt::Debug for DistinguishableNum {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                DistinguishableNum::F64(e) => {
                    f.debug_tuple("DistinguishableNum::F64").field(e).finish()
                }
                DistinguishableNum::I64(e) => {
                    f.debug_tuple("DistinguishableNum::I64").field(e).finish()
                }
            }
        }
    }
    pub trait Unions: Sized + Send + Sync + 'static {
        fn add_one_integer(&mut self, num: AllIntegers) -> AllIntegers;

        fn add_one_float(&mut self, num: AllFloats) -> AllFloats;

        fn replace_first_char(&mut self, text: AllTextParam<'_>, letter: char) -> AllTextResult;

        fn identify_integer(&mut self, num: AllIntegers) -> u8;

        fn identify_float(&mut self, num: AllFloats) -> u8;

        fn identify_text(&mut self, text: AllTextParam<'_>) -> u8;

        fn add_one_duplicated(&mut self, num: DuplicatedS32) -> DuplicatedS32;

        fn identify_duplicated(&mut self, num: DuplicatedS32) -> u8;

        fn add_one_distinguishable_num(&mut self, num: DistinguishableNum) -> DistinguishableNum;

        fn identify_distinguishable_num(&mut self, num: DistinguishableNum) -> u8;
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
        T: Unions,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Unions> {
            data: T,
            lazy: std::rc::Rc<OnceCell<LazyInitialized>>,
        }
        unsafe impl<T: Unions> Send for EnvWrapper<T> {}
        unsafe impl<T: Unions> Sync for EnvWrapper<T> {}
        let lazy = std::rc::Rc::new(OnceCell::new());
        let env = EnvWrapper {
            data,
            lazy: std::rc::Rc::clone(&lazy),
        };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
            "add-one-integer",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i64,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => AllIntegers::Bool(match arg1 as i32 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        }),
                        1 => AllIntegers::U8(u8::try_from(arg1 as i32).map_err(bad_int)?),
                        2 => AllIntegers::U16(u16::try_from(arg1 as i32).map_err(bad_int)?),
                        3 => AllIntegers::U32(arg1 as i32 as u32),
                        4 => AllIntegers::U64(arg1 as u64),
                        5 => AllIntegers::I8(i8::try_from(arg1 as i32).map_err(bad_int)?),
                        6 => AllIntegers::I16(i16::try_from(arg1 as i32).map_err(bad_int)?),
                        7 => AllIntegers::I32(arg1 as i32),
                        8 => AllIntegers::I64(arg1),
                        _ => return Err(invalid_variant("AllIntegers")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.add_one_integer(param0);
                    match result {
                        AllIntegers::Bool(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg2 + 8,
                                wit_bindgen_wasmer::rt::as_i32(match e {
                                    true => 1,
                                    false => 0,
                                }) as u8,
                            )?;
                        }
                        AllIntegers::U8(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg2 + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e))
                                    as u8,
                            )?;
                        }
                        AllIntegers::U16(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(2i32) as u8)?;
                            caller_memory.store(
                                arg2 + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e))
                                    as u16,
                            )?;
                        }
                        AllIntegers::U32(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(3i32) as u8)?;
                            caller_memory.store(
                                arg2 + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        AllIntegers::U64(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(4i32) as u8)?;
                            caller_memory.store(
                                arg2 + 8,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(e)),
                            )?;
                        }
                        AllIntegers::I8(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(5i32) as u8)?;
                            caller_memory.store(
                                arg2 + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e))
                                    as u8,
                            )?;
                        }
                        AllIntegers::I16(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(6i32) as u8)?;
                            caller_memory.store(
                                arg2 + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e))
                                    as u16,
                            )?;
                        }
                        AllIntegers::I32(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(7i32) as u8)?;
                            caller_memory.store(
                                arg2 + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        AllIntegers::I64(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(8i32) as u8)?;
                            caller_memory.store(
                                arg2 + 8,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(e)),
                            )?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "add-one-float",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i64,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => AllFloats::F32(f32::from_bits(arg1 as u32)),
                        1 => AllFloats::F64(f64::from_bits(arg1 as u64)),
                        _ => return Err(invalid_variant("AllFloats")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.add_one_float(param0);
                    match result {
                        AllFloats::F32(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(arg2 + 8, wit_bindgen_wasmer::rt::as_f32(e))?;
                        }
                        AllFloats::F64(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(arg2 + 8, wit_bindgen_wasmer::rt::as_f64(e))?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "replace-first-char",
            wasmer::Function::new_typed_with_env(
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
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
                    });
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => AllTextParam::Char(char_from_i32(arg1)?),
                        1 => AllTextParam::String({
                            let ptr0 = arg1;
                            let len0 = arg2;
                            _bc.slice_str(ptr0, len0)?
                        }),
                        _ => return Err(invalid_variant("AllTextParam")),
                    };
                    let param1 = char_from_i32(arg3)?;
                    let host = &mut data_mut.data;
                    let result = host.replace_first_char(param0, param1);
                    match result {
                        AllTextResult::Char(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg4 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg4 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        AllTextResult::String(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg4 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            let vec1 = e;
                            let ptr1 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec1.len() as i32,
                            )?;
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory.store_many(ptr1, vec1.as_bytes())?;
                            caller_memory.store(
                                arg4 + 8,
                                wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32),
                            )?;
                            caller_memory.store(arg4 + 4, wit_bindgen_wasmer::rt::as_i32(ptr1))?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "identify-integer",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i64|
                      -> Result<i32, wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => AllIntegers::Bool(match arg1 as i32 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        }),
                        1 => AllIntegers::U8(u8::try_from(arg1 as i32).map_err(bad_int)?),
                        2 => AllIntegers::U16(u16::try_from(arg1 as i32).map_err(bad_int)?),
                        3 => AllIntegers::U32(arg1 as i32 as u32),
                        4 => AllIntegers::U64(arg1 as u64),
                        5 => AllIntegers::I8(i8::try_from(arg1 as i32).map_err(bad_int)?),
                        6 => AllIntegers::I16(i16::try_from(arg1 as i32).map_err(bad_int)?),
                        7 => AllIntegers::I32(arg1 as i32),
                        8 => AllIntegers::I64(arg1),
                        _ => return Err(invalid_variant("AllIntegers")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.identify_integer(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "identify-float",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i64|
                      -> Result<i32, wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => AllFloats::F32(f32::from_bits(arg1 as u32)),
                        1 => AllFloats::F64(f64::from_bits(arg1 as u64)),
                        _ => return Err(invalid_variant("AllFloats")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.identify_float(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "identify-text",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
                    });
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => AllTextParam::Char(char_from_i32(arg1)?),
                        1 => AllTextParam::String({
                            let ptr0 = arg1;
                            let len0 = arg2;
                            _bc.slice_str(ptr0, len0)?
                        }),
                        _ => return Err(invalid_variant("AllTextParam")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.identify_text(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "add-one-duplicated",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => DuplicatedS32::I320(arg1),
                        1 => DuplicatedS32::I321(arg1),
                        2 => DuplicatedS32::I322(arg1),
                        _ => return Err(invalid_variant("DuplicatedS32")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.add_one_duplicated(param0);
                    match result {
                        DuplicatedS32::I320(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg2 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        DuplicatedS32::I321(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg2 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        DuplicatedS32::I322(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(2i32) as u8)?;
                            caller_memory.store(
                                arg2 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "identify-duplicated",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => DuplicatedS32::I320(arg1),
                        1 => DuplicatedS32::I321(arg1),
                        2 => DuplicatedS32::I322(arg1),
                        _ => return Err(invalid_variant("DuplicatedS32")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.identify_duplicated(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "add-one-distinguishable-num",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i64,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => DistinguishableNum::F64(f64::from_bits(arg1 as u64)),
                        1 => DistinguishableNum::I64(arg1),
                        _ => return Err(invalid_variant("DistinguishableNum")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.add_one_distinguishable_num(param0);
                    match result {
                        DistinguishableNum::F64(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(arg2 + 8, wit_bindgen_wasmer::rt::as_f64(e))?;
                        }
                        DistinguishableNum::I64(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg2 + 8,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(e)),
                            )?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "identify-distinguishable-num",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i64|
                      -> Result<i32, wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => DistinguishableNum::F64(f64::from_bits(arg1 as u64)),
                        1 => DistinguishableNum::I64(arg1),
                        _ => return Err(invalid_variant("DistinguishableNum")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.identify_distinguishable_num(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        imports.register_namespace("unions", exports);
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
}
