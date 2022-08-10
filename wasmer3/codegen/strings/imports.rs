#[allow(clippy::all)]
pub mod strings {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct StringsData {}

    pub struct Strings {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<StringsData>,
        func_a: wasmer::TypedFunction<(i32, i32), ()>,
        func_b: wasmer::TypedFunction<(), i32>,
        func_c: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        func_canonical_abi_free: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
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
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<StringsData> {
            let env = wasmer::FunctionEnv::new(&mut store, StringsData::default());
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
            env: wasmer::FunctionEnv<StringsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_a = _instance.exports.get_typed_function(&store, "a")?;
            let func_b = _instance.exports.get_typed_function(&store, "b")?;
            let func_c = _instance.exports.get_typed_function(&store, "c")?;
            let func_canonical_abi_free = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_free")?;
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_realloc")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Strings {
                func_a,
                func_b,
                func_c,
                func_canonical_abi_free,
                func_canonical_abi_realloc,
                memory,
                env,
            })
        }
        pub fn a(&self, store: &mut wasmer::Store, x: &str) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec0.len() as i32,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .store_many(ptr0, vec0.as_bytes())?;
            self.func_a.call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn b(&self, store: &mut wasmer::Store) -> Result<String, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_b.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;

            let data3 = copy_slice(store, _memory, func_canonical_abi_free, ptr3, len3, 1)?;
            Ok(String::from_utf8(data3).map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?)
        }
        pub fn c(
            &self,
            store: &mut wasmer::Store,
            a: &str,
            b: &str,
        ) -> Result<String, wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let vec0 = a;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec0.len() as i32,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .store_many(ptr0, vec0.as_bytes())?;
            let vec1 = b;
            let ptr1 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec1.len() as i32,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .store_many(ptr1, vec1.as_bytes())?;
            let result2 =
                self.func_c
                    .call(store, ptr0, vec0.len() as i32, ptr1, vec1.len() as i32)?;
            let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result2 + 0)?;
            let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result2 + 4)?;
            let ptr5 = load3;
            let len5 = load4;

            let data5 = copy_slice(store, _memory, func_canonical_abi_free, ptr5, len5, 1)?;
            Ok(String::from_utf8(data5).map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?)
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::RawMem;
}
