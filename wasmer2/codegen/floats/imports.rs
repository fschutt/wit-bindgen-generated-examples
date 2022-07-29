pub mod floats {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct FloatsData {}
    impl wasmer::WasmerEnv for FloatsData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            Ok(())
        }
    }
    impl Clone for FloatsData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct Floats {
        state: std::sync::Arc<std::sync::Mutex<FloatsData>>,
        func_float32_param: wasmer::NativeFunc<f32, ()>,
        func_float32_result: wasmer::NativeFunc<(), f32>,
        func_float64_param: wasmer::NativeFunc<f64, ()>,
        func_float64_result: wasmer::NativeFunc<(), f64>,
    }
    impl Floats {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `FloatsData` which needs to be
        /// passed through to `Floats::new`.
        fn add_to_imports(
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<FloatsData>> {
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
            state: std::sync::Arc<std::sync::Mutex<FloatsData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_float32_param = instance
                .exports
                .get_native_function::<f32, ()>("float32-param")?;
            let func_float32_result = instance
                .exports
                .get_native_function::<(), f32>("float32-result")?;
            let func_float64_param = instance
                .exports
                .get_native_function::<f64, ()>("float64-param")?;
            let func_float64_result = instance
                .exports
                .get_native_function::<(), f64>("float64-result")?;
            Ok(Floats {
                func_float32_param,
                func_float32_result,
                func_float64_param,
                func_float64_result,
                state,
            })
        }
        pub fn float32_param(&self, x: f32) -> Result<(), wasmer::RuntimeError> {
            self.func_float32_param.call(x)?;
            Ok(())
        }
        pub fn float64_param(&self, x: f64) -> Result<(), wasmer::RuntimeError> {
            self.func_float64_param.call(x)?;
            Ok(())
        }
        pub fn float32_result(&self) -> Result<f32, wasmer::RuntimeError> {
            let result0 = self.func_float32_result.call()?;
            Ok(result0)
        }
        pub fn float64_result(&self) -> Result<f64, wasmer::RuntimeError> {
            let result0 = self.func_float64_result.call()?;
            Ok(result0)
        }
    }
}
