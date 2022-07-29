#[allow(clippy::all)]
pub mod conventions {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct LudicrousSpeed {
        pub how_fast_are_you_going: u32,
        pub i_am_going_extremely_slow: u64,
    }
    impl core::fmt::Debug for LudicrousSpeed {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

    pub struct Conventions {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<ConventionsData>,
        func_a0: wasmer::TypedFunction<(), ()>,
        func_apple: wasmer::TypedFunction<(), ()>,
        func_apple_pear: wasmer::TypedFunction<(), ()>,
        func_apple_pear_grape: wasmer::TypedFunction<(), ()>,
        func_bool: wasmer::TypedFunction<(), ()>,
        func_explicit: wasmer::TypedFunction<(), ()>,
        func_explicit_kebab: wasmer::TypedFunction<(), ()>,
        func_foo: wasmer::TypedFunction<(i32, i64), ()>,
        func_function_with_dashes: wasmer::TypedFunction<(), ()>,
        func_function_with_no_weird_characters: wasmer::TypedFunction<(), ()>,
        func_garçon: wasmer::TypedFunction<(), ()>,
        func_garçon_hühnervögel_москва_東_京: wasmer::TypedFunction<(), ()>,
        func_hühnervögel: wasmer::TypedFunction<(), ()>,
        func_kebab_case: wasmer::TypedFunction<(), ()>,
        func_москва: wasmer::TypedFunction<(), ()>,
        func_東_京: wasmer::TypedFunction<(), ()>,
    }
    impl Conventions {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ConventionsData` which needs to be
        /// passed through to `Conventions::new`.
        fn add_to_imports(
            store: &mut wasmer::StoreMut<'_>,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<ConventionsData> {
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
            env: wasmer::FunctionEnv<ConventionsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_a0 = _instance.exports.get_typed_function(store, "a0")?;
            let func_apple = _instance.exports.get_typed_function(store, "apple")?;
            let func_apple_pear = _instance.exports.get_typed_function(store, "apple-pear")?;
            let func_apple_pear_grape = _instance
                .exports
                .get_typed_function(store, "apple-pear-grape")?;
            let func_bool = _instance.exports.get_typed_function(store, "bool")?;
            let func_explicit = _instance.exports.get_typed_function(store, "explicit")?;
            let func_explicit_kebab = _instance
                .exports
                .get_typed_function(store, "explicit-kebab")?;
            let func_foo = _instance.exports.get_typed_function(store, "foo")?;
            let func_function_with_dashes = _instance
                .exports
                .get_typed_function(store, "function-with-dashes")?;
            let func_function_with_no_weird_characters = _instance
                .exports
                .get_typed_function(store, "function-with-no-weird-characters")?;
            let func_garçon = _instance.exports.get_typed_function(store, "garçon")?;
            let func_garçon_hühnervögel_москва_東_京 = _instance
                .exports
                .get_typed_function(store, "garçon-hühnervögel-москва-東-京")?;
            let func_hühnervögel = _instance.exports.get_typed_function(store, "hühnervögel")?;
            let func_kebab_case = _instance.exports.get_typed_function(store, "kebab-case")?;
            let func_москва = _instance.exports.get_typed_function(store, "москва")?;
            let func_東_京 = _instance.exports.get_typed_function(store, "東-京")?;
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
                env,
            })
        }
        pub fn kebab_case(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_kebab_case.call(store)?;
            Ok(())
        }
        pub fn foo(
            &self,
            store: &mut wasmer::Store,
            x: LudicrousSpeed,
        ) -> Result<(), wasmer::RuntimeError> {
            let LudicrousSpeed {
                how_fast_are_you_going: how_fast_are_you_going0,
                i_am_going_extremely_slow: i_am_going_extremely_slow0,
            } = x;
            self.func_foo.call(
                store,
                wit_bindgen_wasmer::rt::as_i32(how_fast_are_you_going0),
                wit_bindgen_wasmer::rt::as_i64(i_am_going_extremely_slow0),
            )?;
            Ok(())
        }
        pub fn function_with_dashes(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_function_with_dashes.call(store)?;
            Ok(())
        }
        pub fn function_with_no_weird_characters(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_function_with_no_weird_characters.call(store)?;
            Ok(())
        }
        pub fn apple(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_apple.call(store)?;
            Ok(())
        }
        pub fn apple_pear(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_apple_pear.call(store)?;
            Ok(())
        }
        pub fn apple_pear_grape(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_apple_pear_grape.call(store)?;
            Ok(())
        }
        pub fn garçon(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_garçon.call(store)?;
            Ok(())
        }
        pub fn hühnervögel(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_hühnervögel.call(store)?;
            Ok(())
        }
        pub fn москва(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_москва.call(store)?;
            Ok(())
        }
        pub fn 東_京(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_東_京.call(store)?;
            Ok(())
        }
        pub fn garçon_hühnervögel_москва_東_京(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_garçon_hühnervögel_москва_東_京.call(store)?;
            Ok(())
        }
        pub fn a0(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_a0.call(store)?;
            Ok(())
        }
        pub fn explicit(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_explicit.call(store)?;
            Ok(())
        }
        pub fn explicit_kebab(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_explicit_kebab.call(store)?;
            Ok(())
        }
        pub fn bool(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_bool.call(store)?;
            Ok(())
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
}
