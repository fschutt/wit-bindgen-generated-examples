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

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ConventionsData {}
    impl wasmer::WasmerEnv for ConventionsData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            Ok(())
        }
    }
    impl Clone for ConventionsData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct Conventions {
        state: std::sync::Arc<std::sync::Mutex<ConventionsData>>,
        func_a0: wasmer::NativeFunc<(), ()>,
        func_apple: wasmer::NativeFunc<(), ()>,
        func_apple_pear: wasmer::NativeFunc<(), ()>,
        func_apple_pear_grape: wasmer::NativeFunc<(), ()>,
        func_bool: wasmer::NativeFunc<(), ()>,
        func_explicit: wasmer::NativeFunc<(), ()>,
        func_explicit_kebab: wasmer::NativeFunc<(), ()>,
        func_foo: wasmer::NativeFunc<(i32, i64), ()>,
        func_function_with_dashes: wasmer::NativeFunc<(), ()>,
        func_function_with_no_weird_characters: wasmer::NativeFunc<(), ()>,
        func_garçon: wasmer::NativeFunc<(), ()>,
        func_garçon_hühnervögel_москва_東_京: wasmer::NativeFunc<(), ()>,
        func_hühnervögel: wasmer::NativeFunc<(), ()>,
        func_kebab_case: wasmer::NativeFunc<(), ()>,
        func_москва: wasmer::NativeFunc<(), ()>,
        func_東_京: wasmer::NativeFunc<(), ()>,
    }
    impl Conventions {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ConventionsData` which needs to be
        /// passed through to `Conventions::new`.
        fn add_to_imports(
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<ConventionsData>> {
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
            state: std::sync::Arc<std::sync::Mutex<ConventionsData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_a0 = instance.exports.get_native_function::<(), ()>("a0")?;
            let func_apple = instance.exports.get_native_function::<(), ()>("apple")?;
            let func_apple_pear = instance
                .exports
                .get_native_function::<(), ()>("apple-pear")?;
            let func_apple_pear_grape = instance
                .exports
                .get_native_function::<(), ()>("apple-pear-grape")?;
            let func_bool = instance.exports.get_native_function::<(), ()>("bool")?;
            let func_explicit = instance.exports.get_native_function::<(), ()>("explicit")?;
            let func_explicit_kebab = instance
                .exports
                .get_native_function::<(), ()>("explicit-kebab")?;
            let func_foo = instance
                .exports
                .get_native_function::<(i32, i64), ()>("foo")?;
            let func_function_with_dashes = instance
                .exports
                .get_native_function::<(), ()>("function-with-dashes")?;
            let func_function_with_no_weird_characters = instance
                .exports
                .get_native_function::<(), ()>("function-with-no-weird-characters")?;
            let func_garçon = instance.exports.get_native_function::<(), ()>("garçon")?;
            let func_garçon_hühnervögel_москва_東_京 = instance
                .exports
                .get_native_function::<(), ()>("garçon-hühnervögel-москва-東-京")?;
            let func_hühnervögel = instance
                .exports
                .get_native_function::<(), ()>("hühnervögel")?;
            let func_kebab_case = instance
                .exports
                .get_native_function::<(), ()>("kebab-case")?;
            let func_москва = instance.exports.get_native_function::<(), ()>("москва")?;
            let func_東_京 = instance.exports.get_native_function::<(), ()>("東-京")?;
            Ok(Conventions {
                func_a0,
                func_apple,
                func_apple_pear,
                func_apple_pear_grape,
                func_bool,
                func_explicit,
                func_explicit_kebab,
                func_foo,
                func_function_with_dashes,
                func_function_with_no_weird_characters,
                func_garçon,
                func_garçon_hühnervögel_москва_東_京,
                func_hühnervögel,
                func_kebab_case,
                func_москва,
                func_東_京,
                state,
            })
        }
        pub fn kebab_case(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_kebab_case.call()?;
            Ok(())
        }
        pub fn foo(&self, x: LudicrousSpeed) -> Result<(), wasmer::RuntimeError> {
            let LudicrousSpeed {
                how_fast_are_you_going: how_fast_are_you_going0,
                i_am_going_extremely_slow: i_am_going_extremely_slow0,
            } = x;
            self.func_foo.call(
                wit_bindgen_wasmer::rt::as_i32(how_fast_are_you_going0),
                wit_bindgen_wasmer::rt::as_i64(i_am_going_extremely_slow0),
            )?;
            Ok(())
        }
        pub fn function_with_dashes(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_function_with_dashes.call()?;
            Ok(())
        }
        pub fn function_with_no_weird_characters(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_function_with_no_weird_characters.call()?;
            Ok(())
        }
        pub fn apple(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_apple.call()?;
            Ok(())
        }
        pub fn apple_pear(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_apple_pear.call()?;
            Ok(())
        }
        pub fn apple_pear_grape(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_apple_pear_grape.call()?;
            Ok(())
        }
        pub fn garçon(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_garçon.call()?;
            Ok(())
        }
        pub fn hühnervögel(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_hühnervögel.call()?;
            Ok(())
        }
        pub fn москва(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_москва.call()?;
            Ok(())
        }
        pub fn 東_京(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_東_京.call()?;
            Ok(())
        }
        pub fn garçon_hühnervögel_москва_東_京(
            &self,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_garçon_hühnervögel_москва_東_京.call()?;
            Ok(())
        }
        pub fn a0(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_a0.call()?;
            Ok(())
        }
        pub fn explicit(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_explicit.call()?;
            Ok(())
        }
        pub fn explicit_kebab(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_explicit_kebab.call()?;
            Ok(())
        }
        pub fn bool(&self) -> Result<(), wasmer::RuntimeError> {
            self.func_bool.call()?;
            Ok(())
        }
    }
}
