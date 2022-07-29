pub mod char {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Char: Sized + wasmer::WasmerEnv + 'static {
        /// A function that accepts a character
        fn take_char(&mut self, x: char) -> ();

        /// A function that returns a character
        fn return_char(&mut self) -> char;
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: Char,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Char> {
            data: T,
        }
        unsafe impl<T: Char> Send for EnvWrapper<T> {}
        unsafe impl<T: Char> Sync for EnvWrapper<T> {}
        impl<T: Char> wasmer::WasmerEnv for EnvWrapper<T> {
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
            "take-char",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = char_from_i32(arg0)?;
                    let result = host.take_char(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert("return-char", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.return_char();
      Ok(wit_bindgen_wasmer::rt::as_i32(result))
    }));
        imports.register("char", exports);
    }
    use wit_bindgen_wasmer::rt::char_from_i32;
}
