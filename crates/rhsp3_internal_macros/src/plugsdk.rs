use proc_macro2::TokenStream;
use syn::{parse2, spanned::Spanned, Error, ItemFn};

pub fn hsp_export(_: TokenStream, item: TokenStream) -> Result<TokenStream, Error> {
    let func = parse2::<ItemFn>(item)?;

    Err(Error::new(func.span(), "Not yet implemented."))
}
