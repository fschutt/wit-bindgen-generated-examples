#[allow(clippy::all)]
pub mod exports {
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
    pub type ListTypedef3Param<'a> = &'a [&'a str];
    pub type ListTypedef3Result = Vec<String>;

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ExportsData {}

    pub struct Exports {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<ExportsData>,
        func_canonical_abi_free: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        func_errno_result: wasmer::TypedFunction<(), i32>,
        func_list_in_record1: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_in_record2: wasmer::TypedFunction<(), i32>,
        func_list_in_record3: wasmer::TypedFunction<(i32, i32), i32>,
        func_list_in_record4: wasmer::TypedFunction<(i32, i32), i32>,
        func_list_in_variant1:
            wasmer::TypedFunction<(i32, i32, i32, i32, i32, i32, i32, i32, i32), ()>,
        func_list_in_variant2: wasmer::TypedFunction<(), i32>,
        func_list_in_variant3: wasmer::TypedFunction<(i32, i32, i32), i32>,
        func_list_typedefs: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        func_test_imports: wasmer::TypedFunction<(), ()>,
        memory: wasmer::Memory,
    }
    impl Exports {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ExportsData` which needs to be
        /// passed through to `Exports::new`.
        fn add_to_imports(
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<ExportsData> {
            let env = wasmer::FunctionEnv::new(&mut store, ExportsData::default());
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
            env: wasmer::FunctionEnv<ExportsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_canonical_abi_free = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_free")?;
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_realloc")?;
            let func_errno_result = _instance
                .exports
                .get_typed_function(&store, "errno-result")?;
            let func_list_in_record1 = _instance
                .exports
                .get_typed_function(&store, "list-in-record1")?;
            let func_list_in_record2 = _instance
                .exports
                .get_typed_function(&store, "list-in-record2")?;
            let func_list_in_record3 = _instance
                .exports
                .get_typed_function(&store, "list-in-record3")?;
            let func_list_in_record4 = _instance
                .exports
                .get_typed_function(&store, "list-in-record4")?;
            let func_list_in_variant1 = _instance
                .exports
                .get_typed_function(&store, "list-in-variant1")?;
            let func_list_in_variant2 = _instance
                .exports
                .get_typed_function(&store, "list-in-variant2")?;
            let func_list_in_variant3 = _instance
                .exports
                .get_typed_function(&store, "list-in-variant3")?;
            let func_list_typedefs = _instance
                .exports
                .get_typed_function(&store, "list-typedefs")?;
            let func_test_imports = _instance
                .exports
                .get_typed_function(&store, "test-imports")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Exports {
                func_canonical_abi_free,
                func_canonical_abi_realloc,
                func_errno_result,
                func_list_in_record1,
                func_list_in_record2,
                func_list_in_record3,
                func_list_in_record4,
                func_list_in_variant1,
                func_list_in_variant2,
                func_list_in_variant3,
                func_list_typedefs,
                func_test_imports,
                memory,
                env,
            })
        }
        pub fn test_imports(&self, store: &mut wasmer::Store) -> Result<(), wasmer::RuntimeError> {
            self.func_test_imports.call(store)?;
            Ok(())
        }
        pub fn list_in_record1(
            &self,
            store: &mut wasmer::Store,
            a: ListInRecord1<'_>,
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let ListInRecord1 { a: a0 } = a;
            let vec1 = a0;
            let ptr1 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec1.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr1, vec1.as_bytes())?;
            self.func_list_in_record1
                .call(store, ptr1, vec1.len() as i32)?;
            Ok(())
        }
        pub fn list_in_record2(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<ListInRecord2, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_in_record2.call(store)?;
            let _memory_view = _memory.view(&store);
            let load1 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 0)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;

            let data3 = copy_slice(store, _memory, func_canonical_abi_free, ptr3, len3, 1)?;
            Ok(ListInRecord2 {
                a: String::from_utf8(data3)
                    .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?,
            })
        }
        pub fn list_in_record3(
            &self,
            store: &mut wasmer::Store,
            a: ListInRecord3Param<'_>,
        ) -> Result<ListInRecord3Result, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let ListInRecord3Param { a: a0 } = a;
            let vec1 = a0;
            let ptr1 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec1.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr1, vec1.as_bytes())?;
            let result2 = self
                .func_list_in_record3
                .call(store, ptr1, vec1.len() as i32)?;
            let _memory_view = _memory.view(&store);
            let load3 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result2 + 0)?;
            let _memory_view = _memory.view(&store);
            let load4 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result2 + 4)?;
            let ptr5 = load3;
            let len5 = load4;

            let data5 = copy_slice(store, _memory, func_canonical_abi_free, ptr5, len5, 1)?;
            Ok(ListInRecord3Result {
                a: String::from_utf8(data5)
                    .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?,
            })
        }
        pub fn list_in_record4(
            &self,
            store: &mut wasmer::Store,
            a: ListInAliasParam<'_>,
        ) -> Result<ListInAliasResult, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let ListInRecord4Param { a: a0 } = a;
            let vec1 = a0;
            let ptr1 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec1.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr1, vec1.as_bytes())?;
            let result2 = self
                .func_list_in_record4
                .call(store, ptr1, vec1.len() as i32)?;
            let _memory_view = _memory.view(&store);
            let load3 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result2 + 0)?;
            let _memory_view = _memory.view(&store);
            let load4 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result2 + 4)?;
            let ptr5 = load3;
            let len5 = load4;

            let data5 = copy_slice(store, _memory, func_canonical_abi_free, ptr5, len5, 1)?;
            Ok(ListInRecord4Result {
                a: String::from_utf8(data5)
                    .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?,
            })
        }
        pub fn list_in_variant1(
            &self,
            store: &mut wasmer::Store,
            a: ListInVariant1V1<'_>,
            b: ListInVariant1V2<'_>,
            c: ListInVariant1V3<'_>,
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let (result1_0, result1_1, result1_2) = match a {
                Some(e) => {
                    let vec0 = e;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec0.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store_many(ptr0, vec0.as_bytes())?;
                    (1i32, ptr0, vec0.len() as i32)
                }
                None => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0i32, 0i32)
                    }
                }
            };
            let (result3_0, result3_1, result3_2) = match b {
                Ok(e) => {
                    let () = e;
                    (0i32, 0i32, 0i32)
                }
                Err(e) => {
                    let vec2 = e;
                    let ptr2 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec2.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store_many(ptr2, vec2.as_bytes())?;
                    (1i32, ptr2, vec2.len() as i32)
                }
            };
            let (result5_0, result5_1, result5_2) = match c {
                ListInVariant1V3::String(e) => {
                    let vec4 = e;
                    let ptr4 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec4.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store_many(ptr4, vec4.as_bytes())?;
                    (0i32, ptr4, vec4.len() as i32)
                }
                ListInVariant1V3::F32(e) => (1i32, (e).to_bits() as i32, 0i32),
            };
            self.func_list_in_variant1.call(
                store, result1_0, result1_1, result1_2, result3_0, result3_1, result3_2, result5_0,
                result5_1, result5_2,
            )?;
            Ok(())
        }
        pub fn list_in_variant2(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<ListInVariant2, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_in_variant2.call(store)?;
            let _memory_view = _memory.view(&store);
            let load1 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => None,
                1 => Some({
                    let _memory_view = _memory.view(&store);
                    let load2 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 4)?;
                    let _memory_view = _memory.view(&store);
                    let load3 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 8)?;
                    let ptr4 = load2;
                    let len4 = load3;

                    let data4 = copy_slice(store, _memory, func_canonical_abi_free, ptr4, len4, 1)?;
                    String::from_utf8(data4)
                        .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                }),
                _ => return Err(invalid_variant("option")),
            })
        }
        pub fn list_in_variant3(
            &self,
            store: &mut wasmer::Store,
            a: ListInVariant3Param<'_>,
        ) -> Result<ListInVariant3Result, wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let (result1_0, result1_1, result1_2) = match a {
                Some(e) => {
                    let vec0 = e;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec0.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store_many(ptr0, vec0.as_bytes())?;
                    (1i32, ptr0, vec0.len() as i32)
                }
                None => {
                    let e = ();
                    {
                        let () = e;
                        (0i32, 0i32, 0i32)
                    }
                }
            };
            let result2 = self
                .func_list_in_variant3
                .call(store, result1_0, result1_1, result1_2)?;
            let _memory_view = _memory.view(&store);
            let load3 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result2 + 0)?;
            Ok(match i32::from(load3) {
                0 => None,
                1 => Some({
                    let _memory_view = _memory.view(&store);
                    let load4 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result2 + 4)?;
                    let _memory_view = _memory.view(&store);
                    let load5 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result2 + 8)?;
                    let ptr6 = load4;
                    let len6 = load5;

                    let data6 = copy_slice(store, _memory, func_canonical_abi_free, ptr6, len6, 1)?;
                    String::from_utf8(data6)
                        .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                }),
                _ => return Err(invalid_variant("option")),
            })
        }
        pub fn errno_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Result<(), MyErrno>, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_errno_result.call(store)?;
            let _memory_view = _memory.view(&store);
            let load1 = unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result0 + 0)?;
            Ok(match i32::from(load1) {
                0 => Ok(()),
                1 => Err({
                    let _memory_view = _memory.view(&store);
                    let load2 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(result0 + 1)?;
                    match i32::from(load2) {
                        0 => MyErrno::Success,
                        1 => MyErrno::A,
                        2 => MyErrno::B,
                        _ => return Err(invalid_variant("MyErrno")),
                    }
                }),
                _ => return Err(invalid_variant("expected")),
            })
        }
        pub fn list_typedefs(
            &self,
            store: &mut wasmer::Store,
            a: ListTypedef<'_>,
            c: ListTypedef3Param<'_>,
        ) -> Result<(ListTypedef2, ListTypedef3Result), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let vec0 = a;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec0.len() as i32,
            )?;
            let _memory_view = _memory.view(&store);
            unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr0, vec0.as_bytes())?;
            let vec2 = c;
            let len2 = vec2.len() as i32;
            let result2 =
                func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 4, len2 * 8)?;
            for (i, e) in vec2.into_iter().enumerate() {
                let base = result2 + (i as i32) * 8;
                {
                    let vec1 = e;
                    let ptr1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec1.len() as i32,
                    )?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store_many(ptr1, vec1.as_bytes())?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store(base + 4, wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32))?;
                    let _memory_view = _memory.view(&store);
                    unsafe { _memory_view.data_unchecked_mut() }
                        .store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr1))?;
                }
            }
            let result3 =
                self.func_list_typedefs
                    .call(store, ptr0, vec0.len() as i32, result2, len2)?;
            let _memory_view = _memory.view(&store);
            let load4 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result3 + 0)?;
            let _memory_view = _memory.view(&store);
            let load5 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result3 + 4)?;
            let ptr6 = load4;
            let len6 = load5;
            let _memory_view = _memory.view(&store);
            let load7 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result3 + 8)?;
            let _memory_view = _memory.view(&store);
            let load8 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result3 + 12)?;
            let len12 = load8;
            let base12 = load7;
            let mut result12 = Vec::with_capacity(len12 as usize);
            for i in 0..len12 {
                let base = base12 + i * 8;
                result12.push({
                    let _memory_view = _memory.view(&store);
                    let load9 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(base + 0)?;
                    let _memory_view = _memory.view(&store);
                    let load10 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(base + 4)?;
                    let ptr11 = load9;
                    let len11 = load10;

                    let data11 =
                        copy_slice(store, _memory, func_canonical_abi_free, ptr11, len11, 1)?;
                    String::from_utf8(data11)
                        .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base12, len12 * 8, 4)?;
            Ok((
                copy_slice(store, _memory, func_canonical_abi_free, ptr6, len6, 1)?,
                result12,
            ))
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
