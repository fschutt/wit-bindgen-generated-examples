#[allow(clippy::all)]
pub mod small_anonymous {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Error {
        Success,
        Failure,
    }
    impl core::fmt::Debug for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Error::Success => f.debug_tuple("Error::Success").finish(),
                Error::Failure => f.debug_tuple("Error::Failure").finish(),
            }
        }
    }
    pub trait SmallAnonymous: Sized + Send + Sync + 'static {
        fn option_test(&mut self) -> Result<Option<String>, Error>;
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
        T: SmallAnonymous,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: SmallAnonymous> {
            data: T,
            lazy: std::rc::Rc<OnceCell<LazyInitialized>>,
        }
        unsafe impl<T: SmallAnonymous> Send for EnvWrapper<T> {}
        unsafe impl<T: SmallAnonymous> Sync for EnvWrapper<T> {}
        let lazy = std::rc::Rc::new(OnceCell::new());
        let env = EnvWrapper {
            data,
            lazy: std::rc::Rc::clone(&lazy),
        };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
            "option-test",
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
                    let host = &mut data_mut.data;
                    let result = host.option_test();
                    match result {
                        Ok(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
                            caller_memory
                                .store(arg0 + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            match e {
                                Some(e) => {
                                    let caller_memory = unsafe {
                                        _memory.data_unchecked_mut(&store.as_store_ref())
                                    };
                                    caller_memory.store(
                                        arg0 + 4,
                                        wit_bindgen_wasmer::rt::as_i32(1i32) as u8,
                                    )?;
                                    let vec0 = e;
                                    let ptr0 = func_canonical_abi_realloc.call(
                                        &mut store.as_store_mut(),
                                        0,
                                        0,
                                        1,
                                        vec0.len() as i32,
                                    )?;
                                    let caller_memory = unsafe {
                                        _memory.data_unchecked_mut(&store.as_store_ref())
                                    };
                                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                                    caller_memory.store(
                                        arg0 + 12,
                                        wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg0 + 8, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                                }
                                None => {
                                    let e = ();
                                    {
                                        caller_memory.store(
                                            arg0 + 4,
                                            wit_bindgen_wasmer::rt::as_i32(0i32) as u8,
                                        )?;
                                        let () = e;
                                    }
                                }
                            };
                        }
                        Err(e) => {
                            let caller_memory =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) };
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
        imports.register_namespace("small-anonymous", exports);
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
    use wit_bindgen_wasmer::rt::RawMem;
}
