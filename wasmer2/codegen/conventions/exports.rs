pub mod conventions {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct LudicrousSpeed {
        pub how_fast_are_you_going: u32,
        pub i_am_going_extremely_slow: u64,
    }
    impl std::fmt::Debug for LudicrousSpeed {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("LudicrousSpeed")
                .field("how-fast-are-you-going", &self.how_fast_are_you_going)
                .field("i-am-going-extremely-slow", &self.i_am_going_extremely_slow)
                .finish()
        }
    }
    impl wit_bindgen_wasmer::Endian for LudicrousSpeed {
        fn into_le(self) -> Self {
            Self {
                how_fast_are_you_going: self.how_fast_are_you_going.into_le(),
                i_am_going_extremely_slow: self.i_am_going_extremely_slow.into_le(),
            }
        }
        fn from_le(self) -> Self {
            Self {
                how_fast_are_you_going: self.how_fast_are_you_going.from_le(),
                i_am_going_extremely_slow: self.i_am_going_extremely_slow.from_le(),
            }
        }
    }
    unsafe impl wit_bindgen_wasmer::AllBytesValid for LudicrousSpeed {}
    pub trait Conventions: Sized + wasmer::WasmerEnv + 'static {
        fn kebab_case(&mut self) -> ();

        fn foo(&mut self, x: LudicrousSpeed) -> ();

        fn function_with_dashes(&mut self) -> ();

        fn function_with_no_weird_characters(&mut self) -> ();

        fn apple(&mut self) -> ();

        fn apple_pear(&mut self) -> ();

        fn apple_pear_grape(&mut self) -> ();

        fn garçon(&mut self) -> ();

        fn hühnervögel(&mut self) -> ();

        fn москва(&mut self) -> ();

        fn 東_京(&mut self) -> ();

        fn garçon_hühnervögel_москва_東_京(&mut self) -> ();

        fn a0(&mut self) -> ();

        fn explicit(&mut self) -> ();

        fn explicit_kebab(&mut self) -> ();

        fn bool(&mut self) -> ();
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: Conventions,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Conventions> {
            data: T,
        }
        unsafe impl<T: Conventions> Send for EnvWrapper<T> {}
        unsafe impl<T: Conventions> Sync for EnvWrapper<T> {}
        impl<T: Conventions> wasmer::WasmerEnv for EnvWrapper<T> {
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
        exports.insert("kebab-case", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.kebab_case();
      let () = result;
      Ok(())
    }));
        exports.insert(
            "foo",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i64|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = LudicrousSpeed {
                        how_fast_are_you_going: arg0 as u32,
                        i_am_going_extremely_slow: arg1 as u64,
                    };
                    let result = host.foo(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert("function-with-dashes", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.function_with_dashes();
      let () = result;
      Ok(())
    }));
        exports.insert("function-with-no-weird-characters", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.function_with_no_weird_characters();
      let () = result;
      Ok(())
    }));
        exports.insert("apple", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.apple();
      let () = result;
      Ok(())
    }));
        exports.insert("apple-pear", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.apple_pear();
      let () = result;
      Ok(())
    }));
        exports.insert("apple-pear-grape", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.apple_pear_grape();
      let () = result;
      Ok(())
    }));
        exports.insert("garçon", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.garçon();
      let () = result;
      Ok(())
    }));
        exports.insert("hühnervögel", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.hühnervögel();
      let () = result;
      Ok(())
    }));
        exports.insert("москва", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.москва();
      let () = result;
      Ok(())
    }));
        exports.insert("東-京", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.東_京();
      let () = result;
      Ok(())
    }));
        exports.insert("garçon-hühnervögel-москва-東-京", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.garçon_hühnervögel_москва_東_京();
      let () = result;
      Ok(())
    }));
        exports.insert("a0", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.a0();
      let () = result;
      Ok(())
    }));
        exports.insert("explicit", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.explicit();
      let () = result;
      Ok(())
    }));
        exports.insert("explicit-kebab", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.explicit_kebab();
      let () = result;
      Ok(())
    }));
        exports.insert("bool", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.bool();
      let () = result;
      Ok(())
    }));
        imports.register("conventions", exports);
    }
}
