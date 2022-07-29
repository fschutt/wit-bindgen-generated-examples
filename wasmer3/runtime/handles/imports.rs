#[allow(clippy::all)]
pub mod exports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub struct WasmStateParamRecord<'a> {
        pub a: &'a WasmState2,
    }
    impl<'a> core::fmt::Debug for WasmStateParamRecord<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("WasmStateParamRecord")
                .field("a", &self.a)
                .finish()
        }
    }
    pub type WasmStateParamTuple<'a> = (&'a WasmState2,);
    pub type WasmStateParamOption<'a> = Option<&'a WasmState2>;
    pub type WasmStateParamResult<'a> = Result<&'a WasmState2, u32>;
    pub enum WasmStateParamVariant<'a> {
        WasmState2(&'a WasmState2),
        U32(u32),
    }
    impl<'a> core::fmt::Debug for WasmStateParamVariant<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                WasmStateParamVariant::WasmState2(e) => f
                    .debug_tuple("WasmStateParamVariant::WasmState2")
                    .field(e)
                    .finish(),
                WasmStateParamVariant::U32(e) => f
                    .debug_tuple("WasmStateParamVariant::U32")
                    .field(e)
                    .finish(),
            }
        }
    }
    pub struct WasmStateResultRecord {
        pub a: WasmState2,
    }
    impl core::fmt::Debug for WasmStateResultRecord {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("WasmStateResultRecord")
                .field("a", &self.a)
                .finish()
        }
    }
    pub type WasmStateResultTuple = (WasmState2,);
    pub type WasmStateResultOption = Option<WasmState2>;
    pub type WasmStateResultResult = Result<WasmState2, u32>;
    pub enum WasmStateResultVariant {
        WasmState2(WasmState2),
        U32(u32),
    }
    impl core::fmt::Debug for WasmStateResultVariant {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                WasmStateResultVariant::WasmState2(e) => f
                    .debug_tuple("WasmStateResultVariant::WasmState2")
                    .field(e)
                    .finish(),
                WasmStateResultVariant::U32(e) => f
                    .debug_tuple("WasmStateResultVariant::U32")
                    .field(e)
                    .finish(),
            }
        }
    }
    #[derive(Debug)]
    pub struct WasmState(wit_bindgen_wasmer::rt::ResourceIndex);
    #[derive(Debug)]
    pub struct WasmState2(wit_bindgen_wasmer::rt::ResourceIndex);
    #[derive(Debug)]
    pub struct Markdown(wit_bindgen_wasmer::rt::ResourceIndex);

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ExportsData {
        index_slab0: wit_bindgen_wasmer::rt::IndexSlab,
        resource_slab0: wit_bindgen_wasmer::rt::ResourceSlab,
        dtor0: OnceCell<wasmer::TypedFunction<i32, ()>>,

        index_slab1: wit_bindgen_wasmer::rt::IndexSlab,
        resource_slab1: wit_bindgen_wasmer::rt::ResourceSlab,
        dtor1: OnceCell<wasmer::TypedFunction<i32, ()>>,

        index_slab2: wit_bindgen_wasmer::rt::IndexSlab,
        resource_slab2: wit_bindgen_wasmer::rt::ResourceSlab,
        dtor2: OnceCell<wasmer::TypedFunction<i32, ()>>,
    }

    pub struct Exports {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<ExportsData>,
        func_canonical_abi_free: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        func_markdown_append: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_markdown_create: wasmer::TypedFunction<(), i32>,
        func_markdown_render: wasmer::TypedFunction<i32, i32>,
        func_test_imports: wasmer::TypedFunction<(), ()>,
        func_two_wasm_states: wasmer::TypedFunction<(i32, i32), i32>,
        func_wasm_state2_create: wasmer::TypedFunction<(), i32>,
        func_wasm_state2_param_list: wasmer::TypedFunction<(i32, i32), ()>,
        func_wasm_state2_param_option: wasmer::TypedFunction<(i32, i32), ()>,
        func_wasm_state2_param_record: wasmer::TypedFunction<i32, ()>,
        func_wasm_state2_param_result: wasmer::TypedFunction<(i32, i32), ()>,
        func_wasm_state2_param_tuple: wasmer::TypedFunction<i32, ()>,
        func_wasm_state2_param_variant: wasmer::TypedFunction<(i32, i32), ()>,
        func_wasm_state2_result_list: wasmer::TypedFunction<(), i32>,
        func_wasm_state2_result_option: wasmer::TypedFunction<(), i32>,
        func_wasm_state2_result_record: wasmer::TypedFunction<(), i32>,
        func_wasm_state2_result_result: wasmer::TypedFunction<(), i32>,
        func_wasm_state2_result_tuple: wasmer::TypedFunction<(), i32>,
        func_wasm_state2_result_variant: wasmer::TypedFunction<(), i32>,
        func_wasm_state2_saw_close: wasmer::TypedFunction<(), i32>,
        func_wasm_state_create: wasmer::TypedFunction<(), i32>,
        func_wasm_state_get_val: wasmer::TypedFunction<i32, i32>,
        memory: wasmer::Memory,
    }
    impl Exports {
        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ExportsData` which needs to be
        /// passed through to `Exports::new`.
        fn add_to_imports(
            store: &mut wasmer::StoreMut<'_>,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<ExportsData> {
            let env = wasmer::FunctionEnv::new(store, Default::default());
            let mut canonical_abi = imports
                .get_namespace_exports("canonical_abi")
                .unwrap_or_else(wasmer::Exports::new);

            canonical_abi.insert(
                "resource_drop_wasm-state",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
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
                "resource_clone_wasm-state",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
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
                "resource_get_wasm-state",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
                          idx: u32|
                          -> Result<i32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.index_slab0.get(idx)?;
                        Ok(state.resource_slab0.get(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_new_wasm-state",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
                          val: i32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.resource_slab0.insert(val);
                        Ok(state.index_slab0.insert(resource_idx))
                    },
                ),
            );

            canonical_abi.insert(
                "resource_drop_wasm-state2",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
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
                "resource_clone_wasm-state2",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
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
                "resource_get_wasm-state2",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
                          idx: u32|
                          -> Result<i32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.index_slab1.get(idx)?;
                        Ok(state.resource_slab1.get(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_new_wasm-state2",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
                          val: i32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.resource_slab1.insert(val);
                        Ok(state.index_slab1.insert(resource_idx))
                    },
                ),
            );

            canonical_abi.insert(
                "resource_drop_markdown",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
                          idx: u32|
                          -> Result<(), wasmer::RuntimeError> {
                        let resource_idx = store.data_mut().index_slab2.remove(idx)?;
                        let wasm = match store.data_mut().resource_slab2.drop(resource_idx) {
                            Some(wasm) => wasm,
                            None => return Ok(()),
                        };
                        let dtor = store.data_mut().dtor2.get().unwrap().clone();
                        dtor.call(&mut store, wasm)?;
                        Ok(())
                    },
                ),
            );
            canonical_abi.insert(
                "resource_clone_markdown",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
                          idx: u32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.index_slab2.get(idx)?;
                        state.resource_slab2.clone(resource_idx)?;
                        Ok(state.index_slab2.insert(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_get_markdown",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
                          idx: u32|
                          -> Result<i32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.index_slab2.get(idx)?;
                        Ok(state.resource_slab2.get(resource_idx))
                    },
                ),
            );
            canonical_abi.insert(
                "resource_new_markdown",
                wasmer::Function::new_native(
                    store,
                    &env,
                    move |mut store: wasmer::FunctionEnvMut<ExportsData>,
                          val: i32|
                          -> Result<u32, wasmer::RuntimeError> {
                        let state = &mut *store.data_mut();
                        let resource_idx = state.resource_slab2.insert(val);
                        Ok(state.index_slab2.insert(resource_idx))
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
                    .get_typed_function(store, "canonical_abi_drop_wasm-state")?
                    .clone();
                let dtor1 = instance
                    .exports
                    .get_typed_function(store, "canonical_abi_drop_wasm-state2")?
                    .clone();
                let dtor2 = instance
                    .exports
                    .get_typed_function(store, "canonical_abi_drop_markdown")?
                    .clone();

                env.as_mut(store)
                    .dtor0
                    .set(dtor0)
                    .map_err(|_e| anyhow::anyhow!("Couldn't set canonical_abi_drop_wasm-state"))?;
                env.as_mut(store)
                    .dtor1
                    .set(dtor1)
                    .map_err(|_e| anyhow::anyhow!("Couldn't set canonical_abi_drop_wasm-state2"))?;
                env.as_mut(store)
                    .dtor2
                    .set(dtor2)
                    .map_err(|_e| anyhow::anyhow!("Couldn't set canonical_abi_drop_markdown"))?;
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
            env: wasmer::FunctionEnv<ExportsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_canonical_abi_free = _instance
                .exports
                .get_typed_function(store, "canonical_abi_free")?;
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(store, "canonical_abi_realloc")?;
            let func_markdown_append = _instance
                .exports
                .get_typed_function(store, "markdown::append")?;
            let func_markdown_create = _instance
                .exports
                .get_typed_function(store, "markdown::create")?;
            let func_markdown_render = _instance
                .exports
                .get_typed_function(store, "markdown::render")?;
            let func_test_imports = _instance
                .exports
                .get_typed_function(store, "test-imports")?;
            let func_two_wasm_states = _instance
                .exports
                .get_typed_function(store, "two-wasm-states")?;
            let func_wasm_state2_create = _instance
                .exports
                .get_typed_function(store, "wasm-state2-create")?;
            let func_wasm_state2_param_list = _instance
                .exports
                .get_typed_function(store, "wasm-state2-param-list")?;
            let func_wasm_state2_param_option = _instance
                .exports
                .get_typed_function(store, "wasm-state2-param-option")?;
            let func_wasm_state2_param_record = _instance
                .exports
                .get_typed_function(store, "wasm-state2-param-record")?;
            let func_wasm_state2_param_result = _instance
                .exports
                .get_typed_function(store, "wasm-state2-param-result")?;
            let func_wasm_state2_param_tuple = _instance
                .exports
                .get_typed_function(store, "wasm-state2-param-tuple")?;
            let func_wasm_state2_param_variant = _instance
                .exports
                .get_typed_function(store, "wasm-state2-param-variant")?;
            let func_wasm_state2_result_list = _instance
                .exports
                .get_typed_function(store, "wasm-state2-result-list")?;
            let func_wasm_state2_result_option = _instance
                .exports
                .get_typed_function(store, "wasm-state2-result-option")?;
            let func_wasm_state2_result_record = _instance
                .exports
                .get_typed_function(store, "wasm-state2-result-record")?;
            let func_wasm_state2_result_result = _instance
                .exports
                .get_typed_function(store, "wasm-state2-result-result")?;
            let func_wasm_state2_result_tuple = _instance
                .exports
                .get_typed_function(store, "wasm-state2-result-tuple")?;
            let func_wasm_state2_result_variant = _instance
                .exports
                .get_typed_function(store, "wasm-state2-result-variant")?;
            let func_wasm_state2_saw_close = _instance
                .exports
                .get_typed_function(store, "wasm-state2-saw-close")?;
            let func_wasm_state_create = _instance
                .exports
                .get_typed_function(store, "wasm-state-create")?;
            let func_wasm_state_get_val = _instance
                .exports
                .get_typed_function(store, "wasm-state-get-val")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Exports {
                func_canonical_abi_free,
                func_canonical_abi_realloc,
                func_markdown_append,
                func_markdown_create,
                func_markdown_render,
                func_test_imports,
                func_two_wasm_states,
                func_wasm_state2_create,
                func_wasm_state2_param_list,
                func_wasm_state2_param_option,
                func_wasm_state2_param_record,
                func_wasm_state2_param_result,
                func_wasm_state2_param_tuple,
                func_wasm_state2_param_variant,
                func_wasm_state2_result_list,
                func_wasm_state2_result_option,
                func_wasm_state2_result_record,
                func_wasm_state2_result_result,
                func_wasm_state2_result_tuple,
                func_wasm_state2_result_variant,
                func_wasm_state2_saw_close,
                func_wasm_state_create,
                func_wasm_state_get_val,
                memory,
                env,
            })
        }
        pub fn test_imports(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_test_imports.call(store)?;
            Ok(())
        }
        pub fn wasm_state_create(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<WasmState, wasmer::RuntimeError> {
            let result0 = self.func_wasm_state_create.call(store)?;
            let state = self.env.as_mut(store);
            let handle1 = state.index_slab0.remove(result0 as u32)?;
            Ok(WasmState(handle1))
        }
        pub fn wasm_state_get_val(
            &self,
            store: &mut wasmer::Store,
            a: &WasmState,
        ) -> Result<u32, wasmer::RuntimeError> {
            let obj0 = a;
            let handle0 = {
                let state = self.env.as_mut(store);
                state.resource_slab0.clone(obj0.0)?;
                state.index_slab0.insert(obj0.0)
            };
            let result1 = self.func_wasm_state_get_val.call(store, handle0 as i32)?;
            Ok(result1 as u32)
        }
        pub fn wasm_state2_create(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<WasmState2, wasmer::RuntimeError> {
            let result0 = self.func_wasm_state2_create.call(store)?;
            let state = self.env.as_mut(store);
            let handle1 = state.index_slab1.remove(result0 as u32)?;
            Ok(WasmState2(handle1))
        }
        pub fn wasm_state2_saw_close(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<bool, wasmer::RuntimeError> {
            let result0 = self.func_wasm_state2_saw_close.call(store)?;
            Ok(match result0 {
                0 => false,
                1 => true,
                _ => return Err(invalid_variant("bool")),
            })
        }
        pub fn two_wasm_states(
            &self,
            store: &mut wasmer::Store,
            a: &WasmState,
            b: &WasmState2,
        ) -> Result<(WasmState, WasmState2), wasmer::RuntimeError> {
            let _memory = &self.memory;

            let obj0 = a;
            let handle0 = {
                let state = self.env.as_mut(store);
                state.resource_slab0.clone(obj0.0)?;
                state.index_slab0.insert(obj0.0)
            };

            let obj1 = b;
            let handle1 = {
                let state = self.env.as_mut(store);
                state.resource_slab1.clone(obj1.0)?;
                state.index_slab1.insert(obj1.0)
            };
            let result2 = self
                .func_two_wasm_states
                .call(store, handle0 as i32, handle1 as i32)?;
            let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result2 + 0)?;
            let state = self.env.as_mut(store);
            let handle4 = state.index_slab0.remove(load3 as u32)?;
            let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result2 + 4)?;
            let state = self.env.as_mut(store);
            let handle6 = state.index_slab1.remove(load5 as u32)?;
            Ok((WasmState(handle4), WasmState2(handle6)))
        }
        pub fn wasm_state2_param_record(
            &self,
            store: &mut wasmer::Store,
            a: WasmStateParamRecord<'_>,
        ) -> Result<(), wasmer::RuntimeError> {
            let WasmStateParamRecord { a: a0 } = a;

            let obj1 = a0;
            let handle1 = {
                let state = self.env.as_mut(store);
                state.resource_slab1.clone(obj1.0)?;
                state.index_slab1.insert(obj1.0)
            };
            self.func_wasm_state2_param_record
                .call(store, handle1 as i32)?;
            Ok(())
        }
        pub fn wasm_state2_param_tuple(
            &self,
            store: &mut wasmer::Store,
            a: WasmStateParamTuple<'_>,
        ) -> Result<(), wasmer::RuntimeError> {
            let (t0_0,) = a;

            let obj1 = t0_0;
            let handle1 = {
                let state = self.env.as_mut(store);
                state.resource_slab1.clone(obj1.0)?;
                state.index_slab1.insert(obj1.0)
            };
            self.func_wasm_state2_param_tuple
                .call(store, handle1 as i32)?;
            Ok(())
        }
        pub fn wasm_state2_param_option(
            &self,
            store: &mut wasmer::Store,
            a: WasmStateParamOption<'_>,
        ) -> Result<(), wasmer::RuntimeError> {
            let (result1_0, result1_1) = match a {
                Some(e) => {
                    let obj0 = e;
                    let handle0 = {
                        let state = self.env.as_mut(store);
                        state.resource_slab1.clone(obj0.0)?;
                        state.index_slab1.insert(obj0.0)
                    };
                    (1i32, handle0 as i32)
                }
                None => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0i32)
                    }
                }
            };
            self.func_wasm_state2_param_option
                .call(store, result1_0, result1_1)?;
            Ok(())
        }
        pub fn wasm_state2_param_result(
            &self,
            store: &mut wasmer::Store,
            a: WasmStateParamResult<'_>,
        ) -> Result<(), wasmer::RuntimeError> {
            let (result1_0, result1_1) = match a {
                Ok(e) => {
                    let obj0 = e;
                    let handle0 = {
                        let state = self.env.as_mut(store);
                        state.resource_slab1.clone(obj0.0)?;
                        state.index_slab1.insert(obj0.0)
                    };
                    (0i32, handle0 as i32)
                }
                Err(e) => (1i32, wit_bindgen_wasmer::rt::as_i32(e)),
            };
            self.func_wasm_state2_param_result
                .call(store, result1_0, result1_1)?;
            Ok(())
        }
        pub fn wasm_state2_param_variant(
            &self,
            store: &mut wasmer::Store,
            a: WasmStateParamVariant<'_>,
        ) -> Result<(), wasmer::RuntimeError> {
            let (result1_0, result1_1) = match a {
                WasmStateParamVariant::WasmState2(e) => {
                    let obj0 = e;
                    let handle0 = {
                        let state = self.env.as_mut(store);
                        state.resource_slab1.clone(obj0.0)?;
                        state.index_slab1.insert(obj0.0)
                    };
                    (0i32, handle0 as i32)
                }
                WasmStateParamVariant::U32(e) => (1i32, wit_bindgen_wasmer::rt::as_i32(e)),
            };
            self.func_wasm_state2_param_variant
                .call(store, result1_0, result1_1)?;
            Ok(())
        }
        pub fn wasm_state2_param_list(
            &self,
            store: &mut wasmer::Store,
            a: &[&WasmState2],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec1 = a;
            let len1 = vec1.len() as i32;
            let result1 =
                func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 4, len1 * 4)?;
            for (i, e) in vec1.into_iter().enumerate() {
                let base = result1 + (i as i32) * 4;
                {
                    let obj0 = e;
                    let handle0 = {
                        let state = self.env.as_mut(store);
                        state.resource_slab1.clone(obj0.0)?;
                        state.index_slab1.insert(obj0.0)
                    };
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 0, wit_bindgen_wasmer::rt::as_i32(handle0 as i32))?;
                }
            }
            self.func_wasm_state2_param_list
                .call(store, result1, len1)?;
            Ok(())
        }
        pub fn wasm_state2_result_record(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<WasmStateResultRecord, wasmer::RuntimeError> {
            let result0 = self.func_wasm_state2_result_record.call(store)?;
            let state = self.env.as_mut(store);
            let handle1 = state.index_slab1.remove(result0 as u32)?;
            Ok(WasmStateResultRecord {
                a: WasmState2(handle1),
            })
        }
        pub fn wasm_state2_result_tuple(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<WasmStateResultTuple, wasmer::RuntimeError> {
            let result0 = self.func_wasm_state2_result_tuple.call(store)?;
            let state = self.env.as_mut(store);
            let handle1 = state.index_slab1.remove(result0 as u32)?;
            Ok((WasmState2(handle1),))
        }
        pub fn wasm_state2_result_option(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<WasmStateResultOption, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_wasm_state2_result_option.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => None,
                1 => Some({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    let state = self.env.as_mut(store);
                    let handle3 = state.index_slab1.remove(load2 as u32)?;
                    WasmState2(handle3)
                }),
                _ => return Err(invalid_variant("option")),
            })
        }
        pub fn wasm_state2_result_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<WasmStateResultResult, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_wasm_state2_result_result.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => Ok({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    let state = self.env.as_mut(store);
                    let handle3 = state.index_slab1.remove(load2 as u32)?;
                    WasmState2(handle3)
                }),
                1 => Err({
                    let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    load4 as u32
                }),
                _ => return Err(invalid_variant("expected")),
            })
        }
        pub fn wasm_state2_result_variant(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<WasmStateResultVariant, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_wasm_state2_result_variant.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => WasmStateResultVariant::WasmState2({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    let state = self.env.as_mut(store);
                    let handle3 = state.index_slab1.remove(load2 as u32)?;
                    WasmState2(handle3)
                }),
                1 => WasmStateResultVariant::U32({
                    let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    load4 as u32
                }),
                _ => return Err(invalid_variant("WasmStateResultVariant")),
            })
        }
        pub fn wasm_state2_result_list(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<WasmState2>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_wasm_state2_result_list.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let len5 = load2;
            let base5 = load1;
            let mut result5 = Vec::with_capacity(len5 as usize);
            for i in 0..len5 {
                let base = base5 + i * 4;
                result5.push({
                    let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 0)?;
                    let state = self.env.as_mut(store);
                    let handle4 = state.index_slab1.remove(load3 as u32)?;
                    WasmState2(handle4)
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base5, len5 * 4, 4)?;
            Ok(result5)
        }
        pub fn markdown_create(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Option<Markdown>, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_markdown_create.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => None,
                1 => Some({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    let state = self.env.as_mut(store);
                    let handle3 = state.index_slab2.remove(load2 as u32)?;
                    Markdown(handle3)
                }),
                _ => return Err(invalid_variant("option")),
            })
        }
        pub fn markdown_append(
            &self,
            store: &mut wasmer::Store,
            self_: &Markdown,
            buf: &str,
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;

            let obj0 = self_;
            let handle0 = {
                let state = self.env.as_mut(store);
                state.resource_slab2.clone(obj0.0)?;
                state.index_slab2.insert(obj0.0)
            };
            let vec1 = buf;
            let ptr1 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec1.len() as i32,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .store_many(ptr1, vec1.as_bytes())?;
            self.func_markdown_append
                .call(store, handle0 as i32, ptr1, vec1.len() as i32)?;
            Ok(())
        }
        pub fn markdown_render(
            &self,
            store: &mut wasmer::Store,
            self_: &Markdown,
        ) -> Result<String, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;

            let obj0 = self_;
            let handle0 = {
                let state = self.env.as_mut(store);
                state.resource_slab2.clone(obj0.0)?;
                state.index_slab2.insert(obj0.0)
            };
            let result1 = self.func_markdown_render.call(store, handle0 as i32)?;
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
        pub fn drop_wasm_state(
            &self,
            store: &mut wasmer::Store,
            val: WasmState,
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
        pub fn drop_wasm_state2(
            &self,
            store: &mut wasmer::Store,
            val: WasmState2,
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

        /// Drops the host-owned handle to the resource
        /// specified.
        ///
        /// Note that this may execute the WebAssembly-defined
        /// destructor for this type. This also may not run
        /// the destructor if there are still other references
        /// to this type.
        pub fn drop_markdown(
            &self,
            store: &mut wasmer::Store,
            val: Markdown,
        ) -> Result<(), wasmer::RuntimeError> {
            let state = self.env.as_mut(store);
            let wasm = match state.resource_slab2.drop(val.0) {
                Some(val) => val,
                None => return Ok(()),
            };
            let dtor2 = state.dtor2.get().unwrap().clone();
            dtor2.call(store, wasm)?;
            Ok(())
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::once_cell::unsync::OnceCell;
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
