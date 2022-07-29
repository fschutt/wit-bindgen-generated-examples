pub mod exports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ExportsData {}
    impl wasmer::WasmerEnv for ExportsData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            Ok(())
        }
    }
    impl Clone for ExportsData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct Exports {
        state: std::sync::Arc<std::sync::Mutex<ExportsData>>,
        func_canonical_abi_realloc: wasmer::NativeFunc<(i32, i32, i32, i32), i32>,
        func_many_arguments: wasmer::NativeFunc<i32, ()>,
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
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<ExportsData>> {
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
            state: std::sync::Arc<std::sync::Mutex<ExportsData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_canonical_abi_realloc = instance
                .exports
                .get_native_function::<(i32, i32, i32, i32), i32>("canonical_abi_realloc")?;
            let func_many_arguments = instance
                .exports
                .get_native_function::<i32, ()>("many-arguments")?;
            let memory = instance.exports.get_memory("memory")?.clone();
            Ok(Exports {
                func_canonical_abi_realloc,
                func_many_arguments,
                memory,
                state,
            })
        }
        pub fn many_arguments(
            &self,
            a1: u64,
            a2: u64,
            a3: u64,
            a4: u64,
            a5: u64,
            a6: u64,
            a7: u64,
            a8: u64,
            a9: u64,
            a10: u64,
            a11: u64,
            a12: u64,
            a13: u64,
            a14: u64,
            a15: u64,
            a16: u64,
            a17: u64,
            a18: u64,
            a19: u64,
            a20: u64,
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let memory = &self.memory;
            let ptr0 = func_canonical_abi_realloc.call(0, 0, 8, 160)?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 0,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a1)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 8,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a2)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 16,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a3)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 24,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a4)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 32,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a5)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 40,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a6)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 48,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a7)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 56,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a8)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 64,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a9)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 72,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a10)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 80,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a11)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 88,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a12)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 96,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a13)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 104,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a14)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 112,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a15)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 120,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a16)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 128,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a17)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 136,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a18)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 144,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a19)),
            )?;
            unsafe { memory.data_unchecked_mut() }.store(
                ptr0 + 152,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a20)),
            )?;
            self.func_many_arguments.call(ptr0)?;
            Ok(())
        }
    }
    use wit_bindgen_wasmer::rt::RawMem;
}
