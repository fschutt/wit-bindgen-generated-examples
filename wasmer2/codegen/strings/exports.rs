pub mod strings {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Strings: Sized + wasmer::WasmerEnv + 'static {
        fn a(&mut self, x: &str) -> ();

        fn b(&mut self) -> String;

        fn c(&mut self, a: &str, b: &str) -> String;
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: Strings,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Strings> {
            data: T,
            memory: wasmer::LazyInit<wasmer::Memory>,
            func_canonical_abi_realloc:
                wasmer::LazyInit<wasmer::NativeFunc<(i32, i32, i32, i32), i32>>,
        }
        unsafe impl<T: Strings> Send for EnvWrapper<T> {}
        unsafe impl<T: Strings> Sync for EnvWrapper<T> {}
        impl<T: Strings> wasmer::WasmerEnv for EnvWrapper<T> {
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
            "a",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let mut _bc = unsafe {
                        wit_bindgen_wasmer::BorrowChecker::new(
                            env.memory.get_ref().unwrap().data_unchecked_mut(),
                        )
                    };
                    let host = &mut env.data;
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let param0 = _bc.slice_str(ptr0, len0)?;
                    let result = host.a(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "b",
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
                    let result = host.b();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(0, 0, 1, vec0.len() as i32)?;
                    let caller_memory =
                        unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                    caller_memory
                        .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32))?;
                    caller_memory.store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "c",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let func_canonical_abi_realloc =
                        env.func_canonical_abi_realloc.get_ref().unwrap();
                    let mut _bc = unsafe {
                        wit_bindgen_wasmer::BorrowChecker::new(
                            env.memory.get_ref().unwrap().data_unchecked_mut(),
                        )
                    };
                    let host = &mut env.data;
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let ptr1 = arg2;
                    let len1 = arg3;
                    let param0 = _bc.slice_str(ptr0, len0)?;
                    let param1 = _bc.slice_str(ptr1, len1)?;
                    let result = host.c(param0, param1);
                    let vec2 = result;
                    let ptr2 = func_canonical_abi_realloc.call(0, 0, 1, vec2.len() as i32)?;
                    let caller_memory =
                        unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                    caller_memory.store_many(ptr2, vec2.as_bytes())?;
                    caller_memory
                        .store(arg4 + 4, wit_bindgen_wasmer::rt::as_i32(vec2.len() as i32))?;
                    caller_memory.store(arg4 + 0, wit_bindgen_wasmer::rt::as_i32(ptr2))?;
                    Ok(())
                },
            ),
        );
        imports.register("strings", exports);
    }
    use wit_bindgen_wasmer::rt::RawMem;
}
