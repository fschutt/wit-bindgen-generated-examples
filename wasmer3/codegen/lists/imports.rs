#[allow(clippy::all)]
pub mod lists {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[derive(Clone)]
    pub struct SomeRecordParam<'a> {
        pub x: &'a str,
        pub y: OtherRecordParam<'a>,
        pub z: &'a [OtherRecordParam<'a>],
        pub c1: u32,
        pub c2: u64,
        pub c3: i32,
        pub c4: i64,
    }
    impl<'a> core::fmt::Debug for SomeRecordParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("SomeRecordParam")
                .field("x", &self.x)
                .field("y", &self.y)
                .field("z", &self.z)
                .field("c1", &self.c1)
                .field("c2", &self.c2)
                .field("c3", &self.c3)
                .field("c4", &self.c4)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct SomeRecordResult {
        pub x: String,
        pub y: OtherRecordResult,
        pub z: Vec<OtherRecordResult>,
        pub c1: u32,
        pub c2: u64,
        pub c3: i32,
        pub c4: i64,
    }
    impl core::fmt::Debug for SomeRecordResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("SomeRecordResult")
                .field("x", &self.x)
                .field("y", &self.y)
                .field("z", &self.z)
                .field("c1", &self.c1)
                .field("c2", &self.c2)
                .field("c3", &self.c3)
                .field("c4", &self.c4)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct OtherRecordParam<'a> {
        pub a1: u32,
        pub a2: u64,
        pub a3: i32,
        pub a4: i64,
        pub b: &'a str,
        pub c: &'a [u8],
    }
    impl<'a> core::fmt::Debug for OtherRecordParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("OtherRecordParam")
                .field("a1", &self.a1)
                .field("a2", &self.a2)
                .field("a3", &self.a3)
                .field("a4", &self.a4)
                .field("b", &self.b)
                .field("c", &self.c)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct OtherRecordResult {
        pub a1: u32,
        pub a2: u64,
        pub a3: i32,
        pub a4: i64,
        pub b: String,
        pub c: Vec<u8>,
    }
    impl core::fmt::Debug for OtherRecordResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("OtherRecordResult")
                .field("a1", &self.a1)
                .field("a2", &self.a2)
                .field("a3", &self.a3)
                .field("a4", &self.a4)
                .field("b", &self.b)
                .field("c", &self.c)
                .finish()
        }
    }
    #[derive(Clone)]
    pub enum SomeVariant<'a> {
        A(&'a str),
        B,
        C(u32),
        D(&'a [OtherVariantParam<'a>]),
    }
    impl<'a> core::fmt::Debug for SomeVariant<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                SomeVariant::A(e) => f.debug_tuple("SomeVariant::A").field(e).finish(),
                SomeVariant::B => f.debug_tuple("SomeVariant::B").finish(),
                SomeVariant::C(e) => f.debug_tuple("SomeVariant::C").field(e).finish(),
                SomeVariant::D(e) => f.debug_tuple("SomeVariant::D").field(e).finish(),
            }
        }
    }
    #[derive(Clone)]
    pub enum OtherVariantParam<'a> {
        A,
        B(u32),
        C(&'a str),
    }
    impl<'a> core::fmt::Debug for OtherVariantParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                OtherVariantParam::A => f.debug_tuple("OtherVariantParam::A").finish(),
                OtherVariantParam::B(e) => f.debug_tuple("OtherVariantParam::B").field(e).finish(),
                OtherVariantParam::C(e) => f.debug_tuple("OtherVariantParam::C").field(e).finish(),
            }
        }
    }
    #[derive(Clone)]
    pub enum OtherVariantResult {
        A,
        B(u32),
        C(String),
    }
    impl core::fmt::Debug for OtherVariantResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                OtherVariantResult::A => f.debug_tuple("OtherVariantResult::A").finish(),
                OtherVariantResult::B(e) => {
                    f.debug_tuple("OtherVariantResult::B").field(e).finish()
                }
                OtherVariantResult::C(e) => {
                    f.debug_tuple("OtherVariantResult::C").field(e).finish()
                }
            }
        }
    }
    pub type LoadStoreAllSizesParam<'a> = &'a [(
        &'a str,
        u8,
        i8,
        u16,
        i16,
        u32,
        i32,
        u64,
        i64,
        f32,
        f64,
        char,
    )];
    pub type LoadStoreAllSizesResult =
        Vec<(String, u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char)>;

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct ListsData {}

    pub struct Lists {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<ListsData>,
        func_canonical_abi_free: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        func_list_float32_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_float32_ret: wasmer::TypedFunction<(), i32>,
        func_list_float64_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_float64_ret: wasmer::TypedFunction<(), i32>,
        func_list_s16_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_s16_ret: wasmer::TypedFunction<(), i32>,
        func_list_s32_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_s32_ret: wasmer::TypedFunction<(), i32>,
        func_list_s64_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_s64_ret: wasmer::TypedFunction<(), i32>,
        func_list_s8_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_s8_ret: wasmer::TypedFunction<(), i32>,
        func_list_u16_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_u16_ret: wasmer::TypedFunction<(), i32>,
        func_list_u32_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_u32_ret: wasmer::TypedFunction<(), i32>,
        func_list_u64_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_u64_ret: wasmer::TypedFunction<(), i32>,
        func_list_u8_param: wasmer::TypedFunction<(i32, i32), ()>,
        func_list_u8_ret: wasmer::TypedFunction<(), i32>,
        func_load_store_everything: wasmer::TypedFunction<(i32, i32), i32>,
        func_record_list: wasmer::TypedFunction<(i32, i32), i32>,
        func_record_list_reverse: wasmer::TypedFunction<(i32, i32), i32>,
        func_string_list: wasmer::TypedFunction<(i32, i32), i32>,
        func_string_list_arg: wasmer::TypedFunction<(i32, i32), ()>,
        func_string_list_ret: wasmer::TypedFunction<(), i32>,
        func_tuple_list: wasmer::TypedFunction<(i32, i32), i32>,
        func_tuple_string_list: wasmer::TypedFunction<(i32, i32), i32>,
        func_variant_list: wasmer::TypedFunction<(i32, i32), i32>,
        memory: wasmer::Memory,
    }
    impl Lists {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ListsData` which needs to be
        /// passed through to `Lists::new`.
        fn add_to_imports(
            store: &mut wasmer::StoreMut<'_>,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<ListsData> {
            let env = wasmer::FunctionEnv::new(store, Default::default());
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
            store: &mut wasmer::StoreMut<'_>,
            module: &wasmer::Module,
            imports: &mut wasmer::Imports,
        ) -> anyhow::Result<(Self, wasmer::Instance)> {
            let env = Self::add_to_imports(&mut store.as_store_mut().as_store_mut(), imports);
            let instance = wasmer::Instance::new(&mut store.as_store_mut(), module, &*imports)?;

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
            store: &mut wasmer::StoreMut<'_>,
            _instance: &wasmer::Instance,
            env: wasmer::FunctionEnv<ListsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_canonical_abi_free = _instance
                .exports
                .get_typed_function(store, "canonical_abi_free")?;
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(store, "canonical_abi_realloc")?;
            let func_list_float32_param = _instance
                .exports
                .get_typed_function(store, "list-float32-param")?;
            let func_list_float32_ret = _instance
                .exports
                .get_typed_function(store, "list-float32-ret")?;
            let func_list_float64_param = _instance
                .exports
                .get_typed_function(store, "list-float64-param")?;
            let func_list_float64_ret = _instance
                .exports
                .get_typed_function(store, "list-float64-ret")?;
            let func_list_s16_param = _instance
                .exports
                .get_typed_function(store, "list-s16-param")?;
            let func_list_s16_ret = _instance
                .exports
                .get_typed_function(store, "list-s16-ret")?;
            let func_list_s32_param = _instance
                .exports
                .get_typed_function(store, "list-s32-param")?;
            let func_list_s32_ret = _instance
                .exports
                .get_typed_function(store, "list-s32-ret")?;
            let func_list_s64_param = _instance
                .exports
                .get_typed_function(store, "list-s64-param")?;
            let func_list_s64_ret = _instance
                .exports
                .get_typed_function(store, "list-s64-ret")?;
            let func_list_s8_param = _instance
                .exports
                .get_typed_function(store, "list-s8-param")?;
            let func_list_s8_ret = _instance.exports.get_typed_function(store, "list-s8-ret")?;
            let func_list_u16_param = _instance
                .exports
                .get_typed_function(store, "list-u16-param")?;
            let func_list_u16_ret = _instance
                .exports
                .get_typed_function(store, "list-u16-ret")?;
            let func_list_u32_param = _instance
                .exports
                .get_typed_function(store, "list-u32-param")?;
            let func_list_u32_ret = _instance
                .exports
                .get_typed_function(store, "list-u32-ret")?;
            let func_list_u64_param = _instance
                .exports
                .get_typed_function(store, "list-u64-param")?;
            let func_list_u64_ret = _instance
                .exports
                .get_typed_function(store, "list-u64-ret")?;
            let func_list_u8_param = _instance
                .exports
                .get_typed_function(store, "list-u8-param")?;
            let func_list_u8_ret = _instance.exports.get_typed_function(store, "list-u8-ret")?;
            let func_load_store_everything = _instance
                .exports
                .get_typed_function(store, "load-store-everything")?;
            let func_record_list = _instance.exports.get_typed_function(store, "record-list")?;
            let func_record_list_reverse = _instance
                .exports
                .get_typed_function(store, "record-list-reverse")?;
            let func_string_list = _instance.exports.get_typed_function(store, "string-list")?;
            let func_string_list_arg = _instance
                .exports
                .get_typed_function(store, "string-list-arg")?;
            let func_string_list_ret = _instance
                .exports
                .get_typed_function(store, "string-list-ret")?;
            let func_tuple_list = _instance.exports.get_typed_function(store, "tuple-list")?;
            let func_tuple_string_list = _instance
                .exports
                .get_typed_function(store, "tuple-string-list")?;
            let func_variant_list = _instance
                .exports
                .get_typed_function(store, "variant-list")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Lists {
                func_canonical_abi_free,
                func_canonical_abi_realloc,
                func_list_float32_param,
                func_list_float32_ret,
                func_list_float64_param,
                func_list_float64_ret,
                func_list_s16_param,
                func_list_s16_ret,
                func_list_s32_param,
                func_list_s32_ret,
                func_list_s64_param,
                func_list_s64_ret,
                func_list_s8_param,
                func_list_s8_ret,
                func_list_u16_param,
                func_list_u16_ret,
                func_list_u32_param,
                func_list_u32_ret,
                func_list_u64_param,
                func_list_u64_ret,
                func_list_u8_param,
                func_list_u8_ret,
                func_load_store_everything,
                func_record_list,
                func_record_list_reverse,
                func_string_list,
                func_string_list_arg,
                func_string_list_ret,
                func_tuple_list,
                func_tuple_string_list,
                func_variant_list,
                memory,
                env,
            })
        }
        pub fn list_u8_param(
            &self,
            store: &mut wasmer::Store,
            x: &[u8],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                (vec0.len() as i32) * 1,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store_many(ptr0, &vec0)?;
            self.func_list_u8_param
                .call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_u16_param(
            &self,
            store: &mut wasmer::Store,
            x: &[u16],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                2,
                (vec0.len() as i32) * 2,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store_many(ptr0, &vec0)?;
            self.func_list_u16_param
                .call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_u32_param(
            &self,
            store: &mut wasmer::Store,
            x: &[u32],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                4,
                (vec0.len() as i32) * 4,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store_many(ptr0, &vec0)?;
            self.func_list_u32_param
                .call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_u64_param(
            &self,
            store: &mut wasmer::Store,
            x: &[u64],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                8,
                (vec0.len() as i32) * 8,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store_many(ptr0, &vec0)?;
            self.func_list_u64_param
                .call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_s8_param(
            &self,
            store: &mut wasmer::Store,
            x: &[i8],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                (vec0.len() as i32) * 1,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store_many(ptr0, &vec0)?;
            self.func_list_s8_param
                .call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_s16_param(
            &self,
            store: &mut wasmer::Store,
            x: &[i16],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                2,
                (vec0.len() as i32) * 2,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store_many(ptr0, &vec0)?;
            self.func_list_s16_param
                .call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_s32_param(
            &self,
            store: &mut wasmer::Store,
            x: &[i32],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                4,
                (vec0.len() as i32) * 4,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store_many(ptr0, &vec0)?;
            self.func_list_s32_param
                .call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_s64_param(
            &self,
            store: &mut wasmer::Store,
            x: &[i64],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                8,
                (vec0.len() as i32) * 8,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store_many(ptr0, &vec0)?;
            self.func_list_s64_param
                .call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_float32_param(
            &self,
            store: &mut wasmer::Store,
            x: &[f32],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                4,
                (vec0.len() as i32) * 4,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store_many(ptr0, &vec0)?;
            self.func_list_float32_param
                .call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_float64_param(
            &self,
            store: &mut wasmer::Store,
            x: &[f64],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                8,
                (vec0.len() as i32) * 8,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store_many(ptr0, &vec0)?;
            self.func_list_float64_param
                .call(store, ptr0, vec0.len() as i32)?;
            Ok(())
        }
        pub fn list_u8_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<u8>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_u8_ret.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr3,
                len3,
                1,
            )?)
        }
        pub fn list_u16_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<u16>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_u16_ret.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr3,
                len3,
                2,
            )?)
        }
        pub fn list_u32_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<u32>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_u32_ret.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr3,
                len3,
                4,
            )?)
        }
        pub fn list_u64_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<u64>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_u64_ret.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr3,
                len3,
                8,
            )?)
        }
        pub fn list_s8_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<i8>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_s8_ret.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr3,
                len3,
                1,
            )?)
        }
        pub fn list_s16_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<i16>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_s16_ret.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr3,
                len3,
                2,
            )?)
        }
        pub fn list_s32_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<i32>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_s32_ret.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr3,
                len3,
                4,
            )?)
        }
        pub fn list_s64_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<i64>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_s64_ret.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr3,
                len3,
                8,
            )?)
        }
        pub fn list_float32_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<f32>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_float32_ret.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr3,
                len3,
                4,
            )?)
        }
        pub fn list_float64_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<f64>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_list_float64_ret.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr3,
                len3,
                8,
            )?)
        }
        pub fn tuple_list(
            &self,
            store: &mut wasmer::Store,
            x: &[(u8, i8)],
        ) -> Result<Vec<(i64, u32)>, wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let vec0 = x;
            let ptr0 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                (vec0.len() as i32) * 2,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store_many(ptr0, &vec0)?;
            let result1 = self.func_tuple_list.call(store, ptr0, vec0.len() as i32)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result1 + 0)?;
            let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result1 + 4)?;
            let ptr4 = load2;
            let len4 = load3;
            Ok(copy_slice(
                store,
                _memory,
                func_canonical_abi_free,
                ptr4,
                len4,
                8,
            )?)
        }
        pub fn string_list_arg(
            &self,
            store: &mut wasmer::Store,
            a: &[&str],
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec1 = a;
            let len1 = vec1.len() as i32;
            let result1 =
                func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 4, len1 * 8)?;
            for (i, e) in vec1.into_iter().enumerate() {
                let base = result1 + (i as i32) * 8;
                {
                    let vec0 = e;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec0.len() as i32,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr0, vec0.as_bytes())?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 4, wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                }
            }
            self.func_string_list_arg.call(store, result1, len1)?;
            Ok(())
        }
        pub fn string_list_ret(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Vec<String>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_string_list_ret.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let len6 = load2;
            let base6 = load1;
            let mut result6 = Vec::with_capacity(len6 as usize);
            for i in 0..len6 {
                let base = base6 + i * 8;
                result6.push({
                    let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 0)?;
                    let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 4)?;
                    let ptr5 = load3;
                    let len5 = load4;

                    let data5 = copy_slice(store, _memory, func_canonical_abi_free, ptr5, len5, 1)?;
                    String::from_utf8(data5)
                        .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base6, len6 * 8, 4)?;
            Ok(result6)
        }
        pub fn tuple_string_list(
            &self,
            store: &mut wasmer::Store,
            x: &[(u8, &str)],
        ) -> Result<Vec<(String, u8)>, wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let vec2 = x;
            let len2 = vec2.len() as i32;
            let result2 =
                func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 4, len2 * 12)?;
            for (i, e) in vec2.into_iter().enumerate() {
                let base = result2 + (i as i32) * 12;
                {
                    let (t0_0, t0_1) = e;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 0,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_0)) as u8,
                    )?;
                    let vec1 = t0_1;
                    let ptr1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec1.len() as i32,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr1, vec1.as_bytes())?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 8, wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 4, wit_bindgen_wasmer::rt::as_i32(ptr1))?;
                }
            }
            let result3 = self.func_tuple_string_list.call(store, result2, len2)?;
            let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result3 + 0)?;
            let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result3 + 4)?;
            let len10 = load5;
            let base10 = load4;
            let mut result10 = Vec::with_capacity(len10 as usize);
            for i in 0..len10 {
                let base = base10 + i * 12;
                result10.push({
                    let load6 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 0)?;
                    let load7 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 4)?;
                    let ptr8 = load6;
                    let len8 = load7;

                    let data8 = copy_slice(store, _memory, func_canonical_abi_free, ptr8, len8, 1)?;
                    let load9 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(base + 8)?;
                    (
                        String::from_utf8(data8)
                            .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?,
                        u8::try_from(i32::from(load9)).map_err(bad_int)?,
                    )
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base10, len10 * 12, 4)?;
            Ok(result10)
        }
        pub fn string_list(
            &self,
            store: &mut wasmer::Store,
            x: &[&str],
        ) -> Result<Vec<String>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec1 = x;
            let len1 = vec1.len() as i32;
            let result1 =
                func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 4, len1 * 8)?;
            for (i, e) in vec1.into_iter().enumerate() {
                let base = result1 + (i as i32) * 8;
                {
                    let vec0 = e;
                    let ptr0 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec0.len() as i32,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr0, vec0.as_bytes())?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 4, wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                }
            }
            let result2 = self.func_string_list.call(store, result1, len1)?;
            let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result2 + 0)?;
            let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result2 + 4)?;
            let len8 = load4;
            let base8 = load3;
            let mut result8 = Vec::with_capacity(len8 as usize);
            for i in 0..len8 {
                let base = base8 + i * 8;
                result8.push({
                    let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 0)?;
                    let load6 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 4)?;
                    let ptr7 = load5;
                    let len7 = load6;

                    let data7 = copy_slice(store, _memory, func_canonical_abi_free, ptr7, len7, 1)?;
                    String::from_utf8(data7)
                        .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base8, len8 * 8, 4)?;
            Ok(result8)
        }
        pub fn record_list(
            &self,
            store: &mut wasmer::Store,
            x: &[SomeRecordParam<'_>],
        ) -> Result<Vec<OtherRecordResult>, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec9 = x;
            let len9 = vec9.len() as i32;
            let result9 =
                func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 8, len9 * 96)?;
            for (i, e) in vec9.into_iter().enumerate() {
                let base = result9 + (i as i32) * 96;
                {
                    let SomeRecordParam {
                        x: x0,
                        y: y0,
                        z: z0,
                        c1: c10,
                        c2: c20,
                        c3: c30,
                        c4: c40,
                    } = e;
                    let vec1 = x0;
                    let ptr1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec1.len() as i32,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr1, vec1.as_bytes())?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 4, wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr1))?;
                    let OtherRecordParam {
                        a1: a12,
                        a2: a22,
                        a3: a32,
                        a4: a42,
                        b: b2,
                        c: c2,
                    } = y0;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 8,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(a12)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 16,
                        wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a22)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 24,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(a32)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 32,
                        wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a42)),
                    )?;
                    let vec3 = b2;
                    let ptr3 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec3.len() as i32,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr3, vec3.as_bytes())?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 44, wit_bindgen_wasmer::rt::as_i32(vec3.len() as i32))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 40, wit_bindgen_wasmer::rt::as_i32(ptr3))?;
                    let vec4 = c2;
                    let ptr4 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        (vec4.len() as i32) * 1,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr4, &vec4)?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 52, wit_bindgen_wasmer::rt::as_i32(vec4.len() as i32))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 48, wit_bindgen_wasmer::rt::as_i32(ptr4))?;
                    let vec8 = z0;
                    let len8 = vec8.len() as i32;
                    let result8 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        8,
                        len8 * 48,
                    )?;
                    for (i, e) in vec8.into_iter().enumerate() {
                        let base = result8 + (i as i32) * 48;
                        {
                            let OtherRecordParam {
                                a1: a15,
                                a2: a25,
                                a3: a35,
                                a4: a45,
                                b: b5,
                                c: c5,
                            } = e;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                                base + 0,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(a15)),
                            )?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                                base + 8,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a25)),
                            )?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                                base + 16,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(a35)),
                            )?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                                base + 24,
                                wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a45)),
                            )?;
                            let vec6 = b5;
                            let ptr6 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec6.len() as i32,
                            )?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store_many(ptr6, vec6.as_bytes())?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                                base + 36,
                                wit_bindgen_wasmer::rt::as_i32(vec6.len() as i32),
                            )?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store(base + 32, wit_bindgen_wasmer::rt::as_i32(ptr6))?;
                            let vec7 = c5;
                            let ptr7 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                (vec7.len() as i32) * 1,
                            )?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store_many(ptr7, &vec7)?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                                base + 44,
                                wit_bindgen_wasmer::rt::as_i32(vec7.len() as i32),
                            )?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store(base + 40, wit_bindgen_wasmer::rt::as_i32(ptr7))?;
                        }
                    }
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 60, wit_bindgen_wasmer::rt::as_i32(len8))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 56, wit_bindgen_wasmer::rt::as_i32(result8))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 64,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(c10)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 72,
                        wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(c20)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 80,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(c30)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 88,
                        wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(c40)),
                    )?;
                }
            }
            let result10 = self.func_record_list.call(store, result9, len9)?;
            let load11 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result10 + 0)?;
            let load12 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result10 + 4)?;
            let len23 = load12;
            let base23 = load11;
            let mut result23 = Vec::with_capacity(len23 as usize);
            for i in 0..len23 {
                let base = base23 + i * 48;
                result23.push({
                    let load13 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 0)?;
                    let load14 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i64>(base + 8)?;
                    let load15 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 16)?;
                    let load16 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i64>(base + 24)?;
                    let load17 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 32)?;
                    let load18 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 36)?;
                    let ptr19 = load17;
                    let len19 = load18;

                    let data19 =
                        copy_slice(store, _memory, func_canonical_abi_free, ptr19, len19, 1)?;
                    let load20 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 40)?;
                    let load21 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 44)?;
                    let ptr22 = load20;
                    let len22 = load21;
                    OtherRecordResult {
                        a1: load13 as u32,
                        a2: load14 as u64,
                        a3: load15,
                        a4: load16,
                        b: String::from_utf8(data19)
                            .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?,
                        c: copy_slice(store, _memory, func_canonical_abi_free, ptr22, len22, 1)?,
                    }
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base23, len23 * 48, 8)?;
            Ok(result23)
        }
        pub fn record_list_reverse(
            &self,
            store: &mut wasmer::Store,
            x: &[OtherRecordParam<'_>],
        ) -> Result<Vec<SomeRecordResult>, wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let vec3 = x;
            let len3 = vec3.len() as i32;
            let result3 =
                func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 8, len3 * 48)?;
            for (i, e) in vec3.into_iter().enumerate() {
                let base = result3 + (i as i32) * 48;
                {
                    let OtherRecordParam {
                        a1: a10,
                        a2: a20,
                        a3: a30,
                        a4: a40,
                        b: b0,
                        c: c0,
                    } = e;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 0,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(a10)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 8,
                        wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a20)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 16,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(a30)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 24,
                        wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(a40)),
                    )?;
                    let vec1 = b0;
                    let ptr1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec1.len() as i32,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr1, vec1.as_bytes())?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 36, wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 32, wit_bindgen_wasmer::rt::as_i32(ptr1))?;
                    let vec2 = c0;
                    let ptr2 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        (vec2.len() as i32) * 1,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr2, &vec2)?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 44, wit_bindgen_wasmer::rt::as_i32(vec2.len() as i32))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 40, wit_bindgen_wasmer::rt::as_i32(ptr2))?;
                }
            }
            let result4 = self.func_record_list_reverse.call(store, result3, len3)?;
            let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result4 + 0)?;
            let load6 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result4 + 4)?;
            let len37 = load6;
            let base37 = load5;
            let mut result37 = Vec::with_capacity(len37 as usize);
            for i in 0..len37 {
                let base = base37 + i * 96;
                result37.push({
                    let load7 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 0)?;
                    let load8 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 4)?;
                    let ptr9 = load7;
                    let len9 = load8;

                    let data9 = copy_slice(store, _memory, func_canonical_abi_free, ptr9, len9, 1)?;
                    let load10 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 8)?;
                    let load11 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i64>(base + 16)?;
                    let load12 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 24)?;
                    let load13 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i64>(base + 32)?;
                    let load14 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 40)?;
                    let load15 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 44)?;
                    let ptr16 = load14;
                    let len16 = load15;

                    let data16 =
                        copy_slice(store, _memory, func_canonical_abi_free, ptr16, len16, 1)?;
                    let load17 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 48)?;
                    let load18 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 52)?;
                    let ptr19 = load17;
                    let len19 = load18;
                    let load20 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 56)?;
                    let load21 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 60)?;
                    let len32 = load21;
                    let base32 = load20;
                    let mut result32 = Vec::with_capacity(len32 as usize);
                    for i in 0..len32 {
                        let base = base32 + i * 48;
                        result32.push({
                            let load22 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(base + 0)?;
                            let load23 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i64>(base + 8)?;
                            let load24 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(base + 16)?;
                            let load25 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i64>(base + 24)?;
                            let load26 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(base + 32)?;
                            let load27 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(base + 36)?;
                            let ptr28 = load26;
                            let len28 = load27;

                            let data28 = copy_slice(
                                store,
                                _memory,
                                func_canonical_abi_free,
                                ptr28,
                                len28,
                                1,
                            )?;
                            let load29 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(base + 40)?;
                            let load30 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(base + 44)?;
                            let ptr31 = load29;
                            let len31 = load30;
                            OtherRecordResult {
                                a1: load22 as u32,
                                a2: load23 as u64,
                                a3: load24,
                                a4: load25,
                                b: String::from_utf8(data28)
                                    .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?,
                                c: copy_slice(
                                    store,
                                    _memory,
                                    func_canonical_abi_free,
                                    ptr31,
                                    len31,
                                    1,
                                )?,
                            }
                        });
                    }
                    func_canonical_abi_free.call(
                        &mut store.as_store_mut(),
                        base32,
                        len32 * 48,
                        8,
                    )?;
                    let load33 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 64)?;
                    let load34 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i64>(base + 72)?;
                    let load35 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 80)?;
                    let load36 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i64>(base + 88)?;
                    SomeRecordResult {
                        x: String::from_utf8(data9)
                            .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?,
                        y: OtherRecordResult {
                            a1: load10 as u32,
                            a2: load11 as u64,
                            a3: load12,
                            a4: load13,
                            b: String::from_utf8(data16)
                                .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?,
                            c: copy_slice(
                                store,
                                _memory,
                                func_canonical_abi_free,
                                ptr19,
                                len19,
                                1,
                            )?,
                        },
                        z: result32,
                        c1: load33 as u32,
                        c2: load34 as u64,
                        c3: load35,
                        c4: load36,
                    }
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base37, len37 * 96, 8)?;
            Ok(result37)
        }
        pub fn variant_list(
            &self,
            store: &mut wasmer::Store,
            x: &[SomeVariant<'_>],
        ) -> Result<Vec<OtherVariantResult>, wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let vec3 = x;
            let len3 = vec3.len() as i32;
            let result3 =
                func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 4, len3 * 12)?;
            for (i, e) in vec3.into_iter().enumerate() {
                let base = result3 + (i as i32) * 12;
                {
                    match e {
                        SomeVariant::A(e) => {
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store(base + 0, wit_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
                            let vec0 = e;
                            let ptr0 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                1,
                                vec0.len() as i32,
                            )?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store_many(ptr0, vec0.as_bytes())?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                                base + 8,
                                wit_bindgen_wasmer::rt::as_i32(vec0.len() as i32),
                            )?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store(base + 4, wit_bindgen_wasmer::rt::as_i32(ptr0))?;
                        }
                        SomeVariant::B => {
                            let e = ();
                            {
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .store(base + 0, wit_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
                                let () = e;
                            }
                        }
                        SomeVariant::C(e) => {
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store(base + 0, wit_bindgen_wasmer::rt::as_i32(2i32) as u8)?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                                base + 4,
                                wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(e)),
                            )?;
                        }
                        SomeVariant::D(e) => {
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store(base + 0, wit_bindgen_wasmer::rt::as_i32(3i32) as u8)?;
                            let vec2 = e;
                            let len2 = vec2.len() as i32;
                            let result2 = func_canonical_abi_realloc.call(
                                &mut store.as_store_mut(),
                                0,
                                0,
                                4,
                                len2 * 12,
                            )?;
                            for (i, e) in vec2.into_iter().enumerate() {
                                let base = result2 + (i as i32) * 12;
                                {
                                    match e {
                                        OtherVariantParam::A => {
                                            let e = ();
                                            {
                                                unsafe {
                                                    _memory
                                                        .data_unchecked_mut(&store.as_store_ref())
                                                }
                                                .store(
                                                    base + 0,
                                                    wit_bindgen_wasmer::rt::as_i32(0i32) as u8,
                                                )?;
                                                let () = e;
                                            }
                                        }
                                        OtherVariantParam::B(e) => {
                                            unsafe {
                                                _memory.data_unchecked_mut(&store.as_store_ref())
                                            }
                                            .store(
                                                base + 0,
                                                wit_bindgen_wasmer::rt::as_i32(1i32) as u8,
                                            )?;
                                            unsafe {
                                                _memory.data_unchecked_mut(&store.as_store_ref())
                                            }
                                            .store(
                                                base + 4,
                                                wit_bindgen_wasmer::rt::as_i32(
                                                    wit_bindgen_wasmer::rt::as_i32(e),
                                                ),
                                            )?;
                                        }
                                        OtherVariantParam::C(e) => {
                                            unsafe {
                                                _memory.data_unchecked_mut(&store.as_store_ref())
                                            }
                                            .store(
                                                base + 0,
                                                wit_bindgen_wasmer::rt::as_i32(2i32) as u8,
                                            )?;
                                            let vec1 = e;
                                            let ptr1 = func_canonical_abi_realloc.call(
                                                &mut store.as_store_mut(),
                                                0,
                                                0,
                                                1,
                                                vec1.len() as i32,
                                            )?;
                                            unsafe {
                                                _memory.data_unchecked_mut(&store.as_store_ref())
                                            }
                                            .store_many(ptr1, vec1.as_bytes())?;
                                            unsafe {
                                                _memory.data_unchecked_mut(&store.as_store_ref())
                                            }
                                            .store(
                                                base + 8,
                                                wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32),
                                            )?;
                                            unsafe {
                                                _memory.data_unchecked_mut(&store.as_store_ref())
                                            }
                                            .store(
                                                base + 4,
                                                wit_bindgen_wasmer::rt::as_i32(ptr1),
                                            )?;
                                        }
                                    };
                                }
                            }
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store(base + 8, wit_bindgen_wasmer::rt::as_i32(len2))?;
                            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                .store(base + 4, wit_bindgen_wasmer::rt::as_i32(result2))?;
                        }
                    };
                }
            }
            let result4 = self.func_variant_list.call(store, result3, len3)?;
            let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result4 + 0)?;
            let load6 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result4 + 4)?;
            let len12 = load6;
            let base12 = load5;
            let mut result12 = Vec::with_capacity(len12 as usize);
            for i in 0..len12 {
                let base = base12 + i * 12;
                result12.push({
                    let load7 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(base + 0)?;
                    match i32::from(load7) {
                        0 => OtherVariantResult::A,
                        1 => OtherVariantResult::B({
                            let load8 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(base + 4)?;
                            load8 as u32
                        }),
                        2 => OtherVariantResult::C({
                            let load9 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(base + 4)?;
                            let load10 =
                                unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                                    .load::<i32>(base + 8)?;
                            let ptr11 = load9;
                            let len11 = load10;

                            let data11 = copy_slice(
                                store,
                                _memory,
                                func_canonical_abi_free,
                                ptr11,
                                len11,
                                1,
                            )?;
                            String::from_utf8(data11)
                                .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                        }),
                        _ => return Err(invalid_variant("OtherVariantResult")),
                    }
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base12, len12 * 12, 4)?;
            Ok(result12)
        }
        pub fn load_store_everything(
            &self,
            store: &mut wasmer::Store,
            a: LoadStoreAllSizesParam<'_>,
        ) -> Result<LoadStoreAllSizesResult, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let vec2 = a;
            let len2 = vec2.len() as i32;
            let result2 =
                func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 8, len2 * 64)?;
            for (i, e) in vec2.into_iter().enumerate() {
                let base = result2 + (i as i32) * 64;
                {
                    let (t0_0, t0_1, t0_2, t0_3, t0_4, t0_5, t0_6, t0_7, t0_8, t0_9, t0_10, t0_11) =
                        e;
                    let vec1 = t0_0;
                    let ptr1 = func_canonical_abi_realloc.call(
                        &mut store.as_store_mut(),
                        0,
                        0,
                        1,
                        vec1.len() as i32,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store_many(ptr1, vec1.as_bytes())?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 4, wit_bindgen_wasmer::rt::as_i32(vec1.len() as i32))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr1))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 8,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_1)) as u8,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 9,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_2)) as u8,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 10,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_3)) as u16,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 12,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_4)) as u16,
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 16,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_5)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 20,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_6)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 24,
                        wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(t0_7)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 32,
                        wit_bindgen_wasmer::rt::as_i64(wit_bindgen_wasmer::rt::as_i64(t0_8)),
                    )?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 40, wit_bindgen_wasmer::rt::as_f32(t0_9))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .store(base + 48, wit_bindgen_wasmer::rt::as_f64(t0_10))?;
                    unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }.store(
                        base + 56,
                        wit_bindgen_wasmer::rt::as_i32(wit_bindgen_wasmer::rt::as_i32(t0_11)),
                    )?;
                }
            }
            let result3 = self.func_load_store_everything.call(store, result2, len2)?;
            let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result3 + 0)?;
            let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result3 + 4)?;
            let len20 = load5;
            let base20 = load4;
            let mut result20 = Vec::with_capacity(len20 as usize);
            for i in 0..len20 {
                let base = base20 + i * 64;
                result20.push({
                    let load6 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 0)?;
                    let load7 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 4)?;
                    let ptr8 = load6;
                    let len8 = load7;

                    let data8 = copy_slice(store, _memory, func_canonical_abi_free, ptr8, len8, 1)?;
                    let load9 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u8>(base + 8)?;
                    let load10 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i8>(base + 9)?;
                    let load11 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<u16>(base + 10)?;
                    let load12 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i16>(base + 12)?;
                    let load13 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 16)?;
                    let load14 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 20)?;
                    let load15 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i64>(base + 24)?;
                    let load16 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i64>(base + 32)?;
                    let load17 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<f32>(base + 40)?;
                    let load18 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<f64>(base + 48)?;
                    let load19 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                        .load::<i32>(base + 56)?;
                    (
                        String::from_utf8(data8)
                            .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?,
                        u8::try_from(i32::from(load9)).map_err(bad_int)?,
                        i8::try_from(i32::from(load10)).map_err(bad_int)?,
                        u16::try_from(i32::from(load11)).map_err(bad_int)?,
                        i16::try_from(i32::from(load12)).map_err(bad_int)?,
                        load13 as u32,
                        load14,
                        load15 as u64,
                        load16,
                        load17,
                        load18,
                        char_from_i32(load19)?,
                    )
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base20, len20 * 64, 8)?;
            Ok(result20)
        }
    }
    use core::convert::TryFrom;
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::bad_int;
    use wit_bindgen_wasmer::rt::char_from_i32;
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
