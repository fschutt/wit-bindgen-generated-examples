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

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ManyArgumentsData {}

    pub struct ManyArguments {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<ManyArgumentsData>,
        func_big_argument: wasmer::TypedFunction<i32, ()>,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        func_many_args: wasmer::TypedFunction<i32, ()>,
        memory: wasmer::Memory,
    }
    impl ManyArguments {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ManyArgumentsData` which needs to be
        /// passed through to `ManyArguments::new`.
        fn add_to_imports(
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<ManyArgumentsData> {
            let env = wasmer::FunctionEnv::new(&mut store, ManyArgumentsData::default());
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
            env: wasmer::FunctionEnv<ManyArgumentsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_big_argument = _instance
                .exports
                .get_typed_function(&store, "big-argument")?;
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_realloc")?;
            let func_many_args = _instance.exports.get_typed_function(&store, "many-args")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(ManyArguments {
                func_big_argument,
                func_canonical_abi_realloc,
                func_many_args,
                memory,
                env,
            })
        }
        pub fn many_args(
            &self,
            store: &mut wasmer::Store,
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
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let ptr0 = func_canonical_abi_realloc.call(store, 0, 0, 8, 160)?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 0,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a1)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 8,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a2)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 16,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a3)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 24,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a4)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 32,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a5)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 40,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a6)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 48,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a7)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 56,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a8)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 64,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a9)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 72,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a10)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 80,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a11)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 88,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a12)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 96,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a13)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 104,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a14)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 112,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a15)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 120,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a16)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 128,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a17)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 136,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a18)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 144,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a19)),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 152,
                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a20)),
            )?;
            self.func_many_args.call(store, ptr0)?;
            Ok(())
        }
        pub fn big_argument(
            &self,
            store: &mut wasmer::Store,
            x: BigStruct<'_>,
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let ptr0 = func_canonical_abi_realloc.call(store, 0, 0, 4, 160)?;
            let BigStruct {
                a1: a11,
                a2: a21,
                a3: a31,
                a4: a41,
                a5: a51,
                a6: a61,
                a7: a71,
                a8: a81,
                a9: a91,
                a10: a101,
                a11: a111,
                a12: a121,
                a13: a131,
                a14: a141,
                a15: a151,
                a16: a161,
                a17: a171,
                a18: a181,
                a19: a191,
                a20: a201,
            } = x;
            let vec2 = a11;
            let ptr2 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec2.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr2, vec2.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 4, wit_bindgen_wasmer::rt::as_i32(vec2.len() as i32))?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 0, wit_bindgen_wasmer::rt::as_i32(ptr2))?;
            let vec3 = a21;
            let ptr3 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec3.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr3, vec3.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 12, wit_bindgen_wasmer::rt::as_i32(vec3.len() as i32))?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 8, wit_bindgen_wasmer::rt::as_i32(ptr3))?;
            let vec4 = a31;
            let ptr4 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec4.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr4, vec4.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 20, wit_bindgen_wasmer::rt::as_i32(vec4.len() as i32))?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 16, wit_bindgen_wasmer::rt::as_i32(ptr4))?;
            let vec5 = a41;
            let ptr5 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec5.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr5, vec5.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 28, wit_bindgen_wasmer::rt::as_i32(vec5.len() as i32))?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 24, wit_bindgen_wasmer::rt::as_i32(ptr5))?;
            let vec6 = a51;
            let ptr6 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec6.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr6, vec6.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 36, wit_bindgen_wasmer::rt::as_i32(vec6.len() as i32))?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 32, wit_bindgen_wasmer::rt::as_i32(ptr6))?;
            let vec7 = a61;
            let ptr7 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec7.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr7, vec7.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 44, wit_bindgen_wasmer::rt::as_i32(vec7.len() as i32))?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 40, wit_bindgen_wasmer::rt::as_i32(ptr7))?;
            let vec8 = a71;
            let ptr8 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec8.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr8, vec8.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 52, wit_bindgen_wasmer::rt::as_i32(vec8.len() as i32))?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 48, wit_bindgen_wasmer::rt::as_i32(ptr8))?;
            let vec9 = a81;
            let ptr9 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec9.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr9, vec9.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 60, wit_bindgen_wasmer::rt::as_i32(vec9.len() as i32))?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 56, wit_bindgen_wasmer::rt::as_i32(ptr9))?;
            let vec10 = a91;
            let ptr10 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec10.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr10, vec10.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 68,
                wit_bindgen_wasmer::rt::as_i32(vec10.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 64, wit_bindgen_wasmer::rt::as_i32(ptr10))?;
            let vec11 = a101;
            let ptr11 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec11.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr11, vec11.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 76,
                wit_bindgen_wasmer::rt::as_i32(vec11.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 72, wit_bindgen_wasmer::rt::as_i32(ptr11))?;
            let vec12 = a111;
            let ptr12 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec12.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr12, vec12.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 84,
                wit_bindgen_wasmer::rt::as_i32(vec12.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 80, wit_bindgen_wasmer::rt::as_i32(ptr12))?;
            let vec13 = a121;
            let ptr13 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec13.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr13, vec13.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 92,
                wit_bindgen_wasmer::rt::as_i32(vec13.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 88, wit_bindgen_wasmer::rt::as_i32(ptr13))?;
            let vec14 = a131;
            let ptr14 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec14.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr14, vec14.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 100,
                wit_bindgen_wasmer::rt::as_i32(vec14.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 96, wit_bindgen_wasmer::rt::as_i32(ptr14))?;
            let vec15 = a141;
            let ptr15 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec15.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr15, vec15.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 108,
                wit_bindgen_wasmer::rt::as_i32(vec15.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 104, wit_bindgen_wasmer::rt::as_i32(ptr15))?;
            let vec16 = a151;
            let ptr16 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec16.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr16, vec16.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 116,
                wit_bindgen_wasmer::rt::as_i32(vec16.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 112, wit_bindgen_wasmer::rt::as_i32(ptr16))?;
            let vec17 = a161;
            let ptr17 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec17.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr17, vec17.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 124,
                wit_bindgen_wasmer::rt::as_i32(vec17.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 120, wit_bindgen_wasmer::rt::as_i32(ptr17))?;
            let vec18 = a171;
            let ptr18 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec18.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr18, vec18.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 132,
                wit_bindgen_wasmer::rt::as_i32(vec18.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 128, wit_bindgen_wasmer::rt::as_i32(ptr18))?;
            let vec19 = a181;
            let ptr19 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec19.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr19, vec19.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 140,
                wit_bindgen_wasmer::rt::as_i32(vec19.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 136, wit_bindgen_wasmer::rt::as_i32(ptr19))?;
            let vec20 = a191;
            let ptr20 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec20.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr20, vec20.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 148,
                wit_bindgen_wasmer::rt::as_i32(vec20.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 144, wit_bindgen_wasmer::rt::as_i32(ptr20))?;
            let vec21 = a201;
            let ptr21 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec21.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr21, vec21.as_bytes())?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store(
                ptr0 + 156,
                wit_bindgen_wasmer::rt::as_i32(vec21.len() as i32),
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }
                .store(ptr0 + 152, wit_bindgen_wasmer::rt::as_i32(ptr21))?;
            self.func_big_argument.call(store, ptr0)?;
            Ok(())
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::RawMem;
}
