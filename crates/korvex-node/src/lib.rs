// Node.js adapter: reads the Korvex IR and emits napi-rs-based binding glue.

use korvex_adapter::BindingAdapter;

pub struct NapiBinding;

impl BindingAdapter for NapiBinding {
    fn generate_bridge(&self) {
        todo!("emit napi-rs binding glue")
    }
}

pub struct KorvexNode<A: BindingAdapter = NapiBinding> {
    pub adapter: A,
}
