#[allow(clippy::all)]
pub mod char {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Char: Sized + Send + Sync + 'static {
        /// A function that accepts a character
        fn take_char(&mut self, x: char) -> ();

        /// A function that returns a character
        fn return_char(&mut self) -> char;
    }

    #[must_use = "The returned initializer function must be called
  with the instance and the store before starting the runtime"]
    pub fn add_to_imports<T>(
        store: &mut wasmer::Store,
        imports: &mut wasmer::Imports,
        data: T,
    ) -> impl FnOnce(&wasmer::Instance, &dyn wasmer::AsStoreRef) -> Result<(), anyhow::Error>
    where
        T: Char,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Char> {
            data: T,
        }
        unsafe impl<T: Char> Send for EnvWrapper<T> {}
        unsafe impl<T: Char> Sync for EnvWrapper<T> {}
        let env = EnvWrapper { data };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
            "take-char",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = char_from_i32(arg0)?;
                    let host = &mut data_mut.data;
                    let result = host.take_char(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
    "return-char",
    wasmer::Function::new_typed_with_env(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
      let data_mut = store.data_mut();
      let host = &mut data_mut.data;
      let result = host.return_char();
      Ok(wit_bindgen_wasmer::rt::as_i32(result))
    }
    ));
        imports.register_namespace("char", exports);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| Ok(())
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::char_from_i32;
}
