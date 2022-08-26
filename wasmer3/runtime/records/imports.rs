#[allow(clippy::all)]
pub mod exports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct F1: u8 {const A = 1 << 0;
        const B = 1 << 1;
      }
    }

    impl core::fmt::Display for F1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("F1(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct F2: u8 {const C = 1 << 0;
        const D = 1 << 1;
        const E = 1 << 2;
      }
    }

    impl core::fmt::Display for F2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("F2(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct F8: u8 {const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
        const B4 = 1 << 4;
        const B5 = 1 << 5;
        const B6 = 1 << 6;
        const B7 = 1 << 7;
      }
    }

    impl core::fmt::Display for F8 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("F8(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct F16: u16 {const B0 = 1 << 0;
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
      }
    }

    impl core::fmt::Display for F16 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("F16(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct F32: u32 {const B0 = 1 << 0;
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

    impl core::fmt::Display for F32 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("F32(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct F64: u64 {const B0 = 1 << 0;
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

    impl core::fmt::Display for F64 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("F64(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct R1 {
        pub a: u8,
        pub b: F1,
    }
    impl core::fmt::Debug for R1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("R1")
                .field("a", &self.a)
                .field("b", &self.b)
                .finish()
        }
    }

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ExportsData {}

    pub struct Exports {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<ExportsData>,
        func_multiple_results: wasmer::TypedFunction<(), i32>,
        func_roundtrip_flags1: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_flags2: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_flags3: wasmer::TypedFunction<(i32, i32, i32, i32, i32), i32>,
        func_roundtrip_record1: wasmer::TypedFunction<(i32, i32), i32>,
        func_swap_tuple: wasmer::TypedFunction<(i32, i32), i32>,
        func_test_imports: wasmer::TypedFunction<(), ()>,
        func_tuple0: wasmer::TypedFunction<(), ()>,
        func_tuple1: wasmer::TypedFunction<i32, i32>,
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
            let func_multiple_results = _instance
                .exports
                .get_typed_function(&store, "multiple-results")?;
            let func_roundtrip_flags1 = _instance
                .exports
                .get_typed_function(&store, "roundtrip-flags1")?;
            let func_roundtrip_flags2 = _instance
                .exports
                .get_typed_function(&store, "roundtrip-flags2")?;
            let func_roundtrip_flags3 = _instance
                .exports
                .get_typed_function(&store, "roundtrip-flags3")?;
            let func_roundtrip_record1 = _instance
                .exports
                .get_typed_function(&store, "roundtrip-record1")?;
            let func_swap_tuple = _instance.exports.get_typed_function(&store, "swap-tuple")?;
            let func_test_imports = _instance
                .exports
                .get_typed_function(&store, "test-imports")?;
            let func_tuple0 = _instance.exports.get_typed_function(&store, "tuple0")?;
            let func_tuple1 = _instance.exports.get_typed_function(&store, "tuple1")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Exports {
                func_multiple_results,
                func_roundtrip_flags1,
                func_roundtrip_flags2,
                func_roundtrip_flags3,
                func_roundtrip_record1,
                func_swap_tuple,
                func_test_imports,
                func_tuple0,
                func_tuple1,
                memory,
                env,
            })
        }
        pub fn test_imports(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_test_imports.call(store)?;
            Ok(())
        }
        pub fn multiple_results(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<(u8, u16), wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_multiple_results.call(store)?;
            let _memory_view = _memory.view(&store);
            let load1 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result0 + 0)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<u16>(result0 + 2)?;
            Ok((
                u8::try_from(i32::from(load1)).map_err(bad_int)?,
                u16::try_from(i32::from(load2)).map_err(bad_int)?,
            ))
        }
        pub fn swap_tuple(
            &self,
            store: &mut wasmer::Store,
            a: (u8, u32),
        ) -> Result<(u32, u8), wasmer::RuntimeError> {
            let _memory = &self.memory;
            let (t0_0, t0_1) = a;
            let result1 = self.func_swap_tuple.call(
                store,
                wit_bindgen_wasmer::rt::as_i32(t0_0),
                wit_bindgen_wasmer::rt::as_i32(t0_1),
            )?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result1 + 0)?;
            let _memory_view = _memory.view(&store);
            let load3 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result1 + 4)?;
            Ok((
                load2 as u32,
                u8::try_from(i32::from(load3)).map_err(bad_int)?,
            ))
        }
        pub fn roundtrip_flags1(
            &self,
            store: &mut wasmer::Store,
            a: F1,
        ) -> Result<F1, wasmer::RuntimeError> {
            let flags0 = a;
            let result1 = self
                .func_roundtrip_flags1
                .call(store, (flags0.bits >> 0) as i32)?;
            Ok(validate_flags(
                0 | ((result1 as u8) << 0),
                F1::all().bits(),
                "F1",
                |bits| F1 { bits },
            )?)
        }
        pub fn roundtrip_flags2(
            &self,
            store: &mut wasmer::Store,
            a: F2,
        ) -> Result<F2, wasmer::RuntimeError> {
            let flags0 = a;
            let result1 = self
                .func_roundtrip_flags2
                .call(store, (flags0.bits >> 0) as i32)?;
            Ok(validate_flags(
                0 | ((result1 as u8) << 0),
                F2::all().bits(),
                "F2",
                |bits| F2 { bits },
            )?)
        }
        pub fn roundtrip_flags3(
            &self,
            store: &mut wasmer::Store,
            a: F8,
            b: F16,
            c: F32,
            d: F64,
        ) -> Result<(F8, F16, F32, F64), wasmer::RuntimeError> {
            let _memory = &self.memory;
            let flags0 = a;
            let flags1 = b;
            let flags2 = c;
            let flags3 = d;
            let result4 = self.func_roundtrip_flags3.call(
                store,
                (flags0.bits >> 0) as i32,
                (flags1.bits >> 0) as i32,
                (flags2.bits >> 0) as i32,
                (flags3.bits >> 0) as i32,
                (flags3.bits >> 32) as i32,
            )?;
            let _memory_view = _memory.view(&store);
            let load5 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result4 + 0)?;
            let _memory_view = _memory.view(&store);
            let load6 = unsafe { _memory_view.data_unchecked_mut() }.load::<u16>(result4 + 2)?;
            let _memory_view = _memory.view(&store);
            let load7 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result4 + 4)?;
            let _memory_view = _memory.view(&store);
            let load8 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result4 + 8)?;
            let _memory_view = _memory.view(&store);
            let load9 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result4 + 12)?;
            Ok((
                validate_flags(
                    0 | ((i32::from(load5) as u8) << 0),
                    F8::all().bits(),
                    "F8",
                    |bits| F8 { bits },
                )?,
                validate_flags(
                    0 | ((i32::from(load6) as u16) << 0),
                    F16::all().bits(),
                    "F16",
                    |bits| F16 { bits },
                )?,
                validate_flags(
                    0 | ((load7 as u32) << 0),
                    F32::all().bits(),
                    "F32",
                    |bits| F32 { bits },
                )?,
                validate_flags(
                    0 | ((load8 as u64) << 0) | ((load9 as u64) << 32),
                    F64::all().bits(),
                    "F64",
                    |bits| F64 { bits },
                )?,
            ))
        }
        pub fn roundtrip_record1(
            &self,
            store: &mut wasmer::Store,
            a: R1,
        ) -> Result<R1, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let R1 { a: a0, b: b0 } = a;
            let flags1 = b0;
            let result2 = self.func_roundtrip_record1.call(
                store,
                wit_bindgen_wasmer::rt::as_i32(a0),
                (flags1.bits >> 0) as i32,
            )?;
            let _memory_view = _memory.view(&store);
            let load3 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result2 + 0)?;
            let _memory_view = _memory.view(&store);
            let load4 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result2 + 1)?;
            Ok(R1 {
                a: u8::try_from(i32::from(load3)).map_err(bad_int)?,
                b: validate_flags(
                    0 | ((i32::from(load4) as u8) << 0),
                    F1::all().bits(),
                    "F1",
                    |bits| F1 { bits },
                )?,
            })
        }
        pub fn tuple0(&self, store: &mut wasmer::Store, a: ()) -> Result<(), wasmer::RuntimeError> {
            let () = a;
            self.func_tuple0.call(store)?;
            Ok(())
        }
        pub fn tuple1(
            &self,
            store: &mut wasmer::Store,
            a: (u8,),
        ) -> Result<(u8,), wasmer::RuntimeError> {
            let (t0_0,) = a;
            let result1 = self
                .func_tuple1
                .call(store, wit_bindgen_wasmer::rt::as_i32(t0_0))?;
            Ok((u8::try_from(result1).map_err(bad_int)?,))
        }
    }
    use core::convert::TryFrom;
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::validate_flags;
    use wit_bindgen_wasmer::rt::RawMem;
}
