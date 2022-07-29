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
        func_thunk: wasmer::TypedFunction<(), ()>,
    }
    impl Exports {
        #[allow(unused_variables)]

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
            env: wasmer::FunctionEnv<ExportsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_thunk = _instance.exports.get_typed_function(store, "thunk")?;
            Ok(Exports { func_thunk, env })
        }
        pub fn thunk(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_thunk.call(store)?;
            Ok(())
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
}
