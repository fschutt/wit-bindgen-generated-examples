pub mod resource {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Resource: Sized + wasmer::WasmerEnv + 'static {
        type X: std::fmt::Debug;
        type Y: std::fmt::Debug;
        fn acquire_an_x(&mut self) -> Self::X;

        fn receive_an_x(&mut self, val: &Self::X) -> ();

        fn y_some_constructor(&mut self) -> Self::Y;

        fn y_method_on_y(&mut self, self_: &Self::Y) -> ();

        fn y_method_with_param(&mut self, self_: &Self::Y, x: u32) -> ();

        fn y_method_with_result(&mut self, self_: &Self::Y) -> String;

        fn drop_x(&mut self, state: Self::X) {
            drop(state);
        }
        fn drop_y(&mut self, state: Self::Y) {
            drop(state);
        }
    }

    pub struct ResourceTables<T: Resource> {
        pub(crate) x_table: wit_bindgen_wasmer::Table<T::X>,
        pub(crate) y_table: wit_bindgen_wasmer::Table<T::Y>,
    }
    impl<T: Resource> Default for ResourceTables<T> {
        fn default() -> Self {
            Self {
                x_table: Default::default(),
                y_table: Default::default(),
            }
        }
    }
    impl<T: Resource> Clone for ResourceTables<T> {
        fn clone(&self) -> Self {
            Self::default()
        }
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: Resource,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Resource> {
            data: T,
            tables: ResourceTables<T>,
            memory: wasmer::LazyInit<wasmer::Memory>,
            func_canonical_abi_realloc:
                wasmer::LazyInit<wasmer::NativeFunc<(i32, i32, i32, i32), i32>>,
        }
        unsafe impl<T: Resource> Send for EnvWrapper<T> {}
        unsafe impl<T: Resource> Sync for EnvWrapper<T> {}
        impl<T: Resource> wasmer::WasmerEnv for EnvWrapper<T> {
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
            tables: ResourceTables::default(),
            memory: wasmer::LazyInit::new(),
            func_canonical_abi_realloc: wasmer::LazyInit::new(),
        }));
        let mut exports = wasmer::Exports::new();
        exports.insert("acquire-an-x", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let result = host.acquire_an_x();
          Ok(_tables.x_table.insert(result) as i32)
        }));
        exports.insert(
            "receive-an-x",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = _tables
                        .x_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let result = host.receive_an_x(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert("y::some-constructor", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let result = host.y_some_constructor();
          Ok(_tables.y_table.insert(result) as i32)
        }));
        exports.insert(
            "y::method-on-y",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = _tables
                        .y_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let result = host.y_method_on_y(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "y::method-with-param",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = _tables
                        .y_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let param1 = arg1 as u32;
                    let result = host.y_method_with_param(param0, param1);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "y::method-with-result",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let func_canonical_abi_realloc =
                        env.func_canonical_abi_realloc.get_ref().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = _tables
                        .y_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let result = host.y_method_with_result(param0);
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(0, 0, 1, vec0.len() as i32)?;
                    let caller_memory =
                        unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                    caller_memory
                        .store(arg1 + 4, wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32))?;
                    caller_memory.store(arg1 + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                    Ok(())
                },
            ),
        );
        imports.register("resource", exports);
        let mut canonical_abi = imports
            .get_namespace_exports("canonical_abi")
            .unwrap_or_else(wasmer::Exports::new);
        canonical_abi.insert(
            "resource_drop_x",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      handle: u32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let handle = env.tables.x_table.remove(handle).map_err(|e| {
                        wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
                    })?;
                    env.data.drop_x(handle);
                    Ok(())
                },
            ),
        );
        canonical_abi.insert(
            "resource_drop_y",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      handle: u32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let handle = env.tables.y_table.remove(handle).map_err(|e| {
                        wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
                    })?;
                    env.data.drop_y(handle);
                    Ok(())
                },
            ),
        );
        imports.register("canonical_abi", canonical_abi);
    }
    use wit_bindgen_wasmer::rt::RawMem;
}
