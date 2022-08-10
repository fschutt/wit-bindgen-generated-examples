#[allow(clippy::all)]
pub mod exports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum E1 {
        A,
        B,
    }
    impl core::fmt::Debug for E1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                E1::A => f.debug_tuple("E1::A").finish(),
                E1::B => f.debug_tuple("E1::B").finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C1 {
        A(i32),
        B(i64),
    }
    impl core::fmt::Debug for C1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C1::A(e) => f.debug_tuple("C1::A").field(e).finish(),
                C1::B(e) => f.debug_tuple("C1::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C2 {
        A(i32),
        B(f32),
    }
    impl core::fmt::Debug for C2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C2::A(e) => f.debug_tuple("C2::A").field(e).finish(),
                C2::B(e) => f.debug_tuple("C2::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C3 {
        A(i32),
        B(f64),
    }
    impl core::fmt::Debug for C3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C3::A(e) => f.debug_tuple("C3::A").field(e).finish(),
                C3::B(e) => f.debug_tuple("C3::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C4 {
        A(i64),
        B(f32),
    }
    impl core::fmt::Debug for C4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C4::A(e) => f.debug_tuple("C4::A").field(e).finish(),
                C4::B(e) => f.debug_tuple("C4::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C5 {
        A(i64),
        B(f64),
    }
    impl core::fmt::Debug for C5 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C5::A(e) => f.debug_tuple("C5::A").field(e).finish(),
                C5::B(e) => f.debug_tuple("C5::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C6 {
        A(f32),
        B(f64),
    }
    impl core::fmt::Debug for C6 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C6::A(e) => f.debug_tuple("C6::A").field(e).finish(),
                C6::B(e) => f.debug_tuple("C6::B").field(e).finish(),
            }
        }
    }
    pub type Casts = (C1, C2, C3, C4, C5, C6);
    #[derive(Clone, Copy)]
    pub enum Z1 {
        A(i32),
        B,
    }
    impl core::fmt::Debug for Z1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Z1::A(e) => f.debug_tuple("Z1::A").field(e).finish(),
                Z1::B => f.debug_tuple("Z1::B").finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Z2 {
        A(i64),
        B,
    }
    impl core::fmt::Debug for Z2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Z2::A(e) => f.debug_tuple("Z2::A").field(e).finish(),
                Z2::B => f.debug_tuple("Z2::B").finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Z3 {
        A(f32),
        B,
    }
    impl core::fmt::Debug for Z3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Z3::A(e) => f.debug_tuple("Z3::A").field(e).finish(),
                Z3::B => f.debug_tuple("Z3::B").finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Z4 {
        A(f64),
        B,
    }
    impl core::fmt::Debug for Z4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Z4::A(e) => f.debug_tuple("Z4::A").field(e).finish(),
                Z4::B => f.debug_tuple("Z4::B").finish(),
            }
        }
    }
    pub type Zeros = (Z1, Z2, Z3, Z4);
    pub type OptionTypedef = Option<u32>;
    pub type BoolTypedef = bool;
    pub type ResultTypedef = Result<u32, ()>;

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ExportsData {}

    pub struct Exports {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<ExportsData>,
        func_invert_bool: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_enum: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_option: wasmer::TypedFunction<(i32, f32), i32>,
        func_roundtrip_result: wasmer::TypedFunction<(i32, i32), i32>,
        func_test_imports: wasmer::TypedFunction<(), ()>,
        func_variant_casts: wasmer::TypedFunction<
            (i32, i64, i32, i32, i32, i64, i32, i64, i32, i64, i32, i64),
            i32,
        >,
        func_variant_typedefs: wasmer::TypedFunction<(i32, i32, i32, i32, i32), ()>,
        func_variant_zeros: wasmer::TypedFunction<(i32, i32, i32, i64, i32, f32, i32, f64), i32>,
        memory: wasmer::Memory,
    }
    impl Exports {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ExportsData` which needs to be
        /// passed through to `Exports::new`.
        fn add_to_imports(
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<ExportsData> {
            let env = wasmer::FunctionEnv::new(&mut store, ExportsData::default());
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
            env: wasmer::FunctionEnv<ExportsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_invert_bool = _instance
                .exports
                .get_typed_function(&store, "invert-bool")?;
            let func_roundtrip_enum = _instance
                .exports
                .get_typed_function(&store, "roundtrip-enum")?;
            let func_roundtrip_option = _instance
                .exports
                .get_typed_function(&store, "roundtrip-option")?;
            let func_roundtrip_result = _instance
                .exports
                .get_typed_function(&store, "roundtrip-result")?;
            let func_test_imports = _instance
                .exports
                .get_typed_function(&store, "test-imports")?;
            let func_variant_casts = _instance
                .exports
                .get_typed_function(&store, "variant-casts")?;
            let func_variant_typedefs = _instance
                .exports
                .get_typed_function(&store, "variant-typedefs")?;
            let func_variant_zeros = _instance
                .exports
                .get_typed_function(&store, "variant-zeros")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Exports {
                func_invert_bool,
                func_roundtrip_enum,
                func_roundtrip_option,
                func_roundtrip_result,
                func_test_imports,
                func_variant_casts,
                func_variant_typedefs,
                func_variant_zeros,
                memory,
                env,
            })
        }
        pub fn test_imports(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_test_imports.call(store)?;
            Ok(())
        }
        pub fn roundtrip_option(
            &self,
            store: &mut wasmer::Store,
            a: Option<f32>,
        ) -> Result<Option<u8>, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let (result0_0, result0_1) = match a {
                Some(e) => (1i32, e),
                None => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0.0f32)
                    }
                }
            };
            let result1 = self
                .func_roundtrip_option
                .call(store, result0_0, result0_1)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result1 + 0)?;
            Ok(match i32::from(load2) {
                0 => None,
                1 => Some({
                    let _memory_view = _memory.view(&store);
                    let load3 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result1 + 1)?;
                    u8::try_from(i32::from(load3)).map_err(bad_int)?
                }),
                _ => return Err(invalid_variant("option")),
            })
        }
        pub fn roundtrip_result(
            &self,
            store: &mut wasmer::Store,
            a: Result<u32, f32>,
        ) -> Result<Result<f64, u8>, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let (result0_0, result0_1) = match a {
                Ok(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                Err(e) => (1i32, (e).to_bits() as i32),
            };
            let result1 = self
                .func_roundtrip_result
                .call(store, result0_0, result0_1)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result1 + 0)?;
            Ok(match i32::from(load2) {
                0 => Ok({
                    let _memory_view = _memory.view(&store);
                    let load3 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<f64>(result1 + 8)?;
                    load3
                }),
                1 => Err({
                    let _memory_view = _memory.view(&store);
                    let load4 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result1 + 8)?;
                    u8::try_from(i32::from(load4)).map_err(bad_int)?
                }),
                _ => return Err(invalid_variant("expected")),
            })
        }
        pub fn roundtrip_enum(
            &self,
            store: &mut wasmer::Store,
            a: E1,
        ) -> Result<E1, wasmer::RuntimeError> {
            let result0 = self.func_roundtrip_enum.call(store, a as i32)?;
            Ok(match result0 {
                0 => E1::A,
                1 => E1::B,
                _ => return Err(invalid_variant("E1")),
            })
        }
        pub fn invert_bool(
            &self,
            store: &mut wasmer::Store,
            a: bool,
        ) -> Result<bool, wasmer::RuntimeError> {
            let result0 = self.func_invert_bool.call(
                store,
                match a {
                    true => 1,
                    false => 0,
                },
            )?;
            Ok(match result0 {
                0 => false,
                1 => true,
                _ => return Err(invalid_variant("bool")),
            })
        }
        pub fn variant_casts(
            &self,
            store: &mut wasmer::Store,
            a: Casts,
        ) -> Result<Casts, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let (t0_0, t0_1, t0_2, t0_3, t0_4, t0_5) = a;
            let (result1_0, result1_1) = match t0_0 {
                C1::A(e) => (0i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                C1::B(e) => (1i32, wit_bindgen_wasmer::rt::as_i64(e)),
            };
            let (result2_0, result2_1) = match t0_1 {
                C2::A(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                C2::B(e) => (1i32, (e).to_bits() as i32),
            };
            let (result3_0, result3_1) = match t0_2 {
                C3::A(e) => (0i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                C3::B(e) => (1i32, (e).to_bits() as i64),
            };
            let (result4_0, result4_1) = match t0_3 {
                C4::A(e) => (0i32, wit_bindgen_wasmer::rt::as_i64(e)),
                C4::B(e) => (1i32, i64::from((e).to_bits())),
            };
            let (result5_0, result5_1) = match t0_4 {
                C5::A(e) => (0i32, wit_bindgen_wasmer::rt::as_i64(e)),
                C5::B(e) => (1i32, (e).to_bits() as i64),
            };
            let (result6_0, result6_1) = match t0_5 {
                C6::A(e) => (0i32, i64::from((e).to_bits())),
                C6::B(e) => (1i32, (e).to_bits() as i64),
            };
            let result7 = self.func_variant_casts.call(
                store, result1_0, result1_1, result2_0, result2_1, result3_0, result3_1, result4_0,
                result4_1, result5_0, result5_1, result6_0, result6_1,
            )?;
            let _memory_view = _memory.view(&store);
            let load8 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result7 + 0)?;
            let _memory_view = _memory.view(&store);
            let load11 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result7 + 16)?;
            let _memory_view = _memory.view(&store);
            let load14 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result7 + 24)?;
            let _memory_view = _memory.view(&store);
            let load17 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result7 + 40)?;
            let _memory_view = _memory.view(&store);
            let load20 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result7 + 56)?;
            let _memory_view = _memory.view(&store);
            let load23 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result7 + 72)?;
            Ok((
                match i32::from(load8) {
                    0 => C1::A({
                        let _memory_view = _memory.view(&store);
                        let load9 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<i32>(result7 + 8)?;
                        load9
                    }),
                    1 => C1::B({
                        let _memory_view = _memory.view(&store);
                        let load10 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<i64>(result7 + 8)?;
                        load10
                    }),
                    _ => return Err(invalid_variant("C1")),
                },
                match i32::from(load11) {
                    0 => C2::A({
                        let _memory_view = _memory.view(&store);
                        let load12 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<i32>(result7 + 20)?;
                        load12
                    }),
                    1 => C2::B({
                        let _memory_view = _memory.view(&store);
                        let load13 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<f32>(result7 + 20)?;
                        load13
                    }),
                    _ => return Err(invalid_variant("C2")),
                },
                match i32::from(load14) {
                    0 => C3::A({
                        let _memory_view = _memory.view(&store);
                        let load15 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<i32>(result7 + 32)?;
                        load15
                    }),
                    1 => C3::B({
                        let _memory_view = _memory.view(&store);
                        let load16 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<f64>(result7 + 32)?;
                        load16
                    }),
                    _ => return Err(invalid_variant("C3")),
                },
                match i32::from(load17) {
                    0 => C4::A({
                        let _memory_view = _memory.view(&store);
                        let load18 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<i64>(result7 + 48)?;
                        load18
                    }),
                    1 => C4::B({
                        let _memory_view = _memory.view(&store);
                        let load19 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<f32>(result7 + 48)?;
                        load19
                    }),
                    _ => return Err(invalid_variant("C4")),
                },
                match i32::from(load20) {
                    0 => C5::A({
                        let _memory_view = _memory.view(&store);
                        let load21 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<i64>(result7 + 64)?;
                        load21
                    }),
                    1 => C5::B({
                        let _memory_view = _memory.view(&store);
                        let load22 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<f64>(result7 + 64)?;
                        load22
                    }),
                    _ => return Err(invalid_variant("C5")),
                },
                match i32::from(load23) {
                    0 => C6::A({
                        let _memory_view = _memory.view(&store);
                        let load24 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<f32>(result7 + 80)?;
                        load24
                    }),
                    1 => C6::B({
                        let _memory_view = _memory.view(&store);
                        let load25 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<f64>(result7 + 80)?;
                        load25
                    }),
                    _ => return Err(invalid_variant("C6")),
                },
            ))
        }
        pub fn variant_zeros(
            &self,
            store: &mut wasmer::Store,
            a: Zeros,
        ) -> Result<Zeros, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let (t0_0, t0_1, t0_2, t0_3) = a;
            let (result1_0, result1_1) = match t0_0 {
                Z1::A(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                Z1::B => {
                    let e = ();
                    {
                        let () = e;
                        (1i32, 0i32)
                    }
                }
            };
            let (result2_0, result2_1) = match t0_1 {
                Z2::A(e) => (0i32, wit_bindgen_wasmer::rt::as_i64(e)),
                Z2::B => {
                    let e = ();
                    {
                        let () = e;
                        (1i32, 0i64)
                    }
                }
            };
            let (result3_0, result3_1) = match t0_2 {
                Z3::A(e) => (0i32, e),
                Z3::B => {
                    let e = ();
                    {
                        let () = e;
                        (1i32, 0.0f32)
                    }
                }
            };
            let (result4_0, result4_1) = match t0_3 {
                Z4::A(e) => (0i32, e),
                Z4::B => {
                    let e = ();
                    {
                        let () = e;
                        (1i32, 0.0f64)
                    }
                }
            };
            let result5 = self.func_variant_zeros.call(
                store, result1_0, result1_1, result2_0, result2_1, result3_0, result3_1, result4_0,
                result4_1,
            )?;
            let _memory_view = _memory.view(&store);
            let load6 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result5 + 0)?;
            let _memory_view = _memory.view(&store);
            let load8 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result5 + 8)?;
            let _memory_view = _memory.view(&store);
            let load10 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result5 + 24)?;
            let _memory_view = _memory.view(&store);
            let load12 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result5 + 32)?;
            Ok((
                match i32::from(load6) {
                    0 => Z1::A({
                        let _memory_view = _memory.view(&store);
                        let load7 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<i32>(result5 + 4)?;
                        load7
                    }),
                    1 => Z1::B,
                    _ => return Err(invalid_variant("Z1")),
                },
                match i32::from(load8) {
                    0 => Z2::A({
                        let _memory_view = _memory.view(&store);
                        let load9 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<i64>(result5 + 16)?;
                        load9
                    }),
                    1 => Z2::B,
                    _ => return Err(invalid_variant("Z2")),
                },
                match i32::from(load10) {
                    0 => Z3::A({
                        let _memory_view = _memory.view(&store);
                        let load11 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<f32>(result5 + 28)?;
                        load11
                    }),
                    1 => Z3::B,
                    _ => return Err(invalid_variant("Z3")),
                },
                match i32::from(load12) {
                    0 => Z4::A({
                        let _memory_view = _memory.view(&store);
                        let load13 = unsafe { _memory_view.data_unchecked_mut() }
                            .load::<f64>(result5 + 40)?;
                        load13
                    }),
                    1 => Z4::B,
                    _ => return Err(invalid_variant("Z4")),
                },
            ))
        }
        pub fn variant_typedefs(
            &self,
            store: &mut wasmer::Store,
            a: OptionTypedef,
            b: BoolTypedef,
            c: ResultTypedef,
        ) -> Result<(), wasmer::RuntimeError> {
            let (result0_0, result0_1) = match a {
                Some(e) => (1i32, wit_bindgen_wasmer::rt::as_i32(e)),
                None => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0i32)
                    }
                }
            };
            let (result1_0, result1_1) = match c {
                Ok(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                Err(e) => {
                    let () = e;
                    (1i32, 0i32)
                }
            };
            self.func_variant_typedefs.call(
                store,
                result0_0,
                result0_1,
                match b {
                    true => 1,
                    false => 0,
                },
                result1_0,
                result1_1,
            )?;
            Ok(())
        }
    }
    use core::convert::TryFrom;
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
