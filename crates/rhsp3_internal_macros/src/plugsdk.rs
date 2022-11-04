use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse2, spanned::Spanned, Error, FnArg, GenericArgument, ItemFn, PathArguments, TraitBound,
    Type, TypeImplTrait, TypeParamBound,
};

#[derive(Debug)]
enum HspParamType {
    ImplContext,
    ImplVar(Type),
    OwnedVal(Type),
    RefVal(Type),
    MutRefVal(Type),
    ExtData(Type),
    MutExtData(Type),
}
impl HspParamType {
    fn from_var(ty: &TraitBound) -> Result<HspParamType, Error> {
        match &ty.path.segments.last().unwrap().arguments {
            PathArguments::AngleBracketed(args) => {
                if args.args.len() != 1 {
                    return Err(Error::new(
                        args.span(),
                        "`impl Var` must have exactly one type parameter.",
                    ));
                }
                match args.args.first().unwrap() {
                    GenericArgument::Type(ty) => Ok(HspParamType::ImplVar(ty.clone())),
                    arg => Err(Error::new(
                        arg.span(),
                        "`impl Var` must contain a type parameter, not an arbitrary generic.",
                    )),
                }
            }
            seg => Err(Error::new(seg.span(), "`impl Var` must have exactly one type parameter.")),
        }
    }
    fn from_impl_trait(ty: &TypeImplTrait) -> Result<HspParamType, Error> {
        if ty.bounds.len() != 1 {
            return Err(Error::new(
                ty.bounds.span(),
                "`impl Trait` parameters should not contain more than one bound.",
            ));
        }
        let bound = ty.bounds.first().unwrap();

        match bound {
            TypeParamBound::Trait(ty) => {
                if ty.path.segments.last().unwrap().ident == "HspContext" {
                    if ty.path.segments.iter().any(|x| !x.arguments.is_empty()) {
                        return Err(Error::new(
                            ty.path.span(),
                            "`impl HspContext` should not contain any type parameters.",
                        ));
                    }
                    Ok(HspParamType::ImplContext)
                } else if ty.path.segments.last().unwrap().ident == "Var" {
                    HspParamType::from_var(ty)
                } else {
                    Err(Error::new(
                        bound.span(),
                        "`impl Trait` type not recognized. Note that rhsp3 does not \
                        currently support renaming `HspContext` or `Var` when importing. \
                        If you have a conflict, please use a full crate name instead.",
                    ))
                }
            }
            TypeParamBound::Lifetime(_) => Err(Error::new(
                bound.span(),
                "`impl Trait` parameters should not contain lifetime bounds.",
            )),
        }
    }
    fn from_reference_ty(ty: &Type, is_mut: bool) -> Result<HspParamType, Error> {
        match ty {
            Type::Group(ty) => HspParamType::from_reference_ty(&ty.elem, is_mut),
            Type::Paren(ty) => HspParamType::from_reference_ty(&ty.elem, is_mut),
            Type::Path(_) if !is_mut => Ok(HspParamType::MutRefVal(ty.clone())),
            Type::Path(_) if is_mut => Ok(HspParamType::MutRefVal(ty.clone())),
            Type::ImplTrait(ty) => HspParamType::from_impl_trait(ty),
            _ => Err(Error::new(ty.span(), "Unrecognized type in HSP function parameters.")),
        }
    }
    fn from_type(ty: &Type) -> Result<HspParamType, Error> {
        match ty {
            Type::Group(ty) => HspParamType::from_type(&ty.elem),
            Type::Paren(ty) => HspParamType::from_type(&ty.elem),
            Type::Path(_) => Ok(HspParamType::OwnedVal(ty.clone())),
            Type::Reference(ty) => {
                HspParamType::from_reference_ty(&ty.elem, ty.mutability.is_some())
            }
            Type::ImplTrait(_) => {
                Err(Error::new(ty.span(), "`impl Trait` parameters must be references."))
            }
            _ => Err(Error::new(ty.span(), "Unrecognized type in HSP function parameters.")),
        }
    }
}

fn types_for_func(func: &mut ItemFn) -> Result<Vec<HspParamType>, Error> {
    let mut err: Option<Error> = None;
    macro_rules! add_error {
        ($err:expr) => {
            match &mut err {
                Some(x) => x.combine($err),
                None => err = Some($err),
            }
        };
    }

    let mut args = Vec::new();
    for arg in &mut func.sig.inputs {
        match arg {
            FnArg::Typed(param) => {
                let mut is_ext_arg = false;

                let attrs: Vec<_> = param.attrs.iter().cloned().collect();
                param.attrs.clear();
                for attr in attrs {
                    if attr.path.is_ident("ext_data") && attr.tokens.is_empty() {
                        is_ext_arg = true;
                    } else {
                        param.attrs.push(attr);
                    }
                }

                match HspParamType::from_type(&param.ty) {
                    Ok(v) => {
                        if is_ext_arg {
                            match v {
                                HspParamType::RefVal(ty) => args.push(HspParamType::ExtData(ty)),
                                HspParamType::MutRefVal(ty) => {
                                    args.push(HspParamType::MutExtData(ty))
                                }
                                _ => add_error!(Error::new(
                                    arg.span(),
                                    "Parameters marked with `#[ext_data]` must be reference.",
                                )),
                            }
                        } else {
                            args.push(v)
                        }
                    }
                    Err(e) => add_error!(e),
                }
            }
            arg => {
                add_error!(Error::new(
                    arg.span(),
                    "Function arguments must be ordinary named parameters."
                ))
            }
        }
    }
    match err {
        Some(x) => Err(x),
        None => Ok(args),
    }
}

pub fn hsp_export(_: TokenStream, item: TokenStream) -> Result<TokenStream, Error> {
    let mut func = parse2::<ItemFn>(item)?;

    if func.sig.asyncness.is_some() {
        Err(Error::new(
            func.sig.asyncness.span(),
            "Cannot use #[hsp_export] with async functions.",
        ))
    } else if !func.sig.generics.params.is_empty() {
        Err(Error::new(
            func.sig.generics.span(),
            "Cannot use #[hsp_export] with generic functions.",
        ))
    } else if func.sig.variadic.is_some() {
        Err(Error::new(
            func.sig.variadic.span(),
            "Cannot use #[hsp_export] with varadic functions.",
        ))
    } else {
        let args = types_for_func(&mut func)?;
        println!("{:#?}", args);

        //Err(Error::new(func.span(), "Not yet implemented."))
        Ok(quote! {
            #func
        })
    }
}
