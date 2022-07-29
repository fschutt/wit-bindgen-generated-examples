#[allow(clippy::all)]
pub mod small_anonymous {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Error {
        Success,
        Failure,
    }
    impl core::fmt::Debug for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Error::Success => f.debug_tuple("Error::Success").finish(),
                Error::Failure => f.debug_tuple("Error::Failure").finish(),
            }
        }
    }

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct SmallAnonymousData {}

    pub struct SmallAnonymous {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<SmallAnonymousData>,
        func_canonical_abi_free: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_option_test: wasmer::TypedFunction<(), i32>,
        memory: wasmer::Memory,
    }
    impl SmallAnonymous {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `SmallAnonymousData` which needs to be
        /// passed through to `SmallAnonymous::new`.
        fn add_to_imports(
            store: &mut wasmer::StoreMut<'_>,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<SmallAnonymousData> {
            let env = wasmer::FunctionEnv::new(store, Default::default());
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
            env: wasmer::FunctionEnv<SmallAnonymousData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_canonical_abi_free = _instance
                .exports
                .get_typed_function(store, "canonical_abi_free")?;
            let func_option_test = _instance.exports.get_typed_function(store, "option-test")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(SmallAnonymous {
                func_canonical_abi_free,
                func_option_test,
                memory,
                env,
            })
        }
        pub fn option_test(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Result<Option<String>, Error>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_option_test.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => Ok({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(result0 + 4)?;
                    match i32::from(load2) {
                        0 => None,
                        1 => Some({
                            let load3 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(result0 + 8)?;
                            let load4 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(result0 + 12)?;
                            let ptr5 = load3;
                            let len5 = load4;

                            let data5 =
                                copy_slice(store, _memory, func_canonical_abi_free, ptr5, len5, 1)?;
                            String::from_utf8(data5)
                                .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                        }),
                        _ => return Err(invalid_variant("option")),
                    }
                }),
                1 => Err({
                    let load6 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(result0 + 4)?;
                    match i32::from(load6) {
                        0 => Error::Success,
                        1 => Error::Failure,
                        _ => return Err(invalid_variant("Error")),
                    }
                }),
                _ => return Err(invalid_variant("expected")),
            })
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
