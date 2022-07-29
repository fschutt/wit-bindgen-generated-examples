pub mod floats {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Floats: Sized + wasmer::WasmerEnv + 'static {
        fn float32_param(&mut self, x: f32) -> ();

        fn float64_param(&mut self, x: f64) -> ();

        fn float32_result(&mut self) -> f32;

        fn float64_result(&mut self) -> f64;
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: Floats,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Floats> {
            data: T,
        }
        unsafe impl<T: Floats> Send for EnvWrapper<T> {}
        unsafe impl<T: Floats> Sync for EnvWrapper<T> {}
        impl<T: Floats> wasmer::WasmerEnv for EnvWrapper<T> {
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
        exports.insert(
            "float32-param",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: f32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = arg0;
                    let result = host.float32_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "float64-param",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: f64|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = arg0;
                    let result = host.float64_param(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert("float32-result", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<f32, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.float32_result();
      Ok(result)
    }));
        exports.insert("float64-result", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<f64, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.float64_result();
      Ok(result)
    }));
        imports.register("floats", exports);
    }
}
