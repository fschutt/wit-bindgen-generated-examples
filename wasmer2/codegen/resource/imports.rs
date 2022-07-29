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
        dtor0: wasmer::LazyInit<wasmer::NativeFunc<i32, ()>>,

        index_slab1: wit_bindgen_wasmer::rt::IndexSlab,
        resource_slab1: wit_bindgen_wasmer::rt::ResourceSlab,
        dtor1: wasmer::LazyInit<wasmer::NativeFunc<i32, ()>>,
    }
    impl wasmer::WasmerEnv for ResourceData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            self.dtor0.initialize(
                instance
                    .exports
                    .get_with_generics_weak("canonical_abi_drop_x")?,
            );
            self.dtor1.initialize(
                instance
                    .exports
                    .get_with_generics_weak("canonical_abi_drop_y")?,
            );
            Ok(())
        }
    }
    impl Clone for ResourceData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct Resource {
        state: std::sync::Arc<std::sync::Mutex<ResourceData>>,
        func_acquire_an_x: wasmer::NativeFunc<(), i32>,
        func_canonical_abi_free: wasmer::NativeFunc<(i32, i32, i32), ()>,
        func_receive_an_x: wasmer::NativeFunc<i32, ()>,
        func_y_method_on_y: wasmer::NativeFunc<i32, ()>,
        func_y_method_with_param: wasmer::NativeFunc<(i32, i32), ()>,
        func_y_method_with_result: wasmer::NativeFunc<i32, i32>,
        func_y_some_constructor: wasmer::NativeFunc<(), i32>,
        memory: wasmer::Memory,
    }
    impl Resource {
        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ResourceData` which needs to be
        /// passed through to `Resource::new`.
        fn add_to_imports(
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<ResourceData>> {
            let state = std::sync::Arc::new(std::sync::Mutex::new(Default::default()));
            let mut canonical_abi = imports
                .get_namespace_exports("canonical_abi")
                .unwrap_or_else(wasmer::Exports::new);

            canonical_abi.insert(
                "resource_drop_x",
                wasmer::Function::new_native_with_env(
                    store,
                    state.clone(),
                    move |env: &std::sync::Arc<std::sync::Mutex<ResourceData>>,
                          idx: u32|
                          -> Result<(), wasmer::RuntimeError> {
                        let state = &mut *env.lock().unwrap();
                        let resource_idx = state.index_slab0.remove(idx)?;
                        let wasm = match state.resource_slab0.drop(resource_idx) {
                            Some(wasm) => wasm,
                            None => return Ok(()),
                        };
                        let dtor = state.dtor0.get_ref().unwrap();
                        dtor.call(wasm)?;
                        Ok(())
                    },
                ),
            );
            canonical_abi.insert(
                "resource_clone_x",
                wasmer::Function::new_native_with_env(
                    store,
                    state.clone(),
                    move |env: &std::sync::Arc<std::sync::Mutex<ResourceData>>,
                          idx: u32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *env.lock().unwrap();
                        let resource_idx = state.index_slab0.get(idx)?;
                        state.resource_slab0.clone(resource_idx)?;
                        Ok(state.index_slab0.insert(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_get_x",
                wasmer::Function::new_native_with_env(
                    store,
                    state.clone(),
                    move |env: &std::sync::Arc<std::sync::Mutex<ResourceData>>,
                          idx: u32|
                          -> Result<i32, wasmer::RuntimeError> {
                        let state = &mut *env.lock().unwrap();
                        let resource_idx = state.index_slab0.get(idx)?;
                        Ok(state.resource_slab0.get(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_new_x",
                wasmer::Function::new_native_with_env(
                    store,
                    state.clone(),
                    move |env: &std::sync::Arc<std::sync::Mutex<ResourceData>>,
                          val: i32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *env.lock().unwrap();
                        let resource_idx = state.resource_slab0.insert(val);
                        Ok(state.index_slab0.insert(resource_idx))
                    },
                ),
            );

            canonical_abi.insert(
                "resource_drop_y",
                wasmer::Function::new_native_with_env(
                    store,
                    state.clone(),
                    move |env: &std::sync::Arc<std::sync::Mutex<ResourceData>>,
                          idx: u32|
                          -> Result<(), wasmer::RuntimeError> {
                        let state = &mut *env.lock().unwrap();
                        let resource_idx = state.index_slab1.remove(idx)?;
                        let wasm = match state.resource_slab1.drop(resource_idx) {
                            Some(wasm) => wasm,
                            None => return Ok(()),
                        };
                        let dtor = state.dtor1.get_ref().unwrap();
                        dtor.call(wasm)?;
                        Ok(())
                    },
                ),
            );
            canonical_abi.insert(
                "resource_clone_y",
                wasmer::Function::new_native_with_env(
                    store,
                    state.clone(),
                    move |env: &std::sync::Arc<std::sync::Mutex<ResourceData>>,
                          idx: u32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *env.lock().unwrap();
                        let resource_idx = state.index_slab1.get(idx)?;
                        state.resource_slab1.clone(resource_idx)?;
                        Ok(state.index_slab1.insert(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_get_y",
                wasmer::Function::new_native_with_env(
                    store,
                    state.clone(),
                    move |env: &std::sync::Arc<std::sync::Mutex<ResourceData>>,
                          idx: u32|
                          -> Result<i32, wasmer::RuntimeError> {
                        let state = &mut *env.lock().unwrap();
                        let resource_idx = state.index_slab1.get(idx)?;
                        Ok(state.resource_slab1.get(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_new_y",
                wasmer::Function::new_native_with_env(
                    store,
                    state.clone(),
                    move |env: &std::sync::Arc<std::sync::Mutex<ResourceData>>,
                          val: i32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *env.lock().unwrap();
                        let resource_idx = state.resource_slab1.insert(val);
                        Ok(state.index_slab1.insert(resource_idx))
                    },
                ),
            );
            imports.register("canonical_abi", canonical_abi);
            state
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
            store: &wasmer::Store,
            module: &wasmer::Module,
            imports: &mut wasmer::ImportObject,
        ) -> anyhow::Result<(Self, wasmer::Instance)> {
            let state = Self::add_to_imports(store, imports);
            let instance = wasmer::Instance::new(module, &*imports)?;
            Ok((Self::new(&instance, state)?, instance))
        }

        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// and wrap them all up in the returned structure which can
        /// be used to interact with the wasm module.
        pub fn new(
            instance: &wasmer::Instance,
            state: std::sync::Arc<std::sync::Mutex<ResourceData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_acquire_an_x = instance
                .exports
                .get_native_function::<(), i32>("acquire-an-x")?;
            let func_canonical_abi_free = instance
                .exports
                .get_native_function::<(i32, i32, i32), ()>("canonical_abi_free")?;
            let func_receive_an_x = instance
                .exports
                .get_native_function::<i32, ()>("receive-an-x")?;
            let func_y_method_on_y = instance
                .exports
                .get_native_function::<i32, ()>("y::method-on-y")?;
            let func_y_method_with_param = instance
                .exports
                .get_native_function::<(i32, i32), ()>("y::method-with-param")?;
            let func_y_method_with_result = instance
                .exports
                .get_native_function::<i32, i32>("y::method-with-result")?;
            let func_y_some_constructor = instance
                .exports
                .get_native_function::<(), i32>("y::some-constructor")?;
            let memory = instance.exports.get_memory("memory")?.clone();
            Ok(Resource {
                func_acquire_an_x,
                func_canonical_abi_free,
                func_receive_an_x,
                func_y_method_on_y,
                func_y_method_with_param,
                func_y_method_with_result,
                func_y_some_constructor,
                memory,
                state,
            })
        }
        pub fn acquire_an_x(&self) -> Result<X, wasmer::RuntimeError> {
            let result0 = self.func_acquire_an_x.call()?;
            let handle1 = self
                .state
                .lock()
                .unwrap()
                .index_slab0
                .remove(result0 as u32)?;
            Ok(X(handle1))
        }
        pub fn receive_an_x(&self, val: &X) -> Result<(), wasmer::RuntimeError> {
            let obj0 = val;
            let handle0 = {
                let state = &mut *self.state.lock().unwrap();
                state.resource_slab0.clone(obj0.0)?;
                state.index_slab0.insert(obj0.0)
            };
            self.func_receive_an_x.call(handle0 as i32)?;
            Ok(())
        }
        pub fn y_some_constructor(&self) -> Result<Y, wasmer::RuntimeError> {
            let result0 = self.func_y_some_constructor.call()?;
            let handle1 = self
                .state
                .lock()
                .unwrap()
                .index_slab1
                .remove(result0 as u32)?;
            Ok(Y(handle1))
        }
        pub fn y_method_on_y(&self, self_: &Y) -> Result<(), wasmer::RuntimeError> {
            let obj0 = self_;
            let handle0 = {
                let state = &mut *self.state.lock().unwrap();
                state.resource_slab1.clone(obj0.0)?;
                state.index_slab1.insert(obj0.0)
            };
            self.func_y_method_on_y.call(handle0 as i32)?;
            Ok(())
        }
        pub fn y_method_with_param(&self, self_: &Y, x: u32) -> Result<(), wasmer::RuntimeError> {
            let obj0 = self_;
            let handle0 = {
                let state = &mut *self.state.lock().unwrap();
                state.resource_slab1.clone(obj0.0)?;
                state.index_slab1.insert(obj0.0)
            };
            self.func_y_method_with_param
                .call(handle0 as i32, wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn y_method_with_result(&self, self_: &Y) -> Result<String, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let memory = &self.memory;

            let obj0 = self_;
            let handle0 = {
                let state = &mut *self.state.lock().unwrap();
                state.resource_slab1.clone(obj0.0)?;
                state.index_slab1.insert(obj0.0)
            };
            let result1 = self.func_y_method_with_result.call(handle0 as i32)?;
            let load2 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 0)?;
            let load3 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 4)?;
            let ptr4 = load2;
            let len4 = load3;

            let data4 = copy_slice(memory, func_canonical_abi_free, ptr4, len4, 1)?;
            Ok(String::from_utf8(data4).map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?)
        }

        /// Drops the host-owned handle to the resource
        /// specified.
        ///
        /// Note that this may execute the WebAssembly-defined
        /// destructor for this type. This also may not run
        /// the destructor if there are still other references
        /// to this type.
        pub fn drop_x(&self, val: X) -> Result<(), wasmer::RuntimeError> {
            let data = &mut *self.state.lock().unwrap();
            let wasm = match data.resource_slab0.drop(val.0) {
                Some(val) => val,
                None => return Ok(()),
            };
            data.dtor0.get_ref().unwrap().call(wasm)?;
            Ok(())
        }

        /// Drops the host-owned handle to the resource
        /// specified.
        ///
        /// Note that this may execute the WebAssembly-defined
        /// destructor for this type. This also may not run
        /// the destructor if there are still other references
        /// to this type.
        pub fn drop_y(&self, val: Y) -> Result<(), wasmer::RuntimeError> {
            let data = &mut *self.state.lock().unwrap();
            let wasm = match data.resource_slab1.drop(val.0) {
                Some(val) => val,
                None => return Ok(()),
            };
            data.dtor1.get_ref().unwrap().call(wasm)?;
            Ok(())
        }
    }
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::RawMem;
}
