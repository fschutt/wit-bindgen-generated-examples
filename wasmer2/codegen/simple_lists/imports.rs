pub mod simple_lists {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct SimpleListsData {}
    impl wasmer::WasmerEnv for SimpleListsData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            Ok(())
        }
    }
    impl Clone for SimpleListsData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct SimpleLists {
        state: std::sync::Arc<std::sync::Mutex<SimpleListsData>>,
        func_canonical_abi_free: wasmer::NativeFunc<(i32, i32, i32), ()>,
        func_canonical_abi_realloc: wasmer::NativeFunc<(i32, i32, i32, i32), i32>,
        func_simple_list1: wasmer::NativeFunc<(i32, i32), ()>,
        func_simple_list2: wasmer::NativeFunc<(), i32>,
        func_simple_list4: wasmer::NativeFunc<(i32, i32), i32>,
        memory: wasmer::Memory,
    }
    impl SimpleLists {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `SimpleListsData` which needs to be
        /// passed through to `SimpleLists::new`.
        fn add_to_imports(
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<SimpleListsData>> {
            let state = std::sync::Arc::new(std::sync::Mutex::new(Default::default()));
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
            state: std::sync::Arc<std::sync::Mutex<SimpleListsData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_canonical_abi_free = instance
                .exports
                .get_native_function::<(i32, i32, i32), ()>("canonical_abi_free")?;
            let func_canonical_abi_realloc = instance
                .exports
                .get_native_function::<(i32, i32, i32, i32), i32>("canonical_abi_realloc")?;
            let func_simple_list1 = instance
                .exports
                .get_native_function::<(i32, i32), ()>("simple-list1")?;
            let func_simple_list2 = instance
                .exports
                .get_native_function::<(), i32>("simple-list2")?;
            let func_simple_list4 = instance
                .exports
                .get_native_function::<(i32, i32), i32>("simple-list4")?;
            let memory = instance.exports.get_memory("memory")?.clone();
            Ok(SimpleLists {
                func_canonical_abi_free,
                func_canonical_abi_realloc,
                func_simple_list1,
                func_simple_list2,
                func_simple_list4,
                memory,
                state,
            })
        }
        pub fn simple_list1(&self, l: &[u32]) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let memory = &self.memory;
            let vec0 = l;
            let ptr0 = func_canonical_abi_realloc.call(0, 0, 4, (vec0.len() as i32) * 4)?;
            unsafe { memory.data_unchecked_mut() }.store_many(ptr0, &vec0)?;
            self.func_simple_list1.call(ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn simple_list2(&self) -> Result<Vec<u32>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let memory = &self.memory;
            let result0 = self.func_simple_list2.call()?;
            let load1 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 0)?;
            let load2 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(memory, func_canonical_abi_free, ptr3, len3, 4)?)
        }
        pub fn simple_list4(&self, l: &[&[u32]]) -> Result<Vec<Vec<u32>>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let memory = &self.memory;
            let vec1 = l;
            let len1 = vec1.len() as i32;
            let result1 = func_canonical_abi_realloc.call(0, 0, 4, len1 * 8)?;
            for (i, e) in vec1.into_iter().enumerate() {
                let base = result1 + (i as i32) * 8;
                {
                    let vec0 = e;
                    let ptr0 = func_canonical_abi_realloc.call(0, 0, 4, (vec0.len() as i32) * 4)?;
                    unsafe { memory.data_unchecked_mut() }.store_many(ptr0, &vec0)?;
                    unsafe { memory.data_unchecked_mut() }
                        .store(base + 4, wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32))?;
                    unsafe { memory.data_unchecked_mut() }
                        .store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                }
            }
            let result2 = self.func_simple_list4.call(result1, len1)?;
            let load3 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result2 + 0)?;
            let load4 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result2 + 4)?;
            let len8 = load4;
            let base8 = load3;
            let mut result8 = Vec::with_capacity(len8 as usize);
            for i in 0..len8 {
                let base = base8 + i * 8;
                result8.push({
                    let load5 = unsafe { memory.data_unchecked_mut() }.load::<i32>(base + 0)?;
                    let load6 = unsafe { memory.data_unchecked_mut() }.load::<i32>(base + 4)?;
                    let ptr7 = load5;
                    let len7 = load6;

                    copy_slice(memory, func_canonical_abi_free, ptr7, len7, 4)?
                });
            }
            func_canonical_abi_free.call(base8, len8 * 8, 4)?;
            Ok(result8)
        }
    }
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::RawMem;
}
