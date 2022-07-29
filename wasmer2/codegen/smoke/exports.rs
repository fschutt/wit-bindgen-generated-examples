pub mod smoke {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Smoke: Sized + wasmer::WasmerEnv + 'static {
        fn y(&mut self) -> ();
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: Smoke,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Smoke> {
            data: T,
        }
        unsafe impl<T: Smoke> Send for EnvWrapper<T> {}
        unsafe impl<T: Smoke> Sync for EnvWrapper<T> {}
        impl<T: Smoke> wasmer::WasmerEnv for EnvWrapper<T> {
            fn init_with_instance(
                &mut self,
                instance: &wasmer::Instance,
            ) -> Result<(), wasmer::HostEnvInitError> {
                self.data.init_with_instance(instance)?;
                Ok(())
            }
        }
        let env = std::sync::Arc::new(std::sync::Mutex::new(EnvWrapper { data }));
        let mut exports = wasmer::Exports::new();
        exports.insert("y", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.y();
      let () = result;
      Ok(())
    }));
        imports.register("smoke", exports);
    }
}
