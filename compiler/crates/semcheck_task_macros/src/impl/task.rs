use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{FnArg, ItemFn, PathArguments, Type};

pub fn proc_macro_impl(args: TokenStream, ast: ItemFn) -> TokenStream {
    // マクロ引数
    let (_, args) = split2_token_stream(args, ':');
    let (timeout_ty, timeout_err) = split2_token_stream(args, '=');

    // 関数本体
    let fn_visibility = ast.vis;
    let fn_ident = ast.sig.ident;
    let fn_body = ast.block;

    let fn_args = ast.sig.inputs;
    let fn_args_names = fn_args
        .iter()
        .filter_map(is_arg)
        .collect::<Vec<_>>();

    let fn_ret_type = ast.sig.output;
    let fn_ret_type_unwrapped = match &fn_ret_type {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ty) => unwrap_type(ty),
    };
    let fn_ret_type_unwrapped = match fn_ret_type_unwrapped {
        Some(ty) => ty,
        None => panic!("#[task] macro requires return type to be Option<T>"),
    };

    // マクロ適用結果
    quote! {
        #fn_visibility fn #fn_ident(#fn_args) -> PinnedTask<#fn_ret_type_unwrapped, #timeout_ty> {
            use std::future::poll_fn;
            use std::task::Poll;

            fn __inner(#fn_args) #fn_ret_type {
                #fn_body
            }

            Task::new(
                poll_fn(move |_| {
                    match __inner(#(#fn_args_names),*) {
                        Some(value) => Poll::Ready(value),
                        None => Poll::Pending,
                    }
                }),
                #timeout_err,
            )
        }
    }
}

fn split2_token_stream(stream: TokenStream, c: char) -> (TokenStream, TokenStream) {
    // 指定文字 c で分割 (1回まで)
    let mut result = vec![vec![]];
    for token in stream {
        match &token {
            TokenTree::Punct(punct) if punct.as_char() == c && result.len() < 2 => {
                result.push(vec![]);
            }
            _ => result.last_mut().unwrap().push(token),
        }
    }

    // 分割結果を TokenStream に変換
    let right = result.pop().unwrap().into_iter().collect();
    let left = result.pop().unwrap().into_iter().collect();
    (left, right)
}

fn is_arg(arg: &FnArg) -> Option<TokenStream> {
    if let syn::FnArg::Typed(pat_type) = arg {
        Some(pat_type.pat.to_token_stream())
    } else {
        None
    }
}

fn unwrap_type(ty: &Type) -> Option<TokenStream> {
    if let Type::Path(type_path) = ty {
        let inner_ty = &type_path.path.segments[0].arguments;
        if let PathArguments::AngleBracketed(arg) = inner_ty {
            return Some(arg.args[0].to_token_stream());
        }
    }
    None
}
