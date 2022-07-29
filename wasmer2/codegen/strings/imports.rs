pub mod strings {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct StringsData {}
    impl wasmer::WasmerEnv for StringsData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            Ok(())
        }
    }
    impl Clone for StringsData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct Strings {
        state: std::sync::Arc<std::sync::Mutex<StringsData>>,
        func_a: wasmer::NativeFunc<(i32, i32), ()>,
        func_b: wasmer::NativeFunc<(), i32>,
        func_c: wasmer::NativeFunc<(i32, i32, i32, i32), i32>,
        func_canonical_abi_free: wasmer::NativeFunc<(i32, i32, i32), ()>,
        func_canonical_abi_realloc: wasmer::NativeFunc<(i32, i32, i32, i32), i32>,
        memory: wasmer::Memory,
    }
    impl Strings {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `StringsData` which needs to be
        /// passed through to `Strings::new`.
        fn add_to_imports(
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<StringsData>> {
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
            state: std::sync::Arc<std::sync::Mutex<StringsData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_a = instance
                .exports
                .get_native_function::<(i32, i32), ()>("a")?;
            let func_b = instance.exports.get_native_function::<(), i32>("b")?;
            let func_c = instance
                .exports
                .get_native_function::<(i32, i32, i32, i32), i32>("c")?;
            let func_canonical_abi_free = instance
                .exports
                .get_native_function::<(i32, i32, i32), ()>("canonical_abi_free")?;
            let func_canonical_abi_realloc = instance
                .exports
                .get_native_function::<(i32, i32, i32, i32), i32>("canonical_abi_realloc")?;
            let memory = instance.exports.get_memory("memory")?.clone();
            Ok(Strings {
                func_a,
                func_b,
                func_c,
                func_canonical_abi_free,
                func_canonical_abi_realloc,
                memory,
                state,
            })
        }
        pub fn a(&self, x: &str) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(0, 0, 1, vec0.len() as i32)?;
            unsafe { memory.data_unchecked_mut() }.store_many(ptr0, vec0.as_bytes())?;
            self.func_a.call(ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn b(&self) -> Result<String, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let memory = &self.memory;
            let result0 = self.func_b.call()?;
            let load1 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 0)?;
            let load2 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;

            let data3 = copy_slice(memory, func_canonical_abi_free, ptr3, len3, 1)?;
            Ok(String::from_utf8(data3).map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?)
        }
        pub fn c(&self, a: &str, b: &str) -> Result<String, wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let memory = &self.memory;
            let vec0 = a;
            let ptr0 = func_canonical_abi_realloc.call(0, 0, 1, vec0.len() as i32)?;
            unsafe { memory.data_unchecked_mut() }.store_many(ptr0, vec0.as_bytes())?;
            let vec1 = b;
            let ptr1 = func_canonical_abi_realloc.call(0, 0, 1, vec1.len() as i32)?;
            unsafe { memory.data_unchecked_mut() }.store_many(ptr1, vec1.as_bytes())?;
            let result2 = self
                .func_c
                .call(ptr0, vec0.len() as i32, ptr1, vec1.len() as i32)?;
            let load3 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result2 + 0)?;
            let load4 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result2 + 4)?;
            let ptr5 = load3;
            let len5 = load4;

            let data5 = copy_slice(memory, func_canonical_abi_free, ptr5, len5, 1)?;
            Ok(String::from_utf8(data5).map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?)
        }
    }
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::RawMem;
}
