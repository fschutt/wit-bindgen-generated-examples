#[allow(clippy::all)]
pub mod exports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ExportsData {}

    pub struct Exports {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<ExportsData>,
        func_allocated_bytes: wasmer::TypedFunction<(), i32>,
        func_canonical_abi_free: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        func_list_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_param2: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_param3: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_param4: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_result: wasmer::TypedFunction<(), i32>,
        func_list_result2: wasmer::TypedFunction<(), i32>,
        func_list_result3: wasmer::TypedFunction<(), i32>,
        func_list_roundtrip: wasmer::TypedFunction<(i32, i32), i32>,
        func_string_roundtrip: wasmer::TypedFunction<(i32, i32), i32>,
        func_test_imports: wasmer::TypedFunction<(), ()>,
        memory: wasmer::Memory,
    }
    impl Exports {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ExportsData` which needs to be
        /// passed through to `Exports::new`.
        fn add_to_imports(
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<ExportsData> {
            let env = wasmer::FunctionEnv::new(&mut store, ExportsData::default());
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
            mut store: impl wasmer::AsStoreMut,
            module: &wasmer::Module,
            imports: &mut wasmer::Imports,
        ) -> anyhow::Result<(Self, wasmer::Instance)> {
            let env = Self::add_to_imports(&mut store, imports);
            let instance = wasmer::Instance::new(&mut store, module, &*imports)?;

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
            store: impl wasmer::AsStoreMut,
            _instance: &wasmer::Instance,
            env: wasmer::FunctionEnv<ExportsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_allocated_bytes = _instance
                .exports
                .get_typed_function(&store, "allocated-bytes")?;
            let func_canonical_abi_free = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_free")?;
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_realloc")?;
            let func_list_param = _instance.exports.get_typed_function(&store, "list-param")?;
            let func_list_param2 = _instance
                .exports
                .get_typed_function(&store, "list-param2")?;
            let func_list_param3 = _instance
                .exports
                .get_typed_function(&store, "list-param3")?;
            let func_list_param4 = _instance
                .exports
                .get_typed_function(&store, "list-param4")?;
            let func_list_result = _instance
                .exports
                .get_typed_function(&store, "list-result")?;
            let func_list_result2 = _instance
                .exports
                .get_typed_function(&store, "list-result2")?;
            let func_list_result3 = _instance
                .exports
                .get_typed_function(&store, "list-result3")?;
            let func_list_roundtrip = _instance
                .exports
                .get_typed_function(&store, "list-roundtrip")?;
            let func_string_roundtrip = _instance
                .exports
                .get_typed_function(&store, "string-roundtrip")?;
            let func_test_imports = _instance
                .exports
                .get_typed_function(&store, "test-imports")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Exports {
                func_allocated_bytes,
                func_canonical_abi_free,
                func_canonical_abi_realloc,
                func_list_param,
                func_list_param2,
                func_list_param3,
                func_list_param4,
                func_list_result,
                func_list_result2,
                func_list_result3,
                func_list_roundtrip,
                func_string_roundtrip,
                func_test_imports,
                memory,
                env,
            })
        }
        pub fn test_imports(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_test_imports.call(store)?;
            Ok(())
        }
        pub fn allocated_bytes(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<u32, wasmer::RuntimeError> {
            let result0 = self.func_allocated_bytes.call(store)?;
            Ok(result0 as u32)
        }
        pub fn list_param(
            &self,
            store: &mut wasmer::Store,
            a: &[u8],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = a;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                (vec0.len() as i32) * 1,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr0, &vec0)?;
            self.func_list_param.call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_param2(
            &self,
            store: &mut wasmer::Store,
            a: &str,
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = a;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec0.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr0, vec0.as_bytes())?;
            self.func_list_param2.call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_param3(
            &self,
            store: &mut wasmer::Store,
            a: &[&str],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec1 = a;
            let len1 = vec1.len() as i32;
            let result1 =
                func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 4, len1 * 8)?;
            for (i, e) in vec1.into_iter().enumerate() {
                let base = result1 + (i as i32) * 8;
                {
                    let vec0 = e;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec0.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store_many(ptr0, vec0.as_bytes())?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store(base + 4, wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32))?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                }
            }
            self.func_list_param3.call(store, result1, len1)?;
            Ok(())
        }
        pub fn list_param4(
            &self,
            store: &mut wasmer::Store,
            a: &[&[&str]],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec2 = a;
            let len2 = vec2.len() as i32;
            let result2 =
                func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 4, len2 * 8)?;
            for (i, e) in vec2.into_iter().enumerate() {
                let base = result2 + (i as i32) * 8;
                {
                    let vec1 = e;
                    let len1 = vec1.len() as i32;
                    let result1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        len1 * 8,
                    )?;
                    for (i, e) in vec1.into_iter().enumerate() {
                        let base = result1 + (i as i32) * 8;
                        {
                            let vec0 = e;
                            let ptr0 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec0.len() as i32,
                            )?;
                            let _memory_view = _memory.view(&store);
                            unsafe { _memory_view.data_unchecked_mut() }
                                .store_many(ptr0, vec0.as_bytes())?;
                            let _memory_view = _memory.view(&store);
                            unsafe { _memory_view.data_unchecked_mut() }.store(
                                base + 4,
                                wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32),
                            )?;
                            let _memory_view = _memory.view(&store);
                            unsafe { _memory_view.data_unchecked_mut() }
                                .store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                        }
                    }
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store(base + 4, wit_bindgen_wasmer::rt::as_i32(len1))?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store(base + 0, wit_bindgen_wasmer::rt::as_i32(result1))?;
                }
            }
            self.func_list_param4.call(store, result2, len2)?;
            Ok(())
        }
        pub fn list_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<u8>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_result.call(store)?;
            let _memory_view = _memory.view(&store);
            let load1 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 0)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr3,
                len3,
                1,
            )?)
        }
        pub fn list_result2(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<String, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_result2.call(store)?;
            let _memory_view = _memory.view(&store);
            let load1 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 0)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;

            let data3 = copy_slice(store, _memory, func_canonical_abi_free, ptr3, len3, 1)?;
            Ok(String::from_utf8(data3).map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?)
        }
        pub fn list_result3(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<String>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_result3.call(store)?;
            let _memory_view = _memory.view(&store);
            let load1 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 0)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 4)?;
            let len6 = load2;
            let base6 = load1;
            let mut result6 = Vec::with_capacity(len6 as usize);
            for i in 0..len6 {
                let base = base6 + i * 8;
                result6.push({
                    let _memory_view = _memory.view(&store);
                    let load3 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(base + 0)?;
                    let _memory_view = _memory.view(&store);
                    let load4 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(base + 4)?;
                    let ptr5 = load3;
                    let len5 = load4;

                    let data5 = copy_slice(store, _memory, func_canonical_abi_free, ptr5, len5, 1)?;
                    String::from_utf8(data5)
                        .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base6, len6 * 8, 4)?;
            Ok(result6)
        }
        pub fn list_roundtrip(
            &self,
            store: &mut wasmer::Store,
            a: &[u8],
        ) -> Result<Vec<u8>, wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let vec0 = a;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                (vec0.len() as i32) * 1,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr0, &vec0)?;
            let result1 = self
                .func_list_roundtrip
                .call(store, ptr0, vec0.len() as i32)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result1 + 0)?;
            let _memory_view = _memory.view(&store);
            let load3 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result1 + 4)?;
            let ptr4 = load2;
            let len4 = load3;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr4,
                len4,
                1,
            )?)
        }
        pub fn string_roundtrip(
            &self,
            store: &mut wasmer::Store,
            a: &str,
        ) -> Result<String, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = a;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec0.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr0, vec0.as_bytes())?;
            let result1 = self
                .func_string_roundtrip
                .call(store, ptr0, vec0.len() as i32)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result1 + 0)?;
            let _memory_view = _memory.view(&store);
            let load3 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result1 + 4)?;
            let ptr4 = load2;
            let len4 = load3;

            let data4 = copy_slice(store, _memory, func_canonical_abi_free, ptr4, len4, 1)?;
            Ok(String::from_utf8(data4).map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?)
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::RawMem;
}
