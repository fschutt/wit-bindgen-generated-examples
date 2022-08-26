#[allow(clippy::all)]
pub mod many_arguments {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[derive(Clone)]
    pub struct BigStruct<'a> {
        pub a1: &'a str,
        pub a2: &'a str,
        pub a3: &'a str,
        pub a4: &'a str,
        pub a5: &'a str,
        pub a6: &'a str,
        pub a7: &'a str,
        pub a8: &'a str,
        pub a9: &'a str,
        pub a10: &'a str,
        pub a11: &'a str,
        pub a12: &'a str,
        pub a13: &'a str,
        pub a14: &'a str,
        pub a15: &'a str,
        pub a16: &'a str,
        pub a17: &'a str,
        pub a18: &'a str,
        pub a19: &'a str,
        pub a20: &'a str,
    }
    impl<'a> core::fmt::Debug for BigStruct<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("BigStruct")
                .field("a1", &self.a1)
                .field("a2", &self.a2)
                .field("a3", &self.a3)
                .field("a4", &self.a4)
                .field("a5", &self.a5)
                .field("a6", &self.a6)
                .field("a7", &self.a7)
                .field("a8", &self.a8)
                .field("a9", &self.a9)
                .field("a10", &self.a10)
                .field("a11", &self.a11)
                .field("a12", &self.a12)
                .field("a13", &self.a13)
                .field("a14", &self.a14)
                .field("a15", &self.a15)
                .field("a16", &self.a16)
                .field("a17", &self.a17)
                .field("a18", &self.a18)
                .field("a19", &self.a19)
                .field("a20", &self.a20)
                .finish()
        }
    }
    pub trait ManyArguments: Sized + Send + Sync + 'static {
        fn many_args(
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

        fn big_argument(&mut self, x: BigStruct<'_>) -> ();
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
        T: ManyArguments,
    {
        #[derive(Clone)]
        struct EnvWrapper<T: ManyArguments> {
            data: T,
            lazy: std::rc::Rc<OnceCell<LazyInitialized>>,
        }
        unsafe impl<T: ManyArguments> Send for EnvWrapper<T> {}
        unsafe impl<T: ManyArguments> Sync for EnvWrapper<T> {}
        let lazy = std::rc::Rc::new(OnceCell::new());
        let env = EnvWrapper {
            data,
            lazy: std::rc::Rc::clone(&lazy),
        };
        let env = wasmer::FunctionEnv::new(&mut *store, env);
        let mut exports = wasmer::Exports::new();
        let mut store = store.as_store_mut();
        exports.insert(
            "many-args",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
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
                    let result = host.many_args(
                        param0, param1, param2, param3, param4, param5, param6, param7, param8,
                        param9, param10, param11, param12, param13, param14, param15, param16,
                        param17, param18, param19,
                    );
                    let () = result;
                    Ok(())
                },
            ),
        );
        exports.insert(
            "big-argument",
            wasmer::Function::new_typed_with_env(
                &mut store,
                &env,
                move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,
                      arg0: i32|
                      -> Result<(), wasmer::RuntimeError> {
                    let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
                    let _memory_view = _memory.view(&store);
                    let mut _bc = wit_bindgen_wasmer::BorrowChecker::new(unsafe {
                        _memory_view.data_unchecked_mut()
                    });
                    let data_mut = store.data_mut();
                    let load0 = _bc.load::<i32>(arg0 + 0)?;
                    let load1 = _bc.load::<i32>(arg0 + 4)?;
                    let ptr2 = load0;
                    let len2 = load1;
                    let load3 = _bc.load::<i32>(arg0 + 8)?;
                    let load4 = _bc.load::<i32>(arg0 + 12)?;
                    let ptr5 = load3;
                    let len5 = load4;
                    let load6 = _bc.load::<i32>(arg0 + 16)?;
                    let load7 = _bc.load::<i32>(arg0 + 20)?;
                    let ptr8 = load6;
                    let len8 = load7;
                    let load9 = _bc.load::<i32>(arg0 + 24)?;
                    let load10 = _bc.load::<i32>(arg0 + 28)?;
                    let ptr11 = load9;
                    let len11 = load10;
                    let load12 = _bc.load::<i32>(arg0 + 32)?;
                    let load13 = _bc.load::<i32>(arg0 + 36)?;
                    let ptr14 = load12;
                    let len14 = load13;
                    let load15 = _bc.load::<i32>(arg0 + 40)?;
                    let load16 = _bc.load::<i32>(arg0 + 44)?;
                    let ptr17 = load15;
                    let len17 = load16;
                    let load18 = _bc.load::<i32>(arg0 + 48)?;
                    let load19 = _bc.load::<i32>(arg0 + 52)?;
                    let ptr20 = load18;
                    let len20 = load19;
                    let load21 = _bc.load::<i32>(arg0 + 56)?;
                    let load22 = _bc.load::<i32>(arg0 + 60)?;
                    let ptr23 = load21;
                    let len23 = load22;
                    let load24 = _bc.load::<i32>(arg0 + 64)?;
                    let load25 = _bc.load::<i32>(arg0 + 68)?;
                    let ptr26 = load24;
                    let len26 = load25;
                    let load27 = _bc.load::<i32>(arg0 + 72)?;
                    let load28 = _bc.load::<i32>(arg0 + 76)?;
                    let ptr29 = load27;
                    let len29 = load28;
                    let load30 = _bc.load::<i32>(arg0 + 80)?;
                    let load31 = _bc.load::<i32>(arg0 + 84)?;
                    let ptr32 = load30;
                    let len32 = load31;
                    let load33 = _bc.load::<i32>(arg0 + 88)?;
                    let load34 = _bc.load::<i32>(arg0 + 92)?;
                    let ptr35 = load33;
                    let len35 = load34;
                    let load36 = _bc.load::<i32>(arg0 + 96)?;
                    let load37 = _bc.load::<i32>(arg0 + 100)?;
                    let ptr38 = load36;
                    let len38 = load37;
                    let load39 = _bc.load::<i32>(arg0 + 104)?;
                    let load40 = _bc.load::<i32>(arg0 + 108)?;
                    let ptr41 = load39;
                    let len41 = load40;
                    let load42 = _bc.load::<i32>(arg0 + 112)?;
                    let load43 = _bc.load::<i32>(arg0 + 116)?;
                    let ptr44 = load42;
                    let len44 = load43;
                    let load45 = _bc.load::<i32>(arg0 + 120)?;
                    let load46 = _bc.load::<i32>(arg0 + 124)?;
                    let ptr47 = load45;
                    let len47 = load46;
                    let load48 = _bc.load::<i32>(arg0 + 128)?;
                    let load49 = _bc.load::<i32>(arg0 + 132)?;
                    let ptr50 = load48;
                    let len50 = load49;
                    let load51 = _bc.load::<i32>(arg0 + 136)?;
                    let load52 = _bc.load::<i32>(arg0 + 140)?;
                    let ptr53 = load51;
                    let len53 = load52;
                    let load54 = _bc.load::<i32>(arg0 + 144)?;
                    let load55 = _bc.load::<i32>(arg0 + 148)?;
                    let ptr56 = load54;
                    let len56 = load55;
                    let load57 = _bc.load::<i32>(arg0 + 152)?;
                    let load58 = _bc.load::<i32>(arg0 + 156)?;
                    let ptr59 = load57;
                    let len59 = load58;
                    let param0 = BigStruct {
                        a1: _bc.slice_str(ptr2, len2)?,
                        a2: _bc.slice_str(ptr5, len5)?,
                        a3: _bc.slice_str(ptr8, len8)?,
                        a4: _bc.slice_str(ptr11, len11)?,
                        a5: _bc.slice_str(ptr14, len14)?,
                        a6: _bc.slice_str(ptr17, len17)?,
                        a7: _bc.slice_str(ptr20, len20)?,
                        a8: _bc.slice_str(ptr23, len23)?,
                        a9: _bc.slice_str(ptr26, len26)?,
                        a10: _bc.slice_str(ptr29, len29)?,
                        a11: _bc.slice_str(ptr32, len32)?,
                        a12: _bc.slice_str(ptr35, len35)?,
                        a13: _bc.slice_str(ptr38, len38)?,
                        a14: _bc.slice_str(ptr41, len41)?,
                        a15: _bc.slice_str(ptr44, len44)?,
                        a16: _bc.slice_str(ptr47, len47)?,
                        a17: _bc.slice_str(ptr50, len50)?,
                        a18: _bc.slice_str(ptr53, len53)?,
                        a19: _bc.slice_str(ptr56, len56)?,
                        a20: _bc.slice_str(ptr59, len59)?,
                    };
                    let host = &mut data_mut.data;
                    let result = host.big_argument(param0);
                    let () = result;
                    Ok(())
                },
            ),
        );
        imports.register_namespace("many-arguments", exports);
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
