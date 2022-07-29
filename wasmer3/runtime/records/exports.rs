#[allow(clippy::all)]
pub mod imports {
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
    pub trait Imports: Sized + Send + Sync + 'static {
        fn multiple_results(&mut self) -> (u8, u16);

        fn swap_tuple(&mut self, a: (u8, u32)) -> (u32, u8);

        fn roundtrip_flags1(&mut self, a: F1) -> F1;

        fn roundtrip_flags2(&mut self, a: F2) -> F2;

        fn roundtrip_flags3(
            &mut self,
            a: Flag8,
            b: Flag16,
            c: Flag32,
            d: Flag64,
        ) -> (Flag8, Flag16, Flag32, Flag64);

        fn roundtrip_record1(&mut self, a: R1) -> R1;

        fn tuple0(&mut self, a: ()) -> ();

        fn tuple1(&mut self, a: (u8,)) -> (u8,);
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
            "multiple-results",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let host = &mut data_mut.data;
                    let result = host.multiple_results();
                    let (t0_0, t0_1) = result;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(
                        arg0 + 0,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_0)) as u8,
                    )?;
                    caller_memory.store(
                        arg0 + 2,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_1)) as u16,
                    )?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "swap-tuple",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = (u8::try_from(arg0).map_err(bad_int)?, arg1 as u32);
                    let host = &mut data_mut.data;
                    let result = host.swap_tuple(param0);
                    let (t0_0, t0_1) = result;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(
                        arg2 + 0,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_0)),
                    )?;
                    caller_memory.store(
                        arg2 + 4,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_1)) as u8,
                    )?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "roundtrip-flags1",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 =
                        validate_flags(0 | ((arg0 as u8) << 0), F1::all().bits(), "F1", |bits| {
                            F1 { bits }
                        })?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_flags1(param0);
                    let flags0 = result;
                    Ok((flags0.bits >> 0) as i32)
                },
            ),
        );
        exports.insert(
            "roundtrip-flags2",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 =
                        validate_flags(0 | ((arg0 as u8) << 0), F2::all().bits(), "F2", |bits| {
                            F2 { bits }
                        })?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_flags2(param0);
                    let flags0 = result;
                    Ok((flags0.bits >> 0) as i32)
                },
            ),
        );
        exports.insert(
            "roundtrip-flags3",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32,
                      arg5: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = validate_flags(
                        0 | ((arg0 as u8) << 0),
                        Flag8::all().bits(),
                        "Flag8",
                        |bits| Flag8 { bits },
                    )?;
                    let param1 = validate_flags(
                        0 | ((arg1 as u16) << 0),
                        Flag16::all().bits(),
                        "Flag16",
                        |bits| Flag16 { bits },
                    )?;
                    let param2 = validate_flags(
                        0 | ((arg2 as u32) << 0),
                        Flag32::all().bits(),
                        "Flag32",
                        |bits| Flag32 { bits },
                    )?;
                    let param3 = validate_flags(
                        0 | ((arg3 as u64) << 0) | ((arg4 as u64) << 32),
                        Flag64::all().bits(),
                        "Flag64",
                        |bits| Flag64 { bits },
                    )?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_flags3(param0, param1, param2, param3);
                    let (t0_0, t0_1, t0_2, t0_3) = result;
                    let flags1 = t0_0;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(
                        arg5 + 0,
                        wit_bindgen_wasmer::rt::as_i32((flags1.bits >> 0) as i32) as u8,
                    )?;
                    let flags2 = t0_1;
                    caller_memory.store(
                        arg5 + 2,
                        wit_bindgen_wasmer::rt::as_i32((flags2.bits >> 0) as i32) as u16,
                    )?;
                    let flags3 = t0_2;
                    caller_memory.store(
                        arg5 + 4,
                        wit_bindgen_wasmer::rt::as_i32((flags3.bits >> 0) as i32),
                    )?;
                    let flags4 = t0_3;
                    caller_memory.store(
                        arg5 + 12,
                        wit_bindgen_wasmer::rt::as_i32((flags4.bits >> 32) as i32),
                    )?;
                    caller_memory.store(
                        arg5 + 8,
                        wit_bindgen_wasmer::rt::as_i32((flags4.bits >> 0) as i32),
                    )?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "roundtrip-record1",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = R1 {
                        a: u8::try_from(arg0).map_err(bad_int)?,
                        b: validate_flags(
                            0 | ((arg1 as u8) << 0),
                            F1::all().bits(),
                            "F1",
                            |bits| F1 { bits },
                        )?,
                    };
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_record1(param0);
                    let R1 { a: a0, b: b0 } = result;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(
                        arg2 + 0,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(a0)) as u8,
                    )?;
                    let flags1 = b0;
                    caller_memory.store(
                        arg2 + 1,
                        wit_bindgen_wasmer::rt::as_i32((flags1.bits >> 0) as i32) as u8,
                    )?;
                    Ok(())
                },
            ),
        );
        exports.insert(
    "tuple0",
    wasmer::Function::new_native(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<(), wasmer::RuntimeError> {
      let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
      let data_mut = store.data_mut();
      let param0 = ();
      let host = &mut data_mut.data;
      let result = host.tuple0(param0, );
      let () = result;
      Ok(())
    }
    ));
        exports.insert(
            "tuple1",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = (u8::try_from(arg0).map_err(bad_int)?,);
                    let host = &mut data_mut.data;
                    let result = host.tuple1(param0);
                    let (t0_0,) = result;
                    Ok(wit_bindgen_wasmer::rt::as_i32(t0_0))
                },
            ),
        );
        imports.register_namespace("imports", exports);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| {
            let memory = _instance.exports.get_memory("memory")?.clone();
            lazy.set(LazyInitialized { memory })
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
    use wit_bindgen_wasmer::rt::validate_flags;
    use wit_bindgen_wasmer::rt::RawMem;
}
