#[allow(clippy::all)]
pub mod floats {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Floats: Sized + Send + Sync + 'static {
        fn float32_param(&mut self, x: f32) -> ();

        fn float64_param(&mut self, x: f64) -> ();

        fn float32_result(&mut self) -> f32;

        fn float64_result(&mut self) -> f64;
    }

    #[must_use = "The returned initializer function must be called
  with the instance and the store before starting the runtime"]
    pub fn add_to_imports<T>(
        store: &mut wasmer::Store,
        imports: &mut wasmer::Imports,
        data: T,
    ) -> impl FnOnce(&wasmer::Instance, &dyn wasmer::AsStoreRef) -> Result<(), anyhow::Error>
    where
        T: Floats,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Floats> {
            data: T,
        }
        unsafe impl<T: Floats> Send for EnvWrapper<T> {}
        unsafe impl<T: Floats> Sync for EnvWrapper<T> {}
        let env = EnvWrapper { data };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
            "float32-param",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: f32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = arg0;
                    let host = &mut data_mut.data;
                    let result = host.float32_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "float64-param",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: f64|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = arg0;
                    let host = &mut data_mut.data;
                    let result = host.float64_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
    "float32-result",
    wasmer::Function::new_typed_with_env(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<f32, wasmer::RuntimeError> {
      let data_mut = store.data_mut();
      let host = &mut data_mut.data;
      let result = host.float32_result();
      Ok(result)
    }
    ));
        exports.insert(
    "float64-result",
    wasmer::Function::new_typed_with_env(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<f64, wasmer::RuntimeError> {
      let data_mut = store.data_mut();
      let host = &mut data_mut.data;
      let result = host.float64_result();
      Ok(result)
    }
    ));
        imports.register_namespace("floats", exports);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| Ok(())
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
}
