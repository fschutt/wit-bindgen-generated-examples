#[allow(clippy::all)]
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
    impl core::fmt::Debug for E {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                E::A => f.debug_tuple("E::A").finish(),
                E::B => f.debug_tuple("E::B").finish(),
                E::C => f.debug_tuple("E::C").finish(),
            }
        }
    }
    pub trait Imports: Sized + Send + Sync + 'static {
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

    #[must_use = "The returned initializer function must be called
      with the instance and the store before starting the runtime"]
    pub fn add_to_imports<T>(
        store: &mut wasmer::Store,
        imports: &mut wasmer::Imports,
        data: T,
    ) -> impl FnOnce(&wasmer::Instance, &dyn wasmer::AsStoreRef) -> Result<(), anyhow::Error>
    where
        T: Imports,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Imports> {
            data: T,
            tables: std::rc::Rc<core::cell::RefCell<ImportsTables<T>>>,
        }
        unsafe impl<T: Imports> Send for EnvWrapper<T> {}
        unsafe impl<T: Imports> Sync for EnvWrapper<T> {}
        let env = EnvWrapper {
            data,
            tables: std::rc::Rc::default(),
        };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
            "roundtrip-u8",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = u8::try_from(arg0).map_err(bad_int)?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_u8(param0);
                    drop(tables);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-s8",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = i8::try_from(arg0).map_err(bad_int)?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_s8(param0);
                    drop(tables);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-u16",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = u16::try_from(arg0).map_err(bad_int)?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_u16(param0);
                    drop(tables);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-s16",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = i16::try_from(arg0).map_err(bad_int)?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_s16(param0);
                    drop(tables);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-char",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = char_from_i32(arg0)?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_char(param0);
                    drop(tables);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-enum",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = match arg0 {
                        0 => E::A,
                        1 => E::B,
                        2 => E::C,
                        _ => return Err(invalid_variant("E")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_enum(param0);
                    drop(tables);
                    Ok(result as i32)
                },
            ),
        );
        exports.insert(
            "roundtrip-bool",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = match arg0 {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_bool(param0);
                    drop(tables);
                    Ok(match result {
                        true => 1,
                        false => 0,
                    })
                },
            ),
        );
        exports.insert(
            "get-internal",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = tables
                        .host_state_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let host = &mut data_mut.data;
                    let result = host.get_internal(param0);
                    drop(tables);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        imports.register_namespace("imports", exports);
        let mut canonical_abi = imports
            .get_namespace_exports("canonical_abi")
            .unwrap_or_else(wasmer::Exports::new);
        canonical_abi.insert(
            "resource_drop_host-state",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      handle: u32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let mut tables = data_mut.tables.borrow_mut();
                    let handle = tables.host_state_table.remove(handle).map_err(|e| {
                        wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
                    })?;
                    let host = &mut data_mut.data;
                    host.drop_host_state(handle);
                    Ok(())
                },
            ),
        );
        imports.register_namespace("canonical_abi", canonical_abi);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| Ok(())
    }
    use core::convert::TryFrom;
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::char_from_i32;
    use wit_bindgen_wasmer::rt::invalid_variant;
}
