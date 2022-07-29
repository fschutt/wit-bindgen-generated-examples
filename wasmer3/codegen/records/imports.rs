#[allow(clippy::all)]
pub mod records {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Empty {}
    impl core::fmt::Debug for Empty {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
    /// A record containing two scalar fields
    /// that both have the same type
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Scalars {
        /// The first field, named a
        pub a: u32,
        /// The second field, named b
        pub b: u32,
    }
    impl core::fmt::Debug for Scalars {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Scalars")
                .field("a", &self.a)
                .field("b", &self.b)
                .finish()
        }
    }
    impl wit_bindgen_wasmer::Endian for Scalars {
        fn into_le(self) -> Self {
            Self {
                a: self.a.into_le(),
                b: self.b.into_le(),
            }
        }
        fn from_le(self) -> Self {
            Self {
                a: self.a.from_le(),
                b: self.b.from_le(),
            }
        }
    }
    unsafe impl wit_bindgen_wasmer::AllBytesValid for Scalars {}
    /// A record that is really just flags
    /// All of the fields are bool
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ReallyFlags {
        pub a: bool,
        pub b: bool,
        pub c: bool,
        pub d: bool,
        pub e: bool,
        pub f: bool,
        pub g: bool,
        pub h: bool,
        pub i: bool,
    }
    impl core::fmt::Debug for ReallyFlags {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ReallyFlags")
                .field("a", &self.a)
                .field("b", &self.b)
                .field("c", &self.c)
                .field("d", &self.d)
                .field("e", &self.e)
                .field("f", &self.f)
                .field("g", &self.g)
                .field("h", &self.h)
                .field("i", &self.i)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct AggregatesParam<'a> {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: &'a str,
        pub e: ReallyFlags,
    }
    impl<'a> core::fmt::Debug for AggregatesParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("AggregatesParam")
                .field("a", &self.a)
                .field("b", &self.b)
                .field("c", &self.c)
                .field("d", &self.d)
                .field("e", &self.e)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct AggregatesResult {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: String,
        pub e: ReallyFlags,
    }
    impl core::fmt::Debug for AggregatesResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("AggregatesResult")
                .field("a", &self.a)
                .field("b", &self.b)
                .field("c", &self.c)
                .field("d", &self.d)
                .field("e", &self.e)
                .finish()
        }
    }
    pub type IntTypedef = i32;
    pub type TupleTypedef2 = (IntTypedef,);

    /// Auxiliary data associated with the wasm exports.
    #[derive(Default)]
    pub struct RecordsData {}

    pub struct Records {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<RecordsData>,
        func_aggregate_arg: wasmer::TypedFunction<
            (
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
            ),
            (),
        >,
        func_aggregate_result: wasmer::TypedFunction<(), i32>,
        func_canonical_abi_free: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
        func_empty_arg: wasmer::TypedFunction<(), ()>,
        func_empty_result: wasmer::TypedFunction<(), ()>,
        func_flags_arg: wasmer::TypedFunction<(i32, i32, i32, i32, i32, i32, i32, i32, i32), ()>,
        func_flags_result: wasmer::TypedFunction<(), i32>,
        func_scalar_arg: wasmer::TypedFunction<(i32, i32), ()>,
        func_scalar_result: wasmer::TypedFunction<(), i32>,
        func_tuple_arg: wasmer::TypedFunction<(i32, i32), ()>,
        func_tuple_result: wasmer::TypedFunction<(), i32>,
        func_typedef_inout: wasmer::TypedFunction<i32, i32>,
        memory: wasmer::Memory,
    }
    impl Records {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `RecordsData` which needs to be
        /// passed through to `Records::new`.
        fn add_to_imports(
            store: &mut wasmer::StoreMut<'_>,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<RecordsData> {
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
            env: wasmer::FunctionEnv<RecordsData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_aggregate_arg = _instance
                .exports
                .get_typed_function(store, "aggregate-arg")?;
            let func_aggregate_result = _instance
                .exports
                .get_typed_function(store, "aggregate-result")?;
            let func_canonical_abi_free = _instance
                .exports
                .get_typed_function(store, "canonical_abi_free")?;
            let func_canonical_abi_realloc = _instance
                .exports
                .get_typed_function(store, "canonical_abi_realloc")?;
            let func_empty_arg = _instance.exports.get_typed_function(store, "empty-arg")?;
            let func_empty_result = _instance
                .exports
                .get_typed_function(store, "empty-result")?;
            let func_flags_arg = _instance.exports.get_typed_function(store, "flags-arg")?;
            let func_flags_result = _instance
                .exports
                .get_typed_function(store, "flags-result")?;
            let func_scalar_arg = _instance.exports.get_typed_function(store, "scalar-arg")?;
            let func_scalar_result = _instance
                .exports
                .get_typed_function(store, "scalar-result")?;
            let func_tuple_arg = _instance.exports.get_typed_function(store, "tuple-arg")?;
            let func_tuple_result = _instance
                .exports
                .get_typed_function(store, "tuple-result")?;
            let func_typedef_inout = _instance
                .exports
                .get_typed_function(store, "typedef-inout")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(Records {
                func_aggregate_arg,
                func_aggregate_result,
                func_canonical_abi_free,
                func_canonical_abi_realloc,
                func_empty_arg,
                func_empty_result,
                func_flags_arg,
                func_flags_result,
                func_scalar_arg,
                func_scalar_result,
                func_tuple_arg,
                func_tuple_result,
                func_typedef_inout,
                memory,
                env,
            })
        }
        pub fn tuple_arg(
            &self,
            store: &mut wasmer::Store,
            x: (char, u32),
        ) -> Result<(), wasmer::RuntimeError> {
            let (t0_0, t0_1) = x;
            self.func_tuple_arg.call(
                store,
                wit_bindgen_wasmer::rt::as_i32(t0_0),
                wit_bindgen_wasmer::rt::as_i32(t0_1),
            )?;
            Ok(())
        }
        pub fn tuple_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<(char, u32), wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_tuple_result.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            Ok((char_from_i32(load1)?, load2 as u32))
        }
        pub fn empty_arg(
            &self,
            store: &mut wasmer::Store,
            x: Empty,
        ) -> Result<(), wasmer::RuntimeError> {
            let Empty {} = x;
            self.func_empty_arg.call(store)?;
            Ok(())
        }
        pub fn empty_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Empty, wasmer::RuntimeError> {
            self.func_empty_result.call(store)?;
            Ok(Empty {})
        }
        pub fn scalar_arg(
            &self,
            store: &mut wasmer::Store,
            x: Scalars,
        ) -> Result<(), wasmer::RuntimeError> {
            let Scalars { a: a0, b: b0 } = x;
            self.func_scalar_arg.call(
                store,
                wit_bindgen_wasmer::rt::as_i32(a0),
                wit_bindgen_wasmer::rt::as_i32(b0),
            )?;
            Ok(())
        }
        pub fn scalar_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Scalars, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_scalar_result.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            Ok(Scalars {
                a: load1 as u32,
                b: load2 as u32,
            })
        }
        pub fn flags_arg(
            &self,
            store: &mut wasmer::Store,
            x: ReallyFlags,
        ) -> Result<(), wasmer::RuntimeError> {
            let ReallyFlags {
                a: a0,
                b: b0,
                c: c0,
                d: d0,
                e: e0,
                f: f0,
                g: g0,
                h: h0,
                i: i0,
            } = x;
            self.func_flags_arg.call(
                store,
                match a0 {
                    true => 1,
                    false => 0,
                },
                match b0 {
                    true => 1,
                    false => 0,
                },
                match c0 {
                    true => 1,
                    false => 0,
                },
                match d0 {
                    true => 1,
                    false => 0,
                },
                match e0 {
                    true => 1,
                    false => 0,
                },
                match f0 {
                    true => 1,
                    false => 0,
                },
                match g0 {
                    true => 1,
                    false => 0,
                },
                match h0 {
                    true => 1,
                    false => 0,
                },
                match i0 {
                    true => 1,
                    false => 0,
                },
            )?;
            Ok(())
        }
        pub fn flags_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<ReallyFlags, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_flags_result.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 1)?;
            let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 2)?;
            let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 3)?;
            let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 4)?;
            let load6 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 5)?;
            let load7 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 6)?;
            let load8 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 7)?;
            let load9 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 8)?;
            Ok(ReallyFlags {
                a: match i32::from(load1) {
                    0 => false,
                    1 => true,
                    _ => return Err(invalid_variant("bool")),
                },
                b: match i32::from(load2) {
                    0 => false,
                    1 => true,
                    _ => return Err(invalid_variant("bool")),
                },
                c: match i32::from(load3) {
                    0 => false,
                    1 => true,
                    _ => return Err(invalid_variant("bool")),
                },
                d: match i32::from(load4) {
                    0 => false,
                    1 => true,
                    _ => return Err(invalid_variant("bool")),
                },
                e: match i32::from(load5) {
                    0 => false,
                    1 => true,
                    _ => return Err(invalid_variant("bool")),
                },
                f: match i32::from(load6) {
                    0 => false,
                    1 => true,
                    _ => return Err(invalid_variant("bool")),
                },
                g: match i32::from(load7) {
                    0 => false,
                    1 => true,
                    _ => return Err(invalid_variant("bool")),
                },
                h: match i32::from(load8) {
                    0 => false,
                    1 => true,
                    _ => return Err(invalid_variant("bool")),
                },
                i: match i32::from(load9) {
                    0 => false,
                    1 => true,
                    _ => return Err(invalid_variant("bool")),
                },
            })
        }
        pub fn aggregate_arg(
            &self,
            store: &mut wasmer::Store,
            x: AggregatesParam<'_>,
        ) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let _memory = &self.memory;
            let AggregatesParam {
                a: a0,
                b: b0,
                c: c0,
                d: d0,
                e: e0,
            } = x;
            let Scalars { a: a1, b: b1 } = a0;
            let Empty {} = c0;
            let vec3 = d0;
            let ptr3 = func_canonical_abi_realloc.call(
                &mut store.as_store_mut(),
                0,
                0,
                1,
                vec3.len() as i32,
            )?;
            unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .store_many(ptr3, vec3.as_bytes())?;
            let ReallyFlags {
                a: a4,
                b: b4,
                c: c4,
                d: d4,
                e: e4,
                f: f4,
                g: g4,
                h: h4,
                i: i4,
            } = e0;
            self.func_aggregate_arg.call(
                store,
                wit_bindgen_wasmer::rt::as_i32(a1),
                wit_bindgen_wasmer::rt::as_i32(b1),
                wit_bindgen_wasmer::rt::as_i32(b0),
                ptr3,
                vec3.len() as i32,
                match a4 {
                    true => 1,
                    false => 0,
                },
                match b4 {
                    true => 1,
                    false => 0,
                },
                match c4 {
                    true => 1,
                    false => 0,
                },
                match d4 {
                    true => 1,
                    false => 0,
                },
                match e4 {
                    true => 1,
                    false => 0,
                },
                match f4 {
                    true => 1,
                    false => 0,
                },
                match g4 {
                    true => 1,
                    false => 0,
                },
                match h4 {
                    true => 1,
                    false => 0,
                },
                match i4 {
                    true => 1,
                    false => 0,
                },
            )?;
            Ok(())
        }
        pub fn aggregate_result(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<AggregatesResult, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_aggregate_result.call(store)?;
            let load1 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 0)?;
            let load2 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 4)?;
            let load3 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 8)?;
            let load4 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 12)?;
            let load5 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<i32>(result0 + 16)?;
            let ptr6 = load4;
            let len6 = load5;

            let data6 = copy_slice(store, _memory, func_canonical_abi_free, ptr6, len6, 1)?;
            let load7 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 20)?;
            let load8 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 21)?;
            let load9 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 22)?;
            let load10 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 23)?;
            let load11 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 24)?;
            let load12 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 25)?;
            let load13 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 26)?;
            let load14 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 27)?;
            let load15 = unsafe { _memory.data_unchecked_mut(&store.as_store_ref()) }
                .load::<u8>(result0 + 28)?;
            Ok(AggregatesResult {
                a: Scalars {
                    a: load1 as u32,
                    b: load2 as u32,
                },
                b: load3 as u32,
                c: Empty {},
                d: String::from_utf8(data6)
                    .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?,
                e: ReallyFlags {
                    a: match i32::from(load7) {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    },
                    b: match i32::from(load8) {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    },
                    c: match i32::from(load9) {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    },
                    d: match i32::from(load10) {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    },
                    e: match i32::from(load11) {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    },
                    f: match i32::from(load12) {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    },
                    g: match i32::from(load13) {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    },
                    h: match i32::from(load14) {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    },
                    i: match i32::from(load15) {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    },
                },
            })
        }
        pub fn typedef_inout(
            &self,
            store: &mut wasmer::Store,
            e: TupleTypedef2,
        ) -> Result<i32, wasmer::RuntimeError> {
            let (t0_0,) = e;
            let result1 = self
                .func_typedef_inout
                .call(store, wit_bindgen_wasmer::rt::as_i32(t0_0))?;
            Ok(result1)
        }
    }
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    use wit_bindgen_wasmer::rt::char_from_i32;
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
