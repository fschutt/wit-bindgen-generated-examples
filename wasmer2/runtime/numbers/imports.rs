pub mod exports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ExportsData {}
    impl wasmer::WasmerEnv for ExportsData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            Ok(())
        }
    }
    impl Clone for ExportsData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct Exports {
        state: std::sync::Arc<std::sync::Mutex<ExportsData>>,
        func_get_scalar: wasmer::NativeFunc<(), i32>,
        func_roundtrip_char: wasmer::NativeFunc<i32, i32>,
        func_roundtrip_float32: wasmer::NativeFunc<f32, f32>,
        func_roundtrip_float64: wasmer::NativeFunc<f64, f64>,
        func_roundtrip_s16: wasmer::NativeFunc<i32, i32>,
        func_roundtrip_s32: wasmer::NativeFunc<i32, i32>,
        func_roundtrip_s64: wasmer::NativeFunc<i64, i64>,
        func_roundtrip_s8: wasmer::NativeFunc<i32, i32>,
        func_roundtrip_u16: wasmer::NativeFunc<i32, i32>,
        func_roundtrip_u32: wasmer::NativeFunc<i32, i32>,
        func_roundtrip_u64: wasmer::NativeFunc<i64, i64>,
        func_roundtrip_u8: wasmer::NativeFunc<i32, i32>,
        func_set_scalar: wasmer::NativeFunc<i32, ()>,
        func_test_imports: wasmer::NativeFunc<(), ()>,
    }
    impl Exports {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ExportsData` which needs to be
        /// passed through to `Exports::new`.
        fn add_to_imports(
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<ExportsData>> {
            let state = std::sync::Arc::new(std::sync::Mutex::new(Default::default()));
            state
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
            store: &wasmer::Store,
            module: &wasmer::Module,
            imports: &mut wasmer::ImportObject,
        ) -> anyhow::Result<(Self, wasmer::Instance)> {
            let state = Self::add_to_imports(store, imports);
            let instance = wasmer::Instance::new(module, &*imports)?;
            Ok((Self::new(&instance, state)?, instance))
        }

        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// and wrap them all up in the returned structure which can
        /// be used to interact with the wasm module.
        pub fn new(
            instance: &wasmer::Instance,
            state: std::sync::Arc<std::sync::Mutex<ExportsData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_get_scalar = instance
                .exports
                .get_native_function::<(), i32>("get-scalar")?;
            let func_roundtrip_char = instance
                .exports
                .get_native_function::<i32, i32>("roundtrip-char")?;
            let func_roundtrip_float32 = instance
                .exports
                .get_native_function::<f32, f32>("roundtrip-float32")?;
            let func_roundtrip_float64 = instance
                .exports
                .get_native_function::<f64, f64>("roundtrip-float64")?;
            let func_roundtrip_s16 = instance
                .exports
                .get_native_function::<i32, i32>("roundtrip-s16")?;
            let func_roundtrip_s32 = instance
                .exports
                .get_native_function::<i32, i32>("roundtrip-s32")?;
            let func_roundtrip_s64 = instance
                .exports
                .get_native_function::<i64, i64>("roundtrip-s64")?;
            let func_roundtrip_s8 = instance
                .exports
                .get_native_function::<i32, i32>("roundtrip-s8")?;
            let func_roundtrip_u16 = instance
                .exports
                .get_native_function::<i32, i32>("roundtrip-u16")?;
            let func_roundtrip_u32 = instance
                .exports
                .get_native_function::<i32, i32>("roundtrip-u32")?;
            let func_roundtrip_u64 = instance
                .exports
                .get_native_function::<i64, i64>("roundtrip-u64")?;
            let func_roundtrip_u8 = instance
                .exports
                .get_native_function::<i32, i32>("roundtrip-u8")?;
            let func_set_scalar = instance
                .exports
                .get_native_function::<i32, ()>("set-scalar")?;
            let func_test_imports = instance
                .exports
                .get_native_function::<(), ()>("test-imports")?;
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
                state,
            })
        }
        pub fn test_imports(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_test_imports.call()?;
            Ok(())
        }
        pub fn roundtrip_u8(&self, a: u8) -> Result<u8, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_u8
                .call(wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(u8::try_from(result0).map_err(bad_int)?)
        }
        pub fn roundtrip_s8(&self, a: i8) -> Result<i8, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_s8
                .call(wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(i8::try_from(result0).map_err(bad_int)?)
        }
        pub fn roundtrip_u16(&self, a: u16) -> Result<u16, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_u16
                .call(wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(u16::try_from(result0).map_err(bad_int)?)
        }
        pub fn roundtrip_s16(&self, a: i16) -> Result<i16, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_s16
                .call(wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(i16::try_from(result0).map_err(bad_int)?)
        }
        pub fn roundtrip_u32(&self, a: u32) -> Result<u32, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_u32
                .call(wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(result0 as u32)
        }
        pub fn roundtrip_s32(&self, a: i32) -> Result<i32, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_s32
                .call(wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(result0)
        }
        pub fn roundtrip_u64(&self, a: u64) -> Result<u64, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_u64
                .call(wit_bindgen_wasmer::rt::as_i64(a))?;
            Ok(result0 as u64)
        }
        pub fn roundtrip_s64(&self, a: i64) -> Result<i64, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_s64
                .call(wit_bindgen_wasmer::rt::as_i64(a))?;
            Ok(result0)
        }
        pub fn roundtrip_float32(&self, a: f32) -> Result<f32, wasmer::RuntimeError> {
            let result0 = self.func_roundtrip_float32.call(a)?;
            Ok(result0)
        }
        pub fn roundtrip_float64(&self, a: f64) -> Result<f64, wasmer::RuntimeError> {
            let result0 = self.func_roundtrip_float64.call(a)?;
            Ok(result0)
        }
        pub fn roundtrip_char(&self, a: char) -> Result<char, wasmer::RuntimeError> {
            let result0 = self
                .func_roundtrip_char
                .call(wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(char_from_i32(result0)?)
        }
        pub fn set_scalar(&self, a: u32) -> Result<(), wasmer::RuntimeError> {
            self.func_set_scalar
                .call(wit_bindgen_wasmer::rt::as_i32(a))?;
            Ok(())
        }
        pub fn get_scalar(&self) -> Result<u32, wasmer::RuntimeError> {
            let result0 = self.func_get_scalar.call()?;
            Ok(result0 as u32)
        }
    }
    use core::convert::TryFrom;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::char_from_i32;
}
