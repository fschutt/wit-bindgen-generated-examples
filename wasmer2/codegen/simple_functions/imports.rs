pub mod simple_functions {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct SimpleFunctionsData {}
    impl wasmer::WasmerEnv for SimpleFunctionsData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            Ok(())
        }
    }
    impl Clone for SimpleFunctionsData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct SimpleFunctions {
        state: std::sync::Arc<std::sync::Mutex<SimpleFunctionsData>>,
        func_f1: wasmer::NativeFunc<(), ()>,
        func_f2: wasmer::NativeFunc<i32, ()>,
        func_f3: wasmer::NativeFunc<(i32, i32), ()>,
        func_f4: wasmer::NativeFunc<(), i32>,
    }
    impl SimpleFunctions {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `SimpleFunctionsData` which needs to be
        /// passed through to `SimpleFunctions::new`.
        fn add_to_imports(
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<SimpleFunctionsData>> {
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
            state: std::sync::Arc<std::sync::Mutex<SimpleFunctionsData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_f1 = instance.exports.get_native_function::<(), ()>("f1")?;
            let func_f2 = instance.exports.get_native_function::<i32, ()>("f2")?;
            let func_f3 = instance
                .exports
                .get_native_function::<(i32, i32), ()>("f3")?;
            let func_f4 = instance.exports.get_native_function::<(), i32>("f4")?;
            Ok(SimpleFunctions {
                func_f1,
                func_f2,
                func_f3,
                func_f4,
                state,
            })
        }
        pub fn f1(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_f1.call()?;
            Ok(())
        }
        pub fn f2(&self, a: u32) -> Result<(), wasmer::RuntimeError> {
            self.func_f2.call(wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(())
        }
        pub fn f3(&self, a: u32, b: u32) -> Result<(), wasmer::RuntimeError> {
            self.func_f3.call(
                wit_bindgen_wasmer::rt::as_i32(a),
                wit_bindgen_wasmer::rt::as_i32(b),
            )?;
            Ok(())
        }
        pub fn f4(&self) -> Result<u32, wasmer::RuntimeError> {
            let result0 = self.func_f4.call()?;
            Ok(result0 as u32)
        }
    }
}
