#[allow(clippy::all)]
pub mod char {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct CharData {}

    pub struct Char {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<CharData>,
        func_return_char: wasmer::TypedFunction<(), i32>,
        func_take_char: wasmer::TypedFunction<i32, ()>,
    }
    impl Char {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `CharData` which needs to be
        /// passed through to `Char::new`.
        fn add_to_imports(
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<CharData> {
            let env = wasmer::FunctionEnv::new(&mut store, CharData::default());
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
            env: wasmer::FunctionEnv<CharData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_return_char = _instance
                .exports
                .get_typed_function(&store, "return-char")?;
            let func_take_char = _instance.exports.get_typed_function(&store, "take-char")?;
            Ok(Char {
                func_return_char,
                func_take_char,
                env,
            })
        }
        /// A function that accepts a character
        pub fn take_char(
            &self,
            store: &mut wasmer::Store,
            x: char,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_take_char
                .call(store, wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        /// A function that returns a character
        pub fn return_char(&self, store: &mut wasmer::Store) -> Result<char, wasmer::RuntimeError> {
            let result0 = self.func_return_char.call(store)?;
            Ok(char_from_i32(result0)?)
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::char_from_i32;
}
