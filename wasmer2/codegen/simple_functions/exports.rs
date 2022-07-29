pub mod simple_functions {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait SimpleFunctions: Sized + wasmer::WasmerEnv + 'static {
        fn f1(&mut self) -> ();

        fn f2(&mut self, a: u32) -> ();

        fn f3(&mut self, a: u32, b: u32) -> ();

        fn f4(&mut self) -> u32;
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: SimpleFunctions,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: SimpleFunctions> {
            data: T,
        }
        unsafe impl<T: SimpleFunctions> Send for EnvWrapper<T> {}
        unsafe impl<T: SimpleFunctions> Sync for EnvWrapper<T> {}
        impl<T: SimpleFunctions> wasmer::WasmerEnv for EnvWrapper<T> {
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
        exports.insert("f1", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.f1();
      let () = result;
      Ok(())
    }));
        exports.insert(
            "f2",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = arg0 as u32;
                    let result = host.f2(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "f3",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = arg0 as u32;
                    let param1 = arg1 as u32;
                    let result = host.f3(param0, param1);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert("f4", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.f4();
      Ok(wit_bindgen_wasmer::rt::as_i32(result))
    }));
        imports.register("simple-functions", exports);
    }
}
