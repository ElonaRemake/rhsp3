use proc_macro2::{Span, TokenStream};
use proc_macro_crate::FoundCrate;
use quote::quote;
use std::sync::atomic::{AtomicUsize, Ordering};
use syn::Error;

pub fn crate_root(attr_span: Span, name: &str) -> Result<TokenStream, Error> {
    let crate_path = match proc_macro_crate::crate_name(name) {
        Ok(v) => match v {
            FoundCrate::Itself => quote! { crate },
            FoundCrate::Name(name) => {
                let ident = ident!("{name}");
                quote! { #ident }
            }
        },
        Err(_) => return Err(Error::new(attr_span, format!("`{name}` crate not found."))),
    };
    let root = quote! { #crate_path::__macro_export };
    Ok(root)
}

macro_rules! make_id {
    ($name:ident) => {
        pub fn $name() -> usize {
            static ID: AtomicUsize = AtomicUsize::new(0);
            ID.fetch_add(1, Ordering::Relaxed)
        }
    };
}

make_id!(get_ident_id);
make_id!(get_func_registration_id);
make_id!(get_define_registration_id);
