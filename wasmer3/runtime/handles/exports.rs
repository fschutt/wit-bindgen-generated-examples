#[allow(clippy::all)]
pub mod imports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub struct HostStateParamRecord<'a, T: Imports> {
        pub a: &'a T::HostState2,
    }
    impl<'a, T: Imports> core::fmt::Debug for HostStateParamRecord<'a, T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("HostStateParamRecord")
                .field("a", &self.a)
                .finish()
        }
    }
    pub type HostStateParamTuple<'a, T: Imports> = (&'a T::HostState2,);
    pub type HostStateParamOption<'a, T: Imports> = Option<&'a T::HostState2>;
    pub type HostStateParamResult<'a, T: Imports> = Result<&'a T::HostState2, u32>;
    pub enum HostStateParamVariant<'a, T: Imports> {
        HostState2(&'a T::HostState2),
        U32(u32),
    }
    impl<'a, T: Imports> core::fmt::Debug for HostStateParamVariant<'a, T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                HostStateParamVariant::HostState2(e) => f
                    .debug_tuple("HostStateParamVariant::HostState2")
                    .field(e)
                    .finish(),
                HostStateParamVariant::U32(e) => f
                    .debug_tuple("HostStateParamVariant::U32")
                    .field(e)
                    .finish(),
            }
        }
    }
    pub struct HostStateResultRecord<T: Imports> {
        pub a: T::HostState2,
    }
    impl<T: Imports> core::fmt::Debug for HostStateResultRecord<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("HostStateResultRecord")
                .field("a", &self.a)
                .finish()
        }
    }
    pub type HostStateResultTuple<T: Imports> = (T::HostState2,);
    pub type HostStateResultOption<T: Imports> = Option<T::HostState2>;
    pub type HostStateResultResult<T: Imports> = Result<T::HostState2, u32>;
    pub enum HostStateResultVariant<T: Imports> {
        HostState2(T::HostState2),
        U32(u32),
    }
    impl<T: Imports> core::fmt::Debug for HostStateResultVariant<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                HostStateResultVariant::HostState2(e) => f
                    .debug_tuple("HostStateResultVariant::HostState2")
                    .field(e)
                    .finish(),
                HostStateResultVariant::U32(e) => f
                    .debug_tuple("HostStateResultVariant::U32")
                    .field(e)
                    .finish(),
            }
        }
    }
    pub trait Imports: Sized + Send + Sync + 'static {
        type HostState: std::fmt::Debug;
        type HostState2: std::fmt::Debug;
        type Markdown2: std::fmt::Debug;
        type OddName: std::fmt::Debug;
        fn host_state_create(&mut self) -> Self::HostState;

        fn host_state_get(&mut self, a: &Self::HostState) -> u32;

        fn host_state2_create(&mut self) -> Self::HostState2;

        fn host_state2_saw_close(&mut self) -> bool;

        fn two_host_states(
            &mut self,
            a: &Self::HostState,
            b: &Self::HostState2,
        ) -> (Self::HostState, Self::HostState2);

        fn host_state2_param_record(&mut self, a: HostStateParamRecord<'_, Self>) -> ();

        fn host_state2_param_tuple(&mut self, a: HostStateParamTuple<'_, Self>) -> ();

        fn host_state2_param_option(&mut self, a: HostStateParamOption<'_, Self>) -> ();

        fn host_state2_param_result(&mut self, a: HostStateParamResult<'_, Self>) -> ();

        fn host_state2_param_variant(&mut self, a: HostStateParamVariant<'_, Self>) -> ();

        fn host_state2_param_list(&mut self, a: Vec<&Self::HostState2>) -> ();

        fn host_state2_result_record(&mut self) -> HostStateResultRecord<Self>;

        fn host_state2_result_tuple(&mut self) -> HostStateResultTuple<Self>;

        fn host_state2_result_option(&mut self) -> HostStateResultOption<Self>;

        fn host_state2_result_result(&mut self) -> HostStateResultResult<Self>;

        fn host_state2_result_variant(&mut self) -> HostStateResultVariant<Self>;

        fn host_state2_result_list(&mut self) -> Vec<Self::HostState2>;

        fn markdown2_create(&mut self) -> Self::Markdown2;

        fn markdown2_append(&mut self, self_: &Self::Markdown2, buf: &str) -> ();

        fn markdown2_render(&mut self, self_: &Self::Markdown2) -> String;

        fn odd_name_create(&mut self) -> Self::OddName;

        fn odd_name_frob_the_odd(&mut self, self_: &Self::OddName) -> ();

        fn drop_host_state(&mut self, state: Self::HostState) {
            drop(state);
        }
        fn drop_host_state2(&mut self, state: Self::HostState2) {
            drop(state);
        }
        fn drop_markdown2(&mut self, state: Self::Markdown2) {
            drop(state);
        }
        fn drop_odd_name(&mut self, state: Self::OddName) {
            drop(state);
        }
    }

    pub struct ImportsTables<T: Imports> {
        pub(crate) host_state_table: wit_bindgen_wasmer::Table<T::HostState>,
        pub(crate) host_state2_table: wit_bindgen_wasmer::Table<T::HostState2>,
        pub(crate) markdown2_table: wit_bindgen_wasmer::Table<T::Markdown2>,
        pub(crate) odd_name_table: wit_bindgen_wasmer::Table<T::OddName>,
    }
    impl<T: Imports> Default for ImportsTables<T> {
        fn default() -> Self {
            Self {
                host_state_table: Default::default(),
                host_state2_table: Default::default(),
                markdown2_table: Default::default(),
                odd_name_table: Default::default(),
            }
        }
    }
    impl<T: Imports> Clone for ImportsTables<T> {
        fn clone(&self) -> Self {
            Self::default()
        }
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
            tables: std::rc::Rc<core::cell::RefCell<ImportsTables<T>>>,
            lazy: std::rc::Rc<OnceCell<LazyInitialized>>,
        }
        unsafe impl<T: Imports> Send for EnvWrapper<T> {}
        unsafe impl<T: Imports> Sync for EnvWrapper<T> {}
        let lazy = std::rc::Rc::new(OnceCell::new());
        let env = EnvWrapper {
            data,
            tables: std::rc::Rc::default(),
            lazy: std::rc::Rc::clone(&lazy),
        };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
        "host-state-create",
        wasmer::Function::new_native(
        &mut store,
        &env,
        move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
          let data_mut = store.data_mut();
          let tables = data_mut.tables.borrow_mut();
          let host = &mut data_mut.data;
          let result = host.host_state_create();
          drop(tables);
          Ok({
            let data_mut = store.data_mut();
            let mut tables = data_mut.tables.borrow_mut();
            tables.host_state_table.insert(result) as i32
          })
        }
        ));
        exports.insert(
            "host-state-get",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<i32, wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = tables
                        .host_state_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let host = &mut data_mut.data;
                    let result = host.host_state_get(param0);
                    drop(tables);
                    Ok(wit_bindgen_wasmer::rt::as_i32(result))
                },
            ),
        );
        exports.insert(
        "host-state2-create",
        wasmer::Function::new_native(
        &mut store,
        &env,
        move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
          let data_mut = store.data_mut();
          let tables = data_mut.tables.borrow_mut();
          let host = &mut data_mut.data;
          let result = host.host_state2_create();
          drop(tables);
          Ok({
            let data_mut = store.data_mut();
            let mut tables = data_mut.tables.borrow_mut();
            tables.host_state2_table.insert(result) as i32
          })
        }
        ));
        exports.insert(
        "host-state2-saw-close",
        wasmer::Function::new_native(
        &mut store,
        &env,
        move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
          let data_mut = store.data_mut();
          let tables = data_mut.tables.borrow_mut();
          let host = &mut data_mut.data;
          let result = host.host_state2_saw_close();
          drop(tables);
          Ok(match result { true => 1, false => 0 })
        }
        ));
        exports.insert(
            "two-host-states",
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
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = tables
                        .host_state_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let param1 = tables
                        .host_state2_table
                        .get((arg1) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let host = &mut data_mut.data;
                    let result = host.two_host_states(param0, param1);
                    drop(tables);
                    let (t0_0, t0_1) = result;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(
                        arg2 + 0,
                        wit_bindgen_wasmer::rt::as_i32({
                            let data_mut = store.data_mut();
                            let mut tables = data_mut.tables.borrow_mut();
                            tables.host_state_table.insert(t0_0) as i32
                        }),
                    )?;
                    caller_memory.store(
                        arg2 + 4,
                        wit_bindgen_wasmer::rt::as_i32({
                            let data_mut = store.data_mut();
                            let mut tables = data_mut.tables.borrow_mut();
                            tables.host_state2_table.insert(t0_1) as i32
                        }),
                    )?;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "host-state2-param-record",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = HostStateParamRecord {
                        a: tables
                            .host_state2_table
                            .get((arg0) as u32)
                            .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?,
                    };
                    let host = &mut data_mut.data;
                    let result = host.host_state2_param_record(param0);
                    drop(tables);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "host-state2-param-tuple",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = (tables
                        .host_state2_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?,);
                    let host = &mut data_mut.data;
                    let result = host.host_state2_param_tuple(param0);
                    drop(tables);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "host-state2-param-option",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 =
                        match arg0 {
                            0 => None,
                            1 => Some(tables.host_state2_table.get((arg1) as u32).ok_or_else(
                                || wasmer::RuntimeError::new("invalid handle index"),
                            )?),
                            _ => return Err(invalid_variant("option")),
                        };
                    let host = &mut data_mut.data;
                    let result = host.host_state2_param_option(param0);
                    drop(tables);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "host-state2-param-result",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = match arg0 {
                        0 => Ok(tables
                            .host_state2_table
                            .get((arg1) as u32)
                            .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?),
                        1 => Err(arg1 as u32),
                        _ => return Err(invalid_variant("expected")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.host_state2_param_result(param0);
                    drop(tables);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "host-state2-param-variant",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = match arg0 {
                        0 => HostStateParamVariant::HostState2(
                            tables
                                .host_state2_table
                                .get((arg1) as u32)
                                .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?,
                        ),
                        1 => HostStateParamVariant::U32(arg1 as u32),
                        _ => return Err(invalid_variant("HostStateParamVariant")),
                    };
                    let host = &mut data_mut.data;
                    let result = host.host_state2_param_variant(param0);
                    drop(tables);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "host-state2-param-list",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let len1 = arg1;
                    let base1 = arg0;
                    let mut result1 = Vec::with_capacity(len1 as usize);
                    for i in 0..len1 {
                        let base = base1 + i * 4;
                        result1.push({
                            let load0 = _bc.load::<i32>(base + 0)?;
                            tables
                                .host_state2_table
                                .get((load0) as u32)
                                .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?
                        });
                    }
                    let param0 = result1;
                    let host = &mut data_mut.data;
                    let result = host.host_state2_param_list(param0);
                    drop(tables);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
        "host-state2-result-record",
        wasmer::Function::new_native(
        &mut store,
        &env,
        move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
          let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
          let data_mut = store.data_mut();
          let tables = data_mut.tables.borrow_mut();
          let host = &mut data_mut.data;
          let result = host.host_state2_result_record();
          drop(tables);
          let HostStateResultRecord{ a:a0, } = result;
          Ok({
            let data_mut = store.data_mut();
            let mut tables = data_mut.tables.borrow_mut();
            tables.host_state2_table.insert(a0) as i32
          })
        }
        ));
        exports.insert(
        "host-state2-result-tuple",
        wasmer::Function::new_native(
        &mut store,
        &env,
        move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
          let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
          let data_mut = store.data_mut();
          let tables = data_mut.tables.borrow_mut();
          let host = &mut data_mut.data;
          let result = host.host_state2_result_tuple();
          drop(tables);
          let (t0_0, ) = result;
          Ok({
            let data_mut = store.data_mut();
            let mut tables = data_mut.tables.borrow_mut();
            tables.host_state2_table.insert(t0_0) as i32
          })
        }
        ));
        exports.insert(
            "host-state2-result-option",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let host = &mut data_mut.data;
                    let result = host.host_state2_result_option();
                    drop(tables);
                    match result {
                        Some(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                            caller_memory.store(
                                arg0 + 4,
                                wit_bindgen_wasmer::rt::as_i32({
                                    let data_mut = store.data_mut();
                                    let mut tables = data_mut.tables.borrow_mut();
                                    tables.host_state2_table.insert(e) as i32
                                }),
                            )?;
                        }
                        None => {
                            let e = ();
                            {
                                let caller_memory =
                                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
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
            "host-state2-result-result",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let host = &mut data_mut.data;
                    let result = host.host_state2_result_result();
                    drop(tables);
                    match result {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg0 + 4,
                                wit_bindgen_wasmer::rt::as_i32({
                                    let data_mut = store.data_mut();
                                    let mut tables = data_mut.tables.borrow_mut();
                                    tables.host_state2_table.insert(e) as i32
                                }),
                            )?;
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
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
            "host-state2-result-variant",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let host = &mut data_mut.data;
                    let result = host.host_state2_result_variant();
                    drop(tables);
                    match result {
                        HostStateResultVariant::HostState2(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg0 + 4,
                                wit_bindgen_wasmer::rt::as_i32({
                                    let data_mut = store.data_mut();
                                    let mut tables = data_mut.tables.borrow_mut();
                                    tables.host_state2_table.insert(e) as i32
                                }),
                            )?;
                        }
                        HostStateResultVariant::U32(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
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
            "host-state2-result-list",
            wasmer::Function::new_native(
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
                    let tables = data_mut.tables.borrow_mut();
                    let host = &mut data_mut.data;
                    let result = host.host_state2_result_list();
                    drop(tables);
                    let vec0 = result;
                    let len0 = vec0.len() as i32;
                    let result0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        4,
                        len0 * 4,
                    )?;
                    for (i, e) in vec0.into_iter().enumerate() {
                        let base = result0 + (i as i32) * 4;
                        {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory.store(
                                base + 0,
                                wit_bindgen_wasmer::rt::as_i32({
                                    let data_mut = store.data_mut();
                                    let mut tables = data_mut.tables.borrow_mut();
                                    tables.host_state2_table.insert(e) as i32
                                }),
                            )?;
                        }
                    }
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store(arg0 + 4, wit_bindgen_wasmer::rt::as_i32(len0))?;
                    caller_memory.store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(result0))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
          "markdown2::create",
          wasmer::Function::new_native(
          &mut store,
          &env,
          move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
            let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
            let data_mut = store.data_mut();
            let tables = data_mut.tables.borrow_mut();
            let host = &mut data_mut.data;
            let result = host.markdown2_create();
            drop(tables);
            Ok({
              let data_mut = store.data_mut();
              let mut tables = data_mut.tables.borrow_mut();
              tables.markdown2_table.insert(result) as i32
            })
          }
          ));
        exports.insert(
            "markdown2::append",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32,
                      arg2: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let ptr0 = arg1;
                    let len0 = arg2;
                    let param0 = tables
                        .markdown2_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let param1 = _bc.slice_str(ptr0, len0)?;
                    let host = &mut data_mut.data;
                    let result = host.markdown2_append(param0, param1);
                    drop(tables);
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "markdown2::render",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32,
                      arg1: i32|
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
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = tables
                        .markdown2_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let host = &mut data_mut.data;
                    let result = host.markdown2_render(param0);
                    drop(tables);
                    let vec0 = result;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec0.len() as i32,
                    )?;
                    let caller_memory =
                        unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                    caller_memory
                        .store(arg1 + 4, wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32))?;
                    caller_memory.store(arg1 + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                    Ok(())
                },
            ),
        );
        exports.insert(
          "odd-name::create",
          wasmer::Function::new_native(
          &mut store,
          &env,
          move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>| -> Result<i32, wasmer::RuntimeError> {
            let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
            let data_mut = store.data_mut();
            let tables = data_mut.tables.borrow_mut();
            let host = &mut data_mut.data;
            let result = host.odd_name_create();
            drop(tables);
            Ok({
              let data_mut = store.data_mut();
              let mut tables = data_mut.tables.borrow_mut();
              tables.odd_name_table.insert(result) as i32
            })
          }
          ));
        exports.insert(
            "odd-name::frob-the-odd",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let data_mut = store.data_mut();
                    let tables = data_mut.tables.borrow_mut();
                    let param0 = tables
                        .odd_name_table
                        .get((arg0) as u32)
                        .ok_or_else(|| wasmer::RuntimeError::new("invalid handle index"))?;
                    let host = &mut data_mut.data;
                    let result = host.odd_name_frob_the_odd(param0);
                    drop(tables);
                    let () = result;
                    Ok(())
                },
            ),
        );
        imports.register_namespace("imports", exports);
        let mut canonical_abi = imports
            .get_namespace_exports("canonical_abi")
            .unwrap_or_else(wasmer::Exports::new);
        canonical_abi.insert(
            "resource_drop_host-state",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      handle: u32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let mut tables = data_mut.tables.borrow_mut();
                    let handle = tables.host_state_table.remove(handle).map_err(|e| {
                        wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
                    })?;
                    let host = &mut data_mut.data;
                    host.drop_host_state(handle);
                    Ok(())
                },
            ),
        );
        canonical_abi.insert(
            "resource_drop_host-state2",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      handle: u32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let mut tables = data_mut.tables.borrow_mut();
                    let handle = tables.host_state2_table.remove(handle).map_err(|e| {
                        wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
                    })?;
                    let host = &mut data_mut.data;
                    host.drop_host_state2(handle);
                    Ok(())
                },
            ),
        );
        canonical_abi.insert(
            "resource_drop_markdown2",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      handle: u32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let mut tables = data_mut.tables.borrow_mut();
                    let handle = tables.markdown2_table.remove(handle).map_err(|e| {
                        wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
                    })?;
                    let host = &mut data_mut.data;
                    host.drop_markdown2(handle);
                    Ok(())
                },
            ),
        );
        canonical_abi.insert(
            "resource_drop_odd-name",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      handle: u32|
                      -> Result<(), wasmer::RuntimeError> {
                    let data_mut = store.data_mut();
                    let mut tables = data_mut.tables.borrow_mut();
                    let handle = tables.odd_name_table.remove(handle).map_err(|e| {
                        wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
                    })?;
                    let host = &mut data_mut.data;
                    host.drop_odd_name(handle);
                    Ok(())
                },
            ),
        );
        imports.register_namespace("canonical_abi", canonical_abi);
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
