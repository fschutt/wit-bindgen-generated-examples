pub mod imports {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    pub trait Imports: Sized + wasmer::WasmerEnv + 'static {
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

    pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
    where
        T: Imports,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: Imports> {
            data: T,
            memory: wasmer::LazyInit<wasmer::Memory>,
        }
        unsafe impl<T: Imports> Send for EnvWrapper<T> {}
        unsafe impl<T: Imports> Sync for EnvWrapper<T> {}
        impl<T: Imports> wasmer::WasmerEnv for EnvWrapper<T> {
            fn init_with_instance(
                &mut self,
                instance: &wasmer::Instance,
            ) -> Result<(), wasmer::HostEnvInitError> {
                self.data.init_with_instance(instance)?;
                self.memory
                    .initialize(instance.exports.get_with_generics_weak("memory")?);
                Ok(())
            }
        }
        let env = std::sync::Arc::new(std::sync::Mutex::new(EnvWrapper {
            data,
            memory: wasmer::LazyInit::new(),
        }));
        let mut exports = wasmer::Exports::new();
        exports.insert(
            "many-arguments",
            wasmer::Function::new_native_with_env(
                store,
                env.clone(),
                move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let env = &mut *env.lock().unwrap();
                    let mut _bc = unsafe {
                        wit_bindgen_wasmer::BorrowChecker::new(
                            env.memory.get_ref().unwrap().data_unchecked_mut(),
                        )
                    };
                    let host = &mut env.data;
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
        imports.register("imports", exports);
    }
    use wit_bindgen_wasmer::rt::RawMem;
}
