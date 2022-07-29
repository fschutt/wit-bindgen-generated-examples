pub mod integers {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Integers: Sized + wasmer::WasmerEnv + 'static {
        fn a1(&mut self, x: u8) -> ();

        fn a2(&mut self, x: i8) -> ();

        fn a3(&mut self, x: u16) -> ();

        fn a4(&mut self, x: i16) -> ();

        fn a5(&mut self, x: u32) -> ();

        fn a6(&mut self, x: i32) -> ();

        fn a7(&mut self, x: u64) -> ();

        fn a8(&mut self, x: i64) -> ();

        fn a9(
            &mut self,
            p1: u8,
            p2: i8,
            p3: u16,
            p4: i16,
            p5: u32,
            p6: i32,
            p7: u64,
            p8: i64,
        ) -> ();

        fn r1(&mut self) -> u8;

        fn r2(&mut self) -> i8;

        fn r3(&mut self) -> u16;

        fn r4(&mut self) -> i16;

        fn r5(&mut self) -> u32;

        fn r6(&mut self) -> i32;

        fn r7(&mut self) -> u64;

        fn r8(&mut self) -> i64;

        fn pair_ret(&mut self) -> (i64, u8);
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: Integers,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Integers> {
            data: T,
            memory: wasmer::LazyInit<wasmer::Memory>,
        }
        unsafe impl<T: Integers> Send for EnvWrapper<T> {}
        unsafe impl<T: Integers> Sync for EnvWrapper<T> {}
        impl<T: Integers> wasmer::WasmerEnv for EnvWrapper<T> {
            fn init_with_instance(
                &mut self,
                instance: &wasmer::Instance,
            ) -> Result<(), wasmer::HostEnvInitError> {
                self.data.init_with_instance(instance)?;
                self.memory
                    .initialize(instance.exports.get_with_generics_weak("memory")?);
                Ok(())
            }
        }
        let env = std::sync::Arc::new(std::sync::Mutex::new(EnvWrapper {
            data,
            memory: wasmer::LazyInit::new(),
        }));
        let mut exports = wasmer::Exports::new();
        exports.insert(
            "a1",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = u8::try_from(arg0).map_err(bad_int)?;
                    let result = host.a1(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "a2",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = i8::try_from(arg0).map_err(bad_int)?;
                    let result = host.a2(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "a3",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = u16::try_from(arg0).map_err(bad_int)?;
                    let result = host.a3(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "a4",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = i16::try_from(arg0).map_err(bad_int)?;
                    let result = host.a4(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "a5",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = arg0 as u32;
                    let result = host.a5(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "a6",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = arg0;
                    let result = host.a6(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "a7",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i64|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = arg0 as u64;
                    let result = host.a7(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "a8",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i64|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = arg0;
                    let result = host.a8(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "a9",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32,
                      arg5: i32,
                      arg6: i64,
                      arg7: i64|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = u8::try_from(arg0).map_err(bad_int)?;
                    let param1 = i8::try_from(arg1).map_err(bad_int)?;
                    let param2 = u16::try_from(arg2).map_err(bad_int)?;
                    let param3 = i16::try_from(arg3).map_err(bad_int)?;
                    let param4 = arg4 as u32;
                    let param5 = arg5;
                    let param6 = arg6 as u64;
                    let param7 = arg7;
                    let result = host.a9(
                        param0, param1, param2, param3, param4, param5, param6, param7,
                    );
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert("r1", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.r1();
      Ok(wit_bindgen_wasmer::rt::as_i32(result))
    }));
        exports.insert("r2", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.r2();
      Ok(wit_bindgen_wasmer::rt::as_i32(result))
    }));
        exports.insert("r3", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.r3();
      Ok(wit_bindgen_wasmer::rt::as_i32(result))
    }));
        exports.insert("r4", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.r4();
      Ok(wit_bindgen_wasmer::rt::as_i32(result))
    }));
        exports.insert("r5", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.r5();
      Ok(wit_bindgen_wasmer::rt::as_i32(result))
    }));
        exports.insert("r6", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.r6();
      Ok(wit_bindgen_wasmer::rt::as_i32(result))
    }));
        exports.insert("r7", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i64, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.r7();
      Ok(wit_bindgen_wasmer::rt::as_i64(result))
    }));
        exports.insert("r8", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i64, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.r8();
      Ok(wit_bindgen_wasmer::rt::as_i64(result))
    }));
        exports.insert(
            "pair-ret",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let result = host.pair_ret();
                    let (t0_0, t0_1) = result;
                    let caller_memory =
                        unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                    caller_memory.store(
                        arg0 + 0,
                        wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(t0_0)),
                    )?;
                    caller_memory.store(
                        arg0 + 8,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_1)) as u8,
                    )?;
                    Ok(())
                },
            ),
        );
        imports.register("integers", exports);
    }
    use core::convert::TryFrom;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::RawMem;
}
