#[allow(clippy::all)]
pub mod imports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Imports: Sized + Send + Sync + 'static {
        fn roundtrip_u8(&mut self, a: u8) -> u8;

        fn roundtrip_s8(&mut self, a: i8) -> i8;

        fn roundtrip_u16(&mut self, a: u16) -> u16;

        fn roundtrip_s16(&mut self, a: i16) -> i16;

        fn roundtrip_u32(&mut self, a: u32) -> u32;

        fn roundtrip_s32(&mut self, a: i32) -> i32;

        fn roundtrip_u64(&mut self, a: u64) -> u64;

        fn roundtrip_s64(&mut self, a: i64) -> i64;

        fn roundtrip_float32(&mut self, a: f32) -> f32;

        fn roundtrip_float64(&mut self, a: f64) -> f64;

        fn roundtrip_char(&mut self, a: char) -> char;

        fn set_scalar(&mut self, a: u32) -> ();

        fn get_scalar(&mut self) -> u32;
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
        }
        unsafe impl<T: Imports> Send for EnvWrapper<T> {}
        unsafe impl<T: Imports> Sync for EnvWrapper<T> {}
        let env = EnvWrapper { data };
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
                    let param0 = u8::try_from(arg0).map_err(bad_int)?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_u8(param0);
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
                    let param0 = i8::try_from(arg0).map_err(bad_int)?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_s8(param0);
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
                    let param0 = u16::try_from(arg0).map_err(bad_int)?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_u16(param0);
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
                    let param0 = i16::try_from(arg0).map_err(bad_int)?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_s16(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-u32",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = arg0 as u32;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_u32(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-s32",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = arg0;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_s32(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-u64",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i64|
                      -> Result<i64, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = arg0 as u64;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_u64(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i64(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-s64",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i64|
                      -> Result<i64, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = arg0;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_s64(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i64(result))
                },
            ),
        );
        exports.insert(
            "roundtrip-float32",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: f32|
                      -> Result<f32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = arg0;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_float32(param0);
                    Ok(result)
                },
            ),
        );
        exports.insert(
            "roundtrip-float64",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: f64|
                      -> Result<f64, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = arg0;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_float64(param0);
                    Ok(result)
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
                    let param0 = char_from_i32(arg0)?;
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_char(param0);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
            "set-scalar",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let param0 = arg0 as u32;
                    let host = &mut data_mut.data;
                    let result = host.set_scalar(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
    "get-scalar",
    wasmer::Function::new_native(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
      let data_mut = store.data_mut();
      let host = &mut data_mut.data;
      let result = host.get_scalar();
      Ok(wit_bindgen_wasmer::rt::as_i32(result))
    }
    ));
        imports.register_namespace("imports", exports);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| Ok(())
    }
    use core::convert::TryFrom;
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::char_from_i32;
}
