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

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct UnionsData {}

    pub struct Unions {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<UnionsData>,
        func_add_one_distinguishable_num: wasmer::TypedFunction<(i32, i64), i32>,
        func_add_one_duplicated: wasmer::TypedFunction<(i32, i32), i32>,
        func_add_one_float: wasmer::TypedFunction<(i32, i64), i32>,
        func_add_one_integer: wasmer::TypedFunction<(i32, i64), i32>,
        func_canonical_abi_free: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        func_identify_distinguishable_num: wasmer::TypedFunction<(i32, i64), i32>,
        func_identify_duplicated: wasmer::TypedFunction<(i32, i32), i32>,
        func_identify_float: wasmer::TypedFunction<(i32, i64), i32>,
        func_identify_integer: wasmer::TypedFunction<(i32, i64), i32>,
        func_identify_text: wasmer::TypedFunction<(i32, i32, i32), i32>,
        func_replace_first_char: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        memory: wasmer::Memory,
    }
    impl Unions {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `UnionsData` which needs to be
        /// passed through to `Unions::new`.
        fn add_to_imports(
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<UnionsData> {
            let env = wasmer::FunctionEnv::new(&mut store, UnionsData::default());
            env
        }

        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        ///
        /// The `imports` provided will have intrinsics added to it
        /// automatically, so it's not necessary to call
        /// `add_to_imports` beforehand. This function will
        /// instantiate the `module` otherwise using `imports`, and
        /// both an instance of this structure and the underlying
        /// `wasmer::Instance` will be returned.
        pub fn instantiate(
            mut store: impl wasmer::AsStoreMut,
            module: &wasmer::Module,
            imports: &mut wasmer::Imports,
        ) -> anyhow::Result<(Self, wasmer::Instance)> {
            let env = Self::add_to_imports(&mut store, imports);
            let instance = wasmer::Instance::new(&mut store, module, &*imports)?;

            Ok((Self::new(store, &instance, env)?, instance))
        }

        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// and wrap them all up in the returned structure which can
        /// be used to interact with the wasm module.
        pub fn new(
            store: impl wasmer::AsStoreMut,
            _instance: &wasmer::Instance,
            env: wasmer::FunctionEnv<UnionsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_add_one_distinguishable_num = _instance
                .exports
                .get_typed_function(&store, "add-one-distinguishable-num")?;
            let func_add_one_duplicated = _instance
                .exports
                .get_typed_function(&store, "add-one-duplicated")?;
            let func_add_one_float = _instance
                .exports
                .get_typed_function(&store, "add-one-float")?;
            let func_add_one_integer = _instance
                .exports
                .get_typed_function(&store, "add-one-integer")?;
            let func_canonical_abi_free = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_free")?;
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_realloc")?;
            let func_identify_distinguishable_num = _instance
                .exports
                .get_typed_function(&store, "identify-distinguishable-num")?;
            let func_identify_duplicated = _instance
                .exports
                .get_typed_function(&store, "identify-duplicated")?;
            let func_identify_float = _instance
                .exports
                .get_typed_function(&store, "identify-float")?;
            let func_identify_integer = _instance
                .exports
                .get_typed_function(&store, "identify-integer")?;
            let func_identify_text = _instance
                .exports
                .get_typed_function(&store, "identify-text")?;
            let func_replace_first_char = _instance
                .exports
                .get_typed_function(&store, "replace-first-char")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Unions {
                func_add_one_distinguishable_num,
                func_add_one_duplicated,
                func_add_one_float,
                func_add_one_integer,
                func_canonical_abi_free,
                func_canonical_abi_realloc,
                func_identify_distinguishable_num,
                func_identify_duplicated,
                func_identify_float,
                func_identify_integer,
                func_identify_text,
                func_replace_first_char,
                memory,
                env,
            })
        }
        pub fn add_one_integer(
            &self,
            store: &mut wasmer::Store,
            num: AllIntegers,
        ) -> Result<AllIntegers, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let (result0_0, result0_1) = match num {
                AllIntegers::Bool(e) => (
                    0i32,
                    i64::from(match e {
                        true => 1,
                        false => 0,
                    }),
                ),
                AllIntegers::U8(e) => (1i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::U16(e) => (2i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::U32(e) => (3i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::U64(e) => (4i32, wit_bindgen_wasmer::rt::as_i64(e)),
                AllIntegers::I8(e) => (5i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::I16(e) => (6i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::I32(e) => (7i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::I64(e) => (8i32, wit_bindgen_wasmer::rt::as_i64(e)),
            };
            let result1 = self
                .func_add_one_integer
                .call(store, result0_0, result0_1)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result1 + 0)?;
            Ok(match i32::from(load2) {
                0 => AllIntegers::Bool({
                    let _memory_view = _memory.view(&store);
                    let load3 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result1 + 8)?;
                    match i32::from(load3) {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    }
                }),
                1 => AllIntegers::U8({
                    let _memory_view = _memory.view(&store);
                    let load4 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result1 + 8)?;
                    u8::try_from(i32::from(load4)).map_err(bad_int)?
                }),
                2 => AllIntegers::U16({
                    let _memory_view = _memory.view(&store);
                    let load5 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<u16>(result1 + 8)?;
                    u16::try_from(i32::from(load5)).map_err(bad_int)?
                }),
                3 => AllIntegers::U32({
                    let _memory_view = _memory.view(&store);
                    let load6 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result1 + 8)?;
                    load6 as u32
                }),
                4 => AllIntegers::U64({
                    let _memory_view = _memory.view(&store);
                    let load7 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i64>(result1 + 8)?;
                    load7 as u64
                }),
                5 => AllIntegers::I8({
                    let _memory_view = _memory.view(&store);
                    let load8 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i8>(result1 + 8)?;
                    i8::try_from(i32::from(load8)).map_err(bad_int)?
                }),
                6 => AllIntegers::I16({
                    let _memory_view = _memory.view(&store);
                    let load9 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i16>(result1 + 8)?;
                    i16::try_from(i32::from(load9)).map_err(bad_int)?
                }),
                7 => AllIntegers::I32({
                    let _memory_view = _memory.view(&store);
                    let load10 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result1 + 8)?;
                    load10
                }),
                8 => AllIntegers::I64({
                    let _memory_view = _memory.view(&store);
                    let load11 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i64>(result1 + 8)?;
                    load11
                }),
                _ => return Err(invalid_variant("AllIntegers")),
            })
        }
        pub fn add_one_float(
            &self,
            store: &mut wasmer::Store,
            num: AllFloats,
        ) -> Result<AllFloats, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let (result0_0, result0_1) = match num {
                AllFloats::F32(e) => (0i32, i64::from((e).to_bits())),
                AllFloats::F64(e) => (1i32, (e).to_bits() as i64),
            };
            let result1 = self.func_add_one_float.call(store, result0_0, result0_1)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result1 + 0)?;
            Ok(match i32::from(load2) {
                0 => AllFloats::F32({
                    let _memory_view = _memory.view(&store);
                    let load3 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<f32>(result1 + 8)?;
                    load3
                }),
                1 => AllFloats::F64({
                    let _memory_view = _memory.view(&store);
                    let load4 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<f64>(result1 + 8)?;
                    load4
                }),
                _ => return Err(invalid_variant("AllFloats")),
            })
        }
        pub fn replace_first_char(
            &self,
            store: &mut wasmer::Store,
            text: AllTextParam<'_>,
            letter: char,
        ) -> Result<AllTextResult, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let (result1_0, result1_1, result1_2) = match text {
                AllTextParam::Char(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e), 0i32),
                AllTextParam::String(e) => {
                    let vec0 = e;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec0.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store_many(ptr0, vec0.as_bytes())?;
                    (1i32, ptr0, vec0.len() as i32)
                }
            };
            let result2 = self.func_replace_first_char.call(
                store,
                result1_0,
                result1_1,
                result1_2,
                wit_bindgen_wasmer::rt::as_i32(letter),
            )?;
            let _memory_view = _memory.view(&store);
            let load3 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result2 + 0)?;
            Ok(match i32::from(load3) {
                0 => AllTextResult::Char({
                    let _memory_view = _memory.view(&store);
                    let load4 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result2 + 4)?;
                    char_from_i32(load4)?
                }),
                1 => AllTextResult::String({
                    let _memory_view = _memory.view(&store);
                    let load5 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result2 + 4)?;
                    let _memory_view = _memory.view(&store);
                    let load6 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result2 + 8)?;
                    let ptr7 = load5;
                    let len7 = load6;

                    let data7 = copy_slice(store, _memory, func_canonical_abi_free, ptr7, len7, 1)?;
                    String::from_utf8(data7)
                        .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                }),
                _ => return Err(invalid_variant("AllTextResult")),
            })
        }
        pub fn identify_integer(
            &self,
            store: &mut wasmer::Store,
            num: AllIntegers,
        ) -> Result<u8, wasmer::RuntimeError> {
            let (result0_0, result0_1) = match num {
                AllIntegers::Bool(e) => (
                    0i32,
                    i64::from(match e {
                        true => 1,
                        false => 0,
                    }),
                ),
                AllIntegers::U8(e) => (1i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::U16(e) => (2i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::U32(e) => (3i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::U64(e) => (4i32, wit_bindgen_wasmer::rt::as_i64(e)),
                AllIntegers::I8(e) => (5i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::I16(e) => (6i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::I32(e) => (7i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                AllIntegers::I64(e) => (8i32, wit_bindgen_wasmer::rt::as_i64(e)),
            };
            let result1 = self
                .func_identify_integer
                .call(store, result0_0, result0_1)?;
            Ok(u8::try_from(result1).map_err(bad_int)?)
        }
        pub fn identify_float(
            &self,
            store: &mut wasmer::Store,
            num: AllFloats,
        ) -> Result<u8, wasmer::RuntimeError> {
            let (result0_0, result0_1) = match num {
                AllFloats::F32(e) => (0i32, i64::from((e).to_bits())),
                AllFloats::F64(e) => (1i32, (e).to_bits() as i64),
            };
            let result1 = self.func_identify_float.call(store, result0_0, result0_1)?;
            Ok(u8::try_from(result1).map_err(bad_int)?)
        }
        pub fn identify_text(
            &self,
            store: &mut wasmer::Store,
            text: AllTextParam<'_>,
        ) -> Result<u8, wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let (result1_0, result1_1, result1_2) = match text {
                AllTextParam::Char(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e), 0i32),
                AllTextParam::String(e) => {
                    let vec0 = e;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec0.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store_many(ptr0, vec0.as_bytes())?;
                    (1i32, ptr0, vec0.len() as i32)
                }
            };
            let result2 = self
                .func_identify_text
                .call(store, result1_0, result1_1, result1_2)?;
            Ok(u8::try_from(result2).map_err(bad_int)?)
        }
        pub fn add_one_duplicated(
            &self,
            store: &mut wasmer::Store,
            num: DuplicatedS32,
        ) -> Result<DuplicatedS32, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let (result0_0, result0_1) = match num {
                DuplicatedS32::I320(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                DuplicatedS32::I321(e) => (1i32, wit_bindgen_wasmer::rt::as_i32(e)),
                DuplicatedS32::I322(e) => (2i32, wit_bindgen_wasmer::rt::as_i32(e)),
            };
            let result1 = self
                .func_add_one_duplicated
                .call(store, result0_0, result0_1)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result1 + 0)?;
            Ok(match i32::from(load2) {
                0 => DuplicatedS32::I320({
                    let _memory_view = _memory.view(&store);
                    let load3 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result1 + 4)?;
                    load3
                }),
                1 => DuplicatedS32::I321({
                    let _memory_view = _memory.view(&store);
                    let load4 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result1 + 4)?;
                    load4
                }),
                2 => DuplicatedS32::I322({
                    let _memory_view = _memory.view(&store);
                    let load5 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result1 + 4)?;
                    load5
                }),
                _ => return Err(invalid_variant("DuplicatedS32")),
            })
        }
        pub fn identify_duplicated(
            &self,
            store: &mut wasmer::Store,
            num: DuplicatedS32,
        ) -> Result<u8, wasmer::RuntimeError> {
            let (result0_0, result0_1) = match num {
                DuplicatedS32::I320(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                DuplicatedS32::I321(e) => (1i32, wit_bindgen_wasmer::rt::as_i32(e)),
                DuplicatedS32::I322(e) => (2i32, wit_bindgen_wasmer::rt::as_i32(e)),
            };
            let result1 = self
                .func_identify_duplicated
                .call(store, result0_0, result0_1)?;
            Ok(u8::try_from(result1).map_err(bad_int)?)
        }
        pub fn add_one_distinguishable_num(
            &self,
            store: &mut wasmer::Store,
            num: DistinguishableNum,
        ) -> Result<DistinguishableNum, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let (result0_0, result0_1) = match num {
                DistinguishableNum::F64(e) => (0i32, (e).to_bits() as i64),
                DistinguishableNum::I64(e) => (1i32, wit_bindgen_wasmer::rt::as_i64(e)),
            };
            let result1 = self
                .func_add_one_distinguishable_num
                .call(store, result0_0, result0_1)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result1 + 0)?;
            Ok(match i32::from(load2) {
                0 => DistinguishableNum::F64({
                    let _memory_view = _memory.view(&store);
                    let load3 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<f64>(result1 + 8)?;
                    load3
                }),
                1 => DistinguishableNum::I64({
                    let _memory_view = _memory.view(&store);
                    let load4 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i64>(result1 + 8)?;
                    load4
                }),
                _ => return Err(invalid_variant("DistinguishableNum")),
            })
        }
        pub fn identify_distinguishable_num(
            &self,
            store: &mut wasmer::Store,
            num: DistinguishableNum,
        ) -> Result<u8, wasmer::RuntimeError> {
            let (result0_0, result0_1) = match num {
                DistinguishableNum::F64(e) => (0i32, (e).to_bits() as i64),
                DistinguishableNum::I64(e) => (1i32, wit_bindgen_wasmer::rt::as_i64(e)),
            };
            let result1 = self
                .func_identify_distinguishable_num
                .call(store, result0_0, result0_1)?;
            Ok(u8::try_from(result1).map_err(bad_int)?)
        }
    }
    use core::convert::TryFrom;
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::char_from_i32;
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
