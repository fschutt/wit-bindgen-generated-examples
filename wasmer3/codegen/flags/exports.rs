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

    pub trait Flags: Sized + Send + Sync + 'static {
        fn roundtrip_flag1(&mut self, x: Flag1) -> Flag1;

        fn roundtrip_flag2(&mut self, x: Flag2) -> Flag2;

        fn roundtrip_flag4(&mut self, x: Flag4) -> Flag4;

        fn roundtrip_flag8(&mut self, x: Flag8) -> Flag8;

        fn roundtrip_flag16(&mut self, x: Flag16) -> Flag16;

        fn roundtrip_flag32(&mut self, x: Flag32) -> Flag32;

        fn roundtrip_flag64(&mut self, x: Flag64) -> Flag64;
    }
    pub struct LazyInitialized {
        memory: wasmer::Memory,
    }

    #[must_use = "The returned initializer function must be called
  with the instance and the store before starting the runtime"]
    pub fn add_to_imports<T>(
        store: &mut wasmer::Store,
        imports: &mut wasmer::Imports,
        data: T,
    ) -> impl FnOnce(&wasmer::Instance, &dyn wasmer::AsStoreRef) -> Result<(), anyhow::Error>
    where
        T: Flags,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Flags> {
            data: T,
            lazy: std::rc::Rc<OnceCell<LazyInitialized>>,
        }
        unsafe impl<T: Flags> Send for EnvWrapper<T> {}
        unsafe impl<T: Flags> Sync for EnvWrapper<T> {}
        let lazy = std::rc::Rc::new(OnceCell::new());
        let env = EnvWrapper {
            data,
            lazy: std::rc::Rc::clone(&lazy),
        };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
            "roundtrip-flag1",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = validate_flags(
                        0 | ((arg0 as u8) << 0),
                        Flag1::all().bits(),
                        "Flag1",
                        |bits| Flag1 { bits },
                    )?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_flag1(param0);
                    let flags0 = result;
                    Ok((flags0.bits >> 0) as i32)
                },
            ),
        );
        exports.insert(
            "roundtrip-flag2",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = validate_flags(
                        0 | ((arg0 as u8) << 0),
                        Flag2::all().bits(),
                        "Flag2",
                        |bits| Flag2 { bits },
                    )?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_flag2(param0);
                    let flags0 = result;
                    Ok((flags0.bits >> 0) as i32)
                },
            ),
        );
        exports.insert(
            "roundtrip-flag4",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = validate_flags(
                        0 | ((arg0 as u8) << 0),
                        Flag4::all().bits(),
                        "Flag4",
                        |bits| Flag4 { bits },
                    )?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_flag4(param0);
                    let flags0 = result;
                    Ok((flags0.bits >> 0) as i32)
                },
            ),
        );
        exports.insert(
            "roundtrip-flag8",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = validate_flags(
                        0 | ((arg0 as u8) << 0),
                        Flag8::all().bits(),
                        "Flag8",
                        |bits| Flag8 { bits },
                    )?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_flag8(param0);
                    let flags0 = result;
                    Ok((flags0.bits >> 0) as i32)
                },
            ),
        );
        exports.insert(
            "roundtrip-flag16",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = validate_flags(
                        0 | ((arg0 as u16) << 0),
                        Flag16::all().bits(),
                        "Flag16",
                        |bits| Flag16 { bits },
                    )?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_flag16(param0);
                    let flags0 = result;
                    Ok((flags0.bits >> 0) as i32)
                },
            ),
        );
        exports.insert(
            "roundtrip-flag32",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = validate_flags(
                        0 | ((arg0 as u32) << 0),
                        Flag32::all().bits(),
                        "Flag32",
                        |bits| Flag32 { bits },
                    )?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_flag32(param0);
                    let flags0 = result;
                    Ok((flags0.bits >> 0) as i32)
                },
            ),
        );
        exports.insert(
            "roundtrip-flag64",
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
                    let param0 = validate_flags(
                        0 | ((arg0 as u64) << 0) | ((arg1 as u64) << 32),
                        Flag64::all().bits(),
                        "Flag64",
                        |bits| Flag64 { bits },
                    )?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_flag64(param0);
                    let flags0 = result;
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store(
                        arg2 + 4,
                        wit_bindgen_wasmer::rt::as_i32((flags0.bits >> 32) as i32),
                    )?;
                    caller_memory.store(
                        arg2 + 0,
                        wit_bindgen_wasmer::rt::as_i32((flags0.bits >> 0) as i32),
                    )?;
                    Ok(())
                },
            ),
        );
        imports.register_namespace("flags", exports);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| {
            let memory = _instance.exports.get_memory("memory")?.clone();
            lazy.set(LazyInitialized { memory })
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
}
