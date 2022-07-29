pub mod imports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum E {
        A,
        B,
        C,
    }
    impl std::fmt::Debug for E {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                E::A => f.debug_tuple("E::A").finish(),
                E::B => f.debug_tuple("E::B").finish(),
                E::C => f.debug_tuple("E::C").finish(),
            }
        }
    }
    pub trait Imports: Sized + wasmer::WasmerEnv + 'static {
        type HostState: std::fmt::Debug;
        fn roundtrip_u8(&mut self, a: u8) -> u8;

        fn roundtrip_s8(&mut self, a: i8) -> i8;

        fn roundtrip_u16(&mut self, a: u16) -> u16;

        fn roundtrip_s16(&mut self, a: i16) -> i16;

        fn roundtrip_char(&mut self, a: char) -> char;

        fn roundtrip_enum(&mut self, a: E) -> E;

        fn roundtrip_bool(&mut self, a: bool) -> bool;

        fn get_internal(&mut self, a: &Self::HostState) -> u32;

        fn drop_host_state(&mut self, state: Self::HostState) {
            drop(state);
        }
    }

    pub struct ImportsTables<T: Imports> {
        pub(crate) host_state_table: wit_bindgen_wasmer::Table<T::HostState>,
    }
    impl<T: Imports> Default for ImportsTables<T> {
        fn default() -> Self {
            Self {
                host_state_table: Default::default(),
            }
        }
    }
    impl<T: Imports> Clone for ImportsTables<T> {
        fn clone(&self) -> Self {
            Self::default()
        }
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: Imports,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Imports> {
            data: T,
            tables: ImportsTables<T>,
        }
        unsafe impl<T: Imports> Send for EnvWrapper<T> {}
        unsafe impl<T: Imports> Sync for EnvWrapper<T> {}
        impl<T: Imports> wasmer::WasmerEnv for EnvWrapper<T> {
            fn init_with_instance(
                &mut self,
                instance: &wasmer::Instance,
            ) -> Result<(), wasmer::HostEnvInitError> {
                self.data.init_with_instance(instance)?;
                Ok(())
            }
        }
        let env = std::sync::Arc::new(std::sync::Mutex::new(EnvWrapper {
            data,
            tables: ImportsTables::default(),
        }));
        let mut exports = wasmer::Exports::new();
        exports.insert(
            "roundtrip-u8",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = u8::try_from(arg0).map_err(bad_int)?;
                    let result = host.roundtrip_u8(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-s8",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = i8::try_from(arg0).map_err(bad_int)?;
                    let result = host.roundtrip_s8(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-u16",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = u16::try_from(arg0).map_err(bad_int)?;
                    let result = host.roundtrip_u16(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-s16",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = i16::try_from(arg0).map_err(bad_int)?;
                    let result = host.roundtrip_s16(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-char",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = char_from_i32(arg0)?;
                    let result = host.roundtrip_char(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-enum",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = match arg0 {
                        0 => E::A,
                        1 => E::B,
                        2 => E::C,
                        _ => return Err(invalid_variant("E")),
                    };
                    let result = host.roundtrip_enum(param0);
                    Ok(result as i32)
                },
            ),
        );
        exports.insert(
            "roundtrip-bool",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = match arg0 {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    };
                    let result = host.roundtrip_bool(param0);
                    Ok(match result {
                        true => 1,
                        false => 0,
                    })
                },
            ),
        );
        exports.insert(
            "get-internal",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let _tables = &mut env.tables;
                    let param0 = _tables
                        .host_state_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let result = host.get_internal(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        imports.register("imports", exports);
        let mut canonical_abi = imports
            .get_namespace_exports("canonical_abi")
            .unwrap_or_else(wasmer::Exports::new);
        canonical_abi.insert(
            "resource_drop_host-state",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      handle: u32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let handle = env.tables.host_state_table.remove(handle).map_err(|e| {
                        wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
                    })?;
                    env.data.drop_host_state(handle);
                    Ok(())
                },
            ),
        );
        imports.register("canonical_abi", canonical_abi);
    }
    use core::convert::TryFrom;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::char_from_i32;
    use wit_bindgen_wasmer::rt::invalid_variant;
}
