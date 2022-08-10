#[allow(clippy::all)]
pub mod variants {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum E1 {
        A,
    }
    impl core::fmt::Debug for E1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                E1::A => f.debug_tuple("E1::A").finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    impl core::fmt::Debug for U1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                U1::U32(e) => f.debug_tuple("U1::U32").field(e).finish(),
                U1::F32(e) => f.debug_tuple("U1::F32").field(e).finish(),
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Empty {}
    impl core::fmt::Debug for Empty {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Empty").finish()
        }
    }
    impl wit_bindgen_wasmer::Endian for Empty {
        fn into_le(self) -> Self {
            Self {}
        }
        fn from_le(self) -> Self {
            Self {}
        }
    }
    unsafe impl wit_bindgen_wasmer::AllBytesValid for Empty {}
    #[derive(Clone)]
    pub enum V1Param<'a> {
        A,
        B(U1),
        C(E1),
        D(&'a str),
        E(Empty),
        F,
        G(u32),
    }
    impl<'a> core::fmt::Debug for V1Param<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                V1Param::A => f.debug_tuple("V1Param::A").finish(),
                V1Param::B(e) => f.debug_tuple("V1Param::B").field(e).finish(),
                V1Param::C(e) => f.debug_tuple("V1Param::C").field(e).finish(),
                V1Param::D(e) => f.debug_tuple("V1Param::D").field(e).finish(),
                V1Param::E(e) => f.debug_tuple("V1Param::E").field(e).finish(),
                V1Param::F => f.debug_tuple("V1Param::F").finish(),
                V1Param::G(e) => f.debug_tuple("V1Param::G").field(e).finish(),
            }
        }
    }
    #[derive(Clone)]
    pub enum V1Result {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    impl core::fmt::Debug for V1Result {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                V1Result::A => f.debug_tuple("V1Result::A").finish(),
                V1Result::B(e) => f.debug_tuple("V1Result::B").field(e).finish(),
                V1Result::C(e) => f.debug_tuple("V1Result::C").field(e).finish(),
                V1Result::D(e) => f.debug_tuple("V1Result::D").field(e).finish(),
                V1Result::E(e) => f.debug_tuple("V1Result::E").field(e).finish(),
                V1Result::F => f.debug_tuple("V1Result::F").finish(),
                V1Result::G(e) => f.debug_tuple("V1Result::G").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    impl core::fmt::Debug for Casts1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Casts1::A(e) => f.debug_tuple("Casts1::A").field(e).finish(),
                Casts1::B(e) => f.debug_tuple("Casts1::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    impl core::fmt::Debug for Casts2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Casts2::A(e) => f.debug_tuple("Casts2::A").field(e).finish(),
                Casts2::B(e) => f.debug_tuple("Casts2::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    impl core::fmt::Debug for Casts3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Casts3::A(e) => f.debug_tuple("Casts3::A").field(e).finish(),
                Casts3::B(e) => f.debug_tuple("Casts3::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    impl core::fmt::Debug for Casts4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Casts4::A(e) => f.debug_tuple("Casts4::A").field(e).finish(),
                Casts4::B(e) => f.debug_tuple("Casts4::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    impl core::fmt::Debug for Casts5 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Casts5::A(e) => f.debug_tuple("Casts5::A").field(e).finish(),
                Casts5::B(e) => f.debug_tuple("Casts5::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    impl core::fmt::Debug for Casts6 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Casts6::A(e) => f.debug_tuple("Casts6::A").field(e).finish(),
                Casts6::B(e) => f.debug_tuple("Casts6::B").field(e).finish(),
            }
        }
    }
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    impl MyErrno {
        pub fn name(&self) -> &'static str {
            match self {
                MyErrno::Bad1 => "bad1",
                MyErrno::Bad2 => "bad2",
            }
        }
        pub fn message(&self) -> &'static str {
            match self {
                MyErrno::Bad1 => "",
                MyErrno::Bad2 => "",
            }
        }
    }
    impl core::fmt::Debug for MyErrno {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("MyErrno")
                .field("code", &(*self as i32))
                .field("name", &self.name())
                .field("message", &self.message())
                .finish()
        }
    }
    impl core::fmt::Display for MyErrno {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} (error {})", self.name(), *self as i32)
        }
    }

    impl std::error::Error for MyErrno {}
    #[derive(Clone)]
    pub struct IsCloneParam<'a> {
        pub v1: V1Param<'a>,
    }
    impl<'a> core::fmt::Debug for IsCloneParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("IsCloneParam")
                .field("v1", &self.v1)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct IsCloneResult {
        pub v1: V1Result,
    }
    impl core::fmt::Debug for IsCloneResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("IsCloneResult")
                .field("v1", &self.v1)
                .finish()
        }
    }

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct VariantsData {}

    pub struct Variants {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<VariantsData>,
        func_bool_arg: wasmer::TypedFunction<i32, ()>,
        func_bool_result: wasmer::TypedFunction<(), i32>,
        func_canonical_abi_free: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        func_casts: wasmer::TypedFunction<
            (
                i32,
                i32,
                i32,
                i64,
                i32,
                i64,
                i32,
                i64,
                i32,
                i64,
                i32,
                i32,
                i32,
            ),
            i32,
        >,
        func_e1_arg: wasmer::TypedFunction<i32, ()>,
        func_e1_result: wasmer::TypedFunction<(), i32>,
        func_expected_arg: wasmer::TypedFunction<
            (
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
            ),
            (),
        >,
        func_expected_result: wasmer::TypedFunction<(), i32>,
        func_expected_simple: wasmer::TypedFunction<(), i32>,
        func_is_clone_arg: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_is_clone_return: wasmer::TypedFunction<(), i32>,
        func_option_arg: wasmer::TypedFunction<
            (
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                f32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
            ),
            (),
        >,
        func_option_result: wasmer::TypedFunction<(), i32>,
        func_return_expected_sugar: wasmer::TypedFunction<(), i32>,
        func_return_expected_sugar2: wasmer::TypedFunction<(), i32>,
        func_return_expected_sugar3: wasmer::TypedFunction<(), i32>,
        func_return_expected_sugar4: wasmer::TypedFunction<(), i32>,
        func_return_option_sugar: wasmer::TypedFunction<(), i32>,
        func_return_option_sugar2: wasmer::TypedFunction<(), i32>,
        func_u1_arg: wasmer::TypedFunction<(i32, i32), ()>,
        func_u1_result: wasmer::TypedFunction<(), i32>,
        func_v1_arg: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_v1_result: wasmer::TypedFunction<(), i32>,
        memory: wasmer::Memory,
    }
    impl Variants {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `VariantsData` which needs to be
        /// passed through to `Variants::new`.
        fn add_to_imports(
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<VariantsData> {
            let env = wasmer::FunctionEnv::new(&mut store, VariantsData::default());
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
            env: wasmer::FunctionEnv<VariantsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_bool_arg = _instance.exports.get_typed_function(&store, "bool-arg")?;
            let func_bool_result = _instance
                .exports
                .get_typed_function(&store, "bool-result")?;
            let func_canonical_abi_free = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_free")?;
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_realloc")?;
            let func_casts = _instance.exports.get_typed_function(&store, "casts")?;
            let func_e1_arg = _instance.exports.get_typed_function(&store, "e1-arg")?;
            let func_e1_result = _instance.exports.get_typed_function(&store, "e1-result")?;
            let func_expected_arg = _instance
                .exports
                .get_typed_function(&store, "expected-arg")?;
            let func_expected_result = _instance
                .exports
                .get_typed_function(&store, "expected-result")?;
            let func_expected_simple = _instance
                .exports
                .get_typed_function(&store, "expected-simple")?;
            let func_is_clone_arg = _instance
                .exports
                .get_typed_function(&store, "is-clone-arg")?;
            let func_is_clone_return = _instance
                .exports
                .get_typed_function(&store, "is-clone-return")?;
            let func_option_arg = _instance.exports.get_typed_function(&store, "option-arg")?;
            let func_option_result = _instance
                .exports
                .get_typed_function(&store, "option-result")?;
            let func_return_expected_sugar = _instance
                .exports
                .get_typed_function(&store, "return-expected-sugar")?;
            let func_return_expected_sugar2 = _instance
                .exports
                .get_typed_function(&store, "return-expected-sugar2")?;
            let func_return_expected_sugar3 = _instance
                .exports
                .get_typed_function(&store, "return-expected-sugar3")?;
            let func_return_expected_sugar4 = _instance
                .exports
                .get_typed_function(&store, "return-expected-sugar4")?;
            let func_return_option_sugar = _instance
                .exports
                .get_typed_function(&store, "return-option-sugar")?;
            let func_return_option_sugar2 = _instance
                .exports
                .get_typed_function(&store, "return-option-sugar2")?;
            let func_u1_arg = _instance.exports.get_typed_function(&store, "u1-arg")?;
            let func_u1_result = _instance.exports.get_typed_function(&store, "u1-result")?;
            let func_v1_arg = _instance.exports.get_typed_function(&store, "v1-arg")?;
            let func_v1_result = _instance.exports.get_typed_function(&store, "v1-result")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Variants {
                func_bool_arg,
                func_bool_result,
                func_canonical_abi_free,
                func_canonical_abi_realloc,
                func_casts,
                func_e1_arg,
                func_e1_result,
                func_expected_arg,
                func_expected_result,
                func_expected_simple,
                func_is_clone_arg,
                func_is_clone_return,
                func_option_arg,
                func_option_result,
                func_return_expected_sugar,
                func_return_expected_sugar2,
                func_return_expected_sugar3,
                func_return_expected_sugar4,
                func_return_option_sugar,
                func_return_option_sugar2,
                func_u1_arg,
                func_u1_result,
                func_v1_arg,
                func_v1_result,
                memory,
                env,
            })
        }
        pub fn e1_arg(&self, store: &mut wasmer::Store, x: E1) -> Result<(), wasmer::RuntimeError> {
            self.func_e1_arg.call(store, x as i32)?;
            Ok(())
        }
        pub fn e1_result(&self, store: &mut wasmer::Store) -> Result<E1, wasmer::RuntimeError> {
            let result0 = self.func_e1_result.call(store)?;
            Ok(match result0 {
                0 => E1::A,
                _ => return Err(invalid_variant("E1")),
            })
        }
        pub fn u1_arg(&self, store: &mut wasmer::Store, x: U1) -> Result<(), wasmer::RuntimeError> {
            let (result0_0, result0_1) = match x {
                U1::U32(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                U1::F32(e) => (1i32, (e).to_bits() as i32),
            };
            self.func_u1_arg.call(store, result0_0, result0_1)?;
            Ok(())
        }
        pub fn u1_result(&self, store: &mut wasmer::Store) -> Result<U1, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_u1_result.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => U1::U32({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    load2 as u32
                }),
                1 => U1::F32({
                    let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<f32>(result0 + 4)?;
                    load3
                }),
                _ => return Err(invalid_variant("U1")),
            })
        }
        pub fn v1_arg(
            &self,
            store: &mut wasmer::Store,
            x: V1Param<'_>,
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let (result3_0, result3_1, result3_2) = match x {
                V1Param::A => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0i32, 0i32)
                    }
                }
                V1Param::B(e) => {
                    let (result0_0, result0_1) = match e {
                        U1::U32(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                        U1::F32(e) => (1i32, (e).to_bits() as i32),
                    };
                    (1i32, result0_0, result0_1)
                }
                V1Param::C(e) => (2i32, e as i32, 0i32),
                V1Param::D(e) => {
                    let vec1 = e;
                    let ptr1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec1.len() as i32,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr1, vec1.as_bytes())?;
                    (3i32, ptr1, vec1.len() as i32)
                }
                V1Param::E(e) => {
                    let Empty {} = e;
                    (4i32, 0i32, 0i32)
                }
                V1Param::F => {
                    let e = ();
                    {
                        let () = e;
                        (5i32, 0i32, 0i32)
                    }
                }
                V1Param::G(e) => (6i32, wit_bindgen_wasmer::rt::as_i32(e), 0i32),
            };
            self.func_v1_arg
                .call(store, result3_0, result3_1, result3_2)?;
            Ok(())
        }
        pub fn v1_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<V1Result, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_v1_result.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => V1Result::A,
                1 => V1Result::B({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(result0 + 4)?;
                    match i32::from(load2) {
                        0 => U1::U32({
                            let load3 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(result0 + 8)?;
                            load3 as u32
                        }),
                        1 => U1::F32({
                            let load4 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<f32>(result0 + 8)?;
                            load4
                        }),
                        _ => return Err(invalid_variant("U1")),
                    }
                }),
                2 => V1Result::C({
                    let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(result0 + 4)?;
                    match i32::from(load5) {
                        0 => E1::A,
                        _ => return Err(invalid_variant("E1")),
                    }
                }),
                3 => V1Result::D({
                    let load6 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    let load7 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 8)?;
                    let ptr8 = load6;
                    let len8 = load7;

                    let data8 = copy_slice(store, _memory, func_canonical_abi_free, ptr8, len8, 1)?;
                    String::from_utf8(data8)
                        .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                }),
                4 => V1Result::E(Empty {}),
                5 => V1Result::F,
                6 => V1Result::G({
                    let load9 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    load9 as u32
                }),
                _ => return Err(invalid_variant("V1Result")),
            })
        }
        pub fn bool_arg(
            &self,
            store: &mut wasmer::Store,
            x: bool,
        ) -> Result<(), wasmer::RuntimeError> {
            self.func_bool_arg.call(
                store,
                match x {
                    true => 1,
                    false => 0,
                },
            )?;
            Ok(())
        }
        pub fn bool_result(&self, store: &mut wasmer::Store) -> Result<bool, wasmer::RuntimeError> {
            let result0 = self.func_bool_result.call(store)?;
            Ok(match result0 {
                0 => false,
                1 => true,
                _ => return Err(invalid_variant("bool")),
            })
        }
        pub fn option_arg(
            &self,
            store: &mut wasmer::Store,
            a: Option<bool>,
            b: Option<()>,
            c: Option<u32>,
            d: Option<E1>,
            e: Option<f32>,
            f: Option<U1>,
            g: Option<Option<bool>>,
        ) -> Result<(), wasmer::RuntimeError> {
            let (result0_0, result0_1) = match a {
                Some(e) => (
                    1i32,
                    match e {
                        true => 1,
                        false => 0,
                    },
                ),
                None => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0i32)
                    }
                }
            };
            let result2 = match b {
                Some(e) => {
                    let () = e;
                    1i32
                }
                None => {
                    let e = ();
                    {
                        let () = e;
                        0i32
                    }
                }
            };
            let (result3_0, result3_1) = match c {
                Some(e) => (1i32, wit_bindgen_wasmer::rt::as_i32(e)),
                None => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0i32)
                    }
                }
            };
            let (result4_0, result4_1) = match d {
                Some(e) => (1i32, e as i32),
                None => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0i32)
                    }
                }
            };
            let (result5_0, result5_1) = match e {
                Some(e) => (1i32, e),
                None => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0.0f32)
                    }
                }
            };
            let (result7_0, result7_1, result7_2) = match f {
                Some(e) => {
                    let (result6_0, result6_1) = match e {
                        U1::U32(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                        U1::F32(e) => (1i32, (e).to_bits() as i32),
                    };
                    (1i32, result6_0, result6_1)
                }
                None => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0i32, 0i32)
                    }
                }
            };
            let (result9_0, result9_1, result9_2) = match g {
                Some(e) => {
                    let (result8_0, result8_1) = match e {
                        Some(e) => (
                            1i32,
                            match e {
                                true => 1,
                                false => 0,
                            },
                        ),
                        None => {
                            let e = ();
                            {
                                let () = e;
                                (0i32, 0i32)
                            }
                        }
                    };
                    (1i32, result8_0, result8_1)
                }
                None => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0i32, 0i32)
                    }
                }
            };
            self.func_option_arg.call(
                store, result0_0, result0_1, result2, result3_0, result3_1, result4_0, result4_1,
                result5_0, result5_1, result7_0, result7_1, result7_2, result9_0, result9_1,
                result9_2,
            )?;
            Ok(())
        }
        pub fn option_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<
            (
                Option<bool>,
                Option<()>,
                Option<u32>,
                Option<E1>,
                Option<f32>,
                Option<U1>,
                Option<Option<bool>>,
            ),
            wasmer::RuntimeError,
        > {
            let _memory = &self.memory;
            let result0 = self.func_option_result.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 2)?;
            let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 4)?;
            let load6 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 12)?;
            let load8 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 16)?;
            let load10 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 24)?;
            let load14 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 36)?;
            Ok((
                match i32::from(load1) {
                    0 => None,
                    1 => Some({
                        let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<u8>(result0 + 1)?;
                        match i32::from(load2) {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        }
                    }),
                    _ => return Err(invalid_variant("option")),
                },
                match i32::from(load3) {
                    0 => None,
                    1 => Some(()),
                    _ => return Err(invalid_variant("option")),
                },
                match i32::from(load4) {
                    0 => None,
                    1 => Some({
                        let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result0 + 8)?;
                        load5 as u32
                    }),
                    _ => return Err(invalid_variant("option")),
                },
                match i32::from(load6) {
                    0 => None,
                    1 => Some({
                        let load7 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<u8>(result0 + 13)?;
                        match i32::from(load7) {
                            0 => E1::A,
                            _ => return Err(invalid_variant("E1")),
                        }
                    }),
                    _ => return Err(invalid_variant("option")),
                },
                match i32::from(load8) {
                    0 => None,
                    1 => Some({
                        let load9 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<f32>(result0 + 20)?;
                        load9
                    }),
                    _ => return Err(invalid_variant("option")),
                },
                match i32::from(load10) {
                    0 => None,
                    1 => Some({
                        let load11 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<u8>(result0 + 28)?;
                        match i32::from(load11) {
                            0 => U1::U32({
                                let load12 =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                        .load::<i32>(result0 + 32)?;
                                load12 as u32
                            }),
                            1 => U1::F32({
                                let load13 =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                        .load::<f32>(result0 + 32)?;
                                load13
                            }),
                            _ => return Err(invalid_variant("U1")),
                        }
                    }),
                    _ => return Err(invalid_variant("option")),
                },
                match i32::from(load14) {
                    0 => None,
                    1 => Some({
                        let load15 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<u8>(result0 + 37)?;
                        match i32::from(load15) {
                            0 => None,
                            1 => Some({
                                let load16 =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                        .load::<u8>(result0 + 38)?;
                                match i32::from(load16) {
                                    0 => false,
                                    1 => true,
                                    _ => return Err(invalid_variant("bool")),
                                }
                            }),
                            _ => return Err(invalid_variant("option")),
                        }
                    }),
                    _ => return Err(invalid_variant("option")),
                },
            ))
        }
        pub fn casts(
            &self,
            store: &mut wasmer::Store,
            a: Casts1,
            b: Casts2,
            c: Casts3,
            d: Casts4,
            e: Casts5,
            f: Casts6,
        ) -> Result<(Casts1, Casts2, Casts3, Casts4, Casts5, Casts6), wasmer::RuntimeError>
        {
            let _memory = &self.memory;
            let (result0_0, result0_1) = match a {
                Casts1::A(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                Casts1::B(e) => (1i32, (e).to_bits() as i32),
            };
            let (result1_0, result1_1) = match b {
                Casts2::A(e) => (0i32, (e).to_bits() as i64),
                Casts2::B(e) => (1i32, i64::from((e).to_bits())),
            };
            let (result2_0, result2_1) = match c {
                Casts3::A(e) => (0i32, (e).to_bits() as i64),
                Casts3::B(e) => (1i32, wit_bindgen_wasmer::rt::as_i64(e)),
            };
            let (result3_0, result3_1) = match d {
                Casts4::A(e) => (0i32, i64::from(wit_bindgen_wasmer::rt::as_i32(e))),
                Casts4::B(e) => (1i32, wit_bindgen_wasmer::rt::as_i64(e)),
            };
            let (result4_0, result4_1) = match e {
                Casts5::A(e) => (0i32, i64::from((e).to_bits())),
                Casts5::B(e) => (1i32, wit_bindgen_wasmer::rt::as_i64(e)),
            };
            let (result7_0, result7_1, result7_2) = match f {
                Casts6::A(e) => {
                    let (t5_0, t5_1) = e;
                    (
                        0i32,
                        (t5_0).to_bits() as i32,
                        wit_bindgen_wasmer::rt::as_i32(t5_1),
                    )
                }
                Casts6::B(e) => {
                    let (t6_0, t6_1) = e;
                    (
                        1i32,
                        wit_bindgen_wasmer::rt::as_i32(t6_0),
                        wit_bindgen_wasmer::rt::as_i32(t6_1),
                    )
                }
            };
            let result8 = self.func_casts.call(
                store, result0_0, result0_1, result1_0, result1_1, result2_0, result2_1, result3_0,
                result3_1, result4_0, result4_1, result7_0, result7_1, result7_2,
            )?;
            let load9 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result8 + 0)?;
            let load12 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result8 + 8)?;
            let load15 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result8 + 24)?;
            let load18 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result8 + 40)?;
            let load21 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result8 + 56)?;
            let load24 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result8 + 72)?;
            Ok((
                match i32::from(load9) {
                    0 => Casts1::A({
                        let load10 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result8 + 4)?;
                        load10
                    }),
                    1 => Casts1::B({
                        let load11 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<f32>(result8 + 4)?;
                        load11
                    }),
                    _ => return Err(invalid_variant("Casts1")),
                },
                match i32::from(load12) {
                    0 => Casts2::A({
                        let load13 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<f64>(result8 + 16)?;
                        load13
                    }),
                    1 => Casts2::B({
                        let load14 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<f32>(result8 + 16)?;
                        load14
                    }),
                    _ => return Err(invalid_variant("Casts2")),
                },
                match i32::from(load15) {
                    0 => Casts3::A({
                        let load16 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<f64>(result8 + 32)?;
                        load16
                    }),
                    1 => Casts3::B({
                        let load17 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i64>(result8 + 32)?;
                        load17 as u64
                    }),
                    _ => return Err(invalid_variant("Casts3")),
                },
                match i32::from(load18) {
                    0 => Casts4::A({
                        let load19 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result8 + 48)?;
                        load19 as u32
                    }),
                    1 => Casts4::B({
                        let load20 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i64>(result8 + 48)?;
                        load20
                    }),
                    _ => return Err(invalid_variant("Casts4")),
                },
                match i32::from(load21) {
                    0 => Casts5::A({
                        let load22 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<f32>(result8 + 64)?;
                        load22
                    }),
                    1 => Casts5::B({
                        let load23 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i64>(result8 + 64)?;
                        load23
                    }),
                    _ => return Err(invalid_variant("Casts5")),
                },
                match i32::from(load24) {
                    0 => Casts6::A({
                        let load25 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<f32>(result8 + 76)?;
                        let load26 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result8 + 80)?;
                        (load25, load26 as u32)
                    }),
                    1 => Casts6::B({
                        let load27 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result8 + 76)?;
                        let load28 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result8 + 80)?;
                        (load27 as u32, load28 as u32)
                    }),
                    _ => return Err(invalid_variant("Casts6")),
                },
            ))
        }
        pub fn expected_arg(
            &self,
            store: &mut wasmer::Store,
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1Param<'_>>,
            f: Result<&str, &[u8]>,
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let result0 = match a {
                Ok(e) => {
                    let () = e;
                    0i32
                }
                Err(e) => {
                    let () = e;
                    1i32
                }
            };
            let (result1_0, result1_1) = match b {
                Ok(e) => {
                    let () = e;
                    (0i32, 0i32)
                }
                Err(e) => (1i32, e as i32),
            };
            let (result2_0, result2_1) = match c {
                Ok(e) => (0i32, e as i32),
                Err(e) => {
                    let () = e;
                    (1i32, 0i32)
                }
            };
            let result5 = match d {
                Ok(e) => {
                    let () = e;
                    0i32
                }
                Err(e) => {
                    let () = e;
                    1i32
                }
            };
            let (result10_0, result10_1, result10_2, result10_3) = match e {
                Ok(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e), 0i32, 0i32),
                Err(e) => {
                    let (result9_0, result9_1, result9_2) = match e {
                        V1Param::A => {
                            let e = ();
                            {
                                let () = e;
                                (0i32, 0i32, 0i32)
                            }
                        }
                        V1Param::B(e) => {
                            let (result6_0, result6_1) = match e {
                                U1::U32(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                                U1::F32(e) => (1i32, (e).to_bits() as i32),
                            };
                            (1i32, result6_0, result6_1)
                        }
                        V1Param::C(e) => (2i32, e as i32, 0i32),
                        V1Param::D(e) => {
                            let vec7 = e;
                            let ptr7 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec7.len() as i32,
                            )?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store_many(ptr7, vec7.as_bytes())?;
                            (3i32, ptr7, vec7.len() as i32)
                        }
                        V1Param::E(e) => {
                            let Empty {} = e;
                            (4i32, 0i32, 0i32)
                        }
                        V1Param::F => {
                            let e = ();
                            {
                                let () = e;
                                (5i32, 0i32, 0i32)
                            }
                        }
                        V1Param::G(e) => (6i32, wit_bindgen_wasmer::rt::as_i32(e), 0i32),
                    };
                    (1i32, result9_0, result9_1, result9_2)
                }
            };
            let (result13_0, result13_1, result13_2) = match f {
                Ok(e) => {
                    let vec11 = e;
                    let ptr11 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec11.len() as i32,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr11, vec11.as_bytes())?;
                    (0i32, ptr11, vec11.len() as i32)
                }
                Err(e) => {
                    let vec12 = e;
                    let ptr12 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        (vec12.len() as i32) * 1,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr12, &vec12)?;
                    (1i32, ptr12, vec12.len() as i32)
                }
            };
            self.func_expected_arg.call(
                store, result0, result1_0, result1_1, result2_0, result2_1, result5, result10_0,
                result10_1, result10_2, result10_3, result13_0, result13_1, result13_2,
            )?;
            Ok(())
        }
        pub fn expected_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<
            (
                Result<(), ()>,
                Result<(), E1>,
                Result<E1, ()>,
                Result<(), ()>,
                Result<u32, V1Result>,
                Result<String, Vec<u8>>,
            ),
            wasmer::RuntimeError,
        > {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_expected_result.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 1)?;
            let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 3)?;
            let load6 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 5)?;
            let load7 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 8)?;
            let load18 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 24)?;
            Ok((
                match i32::from(load1) {
                    0 => Ok(()),
                    1 => Err(()),
                    _ => return Err(invalid_variant("expected")),
                },
                match i32::from(load2) {
                    0 => Ok(()),
                    1 => Err({
                        let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<u8>(result0 + 2)?;
                        match i32::from(load3) {
                            0 => E1::A,
                            _ => return Err(invalid_variant("E1")),
                        }
                    }),
                    _ => return Err(invalid_variant("expected")),
                },
                match i32::from(load4) {
                    0 => Ok({
                        let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<u8>(result0 + 4)?;
                        match i32::from(load5) {
                            0 => E1::A,
                            _ => return Err(invalid_variant("E1")),
                        }
                    }),
                    1 => Err(()),
                    _ => return Err(invalid_variant("expected")),
                },
                match i32::from(load6) {
                    0 => Ok(()),
                    1 => Err(()),
                    _ => return Err(invalid_variant("expected")),
                },
                match i32::from(load7) {
                    0 => Ok({
                        let load8 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result0 + 12)?;
                        load8 as u32
                    }),
                    1 => Err({
                        let load9 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<u8>(result0 + 12)?;
                        match i32::from(load9) {
                            0 => V1Result::A,
                            1 => V1Result::B({
                                let load10 =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                        .load::<u8>(result0 + 16)?;
                                match i32::from(load10) {
                                    0 => U1::U32({
                                        let load11 = unsafe {
                                            _memory.data_unchecked_mut(&store.as_store_ref())
                                        }
                                        .load::<i32>(result0 + 20)?;
                                        load11 as u32
                                    }),
                                    1 => U1::F32({
                                        let load12 = unsafe {
                                            _memory.data_unchecked_mut(&store.as_store_ref())
                                        }
                                        .load::<f32>(result0 + 20)?;
                                        load12
                                    }),
                                    _ => return Err(invalid_variant("U1")),
                                }
                            }),
                            2 => V1Result::C({
                                let load13 =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                        .load::<u8>(result0 + 16)?;
                                match i32::from(load13) {
                                    0 => E1::A,
                                    _ => return Err(invalid_variant("E1")),
                                }
                            }),
                            3 => V1Result::D({
                                let load14 =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                        .load::<i32>(result0 + 16)?;
                                let load15 =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                        .load::<i32>(result0 + 20)?;
                                let ptr16 = load14;
                                let len16 = load15;

                                let data16 = copy_slice(
                                    store,
                                    _memory,
                                    func_canonical_abi_free,
                                    ptr16,
                                    len16,
                                    1,
                                )?;
                                String::from_utf8(data16)
                                    .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                            }),
                            4 => V1Result::E(Empty {}),
                            5 => V1Result::F,
                            6 => V1Result::G({
                                let load17 =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                        .load::<i32>(result0 + 16)?;
                                load17 as u32
                            }),
                            _ => return Err(invalid_variant("V1Result")),
                        }
                    }),
                    _ => return Err(invalid_variant("expected")),
                },
                match i32::from(load18) {
                    0 => Ok({
                        let load19 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result0 + 28)?;
                        let load20 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result0 + 32)?;
                        let ptr21 = load19;
                        let len21 = load20;

                        let data21 =
                            copy_slice(store, _memory, func_canonical_abi_free, ptr21, len21, 1)?;
                        String::from_utf8(data21)
                            .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                    }),
                    1 => Err({
                        let load22 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result0 + 28)?;
                        let load23 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result0 + 32)?;
                        let ptr24 = load22;
                        let len24 = load23;

                        copy_slice(store, _memory, func_canonical_abi_free, ptr24, len24, 1)?
                    }),
                    _ => return Err(invalid_variant("expected")),
                },
            ))
        }
        pub fn return_expected_sugar(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Result<i32, MyErrno>, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_return_expected_sugar.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => Ok({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    load2
                }),
                1 => Err({
                    let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(result0 + 4)?;
                    match i32::from(load3) {
                        0 => MyErrno::Bad1,
                        1 => MyErrno::Bad2,
                        _ => return Err(invalid_variant("MyErrno")),
                    }
                }),
                _ => return Err(invalid_variant("expected")),
            })
        }
        pub fn return_expected_sugar2(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Result<(), MyErrno>, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_return_expected_sugar2.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => Ok(()),
                1 => Err({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(result0 + 1)?;
                    match i32::from(load2) {
                        0 => MyErrno::Bad1,
                        1 => MyErrno::Bad2,
                        _ => return Err(invalid_variant("MyErrno")),
                    }
                }),
                _ => return Err(invalid_variant("expected")),
            })
        }
        pub fn return_expected_sugar3(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Result<MyErrno, MyErrno>, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_return_expected_sugar3.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => Ok({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(result0 + 1)?;
                    match i32::from(load2) {
                        0 => MyErrno::Bad1,
                        1 => MyErrno::Bad2,
                        _ => return Err(invalid_variant("MyErrno")),
                    }
                }),
                1 => Err({
                    let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(result0 + 1)?;
                    match i32::from(load3) {
                        0 => MyErrno::Bad1,
                        1 => MyErrno::Bad2,
                        _ => return Err(invalid_variant("MyErrno")),
                    }
                }),
                _ => return Err(invalid_variant("expected")),
            })
        }
        pub fn return_expected_sugar4(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Result<(i32, u32), MyErrno>, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_return_expected_sugar4.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => Ok({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 8)?;
                    (load2, load3 as u32)
                }),
                1 => Err({
                    let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(result0 + 4)?;
                    match i32::from(load4) {
                        0 => MyErrno::Bad1,
                        1 => MyErrno::Bad2,
                        _ => return Err(invalid_variant("MyErrno")),
                    }
                }),
                _ => return Err(invalid_variant("expected")),
            })
        }
        pub fn return_option_sugar(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Option<i32>, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_return_option_sugar.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => None,
                1 => Some({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    load2
                }),
                _ => return Err(invalid_variant("option")),
            })
        }
        pub fn return_option_sugar2(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Option<MyErrno>, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_return_option_sugar2.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => None,
                1 => Some({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(result0 + 1)?;
                    match i32::from(load2) {
                        0 => MyErrno::Bad1,
                        1 => MyErrno::Bad2,
                        _ => return Err(invalid_variant("MyErrno")),
                    }
                }),
                _ => return Err(invalid_variant("option")),
            })
        }
        pub fn expected_simple(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Result<u32, i32>, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_expected_simple.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => Ok({
                    let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    load2 as u32
                }),
                1 => Err({
                    let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(result0 + 4)?;
                    load3
                }),
                _ => return Err(invalid_variant("expected")),
            })
        }
        pub fn is_clone_arg(
            &self,
            store: &mut wasmer::Store,
            a: IsCloneParam<'_>,
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let IsCloneParam { v1: v10 } = a;
            let (result4_0, result4_1, result4_2) = match v10 {
                V1Param::A => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0i32, 0i32)
                    }
                }
                V1Param::B(e) => {
                    let (result1_0, result1_1) = match e {
                        U1::U32(e) => (0i32, wit_bindgen_wasmer::rt::as_i32(e)),
                        U1::F32(e) => (1i32, (e).to_bits() as i32),
                    };
                    (1i32, result1_0, result1_1)
                }
                V1Param::C(e) => (2i32, e as i32, 0i32),
                V1Param::D(e) => {
                    let vec2 = e;
                    let ptr2 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec2.len() as i32,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr2, vec2.as_bytes())?;
                    (3i32, ptr2, vec2.len() as i32)
                }
                V1Param::E(e) => {
                    let Empty {} = e;
                    (4i32, 0i32, 0i32)
                }
                V1Param::F => {
                    let e = ();
                    {
                        let () = e;
                        (5i32, 0i32, 0i32)
                    }
                }
                V1Param::G(e) => (6i32, wit_bindgen_wasmer::rt::as_i32(e), 0i32),
            };
            self.func_is_clone_arg
                .call(store, result4_0, result4_1, result4_2)?;
            Ok(())
        }
        pub fn is_clone_return(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<IsCloneResult, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_is_clone_return.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            Ok(IsCloneResult {
                v1: match i32::from(load1) {
                    0 => V1Result::A,
                    1 => V1Result::B({
                        let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<u8>(result0 + 4)?;
                        match i32::from(load2) {
                            0 => U1::U32({
                                let load3 =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                        .load::<i32>(result0 + 8)?;
                                load3 as u32
                            }),
                            1 => U1::F32({
                                let load4 =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                        .load::<f32>(result0 + 8)?;
                                load4
                            }),
                            _ => return Err(invalid_variant("U1")),
                        }
                    }),
                    2 => V1Result::C({
                        let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<u8>(result0 + 4)?;
                        match i32::from(load5) {
                            0 => E1::A,
                            _ => return Err(invalid_variant("E1")),
                        }
                    }),
                    3 => V1Result::D({
                        let load6 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result0 + 4)?;
                        let load7 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result0 + 8)?;
                        let ptr8 = load6;
                        let len8 = load7;

                        let data8 =
                            copy_slice(store, _memory, func_canonical_abi_free, ptr8, len8, 1)?;
                        String::from_utf8(data8)
                            .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                    }),
                    4 => V1Result::E(Empty {}),
                    5 => V1Result::F,
                    6 => V1Result::G({
                        let load9 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                            .load::<i32>(result0 + 4)?;
                        load9 as u32
                    }),
                    _ => return Err(invalid_variant("V1Result")),
                },
            })
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
