#[allow(clippy::all)]
pub mod resource {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[derive(Debug)]
    pub struct X(wit_bindgen_wasmer::rt::ResourceIndex);
    #[derive(Debug)]
    pub struct Y(wit_bindgen_wasmer::rt::ResourceIndex);

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ResourceData {
        index_slab0: wit_bindgen_wasmer::rt::IndexSlab,
        resource_slab0: wit_bindgen_wasmer::rt::ResourceSlab,
        dtor0: OnceCell<wasmer::TypedFunction<i32, ()>>,

        index_slab1: wit_bindgen_wasmer::rt::IndexSlab,
        resource_slab1: wit_bindgen_wasmer::rt::ResourceSlab,
        dtor1: OnceCell<wasmer::TypedFunction<i32, ()>>,
    }

    pub struct Resource {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<ResourceData>,
        func_acquire_an_x: wasmer::TypedFunction<(), i32>,
        func_canonical_abi_free: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_receive_an_x: wasmer::TypedFunction<i32, ()>,
        func_y_method_on_y: wasmer::TypedFunction<i32, ()>,
        func_y_method_with_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_y_method_with_result: wasmer::TypedFunction<i32, i32>,
        func_y_some_constructor: wasmer::TypedFunction<(), i32>,
        memory: wasmer::Memory,
    }
    impl Resource {
        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ResourceData` which needs to be
        /// passed through to `Resource::new`.
        fn add_to_imports(
            store: &mut wasmer::StoreMut<'_>,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<ResourceData> {
            let env = wasmer::FunctionEnv::new(store, Default::default());
            let mut canonical_abi = imports
                .get_namespace_exports("canonical_abi")
                .unwrap_or_else(wasmer::Exports::new);

            canonical_abi.insert(
                "resource_drop_x",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ResourceData>,
                          idx: u32|
                          -> Result<(), wasmer::RuntimeError> {
                        let resource_idx = store.data_mut().index_slab0.remove(idx)?;
                        let wasm = match store.data_mut().resource_slab0.drop(resource_idx) {
                            Some(wasm) => wasm,
                            None => return Ok(()),
                        };
                        let dtor = store.data_mut().dtor0.get().unwrap().clone();
                        dtor.call(&mut store, wasm)?;
                        Ok(())
                    },
                ),
            );
            canonical_abi.insert(
                "resource_clone_x",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ResourceData>,
                          idx: u32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.index_slab0.get(idx)?;
                        state.resource_slab0.clone(resource_idx)?;
                        Ok(state.index_slab0.insert(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_get_x",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ResourceData>,
                          idx: u32|
                          -> Result<i32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.index_slab0.get(idx)?;
                        Ok(state.resource_slab0.get(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_new_x",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ResourceData>,
                          val: i32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.resource_slab0.insert(val);
                        Ok(state.index_slab0.insert(resource_idx))
                    },
                ),
            );

            canonical_abi.insert(
                "resource_drop_y",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ResourceData>,
                          idx: u32|
                          -> Result<(), wasmer::RuntimeError> {
                        let resource_idx = store.data_mut().index_slab1.remove(idx)?;
                        let wasm = match store.data_mut().resource_slab1.drop(resource_idx) {
                            Some(wasm) => wasm,
                            None => return Ok(()),
                        };
                        let dtor = store.data_mut().dtor1.get().unwrap().clone();
                        dtor.call(&mut store, wasm)?;
                        Ok(())
                    },
                ),
            );
            canonical_abi.insert(
                "resource_clone_y",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ResourceData>,
                          idx: u32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.index_slab1.get(idx)?;
                        state.resource_slab1.clone(resource_idx)?;
                        Ok(state.index_slab1.insert(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_get_y",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ResourceData>,
                          idx: u32|
                          -> Result<i32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.index_slab1.get(idx)?;
                        Ok(state.resource_slab1.get(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_new_y",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ResourceData>,
                          val: i32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.resource_slab1.insert(val);
                        Ok(state.index_slab1.insert(resource_idx))
                    },
                ),
            );
            imports.register_namespace("canonical_abi", canonical_abi);
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
            store: &mut wasmer::StoreMut<'_>,
            module: &wasmer::Module,
            imports: &mut wasmer::Imports,
        ) -> anyhow::Result<(Self, wasmer::Instance)> {
            let env = Self::add_to_imports(&mut store.as_store_mut().as_store_mut(), imports);
            let instance = wasmer::Instance::new(&mut store.as_store_mut(), module, &*imports)?;
            {
                let dtor0 = instance
                    .exports
                    .get_typed_function(store, "canonical_abi_drop_x")?
                    .clone();
                let dtor1 = instance
                    .exports
                    .get_typed_function(store, "canonical_abi_drop_y")?
                    .clone();

                env.as_mut(store)
                    .dtor0
                    .set(dtor0)
                    .map_err(|_e| anyhow::anyhow!("Couldn't set canonical_abi_drop_x"))?;
                env.as_mut(store)
                    .dtor1
                    .set(dtor1)
                    .map_err(|_e| anyhow::anyhow!("Couldn't set canonical_abi_drop_y"))?;
            }

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
            store: &mut wasmer::StoreMut<'_>,
            _instance: &wasmer::Instance,
            env: wasmer::FunctionEnv<ResourceData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_acquire_an_x = _instance
                .exports
                .get_typed_function(store, "acquire-an-x")?;
            let func_canonical_abi_free = _instance
                .exports
                .get_typed_function(store, "canonical_abi_free")?;
            let func_receive_an_x = _instance
                .exports
                .get_typed_function(store, "receive-an-x")?;
            let func_y_method_on_y = _instance
                .exports
                .get_typed_function(store, "y::method-on-y")?;
            let func_y_method_with_param = _instance
                .exports
                .get_typed_function(store, "y::method-with-param")?;
            let func_y_method_with_result = _instance
                .exports
                .get_typed_function(store, "y::method-with-result")?;
            let func_y_some_constructor = _instance
                .exports
                .get_typed_function(store, "y::some-constructor")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Resource {
                func_acquire_an_x,
                func_canonical_abi_free,
                func_receive_an_x,
                func_y_method_on_y,
                func_y_method_with_param,
                func_y_method_with_result,
                func_y_some_constructor,
                memory,
                env,
            })
        }
        pub fn acquire_an_x(&self, store: &mut wasmer::Store) -> Result<X, wasmer::RuntimeError> {
            let result0 = self.func_acquire_an_x.call(store)?;
            let state = self.env.as_mut(store);
            let handle1 = state.index_slab0.remove(result0 as u32)?;
            Ok(X(handle1))
        }
        pub fn receive_an_x(
            &self,
            store: &mut wasmer::Store,
            val: &X,
        ) -> Result<(), wasmer::RuntimeError> {
            let obj0 = val;
            let handle0 = {
                let state = self.env.as_mut(store);
                state.resource_slab0.clone(obj0.0)?;
                state.index_slab0.insert(obj0.0)
            };
            self.func_receive_an_x.call(store, handle0 as i32)?;
            Ok(())
        }
        pub fn y_some_constructor(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Y, wasmer::RuntimeError> {
            let result0 = self.func_y_some_constructor.call(store)?;
            let state = self.env.as_mut(store);
            let handle1 = state.index_slab1.remove(result0 as u32)?;
            Ok(Y(handle1))
        }
        pub fn y_method_on_y(
            &self,
            store: &mut wasmer::Store,
            self_: &Y,
        ) -> Result<(), wasmer::RuntimeError> {
            let obj0 = self_;
            let handle0 = {
                let state = self.env.as_mut(store);
                state.resource_slab1.clone(obj0.0)?;
                state.index_slab1.insert(obj0.0)
            };
            self.func_y_method_on_y.call(store, handle0 as i32)?;
            Ok(())
        }
        pub fn y_method_with_param(
            &self,
            store: &mut wasmer::Store,
            self_: &Y,
            x: u32,
        ) -> Result<(), wasmer::RuntimeError> {
            let obj0 = self_;
            let handle0 = {
                let state = self.env.as_mut(store);
                state.resource_slab1.clone(obj0.0)?;
                state.index_slab1.insert(obj0.0)
            };
            self.func_y_method_with_param.call(
                store,
                handle0 as i32,
                wit_bindgen_wasmer::rt::as_i32(x),
            )?;
            Ok(())
        }
        pub fn y_method_with_result(
            &self,
            store: &mut wasmer::Store,
            self_: &Y,
        ) -> Result<String, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;

            let obj0 = self_;
            let handle0 = {
                let state = self.env.as_mut(store);
                state.resource_slab1.clone(obj0.0)?;
                state.index_slab1.insert(obj0.0)
            };
            let result1 = self.func_y_method_with_result.call(store, handle0 as i32)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result1 + 0)?;
            let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result1 + 4)?;
            let ptr4 = load2;
            let len4 = load3;

            let data4 = copy_slice(store, _memory, func_canonical_abi_free, ptr4, len4, 1)?;
            Ok(String::from_utf8(data4).map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?)
        }

        /// Drops the host-owned handle to the resource
        /// specified.
        ///
        /// Note that this may execute the WebAssembly-defined
        /// destructor for this type. This also may not run
        /// the destructor if there are still other references
        /// to this type.
        pub fn drop_x(
            &self,
            store: &mut wasmer::Store,
            val: X,
        ) -> Result<(), wasmer::RuntimeError> {
            let state = self.env.as_mut(store);
            let wasm = match state.resource_slab0.drop(val.0) {
                Some(val) => val,
                None => return Ok(()),
            };
            let dtor0 = state.dtor0.get().unwrap().clone();
            dtor0.call(store, wasm)?;
            Ok(())
        }

        /// Drops the host-owned handle to the resource
        /// specified.
        ///
        /// Note that this may execute the WebAssembly-defined
        /// destructor for this type. This also may not run
        /// the destructor if there are still other references
        /// to this type.
        pub fn drop_y(
            &self,
            store: &mut wasmer::Store,
            val: Y,
        ) -> Result<(), wasmer::RuntimeError> {
            let state = self.env.as_mut(store);
            let wasm = match state.resource_slab1.drop(val.0) {
                Some(val) => val,
                None => return Ok(()),
            };
            let dtor1 = state.dtor1.get().unwrap().clone();
            dtor1.call(store, wasm)?;
            Ok(())
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::once_cell::unsync::OnceCell;
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::RawMem;
}
