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
        func_invalid_bool: wasmer::TypedFunction<(), ()>,
        func_invalid_char: wasmer::TypedFunction<(), ()>,
        func_invalid_enum: wasmer::TypedFunction<(), ()>,
        func_invalid_handle: wasmer::TypedFunction<(), ()>,
        func_invalid_handle_close: wasmer::TypedFunction<(), ()>,
        func_invalid_s16: wasmer::TypedFunction<(), ()>,
        func_invalid_s8: wasmer::TypedFunction<(), ()>,
        func_invalid_u16: wasmer::TypedFunction<(), ()>,
        func_invalid_u8: wasmer::TypedFunction<(), ()>,
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
            let func_invalid_bool = _instance
                .exports
                .get_typed_function(store, "invalid-bool")?;
            let func_invalid_char = _instance
                .exports
                .get_typed_function(store, "invalid-char")?;
            let func_invalid_enum = _instance
                .exports
                .get_typed_function(store, "invalid-enum")?;
            let func_invalid_handle = _instance
                .exports
                .get_typed_function(store, "invalid-handle")?;
            let func_invalid_handle_close = _instance
                .exports
                .get_typed_function(store, "invalid-handle-close")?;
            let func_invalid_s16 = _instance.exports.get_typed_function(store, "invalid-s16")?;
            let func_invalid_s8 = _instance.exports.get_typed_function(store, "invalid-s8")?;
            let func_invalid_u16 = _instance.exports.get_typed_function(store, "invalid-u16")?;
            let func_invalid_u8 = _instance.exports.get_typed_function(store, "invalid-u8")?;
            Ok(Exports {
                func_invalid_bool,
                func_invalid_char,
                func_invalid_enum,
                func_invalid_handle,
                func_invalid_handle_close,
                func_invalid_s16,
                func_invalid_s8,
                func_invalid_u16,
                func_invalid_u8,
                env,
            })
        }
        pub fn invalid_u8(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_u8.call(store)?;
            Ok(())
        }
        pub fn invalid_s8(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_s8.call(store)?;
            Ok(())
        }
        pub fn invalid_u16(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_u16.call(store)?;
            Ok(())
        }
        pub fn invalid_s16(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_s16.call(store)?;
            Ok(())
        }
        pub fn invalid_char(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_char.call(store)?;
            Ok(())
        }
        pub fn invalid_bool(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_bool.call(store)?;
            Ok(())
        }
        pub fn invalid_enum(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_enum.call(store)?;
            Ok(())
        }
        pub fn invalid_handle(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_handle.call(store)?;
            Ok(())
        }
        pub fn invalid_handle_close(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_handle_close.call(store)?;
            Ok(())
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
}
