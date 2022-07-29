#[allow(clippy::all)]
pub mod imports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum E1 {
        A,
        B,
    }
    impl core::fmt::Debug for E1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                E1::A => f.debug_tuple("E1::A").finish(),
                E1::B => f.debug_tuple("E1::B").finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C1 {
        A(i32),
        B(i64),
    }
    impl core::fmt::Debug for C1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C1::A(e) => f.debug_tuple("C1::A").field(e).finish(),
                C1::B(e) => f.debug_tuple("C1::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C2 {
        A(i32),
        B(f32),
    }
    impl core::fmt::Debug for C2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C2::A(e) => f.debug_tuple("C2::A").field(e).finish(),
                C2::B(e) => f.debug_tuple("C2::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C3 {
        A(i32),
        B(f64),
    }
    impl core::fmt::Debug for C3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C3::A(e) => f.debug_tuple("C3::A").field(e).finish(),
                C3::B(e) => f.debug_tuple("C3::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C4 {
        A(i64),
        B(f32),
    }
    impl core::fmt::Debug for C4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C4::A(e) => f.debug_tuple("C4::A").field(e).finish(),
                C4::B(e) => f.debug_tuple("C4::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C5 {
        A(i64),
        B(f64),
    }
    impl core::fmt::Debug for C5 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C5::A(e) => f.debug_tuple("C5::A").field(e).finish(),
                C5::B(e) => f.debug_tuple("C5::B").field(e).finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum C6 {
        A(f32),
        B(f64),
    }
    impl core::fmt::Debug for C6 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                C6::A(e) => f.debug_tuple("C6::A").field(e).finish(),
                C6::B(e) => f.debug_tuple("C6::B").field(e).finish(),
            }
        }
    }
    pub type Casts = (C1, C2, C3, C4, C5, C6);
    #[derive(Clone, Copy)]
    pub enum Z1 {
        A(i32),
        B,
    }
    impl core::fmt::Debug for Z1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Z1::A(e) => f.debug_tuple("Z1::A").field(e).finish(),
                Z1::B => f.debug_tuple("Z1::B").finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Z2 {
        A(i64),
        B,
    }
    impl core::fmt::Debug for Z2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Z2::A(e) => f.debug_tuple("Z2::A").field(e).finish(),
                Z2::B => f.debug_tuple("Z2::B").finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Z3 {
        A(f32),
        B,
    }
    impl core::fmt::Debug for Z3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Z3::A(e) => f.debug_tuple("Z3::A").field(e).finish(),
                Z3::B => f.debug_tuple("Z3::B").finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Z4 {
        A(f64),
        B,
    }
    impl core::fmt::Debug for Z4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Z4::A(e) => f.debug_tuple("Z4::A").field(e).finish(),
                Z4::B => f.debug_tuple("Z4::B").finish(),
            }
        }
    }
    pub type Zeros = (Z1, Z2, Z3, Z4);
    pub type OptionTypedef = Option<u32>;
    pub type BoolTypedef = bool;
    pub type ResultTypedef = Result<u32, ()>;
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum MyErrno {
        Success,
        A,
        B,
    }
    impl MyErrno {
        pub fn name(&self) -> &'static str {
            match self {
                MyErrno::Success => "success",
                MyErrno::A => "a",
                MyErrno::B => "b",
            }
        }
        pub fn message(&self) -> &'static str {
            match self {
                MyErrno::Success => "",
                MyErrno::A => "",
                MyErrno::B => "",
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
    pub trait Imports: Sized + Send + Sync + 'static {
        fn roundtrip_option(&mut self, a: Option<f32>) -> Option<u8>;

        fn roundtrip_result(&mut self, a: Result<u32, f32>) -> Result<f64, u8>;

        fn roundtrip_enum(&mut self, a: E1) -> E1;

        fn invert_bool(&mut self, a: bool) -> bool;

        fn variant_casts(&mut self, a: Casts) -> Casts;

        fn variant_zeros(&mut self, a: Zeros) -> Zeros;

        fn variant_typedefs(&mut self, a: OptionTypedef, b: BoolTypedef, c: ResultTypedef) -> ();

        fn variant_enums(
            &mut self,
            a: bool,
            b: Result<(), ()>,
            c: MyErrno,
        ) -> (bool, Result<(), ()>, MyErrno);
    }
    pub struct LazyInitialized {
        memory: wasmer::Memory,
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
            lazy: std::rc::Rc<OnceCell<LazyInitialized>>,
        }
        unsafe impl<T: Imports> Send for EnvWrapper<T> {}
        unsafe impl<T: Imports> Sync for EnvWrapper<T> {}
        let lazy = std::rc::Rc::new(OnceCell::new());
        let env = EnvWrapper {
            data,
            lazy: std::rc::Rc::clone(&lazy),
        };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
            "roundtrip-option",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: f32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => None,
                        1 => Some(arg1),
                        _ => return Err(invalid_variant("option")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_option(param0);
                    match result {
                        Some(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg2 + 1,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e))
                                    as u8,
                            )?;
                        }
                        None => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                                caller_memory
                                    .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "roundtrip-result",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => Ok(arg1 as u32),
                        1 => Err(f32::from_bits(arg1 as u32)),
                        _ => return Err(invalid_variant("expected")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_result(param0);
                    match result {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(arg2 + 8, wit_bindgen_wasmer::rt::as_f64(e))?;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg2 + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e))
                                    as u8,
                            )?;
                        }
                    };
                    Ok(())
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
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => E1::A,
                        1 => E1::B,
                        _ => return Err(invalid_variant("E1")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.roundtrip_enum(param0);
                    Ok(result as i32)
                },
            ),
        );
        exports.insert(
            "invert-bool",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.invert_bool(param0);
                    Ok(match result {
                        true => 1,
                        false => 0,
                    })
                },
            ),
        );
        exports.insert(
            "variant-casts",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i64,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32,
                      arg5: i64,
                      arg6: i32,
                      arg7: i64,
                      arg8: i32,
                      arg9: i64,
                      arg10: i32,
                      arg11: i64,
                      arg12: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = (
                        match arg0 {
                            0 => C1::A(arg1 as i32),
                            1 => C1::B(arg1),
                            _ => return Err(invalid_variant("C1")),
                        },
                        match arg2 {
                            0 => C2::A(arg3),
                            1 => C2::B(f32::from_bits(arg3 as u32)),
                            _ => return Err(invalid_variant("C2")),
                        },
                        match arg4 {
                            0 => C3::A(arg5 as i32),
                            1 => C3::B(f64::from_bits(arg5 as u64)),
                            _ => return Err(invalid_variant("C3")),
                        },
                        match arg6 {
                            0 => C4::A(arg7),
                            1 => C4::B(f32::from_bits(arg7 as u32)),
                            _ => return Err(invalid_variant("C4")),
                        },
                        match arg8 {
                            0 => C5::A(arg9),
                            1 => C5::B(f64::from_bits(arg9 as u64)),
                            _ => return Err(invalid_variant("C5")),
                        },
                        match arg10 {
                            0 => C6::A(f32::from_bits(arg11 as u32)),
                            1 => C6::B(f64::from_bits(arg11 as u64)),
                            _ => return Err(invalid_variant("C6")),
                        },
                    );
                    let host = &mut data_mut.data;
                    let result = host.variant_casts(param0);
                    let (t0_0, t0_1, t0_2, t0_3, t0_4, t0_5) = result;
                    match t0_0 {
                        C1::A(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg12 + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        C1::B(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg12 + 8,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(e)),
                            )?;
                        }
                    };
                    match t0_1 {
                        C2::A(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 16, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg12 + 20,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        C2::B(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 16, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(arg12 + 20, wit_bindgen_wasmer::rt::as_f32(e))?;
                        }
                    };
                    match t0_2 {
                        C3::A(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 24, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg12 + 32,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        C3::B(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 24, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(arg12 + 32, wit_bindgen_wasmer::rt::as_f64(e))?;
                        }
                    };
                    match t0_3 {
                        C4::A(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 40, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg12 + 48,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(e)),
                            )?;
                        }
                        C4::B(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 40, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(arg12 + 48, wit_bindgen_wasmer::rt::as_f32(e))?;
                        }
                    };
                    match t0_4 {
                        C5::A(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 56, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg12 + 64,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(e)),
                            )?;
                        }
                        C5::B(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 56, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(arg12 + 64, wit_bindgen_wasmer::rt::as_f64(e))?;
                        }
                    };
                    match t0_5 {
                        C6::A(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 72, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(arg12 + 80, wit_bindgen_wasmer::rt::as_f32(e))?;
                        }
                        C6::B(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg12 + 72, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(arg12 + 80, wit_bindgen_wasmer::rt::as_f64(e))?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "variant-zeros",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i64,
                      arg4: i32,
                      arg5: f32,
                      arg6: i32,
                      arg7: f64,
                      arg8: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = (
                        match arg0 {
                            0 => Z1::A(arg1),
                            1 => Z1::B,
                            _ => return Err(invalid_variant("Z1")),
                        },
                        match arg2 {
                            0 => Z2::A(arg3),
                            1 => Z2::B,
                            _ => return Err(invalid_variant("Z2")),
                        },
                        match arg4 {
                            0 => Z3::A(arg5),
                            1 => Z3::B,
                            _ => return Err(invalid_variant("Z3")),
                        },
                        match arg6 {
                            0 => Z4::A(arg7),
                            1 => Z4::B,
                            _ => return Err(invalid_variant("Z4")),
                        },
                    );
                    let host = &mut data_mut.data;
                    let result = host.variant_zeros(param0);
                    let (t0_0, t0_1, t0_2, t0_3) = result;
                    match t0_0 {
                        Z1::A(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg8 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg8 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        Z1::B => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                                caller_memory
                                    .store(arg8 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    match t0_1 {
                        Z2::A(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg8 + 8, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg8 + 16,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(e)),
                            )?;
                        }
                        Z2::B => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                                caller_memory
                                    .store(arg8 + 8, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    match t0_2 {
                        Z3::A(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg8 + 24, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(arg8 + 28, wit_bindgen_wasmer::rt::as_f32(e))?;
                        }
                        Z3::B => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                                caller_memory
                                    .store(arg8 + 24, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    match t0_3 {
                        Z4::A(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg8 + 32, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(arg8 + 40, wit_bindgen_wasmer::rt::as_f64(e))?;
                        }
                        Z4::B => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                                caller_memory
                                    .store(arg8 + 32, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "variant-typedefs",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => None,
                        1 => Some(arg1 as u32),
                        _ => return Err(invalid_variant("option")),
                    };
                    let param1 = match arg2 {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    };
                    let param2 = match arg3 {
                        0 => Ok(arg4 as u32),
                        1 => Err(()),
                        _ => return Err(invalid_variant("expected")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.variant_typedefs(param0, param1, param2);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "variant-enums",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    };
                    let param1 = match arg1 {
                        0 => Ok(()),
                        1 => Err(()),
                        _ => return Err(invalid_variant("expected")),
                    };
                    let param2 = match arg2 {
                        0 => MyErrno::Success,
                        1 => MyErrno::A,
                        2 => MyErrno::B,
                        _ => return Err(invalid_variant("MyErrno")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.variant_enums(param0, param1, param2);
                    let (t0_0, t0_1, t0_2) = result;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(
                        arg3 + 0,
                        wit_bindgen_wasmer::rt::as_i32(match t0_0 {
                            true => 1,
                            false => 0,
                        }) as u8,
                    )?;
                    match t0_1 {
                        Ok(e) => {
                            caller_memory
                                .store(arg3 + 1, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            let () = e;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg3 + 1, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            let () = e;
                        }
                    };
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory
                        .store(arg3 + 2, wit_bindgen_wasmer::rt::as_i32(t0_2 as i32) as u8)?;
                    Ok(())
                },
            ),
        );
        imports.register_namespace("imports", exports);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| {
            let memory = _instance.exports.get_memory("memory")?.clone();
            lazy.set(LazyInitialized { memory })
                .map_err(|_e| anyhow::anyhow!("Couldn't set lazy initialized data"))?;
            Ok(())
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::once_cell::unsync::OnceCell;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
