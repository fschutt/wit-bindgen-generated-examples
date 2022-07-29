#[allow(clippy::all)]
pub mod smoke {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Smoke: Sized + Send + Sync + 'static {
        fn y(&mut self) -> ();
    }

    #[must_use = "The returned initializer function must be called
  with the instance and the store before starting the runtime"]
    pub fn add_to_imports<T>(
        store: &mut wasmer::Store,
        imports: &mut wasmer::Imports,
        data: T,
    ) -> impl FnOnce(&wasmer::Instance, &dyn wasmer::AsStoreRef) -> Result<(), anyhow::Error>
    where
        T: Smoke,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Smoke> {
            data: T,
        }
        unsafe impl<T: Smoke> Send for EnvWrapper<T> {}
        unsafe impl<T: Smoke> Sync for EnvWrapper<T> {}
        let env = EnvWrapper { data };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
    "y",
    wasmer::Function::new_native(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<(), wasmer::RuntimeError> {
      let data_mut = store.data_mut();
      let host = &mut data_mut.data;
      let result = host.y();
      let () = result;
      Ok(())
    }
    ));
        imports.register_namespace("smoke", exports);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| Ok(())
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
}
