// Python adapter: reads the Korvex IR and emits pyo3-based binding glue.

use korvex_adapter::BindingAdapter;

pub struct PyO3Binding;

impl BindingAdapter for PyO3Binding {
    fn generate_bridge(&self) {
        todo!("emit pyo3 binding glue")
    }
}

pub struct KorvexPython<A: BindingAdapter = PyO3Binding> {
    pub adapter: A,
}
