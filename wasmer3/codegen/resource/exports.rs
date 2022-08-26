#[allow(clippy::all)]
pub mod resource {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Resource: Sized + Send + Sync + 'static {
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
    pub struct LazyInitialized {
        memory: wasmer::Memory,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
    }

    #[must_use = "The returned initializer function must be called
      with the instance and the store before starting the runtime"]
    pub fn add_to_imports<T>(
        store: &mut wasmer::Store,
        imports: &mut wasmer::Imports,
        data: T,
    ) -> impl FnOnce(&wasmer::Instance, &dyn wasmer::AsStoreRef) -> Result<(), anyhow::Error>
    where
        T: Resource,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Resource> {
            data: T,
            tables: std::rc::Rc<core::cell::RefCell<ResourceTables<T>>>,
            lazy: std::rc::Rc<OnceCell<LazyInitialized>>,
        }
        unsafe impl<T: Resource> Send for EnvWrapper<T> {}
        unsafe impl<T: Resource> Sync for EnvWrapper<T> {}
        let lazy = std::rc::Rc::new(OnceCell::new());
        let env = EnvWrapper {
            data,
            tables: std::rc::Rc::default(),
            lazy: std::rc::Rc::clone(&lazy),
        };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
        "acquire-an-x",
        wasmer::Function::new_typed_with_env(
        &mut store,
        &env,
        move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
          let data_mut = store.data_mut();
          let tables = data_mut.tables.borrow_mut();
          let host = &mut data_mut.data;
          let result = host.acquire_an_x();
          drop(tables);
          Ok({
            let data_mut = store.data_mut();
            let mut tables = data_mut.tables.borrow_mut();
            tables.x_table.insert(result) as i32
          })
        }
        ));
        exports.insert(
            "receive-an-x",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = tables
                        .x_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let host = &mut data_mut.data;
                    let result = host.receive_an_x(param0);
                    drop(tables);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
        "y::some-constructor",
        wasmer::Function::new_typed_with_env(
        &mut store,
        &env,
        move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
          let data_mut = store.data_mut();
          let tables = data_mut.tables.borrow_mut();
          let host = &mut data_mut.data;
          let result = host.y_some_constructor();
          drop(tables);
          Ok({
            let data_mut = store.data_mut();
            let mut tables = data_mut.tables.borrow_mut();
            tables.y_table.insert(result) as i32
          })
        }
        ));
        exports.insert(
            "y::method-on-y",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = tables
                        .y_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let host = &mut data_mut.data;
                    let result = host.y_method_on_y(param0);
                    drop(tables);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "y::method-with-param",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = tables
                        .y_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let param1 = arg1 as u32;
                    let host = &mut data_mut.data;
                    let result = host.y_method_with_param(param0, param1);
                    drop(tables);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "y::method-with-result",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = tables
                        .y_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let host = &mut data_mut.data;
                    let result = host.y_method_with_result(param0);
                    drop(tables);
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec0.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                    caller_memory
                        .store(arg1 + 4, wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32))?;
                    caller_memory.store(arg1 + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                    Ok(())
                },
            ),
        );
        imports.register_namespace("resource", exports);
        let mut canonical_abi = imports
            .get_namespace_exports("canonical_abi")
            .unwrap_or_else(wasmer::Exports::new);
        canonical_abi.insert(
            "resource_drop_x",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      handle: u32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let mut tables = data_mut.tables.borrow_mut();
                    let handle = tables.x_table.remove(handle).map_err(|e| {
                        wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
                    })?;
                    let host = &mut data_mut.data;
                    host.drop_x(handle);
                    Ok(())
                },
            ),
        );
        canonical_abi.insert(
            "resource_drop_y",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      handle: u32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let mut tables = data_mut.tables.borrow_mut();
                    let handle = tables.y_table.remove(handle).map_err(|e| {
                        wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
                    })?;
                    let host = &mut data_mut.data;
                    host.drop_y(handle);
                    Ok(())
                },
            ),
        );
        imports.register_namespace("canonical_abi", canonical_abi);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| {
            let memory = _instance.exports.get_memory("memory")?.clone();
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(&_store.as_store_ref(), "canonical_abi_realloc")
                .unwrap()
                .clone();
            lazy.set(LazyInitialized {
                memory,
                func_canonical_abi_realloc,
            })
            .map_err(|_e| anyhow::anyhow!("Couldn't set lazy initialized data"))?;
            Ok(())
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::once_cell::unsync::OnceCell;
    use wit_bindgen_wasmer::rt::RawMem;
}
