use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn proc_macro_impl(_: TokenStream, ast: ItemFn) -> TokenStream {
    let fn_visibility = ast.vis;
    let fn_ident = ast.sig.ident;
    let fn_args = ast.sig.inputs;
    let fn_body = ast.block;
    let fn_ret_type = match ast.sig.output {
        syn::ReturnType::Default => quote!{ () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };

    quote! {
        #fn_visibility fn #fn_ident <'_a> (#fn_args) -> impl std::future::Future<Output = #fn_ret_type> {
            use std::future::poll_fn;
            use std::task::{Context, Poll};

            // Option, Result 両対応のためのトレイト
            trait TryCheck {
                fn is_success(&self) -> bool;
            }

            impl<T, E> TryCheck for Result<T, E> {
                fn is_success(&self) -> bool {
                    match self {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                }
            }

            impl<T> TryCheck for Option<T> {
                fn is_success(&self) -> bool {
                    self.is_some()
                }
            }

            // 元の関数をクロージャとして包む
            let __inner = move || -> #fn_ret_type {
                #fn_body
            };

            // async-tree 全体の状態を見ながら実行
            poll_fn(move |ctx: &mut Context<'_>| {
                let result = __inner();
                match (SharedState::check_deadlock(ctx), result.is_success()) {
                    (true,  _) => Poll::Ready(result),
                    (false, true) => Poll::Ready(result),
                    (false, false) => Poll::Pending,
                }
            })
        }
    }
}
