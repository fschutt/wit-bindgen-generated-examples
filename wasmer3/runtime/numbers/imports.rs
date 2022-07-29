#[allow(clippy::all)]
pub mod exports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ExportsData {}

    pub struct Exports {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<ExportsData>,
        func_get_scalar: wasmer::TypedFunction<(), i32>,
        func_roundtrip_char: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_float32: wasmer::TypedFunction<f32, f32>,
        func_roundtrip_float64: wasmer::TypedFunction<f64, f64>,
        func_roundtrip_s16: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_s32: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_s64: wasmer::TypedFunction<i64, i64>,
        func_roundtrip_s8: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_u16: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_u32: wasmer::TypedFunction<i32, i32>,
        func_roundtrip_u64: wasmer::TypedFunction<i64, i64>,
        func_roundtrip_u8: wasmer::TypedFunction<i32, i32>,
        func_set_scalar: wasmer::TypedFunction<i32, ()>,
        func_test_imports: wasmer::TypedFunction<(), ()>,
    }
    impl Exports {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ExportsData` which needs to be
        /// passed through to `Exports::new`.
        fn add_to_imports(
            store: &mut wasmer::StoreMut<'_>,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<ExportsData> {
            let env = wasmer::FunctionEnv::new(store, Default::default());
            env
        }

        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        ///
        /// The `imports` provided will have intrinsics added to it
        /// automatically, so it's not necessary to call
        /// `add_to_imports` beforehand. This function will
        /// instantiate the `module` otherwise using `imports`, and
        /// both an instance of this structure and the underlying
        /// `wasmer::Instance` will be returned.
        pub fn instantiate(
            store: &mut wasmer::StoreMut<'_>,
            module: &wasmer::Module,
            imports: &mut wasmer::Imports,
        ) -> anyhow::Result<(Self, wasmer::Instance)> {
            let env = Self::add_to_imports(&mut store.as_store_mut().as_store_mut(), imports);
            let instance = wasmer::Instance::new(&mut store.as_store_mut(), module, &*imports)?;

            Ok((Self::new(store, &instance, env)?, instance))
        }

        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// and wrap them all up in the returned structure which can
        /// be used to interact with the wasm module.
        pub fn new(
            store: &mut wasmer::StoreMut<'_>,
            _instance: &wasmer::Instance,
            env: wasmer::FunctionEnv<ExportsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_get_scalar = _instance.exports.get_typed_function(store, "get-scalar")?;
            let func_roundtrip_char = _instance
                .exports
                .get_typed_function(store, "roundtrip-char")?;
            let func_roundtrip_float32 = _instance
                .exports
                .get_typed_function(store, "roundtrip-float32")?;
            let func_roundtrip_float64 = _instance
                .exports
                .get_typed_function(store, "roundtrip-float64")?;
            let func_roundtrip_s16 = _instance
                .exports
                .get_typed_function(store, "roundtrip-s16")?;
            let func_roundtrip_s32 = _instance
                .exports
                .get_typed_function(store, "roundtrip-s32")?;
            let func_roundtrip_s64 = _instance
                .exports
                .get_typed_function(store, "roundtrip-s64")?;
            let func_roundtrip_s8 = _instance
                .exports
                .get_typed_function(store, "roundtrip-s8")?;
            let func_roundtrip_u16 = _instance
                .exports
                .get_typed_function(store, "roundtrip-u16")?;
            let func_roundtrip_u32 = _instance
                .exports
                .get_typed_function(store, "roundtrip-u32")?;
            let func_roundtrip_u64 = _instance
                .exports
                .get_typed_function(store, "roundtrip-u64")?;
            let func_roundtrip_u8 = _instance
                .exports
                .get_typed_function(store, "roundtrip-u8")?;
            let func_set_scalar = _instance.exports.get_typed_function(store, "set-scalar")?;
            let func_test_imports = _instance
                .exports
                .get_typed_function(store, "test-imports")?;
            Ok(Exports {
                func_get_scalar,
                func_roundtrip_char,
                func_roundtrip_float32,
                func_roundtrip_float64,
                func_roundtrip_s16,
                func_roundtrip_s32,
                func_roundtrip_s64,
                func_roundtrip_s8,
                func_roundtrip_u16,
                func_roundtrip_u32,
                func_roundtrip_u64,
                func_roundtrip_u8,
                func_set_scalar,
                func_test_imports,
                env,
            })
        }
        pub fn test_imports(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_test_imports.call(store)?;
            Ok(())
        }
        pub fn roundtrip_u8(
            &self,
            store: &mut wasmer::Store,
            a: u8,
        ) -> Result<u8, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_u8
                .call(store, wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(u8::try_from(result0).map_err(bad_int)?)
        }
        pub fn roundtrip_s8(
            &self,
            store: &mut wasmer::Store,
            a: i8,
        ) -> Result<i8, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_s8
                .call(store, wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(i8::try_from(result0).map_err(bad_int)?)
        }
        pub fn roundtrip_u16(
            &self,
            store: &mut wasmer::Store,
            a: u16,
        ) -> Result<u16, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_u16
                .call(store, wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(u16::try_from(result0).map_err(bad_int)?)
        }
        pub fn roundtrip_s16(
            &self,
            store: &mut wasmer::Store,
            a: i16,
        ) -> Result<i16, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_s16
                .call(store, wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(i16::try_from(result0).map_err(bad_int)?)
        }
        pub fn roundtrip_u32(
            &self,
            store: &mut wasmer::Store,
            a: u32,
        ) -> Result<u32, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_u32
                .call(store, wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(result0 as u32)
        }
        pub fn roundtrip_s32(
            &self,
            store: &mut wasmer::Store,
            a: i32,
        ) -> Result<i32, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_s32
                .call(store, wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(result0)
        }
        pub fn roundtrip_u64(
            &self,
            store: &mut wasmer::Store,
            a: u64,
        ) -> Result<u64, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_u64
                .call(store, wit_bindgen_wasmer::rt::as_i64(a))?;
            Ok(result0 as u64)
        }
        pub fn roundtrip_s64(
            &self,
            store: &mut wasmer::Store,
            a: i64,
        ) -> Result<i64, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_s64
                .call(store, wit_bindgen_wasmer::rt::as_i64(a))?;
            Ok(result0)
        }
        pub fn roundtrip_float32(
            &self,
            store: &mut wasmer::Store,
            a: f32,
        ) -> Result<f32, wasmer::RuntimeError> {
            let result0 = self.func_roundtrip_float32.call(store, a)?;
            Ok(result0)
        }
        pub fn roundtrip_float64(
            &self,
            store: &mut wasmer::Store,
            a: f64,
        ) -> Result<f64, wasmer::RuntimeError> {
            let result0 = self.func_roundtrip_float64.call(store, a)?;
            Ok(result0)
        }
        pub fn roundtrip_char(
            &self,
            store: &mut wasmer::Store,
            a: char,
        ) -> Result<char, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_char
                .call(store, wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(char_from_i32(result0)?)
        }
        pub fn set_scalar(
            &self,
            store: &mut wasmer::Store,
            a: u32,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_set_scalar
                .call(store, wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(())
        }
        pub fn get_scalar(&self, store: &mut wasmer::Store) -> Result<u32, wasmer::RuntimeError> {
            let result0 = self.func_get_scalar.call(store)?;
            Ok(result0 as u32)
        }
    }
    use core::convert::TryFrom;
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::char_from_i32;
}
