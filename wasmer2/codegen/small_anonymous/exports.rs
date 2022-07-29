pub mod small_anonymous {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Error {
        Success,
        Failure,
    }
    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::Success => f.debug_tuple("Error::Success").finish(),
                Error::Failure => f.debug_tuple("Error::Failure").finish(),
            }
        }
    }
    pub trait SmallAnonymous: Sized + wasmer::WasmerEnv + 'static {
        fn option_test(&mut self) -> Result<Option<String>, Error>;
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: SmallAnonymous,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: SmallAnonymous> {
            data: T,
            memory: wasmer::LazyInit<wasmer::Memory>,
            func_canonical_abi_realloc:
                wasmer::LazyInit<wasmer::NativeFunc<(i32, i32, i32, i32), i32>>,
        }
        unsafe impl<T: SmallAnonymous> Send for EnvWrapper<T> {}
        unsafe impl<T: SmallAnonymous> Sync for EnvWrapper<T> {}
        impl<T: SmallAnonymous> wasmer::WasmerEnv for EnvWrapper<T> {
            fn init_with_instance(
                &mut self,
                instance: &wasmer::Instance,
            ) -> Result<(), wasmer::HostEnvInitError> {
                self.data.init_with_instance(instance)?;
                self.memory
                    .initialize(instance.exports.get_with_generics_weak("memory")?);
                self.func_canonical_abi_realloc.initialize(
                    instance
                        .exports
                        .get_with_generics_weak("canonical_abi_realloc")?,
                );
                Ok(())
            }
        }
        let env = std::sync::Arc::new(std::sync::Mutex::new(EnvWrapper {
            data,
            memory: wasmer::LazyInit::new(),
            func_canonical_abi_realloc: wasmer::LazyInit::new(),
        }));
        let mut exports = wasmer::Exports::new();
        exports.insert(
            "option-test",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let func_canonical_abi_realloc =
                        env.func_canonical_abi_realloc.get_ref().unwrap();
                    let host = &mut env.data;
                    let result = host.option_test();
                    match result {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            match e {
                                Some(e) => {
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store(
                                        arg0 + 4,
                                        wit_bindgen_wasmer::rt::as_i32(1i32) as u8,
                                    )?;
                                    let vec0 = e;
                                    let ptr0 = func_canonical_abi_realloc.call(
                                        0,
                                        0,
                                        1,
                                        vec0.len() as i32,
                                    )?;
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                                    caller_memory.store(
                                        arg0 + 12,
                                        wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg0 + 8, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                                }
                                None => {
                                    let e = ();
                                    {
                                        caller_memory.store(
                                            arg0 + 4,
                                            wit_bindgen_wasmer::rt::as_i32(0i32) as u8,
                                        )?;
                                        let () = e;
                                    }
                                }
                            };
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory
                                .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        imports.register("small-anonymous", exports);
    }
    use wit_bindgen_wasmer::rt::RawMem;
}
