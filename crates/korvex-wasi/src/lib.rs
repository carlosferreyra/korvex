// WASI adapter: reads the Korvex IR and emits WIT interfaces and wasm component model glue.
// Targets wasm32-wasip1/p2 today; designed for incremental wasip3/wasip4 support.

use korvex_adapter::BindingAdapter;

pub struct WitBinding;

impl BindingAdapter for WitBinding {
    fn generate_bridge(&self) {
        todo!("emit WIT interface and wasm component model glue")
    }
}

pub struct KorvexWasi<A: BindingAdapter = WitBinding> {
    pub adapter: A,
}
