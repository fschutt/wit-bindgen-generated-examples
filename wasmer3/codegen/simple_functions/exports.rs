#[allow(clippy::all)]
pub mod simple_functions {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait SimpleFunctions: Sized + Send + Sync + 'static {
        fn f1(&mut self) -> ();

        fn f2(&mut self, a: u32) -> ();

        fn f3(&mut self, a: u32, b: u32) -> ();

        fn f4(&mut self) -> u32;
    }

    #[must_use = "The returned initializer function must be called
  with the instance and the store before starting the runtime"]
    pub fn add_to_imports<T>(
        store: &mut wasmer::Store,
        imports: &mut wasmer::Imports,
        data: T,
    ) -> impl FnOnce(&wasmer::Instance, &dyn wasmer::AsStoreRef) -> Result<(), anyhow::Error>
    where
        T: SimpleFunctions,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: SimpleFunctions> {
            data: T,
        }
        unsafe impl<T: SimpleFunctions> Send for EnvWrapper<T> {}
        unsafe impl<T: SimpleFunctions> Sync for EnvWrapper<T> {}
        let env = EnvWrapper { data };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
    "f1",
    wasmer::Function::new_native(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<(), wasmer::RuntimeError> {
      let data_mut = store.data_mut();
      let host = &mut data_mut.data;
      let result = host.f1();
      let () = result;
      Ok(())
    }
    ));
        exports.insert(
            "f2",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = arg0 as u32;
                    let host = &mut data_mut.data;
                    let result = host.f2(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "f3",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = arg0 as u32;
                    let param1 = arg1 as u32;
                    let host = &mut data_mut.data;
                    let result = host.f3(param0, param1);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
    "f4",
    wasmer::Function::new_native(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
      let data_mut = store.data_mut();
      let host = &mut data_mut.data;
      let result = host.f4();
      Ok(wit_bindgen_wasmer::rt::as_i32(result))
    }
    ));
        imports.register_namespace("simple-functions", exports);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| Ok(())
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
}
