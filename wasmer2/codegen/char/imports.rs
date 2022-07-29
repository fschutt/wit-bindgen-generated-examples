pub mod char {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct CharData {}
    impl wasmer::WasmerEnv for CharData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            Ok(())
        }
    }
    impl Clone for CharData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct Char {
        state: std::sync::Arc<std::sync::Mutex<CharData>>,
        func_return_char: wasmer::NativeFunc<(), i32>,
        func_take_char: wasmer::NativeFunc<i32, ()>,
    }
    impl Char {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `CharData` which needs to be
        /// passed through to `Char::new`.
        fn add_to_imports(
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<CharData>> {
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
            state: std::sync::Arc<std::sync::Mutex<CharData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_return_char = instance
                .exports
                .get_native_function::<(), i32>("return-char")?;
            let func_take_char = instance
                .exports
                .get_native_function::<i32, ()>("take-char")?;
            Ok(Char {
                func_return_char,
                func_take_char,
                state,
            })
        }
        /// A function that accepts a character
        pub fn take_char(&self, x: char) -> Result<(), wasmer::RuntimeError> {
            self.func_take_char
                .call(wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        /// A function that returns a character
        pub fn return_char(&self) -> Result<char, wasmer::RuntimeError> {
            let result0 = self.func_return_char.call()?;
            Ok(char_from_i32(result0)?)
        }
    }
    use wit_bindgen_wasmer::rt::char_from_i32;
}
