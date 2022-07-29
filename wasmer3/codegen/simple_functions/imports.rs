#[allow(clippy::all)]
pub mod simple_functions {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct SimpleFunctionsData {}

    pub struct SimpleFunctions {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<SimpleFunctionsData>,
        func_f1: wasmer::TypedFunction<(), ()>,
        func_f2: wasmer::TypedFunction<i32, ()>,
        func_f3: wasmer::TypedFunction<(i32, i32), ()>,
        func_f4: wasmer::TypedFunction<(), i32>,
    }
    impl SimpleFunctions {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `SimpleFunctionsData` which needs to be
        /// passed through to `SimpleFunctions::new`.
        fn add_to_imports(
            store: &mut wasmer::StoreMut<'_>,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<SimpleFunctionsData> {
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
            env: wasmer::FunctionEnv<SimpleFunctionsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_f1 = _instance.exports.get_typed_function(store, "f1")?;
            let func_f2 = _instance.exports.get_typed_function(store, "f2")?;
            let func_f3 = _instance.exports.get_typed_function(store, "f3")?;
            let func_f4 = _instance.exports.get_typed_function(store, "f4")?;
            Ok(SimpleFunctions {
                func_f1,
                func_f2,
                func_f3,
                func_f4,
                env,
            })
        }
        pub fn f1(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_f1.call(store)?;
            Ok(())
        }
        pub fn f2(&self, store: &mut wasmer::Store, a: u32) -> Result<(), wasmer::RuntimeError> {
            self.func_f2
                .call(store, wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(())
        }
        pub fn f3(
            &self,
            store: &mut wasmer::Store,
            a: u32,
            b: u32,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_f3.call(
                store,
                wit_bindgen_wasmer::rt::as_i32(a),
                wit_bindgen_wasmer::rt::as_i32(b),
            )?;
            Ok(())
        }
        pub fn f4(&self, store: &mut wasmer::Store) -> Result<u32, wasmer::RuntimeError> {
            let result0 = self.func_f4.call(store)?;
            Ok(result0 as u32)
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
}
