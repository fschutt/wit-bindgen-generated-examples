pub mod simple_lists {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait SimpleLists: Sized + wasmer::WasmerEnv + 'static {
        fn simple_list1(&mut self, l: &[Le<u32>]) -> ();

        fn simple_list2(&mut self) -> Vec<u32>;

        fn simple_list4(&mut self, l: Vec<&[Le<u32>]>) -> Vec<Vec<u32>>;
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: SimpleLists,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: SimpleLists> {
            data: T,
            memory: wasmer::LazyInit<wasmer::Memory>,
            func_canonical_abi_realloc:
                wasmer::LazyInit<wasmer::NativeFunc<(i32, i32, i32, i32), i32>>,
        }
        unsafe impl<T: SimpleLists> Send for EnvWrapper<T> {}
        unsafe impl<T: SimpleLists> Sync for EnvWrapper<T> {}
        impl<T: SimpleLists> wasmer::WasmerEnv for EnvWrapper<T> {
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
            "simple-list1",
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
                    let param0 = _bc.slice(ptr0, len0)?;
                    let result = host.simple_list1(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "simple-list2",
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
                    let result = host.simple_list2();
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(0, 0, 4, (vec0.len() as i32) * 4)?;
                    let caller_memory =
                        unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                    caller_memory.store_many(ptr0, &vec0)?;
                    caller_memory
                        .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32))?;
                    caller_memory.store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "simple-list4",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
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
                    let len3 = arg1;
                    let base3 = arg0;
                    let mut result3 = Vec::with_capacity(len3 as usize);
                    for i in 0..len3 {
                        let base = base3 + i * 8;
                        result3.push({
                            let load0 = _bc.load::<i32>(base + 0)?;
                            let load1 = _bc.load::<i32>(base + 4)?;
                            let ptr2 = load0;
                            let len2 = load1;
                            _bc.slice(ptr2, len2)?
                        });
                    }
                    let param0 = result3;
                    let result = host.simple_list4(param0);
                    let vec5 = result;
                    let len5 = vec5.len() as i32;
                    let result5 = func_canonical_abi_realloc.call(0, 0, 4, len5 * 8)?;
                    for (i, e) in vec5.into_iter().enumerate() {
                        let base = result5 + (i as i32) * 8;
                        {
                            let vec4 = e;
                            let ptr4 = func_canonical_abi_realloc.call(
                                0,
                                0,
                                4,
                                (vec4.len() as i32) * 4,
                            )?;
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory.store_many(ptr4, &vec4)?;
                            caller_memory.store(
                                base + 4,
                                wit_bindgen_wasmer::rt::as_i32(vec4.len() as i32),
                            )?;
                            caller_memory.store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr4))?;
                        }
                    }
                    let caller_memory =
                        unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                    caller_memory.store(arg2 + 4, wit_bindgen_wasmer::rt::as_i32(len5))?;
                    caller_memory.store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(result5))?;
                    Ok(())
                },
            ),
        );
        imports.register("simple-lists", exports);
    }
    use wit_bindgen_wasmer::rt::RawMem;
    use wit_bindgen_wasmer::Le;
}
