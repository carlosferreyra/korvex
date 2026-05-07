extern crate proc_macro;

use proc_macro::TokenStream;

/// Marks a Rust type or function for binding generation.
/// Korvex reads these annotations to build the IR consumed by each language adapter.
#[proc_macro_attribute]
pub fn export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
