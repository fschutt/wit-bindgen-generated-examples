pub mod integers {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct IntegersData {}
    impl wasmer::WasmerEnv for IntegersData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            Ok(())
        }
    }
    impl Clone for IntegersData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct Integers {
        state: std::sync::Arc<std::sync::Mutex<IntegersData>>,
        func_a1: wasmer::NativeFunc<i32, ()>,
        func_a2: wasmer::NativeFunc<i32, ()>,
        func_a3: wasmer::NativeFunc<i32, ()>,
        func_a4: wasmer::NativeFunc<i32, ()>,
        func_a5: wasmer::NativeFunc<i32, ()>,
        func_a6: wasmer::NativeFunc<i32, ()>,
        func_a7: wasmer::NativeFunc<i64, ()>,
        func_a8: wasmer::NativeFunc<i64, ()>,
        func_a9: wasmer::NativeFunc<(i32, i32, i32, i32, i32, i32, i64, i64), ()>,
        func_pair_ret: wasmer::NativeFunc<(), i32>,
        func_r1: wasmer::NativeFunc<(), i32>,
        func_r2: wasmer::NativeFunc<(), i32>,
        func_r3: wasmer::NativeFunc<(), i32>,
        func_r4: wasmer::NativeFunc<(), i32>,
        func_r5: wasmer::NativeFunc<(), i32>,
        func_r6: wasmer::NativeFunc<(), i32>,
        func_r7: wasmer::NativeFunc<(), i64>,
        func_r8: wasmer::NativeFunc<(), i64>,
        memory: wasmer::Memory,
    }
    impl Integers {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `IntegersData` which needs to be
        /// passed through to `Integers::new`.
        fn add_to_imports(
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<IntegersData>> {
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
            state: std::sync::Arc<std::sync::Mutex<IntegersData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_a1 = instance.exports.get_native_function::<i32, ()>("a1")?;
            let func_a2 = instance.exports.get_native_function::<i32, ()>("a2")?;
            let func_a3 = instance.exports.get_native_function::<i32, ()>("a3")?;
            let func_a4 = instance.exports.get_native_function::<i32, ()>("a4")?;
            let func_a5 = instance.exports.get_native_function::<i32, ()>("a5")?;
            let func_a6 = instance.exports.get_native_function::<i32, ()>("a6")?;
            let func_a7 = instance.exports.get_native_function::<i64, ()>("a7")?;
            let func_a8 = instance.exports.get_native_function::<i64, ()>("a8")?;
            let func_a9 = instance
                .exports
                .get_native_function::<(i32, i32, i32, i32, i32, i32, i64, i64), ()>("a9")?;
            let func_pair_ret = instance
                .exports
                .get_native_function::<(), i32>("pair-ret")?;
            let func_r1 = instance.exports.get_native_function::<(), i32>("r1")?;
            let func_r2 = instance.exports.get_native_function::<(), i32>("r2")?;
            let func_r3 = instance.exports.get_native_function::<(), i32>("r3")?;
            let func_r4 = instance.exports.get_native_function::<(), i32>("r4")?;
            let func_r5 = instance.exports.get_native_function::<(), i32>("r5")?;
            let func_r6 = instance.exports.get_native_function::<(), i32>("r6")?;
            let func_r7 = instance.exports.get_native_function::<(), i64>("r7")?;
            let func_r8 = instance.exports.get_native_function::<(), i64>("r8")?;
            let memory = instance.exports.get_memory("memory")?.clone();
            Ok(Integers {
                func_a1,
                func_a2,
                func_a3,
                func_a4,
                func_a5,
                func_a6,
                func_a7,
                func_a8,
                func_a9,
                func_pair_ret,
                func_r1,
                func_r2,
                func_r3,
                func_r4,
                func_r5,
                func_r6,
                func_r7,
                func_r8,
                memory,
                state,
            })
        }
        pub fn a1(&self, x: u8) -> Result<(), wasmer::RuntimeError> {
            self.func_a1.call(wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a2(&self, x: i8) -> Result<(), wasmer::RuntimeError> {
            self.func_a2.call(wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a3(&self, x: u16) -> Result<(), wasmer::RuntimeError> {
            self.func_a3.call(wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a4(&self, x: i16) -> Result<(), wasmer::RuntimeError> {
            self.func_a4.call(wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a5(&self, x: u32) -> Result<(), wasmer::RuntimeError> {
            self.func_a5.call(wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a6(&self, x: i32) -> Result<(), wasmer::RuntimeError> {
            self.func_a6.call(wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a7(&self, x: u64) -> Result<(), wasmer::RuntimeError> {
            self.func_a7.call(wit_bindgen_wasmer::rt::as_i64(x))?;
            Ok(())
        }
        pub fn a8(&self, x: i64) -> Result<(), wasmer::RuntimeError> {
            self.func_a8.call(wit_bindgen_wasmer::rt::as_i64(x))?;
            Ok(())
        }
        pub fn a9(
            &self,
            p1: u8,
            p2: i8,
            p3: u16,
            p4: i16,
            p5: u32,
            p6: i32,
            p7: u64,
            p8: i64,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_a9.call(
                wit_bindgen_wasmer::rt::as_i32(p1),
                wit_bindgen_wasmer::rt::as_i32(p2),
                wit_bindgen_wasmer::rt::as_i32(p3),
                wit_bindgen_wasmer::rt::as_i32(p4),
                wit_bindgen_wasmer::rt::as_i32(p5),
                wit_bindgen_wasmer::rt::as_i32(p6),
                wit_bindgen_wasmer::rt::as_i64(p7),
                wit_bindgen_wasmer::rt::as_i64(p8),
            )?;
            Ok(())
        }
        pub fn r1(&self) -> Result<u8, wasmer::RuntimeError> {
            let result0 = self.func_r1.call()?;
            Ok(u8::try_from(result0).map_err(bad_int)?)
        }
        pub fn r2(&self) -> Result<i8, wasmer::RuntimeError> {
            let result0 = self.func_r2.call()?;
            Ok(i8::try_from(result0).map_err(bad_int)?)
        }
        pub fn r3(&self) -> Result<u16, wasmer::RuntimeError> {
            let result0 = self.func_r3.call()?;
            Ok(u16::try_from(result0).map_err(bad_int)?)
        }
        pub fn r4(&self) -> Result<i16, wasmer::RuntimeError> {
            let result0 = self.func_r4.call()?;
            Ok(i16::try_from(result0).map_err(bad_int)?)
        }
        pub fn r5(&self) -> Result<u32, wasmer::RuntimeError> {
            let result0 = self.func_r5.call()?;
            Ok(result0 as u32)
        }
        pub fn r6(&self) -> Result<i32, wasmer::RuntimeError> {
            let result0 = self.func_r6.call()?;
            Ok(result0)
        }
        pub fn r7(&self) -> Result<u64, wasmer::RuntimeError> {
            let result0 = self.func_r7.call()?;
            Ok(result0 as u64)
        }
        pub fn r8(&self) -> Result<i64, wasmer::RuntimeError> {
            let result0 = self.func_r8.call()?;
            Ok(result0)
        }
        pub fn pair_ret(&self) -> Result<(i64, u8), wasmer::RuntimeError> {
            let memory = &self.memory;
            let result0 = self.func_pair_ret.call()?;
            let load1 = unsafe { memory.data_unchecked_mut() }.load::<i64>(result0 + 0)?;
            let load2 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 8)?;
            Ok((load1, u8::try_from(i32::from(load2)).map_err(bad_int)?))
        }
    }
    use core::convert::TryFrom;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::RawMem;
}
