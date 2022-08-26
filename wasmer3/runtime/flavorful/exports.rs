#[allow(clippy::all)]
pub mod imports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[derive(Clone)]
    pub struct ListInRecord1<'a> {
        pub a: &'a str,
    }
    impl<'a> core::fmt::Debug for ListInRecord1<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ListInRecord1").field("a", &self.a).finish()
        }
    }
    #[derive(Clone)]
    pub struct ListInRecord2 {
        pub a: String,
    }
    impl core::fmt::Debug for ListInRecord2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ListInRecord2").field("a", &self.a).finish()
        }
    }
    #[derive(Clone)]
    pub struct ListInRecord3Param<'a> {
        pub a: &'a str,
    }
    impl<'a> core::fmt::Debug for ListInRecord3Param<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ListInRecord3Param")
                .field("a", &self.a)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct ListInRecord3Result {
        pub a: String,
    }
    impl core::fmt::Debug for ListInRecord3Result {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ListInRecord3Result")
                .field("a", &self.a)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct ListInRecord4Param<'a> {
        pub a: &'a str,
    }
    impl<'a> core::fmt::Debug for ListInRecord4Param<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ListInRecord4Param")
                .field("a", &self.a)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct ListInRecord4Result {
        pub a: String,
    }
    impl core::fmt::Debug for ListInRecord4Result {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ListInRecord4Result")
                .field("a", &self.a)
                .finish()
        }
    }
    pub type ListInAliasParam<'a> = ListInRecord4Param<'a>;
    pub type ListInAliasResult = ListInRecord4Result;
    pub type ListInVariant1V1<'a> = Option<&'a str>;
    pub type ListInVariant1V2<'a> = Result<(), &'a str>;
    #[derive(Clone)]
    pub enum ListInVariant1V3<'a> {
        String(&'a str),
        F32(f32),
    }
    impl<'a> core::fmt::Debug for ListInVariant1V3<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                ListInVariant1V3::String(e) => {
                    f.debug_tuple("ListInVariant1V3::String").field(e).finish()
                }
                ListInVariant1V3::F32(e) => {
                    f.debug_tuple("ListInVariant1V3::F32").field(e).finish()
                }
            }
        }
    }
    pub type ListInVariant2 = Option<String>;
    pub type ListInVariant3Param<'a> = Option<&'a str>;
    pub type ListInVariant3Result = Option<String>;
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
    pub type ListTypedef<'a> = &'a str;
    pub type ListTypedef2 = Vec<u8>;
    pub type ListTypedef3Param<'a> = Vec<&'a str>;
    pub type ListTypedef3Result = Vec<String>;
    pub trait Imports: Sized + Send + Sync + 'static {
        fn list_in_record1(&mut self, a: ListInRecord1<'_>) -> ();

        fn list_in_record2(&mut self) -> ListInRecord2;

        fn list_in_record3(&mut self, a: ListInRecord3Param<'_>) -> ListInRecord3Result;

        fn list_in_record4(&mut self, a: ListInAliasParam<'_>) -> ListInAliasResult;

        fn list_in_variant1(
            &mut self,
            a: ListInVariant1V1<'_>,
            b: ListInVariant1V2<'_>,
            c: ListInVariant1V3<'_>,
        ) -> ();

        fn list_in_variant2(&mut self) -> ListInVariant2;

        fn list_in_variant3(&mut self, a: ListInVariant3Param<'_>) -> ListInVariant3Result;

        fn errno_result(&mut self) -> Result<(), MyErrno>;

        fn list_typedefs(
            &mut self,
            a: ListTypedef<'_>,
            c: ListTypedef3Param<'_>,
        ) -> (ListTypedef2, ListTypedef3Result);

        fn list_of_variants(
            &mut self,
            a: Vec<bool>,
            b: Vec<Result<(), ()>>,
            c: Vec<MyErrno>,
        ) -> (Vec<bool>, Vec<Result<(), ()>>, Vec<MyErrno>);
    }
    pub struct LazyInitialized {
        memory: wasmer::Memory,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
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
            "list-in-record1",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let param0 = ListInRecord1 {
                        a: _bc.slice_str(ptr0, len0)?,
                    };
                    let host = &mut data_mut.data;
                    let result = host.list_in_record1(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-in-record2",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let host = &mut data_mut.data;
                    let result = host.list_in_record2();
                    let ListInRecord2 { a: a0 } = result;
                    let vec1 = a0;
                    let ptr1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec1.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store_many(ptr1, vec1.as_bytes())?;
                    caller_memory
                        .store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32))?;
                    caller_memory.store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(ptr1))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-in-record3",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let param0 = ListInRecord3Param {
                        a: _bc.slice_str(ptr0, len0)?,
                    };
                    let host = &mut data_mut.data;
                    let result = host.list_in_record3(param0);
                    let ListInRecord3Result { a: a1 } = result;
                    let vec2 = a1;
                    let ptr2 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec2.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store_many(ptr2, vec2.as_bytes())?;
                    caller_memory
                        .store(arg2 + 4, wit_bindgen_wasmer::rt::as_i32(vec2.len() as i32))?;
                    caller_memory.store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(ptr2))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-in-record4",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let param0 = ListInRecord4Param {
                        a: _bc.slice_str(ptr0, len0)?,
                    };
                    let host = &mut data_mut.data;
                    let result = host.list_in_record4(param0);
                    let ListInRecord4Result { a: a1 } = result;
                    let vec2 = a1;
                    let ptr2 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec2.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store_many(ptr2, vec2.as_bytes())?;
                    caller_memory
                        .store(arg2 + 4, wit_bindgen_wasmer::rt::as_i32(vec2.len() as i32))?;
                    caller_memory.store(arg2 + 0, wit_bindgen_wasmer::rt::as_i32(ptr2))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-in-variant1",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32,
                      arg5: i32,
                      arg6: i32,
                      arg7: i32,
                      arg8: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
                    });
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => None,
                        1 => Some({
                            let ptr0 = arg1;
                            let len0 = arg2;
                            _bc.slice_str(ptr0, len0)?
                        }),
                        _ => return Err(invalid_variant("option")),
                    };
                    let param1 = match arg3 {
                        0 => Ok(()),
                        1 => Err({
                            let ptr1 = arg4;
                            let len1 = arg5;
                            _bc.slice_str(ptr1, len1)?
                        }),
                        _ => return Err(invalid_variant("expected")),
                    };
                    let param2 = match arg6 {
                        0 => ListInVariant1V3::String({
                            let ptr2 = arg7;
                            let len2 = arg8;
                            _bc.slice_str(ptr2, len2)?
                        }),
                        1 => ListInVariant1V3::F32(f32::from_bits(arg7 as u32)),
                        _ => return Err(invalid_variant("ListInVariant1V3")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.list_in_variant1(param0, param1, param2);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-in-variant2",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let host = &mut data_mut.data;
                    let result = host.list_in_variant2();
                    match result {
                        Some(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            let vec0 = e;
                            let ptr0 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec0.len() as i32,
                            )?;
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory.store_many(ptr0, vec0.as_bytes())?;
                            caller_memory.store(
                                arg0 + 8,
                                wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32),
                            )?;
                            caller_memory.store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                        }
                        None => {
                            let e = ();
                            {
                                let _memory_view = _memory.view(&store);
                                let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
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
            "list-in-variant3",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
                    });
                    let data_mut = store.data_mut();
                    let param0 = match arg0 {
                        0 => None,
                        1 => Some({
                            let ptr0 = arg1;
                            let len0 = arg2;
                            _bc.slice_str(ptr0, len0)?
                        }),
                        _ => return Err(invalid_variant("option")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.list_in_variant3(param0);
                    match result {
                        Some(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg3 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            let vec1 = e;
                            let ptr1 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec1.len() as i32,
                            )?;
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory.store_many(ptr1, vec1.as_bytes())?;
                            caller_memory.store(
                                arg3 + 8,
                                wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32),
                            )?;
                            caller_memory.store(arg3 + 4, wit_bindgen_wasmer::rt::as_i32(ptr1))?;
                        }
                        None => {
                            let e = ();
                            {
                                let _memory_view = _memory.view(&store);
                                let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                                caller_memory
                                    .store(arg3 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                                let () = e;
                            }
                        }
                    };
                    Ok(())
                },
            ),
        );
        exports.insert(
            "errno-result",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let host = &mut data_mut.data;
                    let result = host.errno_result();
                    match result {
                        Ok(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            let () = e;
                        }
                        Err(e) => {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
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
            "list-typedefs",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
                    });
                    let data_mut = store.data_mut();
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let len4 = arg3;
                    let base4 = arg2;
                    let mut result4 = Vec::with_capacity(len4 as usize);
                    for i in 0..len4 {
                        let base = base4 + i * 8;
                        result4.push({
                            let load1 = _bc.load::<i32>(base + 0)?;
                            let load2 = _bc.load::<i32>(base + 4)?;
                            let ptr3 = load1;
                            let len3 = load2;
                            _bc.slice_str(ptr3, len3)?
                        });
                    }
                    let param0 = _bc.slice_str(ptr0, len0)?;
                    let param1 = result4;
                    let host = &mut data_mut.data;
                    let result = host.list_typedefs(param0, param1);
                    let (t5_0, t5_1) = result;
                    let vec6 = t5_0;
                    let ptr6 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        (vec6.len() as i32) * 1,
                    )?;
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store_many(ptr6, &vec6)?;
                    caller_memory
                        .store(arg4 + 4, wit_bindgen_wasmer::rt::as_i32(vec6.len() as i32))?;
                    caller_memory.store(arg4 + 0, wit_bindgen_wasmer::rt::as_i32(ptr6))?;
                    let vec8 = t5_1;
                    let len8 = vec8.len() as i32;
                    let result8 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        len8 * 8,
                    )?;
                    for (i, e) in vec8.into_iter().enumerate() {
                        let base = result8 + (i as i32) * 8;
                        {
                            let vec7 = e;
                            let ptr7 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec7.len() as i32,
                            )?;
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory.store_many(ptr7, vec7.as_bytes())?;
                            caller_memory.store(
                                base + 4,
                                wit_bindgen_wasmer::rt::as_i32(vec7.len() as i32),
                            )?;
                            caller_memory.store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr7))?;
                        }
                    }
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store(arg4 + 12, wit_bindgen_wasmer::rt::as_i32(len8))?;
                    caller_memory.store(arg4 + 8, wit_bindgen_wasmer::rt::as_i32(result8))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "list-of-variants",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32,
                      arg3: i32,
                      arg4: i32,
                      arg5: i32,
                      arg6: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let func_canonical_abi_realloc = store
                        .data()
                        .lazy
                        .get()
                        .unwrap()
                        .func_canonical_abi_realloc
                        .clone();
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
                    });
                    let data_mut = store.data_mut();
                    let len1 = arg1;
                    let base1 = arg0;
                    let mut result1 = Vec::with_capacity(len1 as usize);
                    for i in 0..len1 {
                        let base = base1 + i * 1;
                        result1.push({
                            let load0 = _bc.load::<u8>(base + 0)?;
                            match i32::from(load0) {
                                0 => false,
                                1 => true,
                                _ => return Err(invalid_variant("bool")),
                            }
                        });
                    }
                    let len3 = arg3;
                    let base3 = arg2;
                    let mut result3 = Vec::with_capacity(len3 as usize);
                    for i in 0..len3 {
                        let base = base3 + i * 1;
                        result3.push({
                            let load2 = _bc.load::<u8>(base + 0)?;
                            match i32::from(load2) {
                                0 => Ok(()),
                                1 => Err(()),
                                _ => return Err(invalid_variant("expected")),
                            }
                        });
                    }
                    let len5 = arg5;
                    let base5 = arg4;
                    let mut result5 = Vec::with_capacity(len5 as usize);
                    for i in 0..len5 {
                        let base = base5 + i * 1;
                        result5.push({
                            let load4 = _bc.load::<u8>(base + 0)?;
                            match i32::from(load4) {
                                0 => MyErrno::Success,
                                1 => MyErrno::A,
                                2 => MyErrno::B,
                                _ => return Err(invalid_variant("MyErrno")),
                            }
                        });
                    }
                    let param0 = result1;
                    let param1 = result3;
                    let param2 = result5;
                    let host = &mut data_mut.data;
                    let result = host.list_of_variants(param0, param1, param2);
                    let (t6_0, t6_1, t6_2) = result;
                    let vec7 = t6_0;
                    let len7 = vec7.len() as i32;
                    let result7 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        len7 * 1,
                    )?;
                    for (i, e) in vec7.into_iter().enumerate() {
                        let base = result7 + (i as i32) * 1;
                        {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory.store(
                                base + 0,
                                wit_bindgen_wasmer::rt::as_i32(match e {
                                    true => 1,
                                    false => 0,
                                }) as u8,
                            )?;
                        }
                    }
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store(arg6 + 4, wit_bindgen_wasmer::rt::as_i32(len7))?;
                    caller_memory.store(arg6 + 0, wit_bindgen_wasmer::rt::as_i32(result7))?;
                    let vec8 = t6_1;
                    let len8 = vec8.len() as i32;
                    let result8 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        len8 * 1,
                    )?;
                    for (i, e) in vec8.into_iter().enumerate() {
                        let base = result8 + (i as i32) * 1;
                        {
                            match e {
                                Ok(e) => {
                                    let _memory_view = _memory.view(&store);
                                    let caller_memory =
                                        unsafe { _memory_view.data_unchecked_mut() };
                                    caller_memory.store(
                                        base + 0,
                                        wit_bindgen_wasmer::rt::as_i32(0i32) as u8,
                                    )?;
                                    let () = e;
                                }
                                Err(e) => {
                                    let _memory_view = _memory.view(&store);
                                    let caller_memory =
                                        unsafe { _memory_view.data_unchecked_mut() };
                                    caller_memory.store(
                                        base + 0,
                                        wit_bindgen_wasmer::rt::as_i32(1i32) as u8,
                                    )?;
                                    let () = e;
                                }
                            };
                        }
                    }
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store(arg6 + 12, wit_bindgen_wasmer::rt::as_i32(len8))?;
                    caller_memory.store(arg6 + 8, wit_bindgen_wasmer::rt::as_i32(result8))?;
                    let vec9 = t6_2;
                    let len9 = vec9.len() as i32;
                    let result9 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        len9 * 1,
                    )?;
                    for (i, e) in vec9.into_iter().enumerate() {
                        let base = result9 + (i as i32) * 1;
                        {
                            let _memory_view = _memory.view(&store);
                            let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                            caller_memory
                                .store(base + 0, wit_bindgen_wasmer::rt::as_i32(e as i32) as u8)?;
                        }
                    }
                    let _memory_view = _memory.view(&store);
                    let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
                    caller_memory.store(arg6 + 20, wit_bindgen_wasmer::rt::as_i32(len9))?;
                    caller_memory.store(arg6 + 16, wit_bindgen_wasmer::rt::as_i32(result9))?;
                    Ok(())
                },
            ),
        );
        imports.register_namespace("imports", exports);
        move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| {
            let memory = _instance.exports.get_memory("memory")?.clone();
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(&_store.as_store_ref(), "canonical_abi_realloc")
                .unwrap()
                .clone();
            lazy.set(LazyInitialized {
                memory,
                func_canonical_abi_realloc,
            })
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
