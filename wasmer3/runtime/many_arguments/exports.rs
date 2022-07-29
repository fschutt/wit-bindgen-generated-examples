#[allow(clippy::all)]
pub mod imports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Imports: Sized + Send + Sync + 'static {
        fn many_arguments(
            &mut self,
            a1: u64,
            a2: u64,
            a3: u64,
            a4: u64,
            a5: u64,
            a6: u64,
            a7: u64,
            a8: u64,
            a9: u64,
            a10: u64,
            a11: u64,
            a12: u64,
            a13: u64,
            a14: u64,
            a15: u64,
            a16: u64,
            a17: u64,
            a18: u64,
            a19: u64,
            a20: u64,
        ) -> ();
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
            "many-arguments",
            wasmer::Function::new_native(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory.data_unchecked_mut(&store)
                    });
                    let data_mut = store.data_mut();
                    let load0 = _bc.load::<i64>(arg0 + 0)?;
                    let load1 = _bc.load::<i64>(arg0 + 8)?;
                    let load2 = _bc.load::<i64>(arg0 + 16)?;
                    let load3 = _bc.load::<i64>(arg0 + 24)?;
                    let load4 = _bc.load::<i64>(arg0 + 32)?;
                    let load5 = _bc.load::<i64>(arg0 + 40)?;
                    let load6 = _bc.load::<i64>(arg0 + 48)?;
                    let load7 = _bc.load::<i64>(arg0 + 56)?;
                    let load8 = _bc.load::<i64>(arg0 + 64)?;
                    let load9 = _bc.load::<i64>(arg0 + 72)?;
                    let load10 = _bc.load::<i64>(arg0 + 80)?;
                    let load11 = _bc.load::<i64>(arg0 + 88)?;
                    let load12 = _bc.load::<i64>(arg0 + 96)?;
                    let load13 = _bc.load::<i64>(arg0 + 104)?;
                    let load14 = _bc.load::<i64>(arg0 + 112)?;
                    let load15 = _bc.load::<i64>(arg0 + 120)?;
                    let load16 = _bc.load::<i64>(arg0 + 128)?;
                    let load17 = _bc.load::<i64>(arg0 + 136)?;
                    let load18 = _bc.load::<i64>(arg0 + 144)?;
                    let load19 = _bc.load::<i64>(arg0 + 152)?;
                    let param0 = load0 as u64;
                    let param1 = load1 as u64;
                    let param2 = load2 as u64;
                    let param3 = load3 as u64;
                    let param4 = load4 as u64;
                    let param5 = load5 as u64;
                    let param6 = load6 as u64;
                    let param7 = load7 as u64;
                    let param8 = load8 as u64;
                    let param9 = load9 as u64;
                    let param10 = load10 as u64;
                    let param11 = load11 as u64;
                    let param12 = load12 as u64;
                    let param13 = load13 as u64;
                    let param14 = load14 as u64;
                    let param15 = load15 as u64;
                    let param16 = load16 as u64;
                    let param17 = load17 as u64;
                    let param18 = load18 as u64;
                    let param19 = load19 as u64;
                    let host = &mut data_mut.data;
                    let result = host.many_arguments(
                        param0, param1, param2, param3, param4, param5, param6, param7, param8,
                        param9, param10, param11, param12, param13, param14, param15, param16,
                        param17, param18, param19,
                    );
                    let () = result;
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
    use wit_bindgen_wasmer::rt::RawMem;
}
