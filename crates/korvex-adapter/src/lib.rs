/// The contract every language adapter must satisfy.
///
/// Built-in adapters (PyO3, napi-rs, WASI) live in their respective crates.
/// Third-party crates can depend on `korvex-adapter` and ship their own implementation.
pub trait BindingAdapter {
    fn generate_bridge(&self);
}
