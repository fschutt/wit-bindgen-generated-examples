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
        func_invalid_bool: wasmer::NativeFunc<(), ()>,
        func_invalid_char: wasmer::NativeFunc<(), ()>,
        func_invalid_enum: wasmer::NativeFunc<(), ()>,
        func_invalid_handle: wasmer::NativeFunc<(), ()>,
        func_invalid_handle_close: wasmer::NativeFunc<(), ()>,
        func_invalid_s16: wasmer::NativeFunc<(), ()>,
        func_invalid_s8: wasmer::NativeFunc<(), ()>,
        func_invalid_u16: wasmer::NativeFunc<(), ()>,
        func_invalid_u8: wasmer::NativeFunc<(), ()>,
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
            let func_invalid_bool = instance
                .exports
                .get_native_function::<(), ()>("invalid-bool")?;
            let func_invalid_char = instance
                .exports
                .get_native_function::<(), ()>("invalid-char")?;
            let func_invalid_enum = instance
                .exports
                .get_native_function::<(), ()>("invalid-enum")?;
            let func_invalid_handle = instance
                .exports
                .get_native_function::<(), ()>("invalid-handle")?;
            let func_invalid_handle_close = instance
                .exports
                .get_native_function::<(), ()>("invalid-handle-close")?;
            let func_invalid_s16 = instance
                .exports
                .get_native_function::<(), ()>("invalid-s16")?;
            let func_invalid_s8 = instance
                .exports
                .get_native_function::<(), ()>("invalid-s8")?;
            let func_invalid_u16 = instance
                .exports
                .get_native_function::<(), ()>("invalid-u16")?;
            let func_invalid_u8 = instance
                .exports
                .get_native_function::<(), ()>("invalid-u8")?;
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
                state,
            })
        }
        pub fn invalid_u8(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_u8.call()?;
            Ok(())
        }
        pub fn invalid_s8(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_s8.call()?;
            Ok(())
        }
        pub fn invalid_u16(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_u16.call()?;
            Ok(())
        }
        pub fn invalid_s16(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_s16.call()?;
            Ok(())
        }
        pub fn invalid_char(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_char.call()?;
            Ok(())
        }
        pub fn invalid_bool(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_bool.call()?;
            Ok(())
        }
        pub fn invalid_enum(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_enum.call()?;
            Ok(())
        }
        pub fn invalid_handle(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_handle.call()?;
            Ok(())
        }
        pub fn invalid_handle_close(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_invalid_handle_close.call()?;
            Ok(())
        }
    }
}
