pub mod records {
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Empty {}
    impl std::fmt::Debug for Empty {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for Scalars {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for ReallyFlags {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl<'a> std::fmt::Debug for AggregatesParam<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl std::fmt::Debug for AggregatesResult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    impl wasmer::WasmerEnv for RecordsData {
        fn init_with_instance(
            &mut self,
            instance: &wasmer::Instance,
        ) -> Result<(), wasmer::HostEnvInitError> {
            let _ = instance;
            Ok(())
        }
    }
    impl Clone for RecordsData {
        fn clone(&self) -> Self {
            Self::default()
        }
    }
    pub struct Records {
        state: std::sync::Arc<std::sync::Mutex<RecordsData>>,
        func_aggregate_arg: wasmer::NativeFunc<
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
        func_aggregate_result: wasmer::NativeFunc<(), i32>,
        func_canonical_abi_free: wasmer::NativeFunc<(i32, i32, i32), ()>,
        func_canonical_abi_realloc: wasmer::NativeFunc<(i32, i32, i32, i32), i32>,
        func_empty_arg: wasmer::NativeFunc<(), ()>,
        func_empty_result: wasmer::NativeFunc<(), ()>,
        func_flags_arg: wasmer::NativeFunc<(i32, i32, i32, i32, i32, i32, i32, i32, i32), ()>,
        func_flags_result: wasmer::NativeFunc<(), i32>,
        func_scalar_arg: wasmer::NativeFunc<(i32, i32), ()>,
        func_scalar_result: wasmer::NativeFunc<(), i32>,
        func_tuple_arg: wasmer::NativeFunc<(i32, i32), ()>,
        func_tuple_result: wasmer::NativeFunc<(), i32>,
        func_typedef_inout: wasmer::NativeFunc<i32, i32>,
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
            store: &wasmer::Store,
            imports: &mut wasmer::ImportObject,
        ) -> std::sync::Arc<std::sync::Mutex<RecordsData>> {
            let state = std::sync::Arc::new(std::sync::Mutex::new(Default::default()));
            state
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
            store: &wasmer::Store,
            module: &wasmer::Module,
            imports: &mut wasmer::ImportObject,
        ) -> anyhow::Result<(Self, wasmer::Instance)> {
            let state = Self::add_to_imports(store, imports);
            let instance = wasmer::Instance::new(module, &*imports)?;
            Ok((Self::new(&instance, state)?, instance))
        }

        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// and wrap them all up in the returned structure which can
        /// be used to interact with the wasm module.
        pub fn new(
            instance: &wasmer::Instance,
            state: std::sync::Arc<std::sync::Mutex<RecordsData>>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_aggregate_arg = instance.exports.get_native_function::<(
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
            ), ()>("aggregate-arg")?;
            let func_aggregate_result = instance
                .exports
                .get_native_function::<(), i32>("aggregate-result")?;
            let func_canonical_abi_free = instance
                .exports
                .get_native_function::<(i32, i32, i32), ()>("canonical_abi_free")?;
            let func_canonical_abi_realloc = instance
                .exports
                .get_native_function::<(i32, i32, i32, i32), i32>("canonical_abi_realloc")?;
            let func_empty_arg = instance
                .exports
                .get_native_function::<(), ()>("empty-arg")?;
            let func_empty_result = instance
                .exports
                .get_native_function::<(), ()>("empty-result")?;
            let func_flags_arg = instance
                .exports
                .get_native_function::<(i32, i32, i32, i32, i32, i32, i32, i32, i32), ()>(
                    "flags-arg",
                )?;
            let func_flags_result = instance
                .exports
                .get_native_function::<(), i32>("flags-result")?;
            let func_scalar_arg = instance
                .exports
                .get_native_function::<(i32, i32), ()>("scalar-arg")?;
            let func_scalar_result = instance
                .exports
                .get_native_function::<(), i32>("scalar-result")?;
            let func_tuple_arg = instance
                .exports
                .get_native_function::<(i32, i32), ()>("tuple-arg")?;
            let func_tuple_result = instance
                .exports
                .get_native_function::<(), i32>("tuple-result")?;
            let func_typedef_inout = instance
                .exports
                .get_native_function::<i32, i32>("typedef-inout")?;
            let memory = instance.exports.get_memory("memory")?.clone();
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
                state,
            })
        }
        pub fn tuple_arg(&self, x: (char, u32)) -> Result<(), wasmer::RuntimeError> {
            let (t0_0, t0_1) = x;
            self.func_tuple_arg.call(
                wit_bindgen_wasmer::rt::as_i32(t0_0),
                wit_bindgen_wasmer::rt::as_i32(t0_1),
            )?;
            Ok(())
        }
        pub fn tuple_result(&self) -> Result<(char, u32), wasmer::RuntimeError> {
            let memory = &self.memory;
            let result0 = self.func_tuple_result.call()?;
            let load1 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 0)?;
            let load2 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 4)?;
            Ok((char_from_i32(load1)?, load2 as u32))
        }
        pub fn empty_arg(&self, x: Empty) -> Result<(), wasmer::RuntimeError> {
            let Empty {} = x;
            self.func_empty_arg.call()?;
            Ok(())
        }
        pub fn empty_result(&self) -> Result<Empty, wasmer::RuntimeError> {
            self.func_empty_result.call()?;
            Ok(Empty {})
        }
        pub fn scalar_arg(&self, x: Scalars) -> Result<(), wasmer::RuntimeError> {
            let Scalars { a: a0, b: b0 } = x;
            self.func_scalar_arg.call(
                wit_bindgen_wasmer::rt::as_i32(a0),
                wit_bindgen_wasmer::rt::as_i32(b0),
            )?;
            Ok(())
        }
        pub fn scalar_result(&self) -> Result<Scalars, wasmer::RuntimeError> {
            let memory = &self.memory;
            let result0 = self.func_scalar_result.call()?;
            let load1 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 0)?;
            let load2 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 4)?;
            Ok(Scalars {
                a: load1 as u32,
                b: load2 as u32,
            })
        }
        pub fn flags_arg(&self, x: ReallyFlags) -> Result<(), wasmer::RuntimeError> {
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
        pub fn flags_result(&self) -> Result<ReallyFlags, wasmer::RuntimeError> {
            let memory = &self.memory;
            let result0 = self.func_flags_result.call()?;
            let load1 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 0)?;
            let load2 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 1)?;
            let load3 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 2)?;
            let load4 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 3)?;
            let load5 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 4)?;
            let load6 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 5)?;
            let load7 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 6)?;
            let load8 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 7)?;
            let load9 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 8)?;
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
        pub fn aggregate_arg(&self, x: AggregatesParam<'_>) -> Result<(), wasmer::RuntimeError> {
            let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
            let memory = &self.memory;
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
            let ptr3 = func_canonical_abi_realloc.call(0, 0, 1, vec3.len() as i32)?;
            unsafe { memory.data_unchecked_mut() }.store_many(ptr3, vec3.as_bytes())?;
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
        pub fn aggregate_result(&self) -> Result<AggregatesResult, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let memory = &self.memory;
            let result0 = self.func_aggregate_result.call()?;
            let load1 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 0)?;
            let load2 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 4)?;
            let load3 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 8)?;
            let load4 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 12)?;
            let load5 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result0 + 16)?;
            let ptr6 = load4;
            let len6 = load5;

            let data6 = copy_slice(memory, func_canonical_abi_free, ptr6, len6, 1)?;
            let load7 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 20)?;
            let load8 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 21)?;
            let load9 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 22)?;
            let load10 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 23)?;
            let load11 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 24)?;
            let load12 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 25)?;
            let load13 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 26)?;
            let load14 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 27)?;
            let load15 = unsafe { memory.data_unchecked_mut() }.load::<u8>(result0 + 28)?;
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
        pub fn typedef_inout(&self, e: TupleTypedef2) -> Result<i32, wasmer::RuntimeError> {
            let (t0_0,) = e;
            let result1 = self
                .func_typedef_inout
                .call(wit_bindgen_wasmer::rt::as_i32(t0_0))?;
            Ok(result1)
        }
    }
    use wit_bindgen_wasmer::rt::char_from_i32;
    use wit_bindgen_wasmer::rt::copy_slice;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::RawMem;
}
