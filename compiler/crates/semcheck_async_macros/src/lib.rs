mod r#impl;

use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn communicable(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream
) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(item as ItemFn);
    r#impl::communicable::proc_macro_impl(ast).into()
}
