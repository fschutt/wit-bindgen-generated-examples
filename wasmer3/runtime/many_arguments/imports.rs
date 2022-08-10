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
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        func_many_arguments: wasmer::TypedFunction<i32, ()>,
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
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_realloc")?;
            let func_many_arguments = _instance
                .exports
                .get_typed_function(&store, "many-arguments")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Exports {
                func_canonical_abi_realloc,
                func_many_arguments,
                memory,
                env,
            })
        }
        pub fn many_arguments(
            &self,
            store: &mut wasmer::Store,
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
            let _memory = &self.memory;
            let ptr0 = func_canonical_abi_realloc.call(store, 0, 0, 8, 160)?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 0,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a1)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 8,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a2)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 16,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a3)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 24,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a4)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 32,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a5)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 40,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a6)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 48,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a7)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 56,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a8)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 64,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a9)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 72,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a10)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 80,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a11)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 88,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a12)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 96,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a13)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 104,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a14)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 112,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a15)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 120,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a16)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 128,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a17)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 136,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a18)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 144,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a19)),
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                ptr0 + 152,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a20)),
            )?;
            self.func_many_arguments.call(store, ptr0)?;
            Ok(())
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::RawMem;
}
