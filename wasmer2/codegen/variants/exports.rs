pub mod variants {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum E1 {
        A,
    }
    impl std::fmt::Debug for E1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for U1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                U1::U32(e) => f.debug_tuple("U1::U32").field(e).finish(),
                U1::F32(e) => f.debug_tuple("U1::F32").field(e).finish(),
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Empty {}
    impl std::fmt::Debug for Empty {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl<'a> std::fmt::Debug for V1Param<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for V1Result {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for Casts1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for Casts2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for Casts3 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for Casts4 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for Casts5 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for Casts6 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for MyErrno {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MyErrno")
                .field("code", &(*self as i32))
                .field("name", &self.name())
                .field("message", &self.message())
                .finish()
        }
    }
    impl std::fmt::Display for MyErrno {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} (error {})", self.name(), *self as i32)
        }
    }

    impl std::error::Error for MyErrno {}
    #[derive(Clone)]
    pub struct IsCloneParam<'a> {
        pub v1: V1Param<'a>,
    }
    impl<'a> std::fmt::Debug for IsCloneParam<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("IsCloneParam")
                .field("v1", &self.v1)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct IsCloneResult {
        pub v1: V1Result,
    }
    impl std::fmt::Debug for IsCloneResult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("IsCloneResult")
                .field("v1", &self.v1)
                .finish()
        }
    }
    pub trait Variants: Sized + wasmer::WasmerEnv + 'static {
        fn e1_arg(&mut self, x: E1) -> ();

        fn e1_result(&mut self) -> E1;

        fn u1_arg(&mut self, x: U1) -> ();

        fn u1_result(&mut self) -> U1;

        fn v1_arg(&mut self, x: V1Param<'_>) -> ();

        fn v1_result(&mut self) -> V1Result;

        fn bool_arg(&mut self, x: bool) -> ();

        fn bool_result(&mut self) -> bool;

        fn option_arg(
            &mut self,
            a: Option<bool>,
            b: Option<()>,
            c: Option<u32>,
            d: Option<E1>,
            e: Option<f32>,
            f: Option<U1>,
            g: Option<Option<bool>>,
        ) -> ();

        fn option_result(
            &mut self,
        ) -> (
            Option<bool>,
            Option<()>,
            Option<u32>,
            Option<E1>,
            Option<f32>,
            Option<U1>,
            Option<Option<bool>>,
        );

        fn casts(
            &mut self,
            a: Casts1,
            b: Casts2,
            c: Casts3,
            d: Casts4,
            e: Casts5,
            f: Casts6,
        ) -> (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6);

        fn expected_arg(
            &mut self,
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1Param<'_>>,
            f: Result<&str, &[u8]>,
        ) -> ();

        fn expected_result(
            &mut self,
        ) -> (
            Result<(), ()>,
            Result<(), E1>,
            Result<E1, ()>,
            Result<(), ()>,
            Result<u32, V1Result>,
            Result<String, Vec<u8>>,
        );

        fn return_expected_sugar(&mut self) -> Result<i32, MyErrno>;

        fn return_expected_sugar2(&mut self) -> Result<(), MyErrno>;

        fn return_expected_sugar3(&mut self) -> Result<MyErrno, MyErrno>;

        fn return_expected_sugar4(&mut self) -> Result<(i32, u32), MyErrno>;

        fn return_option_sugar(&mut self) -> Option<i32>;

        fn return_option_sugar2(&mut self) -> Option<MyErrno>;

        fn expected_simple(&mut self) -> Result<u32, i32>;

        fn is_clone_arg(&mut self, a: IsCloneParam<'_>) -> ();

        fn is_clone_return(&mut self) -> IsCloneResult;
    }

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: Variants,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Variants> {
            data: T,
            memory: wasmer::LazyInit<wasmer::Memory>,
            func_canonical_abi_realloc:
                wasmer::LazyInit<wasmer::NativeFunc<(i32, i32, i32, i32), i32>>,
        }
        unsafe impl<T: Variants> Send for EnvWrapper<T> {}
        unsafe impl<T: Variants> Sync for EnvWrapper<T> {}
        impl<T: Variants> wasmer::WasmerEnv for EnvWrapper<T> {
            fn init_with_instance(
                &mut self,
                instance: &wasmer::Instance,
            ) -> Result<(), wasmer::HostEnvInitError> {
                self.data.init_with_instance(instance)?;
                self.memory
                    .initialize(instance.exports.get_with_generics_weak("memory")?);
                self.func_canonical_abi_realloc.initialize(
                    instance
                        .exports
                        .get_with_generics_weak("canonical_abi_realloc")?,
                );
                Ok(())
            }
        }
        let env = std::sync::Arc::new(std::sync::Mutex::new(EnvWrapper {
            data,
            memory: wasmer::LazyInit::new(),
            func_canonical_abi_realloc: wasmer::LazyInit::new(),
        }));
        let mut exports = wasmer::Exports::new();
        exports.insert(
            "e1-arg",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = match arg0 {
                        0 => E1::A,
                        _ => return Err(invalid_variant("E1")),
                    };
                    let result = host.e1_arg(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert("e1-result", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.e1_result();
      Ok(result as i32)
    }));
        exports.insert(
            "u1-arg",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = match arg0 {
                        0 => U1::U32(arg1 as u32),
                        1 => U1::F32(f32::from_bits(arg1 as u32)),
                        _ => return Err(invalid_variant("U1")),
                    };
                    let result = host.u1_arg(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "u1-result",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let result = host.u1_result();
                    match result {
                        U1::U32(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg0 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        U1::F32(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(arg0 + 4, wit_bindgen_wasmer::rt::as_f32(e))?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "v1-arg",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let mut _bc = unsafe {
                        wit_bindgen_wasmer::BorrowChecker::new(
                            env.memory.get_ref().unwrap().data_unchecked_mut(),
                        )
                    };
                    let host = &mut env.data;
                    let param0 = match arg0 {
                        0 => V1Param::A,
                        1 => V1Param::B(match arg1 {
                            0 => U1::U32(arg2 as u32),
                            1 => U1::F32(f32::from_bits(arg2 as u32)),
                            _ => return Err(invalid_variant("U1")),
                        }),
                        2 => V1Param::C(match arg1 {
                            0 => E1::A,
                            _ => return Err(invalid_variant("E1")),
                        }),
                        3 => V1Param::D({
                            let ptr0 = arg1;
                            let len0 = arg2;
                            _bc.slice_str(ptr0, len0)?
                        }),
                        4 => V1Param::E(Empty {}),
                        5 => V1Param::F,
                        6 => V1Param::G(arg1 as u32),
                        _ => return Err(invalid_variant("V1Param")),
                    };
                    let result = host.v1_arg(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "v1-result",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let func_canonical_abi_realloc =
                        env.func_canonical_abi_realloc.get_ref().unwrap();
                    let host = &mut env.data;
                    let result = host.v1_result();
                    match result {
                        V1Result::A => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                        V1Result::B(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            match e {
                                U1::U32(e) => {
                                    caller_memory.store(
                                        arg0 + 4,
                                        wit_bindgen_wasmer::rt::as_i32(0i32) as u8,
                                    )?;
                                    caller_memory.store(
                                        arg0 + 8,
                                        wit_bindgen_wasmer::rt::as_i32(
                                            wit_bindgen_wasmer::rt::as_i32(e),
                                        ),
                                    )?;
                                }
                                U1::F32(e) => {
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store(
                                        arg0 + 4,
                                        wit_bindgen_wasmer::rt::as_i32(1i32) as u8,
                                    )?;
                                    caller_memory
                                        .store(arg0 + 8, wit_bindgen_wasmer::rt::as_f32(e))?;
                                }
                            };
                        }
                        V1Result::C(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(2i32) as u8)?;
                            caller_memory
                                .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                        V1Result::D(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(3i32) as u8)?;
                            let vec0 = e;
                            let ptr0 =
                                func_canonical_abi_realloc.call(0, 0, 1, vec0.len() as i32)?;
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory.store_many(ptr0, vec0.as_bytes())?;
                            caller_memory.store(
                                arg0 + 8,
                                wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32),
                            )?;
                            caller_memory.store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                        }
                        V1Result::E(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(4i32) as u8)?;
                            let Empty {} = e;
                        }
                        V1Result::F => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(5i32) as u8)?;
                                let () = e;
                            }
                        }
                        V1Result::G(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(6i32) as u8)?;
                            caller_memory.store(
                                arg0 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "bool-arg",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = match arg0 {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    };
                    let result = host.bool_arg(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert("bool-result", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let host = &mut env.data;
      let result = host.bool_result();
      Ok(match result { true => 1, false => 0 })
    }));
        exports.insert(
            "option-arg",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32,
                      arg5: i32,
                      arg6: i32,
                      arg7: i32,
                      arg8: f32,
                      arg9: i32,
                      arg10: i32,
                      arg11: i32,
                      arg12: i32,
                      arg13: i32,
                      arg14: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = match arg0 {
                        0 => None,
                        1 => Some(match arg1 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        }),
                        _ => return Err(invalid_variant("option")),
                    };
                    let param1 = match arg2 {
                        0 => None,
                        1 => Some(()),
                        _ => return Err(invalid_variant("option")),
                    };
                    let param2 = match arg3 {
                        0 => None,
                        1 => Some(arg4 as u32),
                        _ => return Err(invalid_variant("option")),
                    };
                    let param3 = match arg5 {
                        0 => None,
                        1 => Some(match arg6 {
                            0 => E1::A,
                            _ => return Err(invalid_variant("E1")),
                        }),
                        _ => return Err(invalid_variant("option")),
                    };
                    let param4 = match arg7 {
                        0 => None,
                        1 => Some(arg8),
                        _ => return Err(invalid_variant("option")),
                    };
                    let param5 = match arg9 {
                        0 => None,
                        1 => Some(match arg10 {
                            0 => U1::U32(arg11 as u32),
                            1 => U1::F32(f32::from_bits(arg11 as u32)),
                            _ => return Err(invalid_variant("U1")),
                        }),
                        _ => return Err(invalid_variant("option")),
                    };
                    let param6 = match arg12 {
                        0 => None,
                        1 => Some(match arg13 {
                            0 => None,
                            1 => Some(match arg14 {
                                0 => false,
                                1 => true,
                                _ => return Err(invalid_variant("bool")),
                            }),
                            _ => return Err(invalid_variant("option")),
                        }),
                        _ => return Err(invalid_variant("option")),
                    };
                    let result =
                        host.option_arg(param0, param1, param2, param3, param4, param5, param6);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "option-result",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let result = host.option_result();
                    let (t0_0, t0_1, t0_2, t0_3, t0_4, t0_5, t0_6) = result;
                    match t0_0 {
                        Some(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg0 + 1,
                                wit_bindgen_wasmer::rt::as_i32(match e {
                                    true => 1,
                                    false => 0,
                                }) as u8,
                            )?;
                        }
                        None => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    match t0_1 {
                        Some(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 2, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            let () = e;
                        }
                        None => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 2, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    match t0_2 {
                        Some(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg0 + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        None => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    match t0_3 {
                        Some(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 12, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory
                                .store(arg0 + 13, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                        None => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 12, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    match t0_4 {
                        Some(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 16, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(arg0 + 20, wit_bindgen_wasmer::rt::as_f32(e))?;
                        }
                        None => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 16, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    match t0_5 {
                        Some(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 24, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            match e {
                                U1::U32(e) => {
                                    caller_memory.store(
                                        arg0 + 28,
                                        wit_bindgen_wasmer::rt::as_i32(0i32) as u8,
                                    )?;
                                    caller_memory.store(
                                        arg0 + 32,
                                        wit_bindgen_wasmer::rt::as_i32(
                                            wit_bindgen_wasmer::rt::as_i32(e),
                                        ),
                                    )?;
                                }
                                U1::F32(e) => {
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store(
                                        arg0 + 28,
                                        wit_bindgen_wasmer::rt::as_i32(1i32) as u8,
                                    )?;
                                    caller_memory
                                        .store(arg0 + 32, wit_bindgen_wasmer::rt::as_f32(e))?;
                                }
                            };
                        }
                        None => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 24, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    match t0_6 {
                        Some(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 36, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            match e {
                                Some(e) => {
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store(
                                        arg0 + 37,
                                        wit_bindgen_wasmer::rt::as_i32(1i32) as u8,
                                    )?;
                                    caller_memory.store(
                                        arg0 + 38,
                                        wit_bindgen_wasmer::rt::as_i32(match e {
                                            true => 1,
                                            false => 0,
                                        }) as u8,
                                    )?;
                                }
                                None => {
                                    let e = ();
                                    {
                                        caller_memory.store(
                                            arg0 + 37,
                                            wit_bindgen_wasmer::rt::as_i32(0i32) as u8,
                                        )?;
                                        let () = e;
                                    }
                                }
                            };
                        }
                        None => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 36, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "casts",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i64,
                      arg4: i32,
                      arg5: i64,
                      arg6: i32,
                      arg7: i64,
                      arg8: i32,
                      arg9: i64,
                      arg10: i32,
                      arg11: i32,
                      arg12: i32,
                      arg13: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let param0 = match arg0 {
                        0 => Casts1::A(arg1),
                        1 => Casts1::B(f32::from_bits(arg1 as u32)),
                        _ => return Err(invalid_variant("Casts1")),
                    };
                    let param1 = match arg2 {
                        0 => Casts2::A(f64::from_bits(arg3 as u64)),
                        1 => Casts2::B(f32::from_bits(arg3 as u32)),
                        _ => return Err(invalid_variant("Casts2")),
                    };
                    let param2 = match arg4 {
                        0 => Casts3::A(f64::from_bits(arg5 as u64)),
                        1 => Casts3::B(arg5 as u64),
                        _ => return Err(invalid_variant("Casts3")),
                    };
                    let param3 = match arg6 {
                        0 => Casts4::A(arg7 as i32 as u32),
                        1 => Casts4::B(arg7),
                        _ => return Err(invalid_variant("Casts4")),
                    };
                    let param4 = match arg8 {
                        0 => Casts5::A(f32::from_bits(arg9 as u32)),
                        1 => Casts5::B(arg9),
                        _ => return Err(invalid_variant("Casts5")),
                    };
                    let param5 = match arg10 {
                        0 => Casts6::A((f32::from_bits(arg11 as u32), arg12 as u32)),
                        1 => Casts6::B((arg11 as u32, arg12 as u32)),
                        _ => return Err(invalid_variant("Casts6")),
                    };
                    let result = host.casts(param0, param1, param2, param3, param4, param5);
                    let (t0_0, t0_1, t0_2, t0_3, t0_4, t0_5) = result;
                    match t0_0 {
                        Casts1::A(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg13 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        Casts1::B(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(arg13 + 4, wit_bindgen_wasmer::rt::as_f32(e))?;
                        }
                    };
                    match t0_1 {
                        Casts2::A(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 8, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(arg13 + 16, wit_bindgen_wasmer::rt::as_f64(e))?;
                        }
                        Casts2::B(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 8, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(arg13 + 16, wit_bindgen_wasmer::rt::as_f32(e))?;
                        }
                    };
                    match t0_2 {
                        Casts3::A(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 24, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(arg13 + 32, wit_bindgen_wasmer::rt::as_f64(e))?;
                        }
                        Casts3::B(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 24, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg13 + 32,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(e)),
                            )?;
                        }
                    };
                    match t0_3 {
                        Casts4::A(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 40, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg13 + 48,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        Casts4::B(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 40, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg13 + 48,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(e)),
                            )?;
                        }
                    };
                    match t0_4 {
                        Casts5::A(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 56, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(arg13 + 64, wit_bindgen_wasmer::rt::as_f32(e))?;
                        }
                        Casts5::B(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 56, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg13 + 64,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(e)),
                            )?;
                        }
                    };
                    match t0_5 {
                        Casts6::A(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 72, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            let (t1_0, t1_1) = e;
                            caller_memory
                                .store(arg13 + 76, wit_bindgen_wasmer::rt::as_f32(t1_0))?;
                            caller_memory.store(
                                arg13 + 80,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t1_1,
                                )),
                            )?;
                        }
                        Casts6::B(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg13 + 72, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            let (t2_0, t2_1) = e;
                            caller_memory.store(
                                arg13 + 76,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t2_0,
                                )),
                            )?;
                            caller_memory.store(
                                arg13 + 80,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t2_1,
                                )),
                            )?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "expected-arg",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32,
                      arg5: i32,
                      arg6: i32,
                      arg7: i32,
                      arg8: i32,
                      arg9: i32,
                      arg10: i32,
                      arg11: i32,
                      arg12: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let mut _bc = unsafe {
                        wit_bindgen_wasmer::BorrowChecker::new(
                            env.memory.get_ref().unwrap().data_unchecked_mut(),
                        )
                    };
                    let host = &mut env.data;
                    let param0 = match arg0 {
                        0 => Ok(()),
                        1 => Err(()),
                        _ => return Err(invalid_variant("expected")),
                    };
                    let param1 = match arg1 {
                        0 => Ok(()),
                        1 => Err(match arg2 {
                            0 => E1::A,
                            _ => return Err(invalid_variant("E1")),
                        }),
                        _ => return Err(invalid_variant("expected")),
                    };
                    let param2 = match arg3 {
                        0 => Ok(match arg4 {
                            0 => E1::A,
                            _ => return Err(invalid_variant("E1")),
                        }),
                        1 => Err(()),
                        _ => return Err(invalid_variant("expected")),
                    };
                    let param3 = match arg5 {
                        0 => Ok(()),
                        1 => Err(()),
                        _ => return Err(invalid_variant("expected")),
                    };
                    let param4 = match arg6 {
                        0 => Ok(arg7 as u32),
                        1 => Err(match arg7 {
                            0 => V1Param::A,
                            1 => V1Param::B(match arg8 {
                                0 => U1::U32(arg9 as u32),
                                1 => U1::F32(f32::from_bits(arg9 as u32)),
                                _ => return Err(invalid_variant("U1")),
                            }),
                            2 => V1Param::C(match arg8 {
                                0 => E1::A,
                                _ => return Err(invalid_variant("E1")),
                            }),
                            3 => V1Param::D({
                                let ptr0 = arg8;
                                let len0 = arg9;
                                _bc.slice_str(ptr0, len0)?
                            }),
                            4 => V1Param::E(Empty {}),
                            5 => V1Param::F,
                            6 => V1Param::G(arg8 as u32),
                            _ => return Err(invalid_variant("V1Param")),
                        }),
                        _ => return Err(invalid_variant("expected")),
                    };
                    let param5 = match arg10 {
                        0 => Ok({
                            let ptr1 = arg11;
                            let len1 = arg12;
                            _bc.slice_str(ptr1, len1)?
                        }),
                        1 => Err({
                            let ptr2 = arg11;
                            let len2 = arg12;
                            _bc.slice(ptr2, len2)?
                        }),
                        _ => return Err(invalid_variant("expected")),
                    };
                    let result = host.expected_arg(param0, param1, param2, param3, param4, param5);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "expected-result",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let func_canonical_abi_realloc =
                        env.func_canonical_abi_realloc.get_ref().unwrap();
                    let host = &mut env.data;
                    let result = host.expected_result();
                    let (t0_0, t0_1, t0_2, t0_3, t0_4, t0_5) = result;
                    match t0_0 {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            let () = e;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            let () = e;
                        }
                    };
                    match t0_1 {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 1, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            let () = e;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 1, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory
                                .store(arg0 + 2, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                    };
                    match t0_2 {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 3, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory
                                .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 3, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            let () = e;
                        }
                    };
                    match t0_3 {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 5, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            let () = e;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 5, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            let () = e;
                        }
                    };
                    match t0_4 {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 8, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg0 + 12,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 8, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            match e {
                                V1Result::A => {
                                    let e = ();
                                    {
                                        caller_memory.store(
                                            arg0 + 12,
                                            wit_bindgen_wasmer::rt::as_i32(0i32) as u8,
                                        )?;
                                        let () = e;
                                    }
                                }
                                V1Result::B(e) => {
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store(
                                        arg0 + 12,
                                        wit_bindgen_wasmer::rt::as_i32(1i32) as u8,
                                    )?;
                                    match e {
                                        U1::U32(e) => {
                                            caller_memory.store(
                                                arg0 + 16,
                                                wit_bindgen_wasmer::rt::as_i32(0i32) as u8,
                                            )?;
                                            caller_memory.store(
                                                arg0 + 20,
                                                wit_bindgen_wasmer::rt::as_i32(
                                                    wit_bindgen_wasmer::rt::as_i32(e),
                                                ),
                                            )?;
                                        }
                                        U1::F32(e) => {
                                            let caller_memory = unsafe {
                                                env.memory.get_ref().unwrap().data_unchecked_mut()
                                            };
                                            caller_memory.store(
                                                arg0 + 16,
                                                wit_bindgen_wasmer::rt::as_i32(1i32) as u8,
                                            )?;
                                            caller_memory.store(
                                                arg0 + 20,
                                                wit_bindgen_wasmer::rt::as_f32(e),
                                            )?;
                                        }
                                    };
                                }
                                V1Result::C(e) => {
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store(
                                        arg0 + 12,
                                        wit_bindgen_wasmer::rt::as_i32(2i32) as u8,
                                    )?;
                                    caller_memory.store(
                                        arg0 + 16,
                                        wit_bindgen_wasmer::rt::as_i32(e as i32) as u8,
                                    )?;
                                }
                                V1Result::D(e) => {
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store(
                                        arg0 + 12,
                                        wit_bindgen_wasmer::rt::as_i32(3i32) as u8,
                                    )?;
                                    let vec3 = e;
                                    let ptr3 = func_canonical_abi_realloc.call(
                                        0,
                                        0,
                                        1,
                                        vec3.len() as i32,
                                    )?;
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store_many(ptr3, vec3.as_bytes())?;
                                    caller_memory.store(
                                        arg0 + 20,
                                        wit_bindgen_wasmer::rt::as_i32(vec3.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg0 + 16, wit_bindgen_wasmer::rt::as_i32(ptr3))?;
                                }
                                V1Result::E(e) => {
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store(
                                        arg0 + 12,
                                        wit_bindgen_wasmer::rt::as_i32(4i32) as u8,
                                    )?;
                                    let Empty {} = e;
                                }
                                V1Result::F => {
                                    let e = ();
                                    {
                                        let caller_memory = unsafe {
                                            env.memory.get_ref().unwrap().data_unchecked_mut()
                                        };
                                        caller_memory.store(
                                            arg0 + 12,
                                            wit_bindgen_wasmer::rt::as_i32(5i32) as u8,
                                        )?;
                                        let () = e;
                                    }
                                }
                                V1Result::G(e) => {
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store(
                                        arg0 + 12,
                                        wit_bindgen_wasmer::rt::as_i32(6i32) as u8,
                                    )?;
                                    caller_memory.store(
                                        arg0 + 16,
                                        wit_bindgen_wasmer::rt::as_i32(
                                            wit_bindgen_wasmer::rt::as_i32(e),
                                        ),
                                    )?;
                                }
                            };
                        }
                    };
                    match t0_5 {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 24, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            let vec5 = e;
                            let ptr5 =
                                func_canonical_abi_realloc.call(0, 0, 1, vec5.len() as i32)?;
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory.store_many(ptr5, vec5.as_bytes())?;
                            caller_memory.store(
                                arg0 + 32,
                                wit_bindgen_wasmer::rt::as_i32(vec5.len() as i32),
                            )?;
                            caller_memory.store(arg0 + 28, wit_bindgen_wasmer::rt::as_i32(ptr5))?;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 24, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            let vec6 = e;
                            let ptr6 = func_canonical_abi_realloc.call(
                                0,
                                0,
                                1,
                                (vec6.len() as i32) * 1,
                            )?;
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory.store_many(ptr6, &vec6)?;
                            caller_memory.store(
                                arg0 + 32,
                                wit_bindgen_wasmer::rt::as_i32(vec6.len() as i32),
                            )?;
                            caller_memory.store(arg0 + 28, wit_bindgen_wasmer::rt::as_i32(ptr6))?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "return-expected-sugar",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let result = host.return_expected_sugar();
                    match result {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg0 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory
                                .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "return-expected-sugar2",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let result = host.return_expected_sugar2();
                    match result {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            let () = e;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory
                                .store(arg0 + 1, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "return-expected-sugar3",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let result = host.return_expected_sugar3();
                    match result {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory
                                .store(arg0 + 1, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory
                                .store(arg0 + 1, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "return-expected-sugar4",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let result = host.return_expected_sugar4();
                    match result {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            let (t0_0, t0_1) = e;
                            caller_memory.store(
                                arg0 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t0_0,
                                )),
                            )?;
                            caller_memory.store(
                                arg0 + 8,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(
                                    t0_1,
                                )),
                            )?;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory
                                .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "return-option-sugar",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let result = host.return_option_sugar();
                    match result {
                        Some(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg0 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        None => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "return-option-sugar2",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let result = host.return_option_sugar2();
                    match result {
                        Some(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory
                                .store(arg0 + 1, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                        None => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "expected-simple",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let host = &mut env.data;
                    let result = host.expected_simple();
                    match result {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg0 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg0 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "is-clone-arg",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let mut _bc = unsafe {
                        wit_bindgen_wasmer::BorrowChecker::new(
                            env.memory.get_ref().unwrap().data_unchecked_mut(),
                        )
                    };
                    let host = &mut env.data;
                    let param0 = IsCloneParam {
                        v1: match arg0 {
                            0 => V1Param::A,
                            1 => V1Param::B(match arg1 {
                                0 => U1::U32(arg2 as u32),
                                1 => U1::F32(f32::from_bits(arg2 as u32)),
                                _ => return Err(invalid_variant("U1")),
                            }),
                            2 => V1Param::C(match arg1 {
                                0 => E1::A,
                                _ => return Err(invalid_variant("E1")),
                            }),
                            3 => V1Param::D({
                                let ptr0 = arg1;
                                let len0 = arg2;
                                _bc.slice_str(ptr0, len0)?
                            }),
                            4 => V1Param::E(Empty {}),
                            5 => V1Param::F,
                            6 => V1Param::G(arg1 as u32),
                            _ => return Err(invalid_variant("V1Param")),
                        },
                    };
                    let result = host.is_clone_arg(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "is-clone-return",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let func_canonical_abi_realloc =
                        env.func_canonical_abi_realloc.get_ref().unwrap();
                    let host = &mut env.data;
                    let result = host.is_clone_return();
                    let IsCloneResult { v1: v10 } = result;
                    match v10 {
                        V1Result::A => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                        V1Result::B(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            match e {
                                U1::U32(e) => {
                                    caller_memory.store(
                                        arg0 + 4,
                                        wit_bindgen_wasmer::rt::as_i32(0i32) as u8,
                                    )?;
                                    caller_memory.store(
                                        arg0 + 8,
                                        wit_bindgen_wasmer::rt::as_i32(
                                            wit_bindgen_wasmer::rt::as_i32(e),
                                        ),
                                    )?;
                                }
                                U1::F32(e) => {
                                    let caller_memory = unsafe {
                                        env.memory.get_ref().unwrap().data_unchecked_mut()
                                    };
                                    caller_memory.store(
                                        arg0 + 4,
                                        wit_bindgen_wasmer::rt::as_i32(1i32) as u8,
                                    )?;
                                    caller_memory
                                        .store(arg0 + 8, wit_bindgen_wasmer::rt::as_f32(e))?;
                                }
                            };
                        }
                        V1Result::C(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(2i32) as u8)?;
                            caller_memory
                                .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                        V1Result::D(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(3i32) as u8)?;
                            let vec1 = e;
                            let ptr1 =
                                func_canonical_abi_realloc.call(0, 0, 1, vec1.len() as i32)?;
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory.store_many(ptr1, vec1.as_bytes())?;
                            caller_memory.store(
                                arg0 + 8,
                                wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32),
                            )?;
                            caller_memory.store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(ptr1))?;
                        }
                        V1Result::E(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(4i32) as u8)?;
                            let Empty {} = e;
                        }
                        V1Result::F => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                                caller_memory
                                    .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(5i32) as u8)?;
                                let () = e;
                            }
                        }
                        V1Result::G(e) => {
                            let caller_memory =
                                unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(6i32) as u8)?;
                            caller_memory.store(
                                arg0 + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                    };
                    Ok(())
                },
            ),
        );
        imports.register("variants", exports);
    }
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
