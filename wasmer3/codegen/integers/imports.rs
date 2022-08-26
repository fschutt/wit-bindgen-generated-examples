#[allow(clippy::all)]
pub mod integers {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct IntegersData {}

    pub struct Integers {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<IntegersData>,
        func_a1: wasmer::TypedFunction<i32, ()>,
        func_a2: wasmer::TypedFunction<i32, ()>,
        func_a3: wasmer::TypedFunction<i32, ()>,
        func_a4: wasmer::TypedFunction<i32, ()>,
        func_a5: wasmer::TypedFunction<i32, ()>,
        func_a6: wasmer::TypedFunction<i32, ()>,
        func_a7: wasmer::TypedFunction<i64, ()>,
        func_a8: wasmer::TypedFunction<i64, ()>,
        func_a9: wasmer::TypedFunction<(i32, i32, i32, i32, i32, i32, i64, i64), ()>,
        func_pair_ret: wasmer::TypedFunction<(), i32>,
        func_r1: wasmer::TypedFunction<(), i32>,
        func_r2: wasmer::TypedFunction<(), i32>,
        func_r3: wasmer::TypedFunction<(), i32>,
        func_r4: wasmer::TypedFunction<(), i32>,
        func_r5: wasmer::TypedFunction<(), i32>,
        func_r6: wasmer::TypedFunction<(), i32>,
        func_r7: wasmer::TypedFunction<(), i64>,
        func_r8: wasmer::TypedFunction<(), i64>,
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
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<IntegersData> {
            let env = wasmer::FunctionEnv::new(&mut store, IntegersData::default());
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
            mut store: impl wasmer::AsStoreMut,
            module: &wasmer::Module,
            imports: &mut wasmer::Imports,
        ) -> anyhow::Result<(Self, wasmer::Instance)> {
            let env = Self::add_to_imports(&mut store, imports);
            let instance = wasmer::Instance::new(&mut store, module, &*imports)?;

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
            store: impl wasmer::AsStoreMut,
            _instance: &wasmer::Instance,
            env: wasmer::FunctionEnv<IntegersData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_a1 = _instance.exports.get_typed_function(&store, "a1")?;
            let func_a2 = _instance.exports.get_typed_function(&store, "a2")?;
            let func_a3 = _instance.exports.get_typed_function(&store, "a3")?;
            let func_a4 = _instance.exports.get_typed_function(&store, "a4")?;
            let func_a5 = _instance.exports.get_typed_function(&store, "a5")?;
            let func_a6 = _instance.exports.get_typed_function(&store, "a6")?;
            let func_a7 = _instance.exports.get_typed_function(&store, "a7")?;
            let func_a8 = _instance.exports.get_typed_function(&store, "a8")?;
            let func_a9 = _instance.exports.get_typed_function(&store, "a9")?;
            let func_pair_ret = _instance.exports.get_typed_function(&store, "pair-ret")?;
            let func_r1 = _instance.exports.get_typed_function(&store, "r1")?;
            let func_r2 = _instance.exports.get_typed_function(&store, "r2")?;
            let func_r3 = _instance.exports.get_typed_function(&store, "r3")?;
            let func_r4 = _instance.exports.get_typed_function(&store, "r4")?;
            let func_r5 = _instance.exports.get_typed_function(&store, "r5")?;
            let func_r6 = _instance.exports.get_typed_function(&store, "r6")?;
            let func_r7 = _instance.exports.get_typed_function(&store, "r7")?;
            let func_r8 = _instance.exports.get_typed_function(&store, "r8")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
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
                env,
            })
        }
        pub fn a1(&self, store: &mut wasmer::Store, x: u8) -> Result<(), wasmer::RuntimeError> {
            self.func_a1
                .call(store, wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a2(&self, store: &mut wasmer::Store, x: i8) -> Result<(), wasmer::RuntimeError> {
            self.func_a2
                .call(store, wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a3(&self, store: &mut wasmer::Store, x: u16) -> Result<(), wasmer::RuntimeError> {
            self.func_a3
                .call(store, wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a4(&self, store: &mut wasmer::Store, x: i16) -> Result<(), wasmer::RuntimeError> {
            self.func_a4
                .call(store, wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a5(&self, store: &mut wasmer::Store, x: u32) -> Result<(), wasmer::RuntimeError> {
            self.func_a5
                .call(store, wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a6(&self, store: &mut wasmer::Store, x: i32) -> Result<(), wasmer::RuntimeError> {
            self.func_a6
                .call(store, wit_bindgen_wasmer::rt::as_i32(x))?;
            Ok(())
        }
        pub fn a7(&self, store: &mut wasmer::Store, x: u64) -> Result<(), wasmer::RuntimeError> {
            self.func_a7
                .call(store, wit_bindgen_wasmer::rt::as_i64(x))?;
            Ok(())
        }
        pub fn a8(&self, store: &mut wasmer::Store, x: i64) -> Result<(), wasmer::RuntimeError> {
            self.func_a8
                .call(store, wit_bindgen_wasmer::rt::as_i64(x))?;
            Ok(())
        }
        pub fn a9(
            &self,
            store: &mut wasmer::Store,
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
                store,
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
        pub fn r1(&self, store: &mut wasmer::Store) -> Result<u8, wasmer::RuntimeError> {
            let result0 = self.func_r1.call(store)?;
            Ok(u8::try_from(result0).map_err(bad_int)?)
        }
        pub fn r2(&self, store: &mut wasmer::Store) -> Result<i8, wasmer::RuntimeError> {
            let result0 = self.func_r2.call(store)?;
            Ok(i8::try_from(result0).map_err(bad_int)?)
        }
        pub fn r3(&self, store: &mut wasmer::Store) -> Result<u16, wasmer::RuntimeError> {
            let result0 = self.func_r3.call(store)?;
            Ok(u16::try_from(result0).map_err(bad_int)?)
        }
        pub fn r4(&self, store: &mut wasmer::Store) -> Result<i16, wasmer::RuntimeError> {
            let result0 = self.func_r4.call(store)?;
            Ok(i16::try_from(result0).map_err(bad_int)?)
        }
        pub fn r5(&self, store: &mut wasmer::Store) -> Result<u32, wasmer::RuntimeError> {
            let result0 = self.func_r5.call(store)?;
            Ok(result0 as u32)
        }
        pub fn r6(&self, store: &mut wasmer::Store) -> Result<i32, wasmer::RuntimeError> {
            let result0 = self.func_r6.call(store)?;
            Ok(result0)
        }
        pub fn r7(&self, store: &mut wasmer::Store) -> Result<u64, wasmer::RuntimeError> {
            let result0 = self.func_r7.call(store)?;
            Ok(result0 as u64)
        }
        pub fn r8(&self, store: &mut wasmer::Store) -> Result<i64, wasmer::RuntimeError> {
            let result0 = self.func_r8.call(store)?;
            Ok(result0)
        }
        pub fn pair_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<(i64, u8), wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_pair_ret.call(store)?;
            let _memory_view = _memory.view(&store);
            let load1 = unsafe { _memory_view.data_unchecked_mut() }.load::<i64>(result0 + 0)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result0 + 8)?;
            Ok((load1, u8::try_from(i32::from(load2)).map_err(bad_int)?))
        }
    }
    use core::convert::TryFrom;
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::RawMem;
}
