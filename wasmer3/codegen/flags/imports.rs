#[allow(clippy::all)]
pub mod flags {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct Flag1: u8 {const B0 = 1 << 0;
      }
    }

    impl core::fmt::Display for Flag1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("Flag1(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct Flag2: u8 {const B0 = 1 << 0;
        const B1 = 1 << 1;
      }
    }

    impl core::fmt::Display for Flag2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("Flag2(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct Flag4: u8 {const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
      }
    }

    impl core::fmt::Display for Flag4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("Flag4(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct Flag8: u8 {const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
        const B4 = 1 << 4;
        const B5 = 1 << 5;
        const B6 = 1 << 6;
        const B7 = 1 << 7;
      }
    }

    impl core::fmt::Display for Flag8 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("Flag8(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

    wit_bindgen_wasmer::bitflags::bitflags! {
      pub struct Flag16: u16 {const B0 = 1 << 0;
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

    impl core::fmt::Display for Flag16 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("Flag16(")?;
            core::fmt::Debug::fmt(self, f)?;
            f.write_str(" (0x")?;
            core::fmt::LowerHex::fmt(&self.bits, f)?;
            f.write_str("))")?;
            Ok(())
        }
    }

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

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct FlagsData {}

    pub struct Flags {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<FlagsData>,
        func_roundtrip_flag1: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_flag16: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_flag2: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_flag32: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_flag4: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_flag64: wasmer::TypedFunction<(i32, i32), i32>,
        func_roundtrip_flag8: wasmer::TypedFunction<i32, i32>,
        memory: wasmer::Memory,
    }
    impl Flags {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `FlagsData` which needs to be
        /// passed through to `Flags::new`.
        fn add_to_imports(
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<FlagsData> {
            let env = wasmer::FunctionEnv::new(&mut store, FlagsData::default());
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
            env: wasmer::FunctionEnv<FlagsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_roundtrip_flag1 = _instance
                .exports
                .get_typed_function(&store, "roundtrip-flag1")?;
            let func_roundtrip_flag16 = _instance
                .exports
                .get_typed_function(&store, "roundtrip-flag16")?;
            let func_roundtrip_flag2 = _instance
                .exports
                .get_typed_function(&store, "roundtrip-flag2")?;
            let func_roundtrip_flag32 = _instance
                .exports
                .get_typed_function(&store, "roundtrip-flag32")?;
            let func_roundtrip_flag4 = _instance
                .exports
                .get_typed_function(&store, "roundtrip-flag4")?;
            let func_roundtrip_flag64 = _instance
                .exports
                .get_typed_function(&store, "roundtrip-flag64")?;
            let func_roundtrip_flag8 = _instance
                .exports
                .get_typed_function(&store, "roundtrip-flag8")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Flags {
                func_roundtrip_flag1,
                func_roundtrip_flag16,
                func_roundtrip_flag2,
                func_roundtrip_flag32,
                func_roundtrip_flag4,
                func_roundtrip_flag64,
                func_roundtrip_flag8,
                memory,
                env,
            })
        }
        pub fn roundtrip_flag1(
            &self,
            store: &mut wasmer::Store,
            x: Flag1,
        ) -> Result<Flag1, wasmer::RuntimeError> {
            let flags0 = x;
            let result1 = self
                .func_roundtrip_flag1
                .call(store, (flags0.bits >> 0) as i32)?;
            Ok(validate_flags(
                0 | ((result1 as u8) << 0),
                Flag1::all().bits(),
                "Flag1",
                |bits| Flag1 { bits },
            )?)
        }
        pub fn roundtrip_flag2(
            &self,
            store: &mut wasmer::Store,
            x: Flag2,
        ) -> Result<Flag2, wasmer::RuntimeError> {
            let flags0 = x;
            let result1 = self
                .func_roundtrip_flag2
                .call(store, (flags0.bits >> 0) as i32)?;
            Ok(validate_flags(
                0 | ((result1 as u8) << 0),
                Flag2::all().bits(),
                "Flag2",
                |bits| Flag2 { bits },
            )?)
        }
        pub fn roundtrip_flag4(
            &self,
            store: &mut wasmer::Store,
            x: Flag4,
        ) -> Result<Flag4, wasmer::RuntimeError> {
            let flags0 = x;
            let result1 = self
                .func_roundtrip_flag4
                .call(store, (flags0.bits >> 0) as i32)?;
            Ok(validate_flags(
                0 | ((result1 as u8) << 0),
                Flag4::all().bits(),
                "Flag4",
                |bits| Flag4 { bits },
            )?)
        }
        pub fn roundtrip_flag8(
            &self,
            store: &mut wasmer::Store,
            x: Flag8,
        ) -> Result<Flag8, wasmer::RuntimeError> {
            let flags0 = x;
            let result1 = self
                .func_roundtrip_flag8
                .call(store, (flags0.bits >> 0) as i32)?;
            Ok(validate_flags(
                0 | ((result1 as u8) << 0),
                Flag8::all().bits(),
                "Flag8",
                |bits| Flag8 { bits },
            )?)
        }
        pub fn roundtrip_flag16(
            &self,
            store: &mut wasmer::Store,
            x: Flag16,
        ) -> Result<Flag16, wasmer::RuntimeError> {
            let flags0 = x;
            let result1 = self
                .func_roundtrip_flag16
                .call(store, (flags0.bits >> 0) as i32)?;
            Ok(validate_flags(
                0 | ((result1 as u16) << 0),
                Flag16::all().bits(),
                "Flag16",
                |bits| Flag16 { bits },
            )?)
        }
        pub fn roundtrip_flag32(
            &self,
            store: &mut wasmer::Store,
            x: Flag32,
        ) -> Result<Flag32, wasmer::RuntimeError> {
            let flags0 = x;
            let result1 = self
                .func_roundtrip_flag32
                .call(store, (flags0.bits >> 0) as i32)?;
            Ok(validate_flags(
                0 | ((result1 as u32) << 0),
                Flag32::all().bits(),
                "Flag32",
                |bits| Flag32 { bits },
            )?)
        }
        pub fn roundtrip_flag64(
            &self,
            store: &mut wasmer::Store,
            x: Flag64,
        ) -> Result<Flag64, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let flags0 = x;
            let result1 = self.func_roundtrip_flag64.call(
                store,
                (flags0.bits >> 0) as i32,
                (flags0.bits >> 32) as i32,
            )?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result1 + 0)?;
            let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result1 + 4)?;
            Ok(validate_flags(
                0 | ((load2 as u64) << 0) | ((load3 as u64) << 32),
                Flag64::all().bits(),
                "Flag64",
                |bits| Flag64 { bits },
            )?)
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::validate_flags;
    use wit_bindgen_wasmer::rt::RawMem;
}
