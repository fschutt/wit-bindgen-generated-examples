#[allow(clippy::all)]
pub mod empty {
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
    #[allow(unused_imports)]
    use wit_bindgen_wasmer::{anyhow, wasmer};
}
