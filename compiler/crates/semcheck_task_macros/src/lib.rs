mod r#impl;

use proc_macro2::TokenStream;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn task(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream
) -> proc_macro::TokenStream {
    let args: TokenStream = attr.into();
    let ast = parse_macro_input!(item as ItemFn);
    r#impl::task::proc_macro_impl(args, ast).into()
}
