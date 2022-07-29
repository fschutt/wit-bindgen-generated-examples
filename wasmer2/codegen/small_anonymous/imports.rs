pub mod small_anonymous {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Error {
        Success,
        Failure,
    }
    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::Success => f.debug_tuple("Error::Success").finish(),
                Error::Failure => f.debug_tuple("Error::Failure").finish(),
            }
        }
    }

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct SmallAnonymousData {}
    impl wasmer::WasmerEnv for SmallAnonymousData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            Ok(())
        }
    }
    impl Clone for SmallAnonymousData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct SmallAnonymous {
        state: std::sync::Arc<std::sync::Mutex<SmallAnonymousData>>,
        func_canonical_abi_free: wasmer::NativeFunc<(i32, i32, i32), ()>,
        func_option_test: wasmer::NativeFunc<(), i32>,
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
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<SmallAnonymousData>> {
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
            state: std::sync::Arc<std::sync::Mutex<SmallAnonymousData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_canonical_abi_free = instance
                .exports
                .get_native_function::<(i32, i32, i32), ()>("canonical_abi_free")?;
            let func_option_test = instance
                .exports
                .get_native_function::<(), i32>("option-test")?;
            let memory = instance.exports.get_memory("memory")?.clone();
            Ok(SmallAnonymous {
                func_canonical_abi_free,
                func_option_test,
                memory,
                state,
            })
        }
        pub fn option_test(&self) -> Result<Result<Option<String>, Error>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let memory = &self.memory;
            let result0 = self.func_option_test.call()?;
            let load1 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => Ok({
                    let load2 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 4)?;
                    match i32::from(load2) {
                        0 => None,
                        1 => Some({
                            let load3 =
                                unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 8)?;
                            let load4 =
                                unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 12)?;
                            let ptr5 = load3;
                            let len5 = load4;

                            let data5 = copy_slice(memory, func_canonical_abi_free, ptr5, len5, 1)?;
                            String::from_utf8(data5)
                                .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                        }),
                        _ => return Err(invalid_variant("option")),
                    }
                }),
                1 => Err({
                    let load6 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 4)?;
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
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
