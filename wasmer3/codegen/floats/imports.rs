#[allow(clippy::all)]
pub mod floats {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct FloatsData {}

    pub struct Floats {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<FloatsData>,
        func_float32_param: wasmer::TypedFunction<f32, ()>,
        func_float32_result: wasmer::TypedFunction<(), f32>,
        func_float64_param: wasmer::TypedFunction<f64, ()>,
        func_float64_result: wasmer::TypedFunction<(), f64>,
    }
    impl Floats {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `FloatsData` which needs to be
        /// passed through to `Floats::new`.
        fn add_to_imports(
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<FloatsData> {
            let env = wasmer::FunctionEnv::new(&mut store, FloatsData::default());
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
            env: wasmer::FunctionEnv<FloatsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_float32_param = _instance
                .exports
                .get_typed_function(&store, "float32-param")?;
            let func_float32_result = _instance
                .exports
                .get_typed_function(&store, "float32-result")?;
            let func_float64_param = _instance
                .exports
                .get_typed_function(&store, "float64-param")?;
            let func_float64_result = _instance
                .exports
                .get_typed_function(&store, "float64-result")?;
            Ok(Floats {
                func_float32_param,
                func_float32_result,
                func_float64_param,
                func_float64_result,
                env,
            })
        }
        pub fn float32_param(
            &self,
            store: &mut wasmer::Store,
            x: f32,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_float32_param.call(store, x)?;
            Ok(())
        }
        pub fn float64_param(
            &self,
            store: &mut wasmer::Store,
            x: f64,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_float64_param.call(store, x)?;
            Ok(())
        }
        pub fn float32_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<f32, wasmer::RuntimeError> {
            let result0 = self.func_float32_result.call(store)?;
            Ok(result0)
        }
        pub fn float64_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<f64, wasmer::RuntimeError> {
            let result0 = self.func_float64_result.call(store)?;
            Ok(result0)
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
}
