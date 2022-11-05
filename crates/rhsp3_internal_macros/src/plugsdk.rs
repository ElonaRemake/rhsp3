use crate::utils::{crate_root, get_ident_id, get_registration_id};
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
    RefVal(Type, bool /* is_mut */),
    ExtData(Type, bool /* is_mut */),
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
            Type::Path(_) => Ok(HspParamType::RefVal(ty.clone(), is_mut)),
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
    let mut impl_context_count = 0;
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
                        if let HspParamType::ImplContext = v {
                            impl_context_count += 1;
                        }

                        if is_ext_arg {
                            match v {
                                HspParamType::RefVal(ty, is_mut) => {
                                    args.push(HspParamType::ExtData(ty, is_mut))
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
    if impl_context_count > 1 {
        add_error!(Error::new(
            func.sig.inputs.span(),
            "Cannot have more than one `HspContext` parameter."
        ));
    }
    match err {
        Some(x) => Err(x),
        None => Ok(args),
    }
}

#[cfg(feature = "plugsdk_cdylib")]
fn make_dylib_shim(
    root: &TokenStream,
    item: &ItemFn,
    args: &[HspParamType],
) -> Result<TokenStream, Error> {
    let mut param_names = Vec::new();
    let mut param_types = Vec::new();
    let mut param_args = Vec::new();
    let mut simple_load = Vec::new();
    let mut load_ref = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        match arg {
            HspParamType::ImplContext => {
                param_args.push(quote! { &mut _hsp3storedctx.context });
            }
            HspParamType::ImplVar(ty) => {
                let in_ident = ident!("_hsp3rawparam_{i}");
                let out_ident = ident!("_hsp3var_{i}");
                param_names.push(quote! { #in_ident });
                param_types.push(quote! { <#ty as #root::VarTypeOwnedCdylib>::HspPointerParam });
                simple_load.push(quote! {
                    let mut #out_ident = #root::dylib::DylibVar::new(#in_ident)?;
                });
                param_args.push(quote! { &mut #out_ident });
            }
            HspParamType::OwnedVal(ty) => {
                let in_ident = ident!("_hsp3rawparam_{i}");
                let out_ident = ident!("_hsp3owned_{i}");
                param_names.push(quote! { #in_ident });
                param_types.push(quote! { <#ty as #root::VarTypeSealed>::HspParam });
                simple_load.push(quote! {
                    let #out_ident =
                        <#ty as #root::VarTypeOwnedSealed>::from_hsp_param(#in_ident)?;
                });
                param_args.push(quote! { #out_ident });
            }
            HspParamType::RefVal(ty, is_mut) => {
                let in_ident = ident!("_hsp3rawparam_{i}");
                let out_ident = ident!("_hsp3ref_{i}");
                param_names.push(quote! { #in_ident });
                param_types.push(quote! { <#ty as #root::VarTypeSealed>::HspParam });
                load_ref.push((in_ident, out_ident.clone(), ty.clone(), *is_mut));
                param_args.push(quote! { #out_ident });
            }
            HspParamType::ExtData(ty, _) => {
                let lock_ident = ident!("_hsp3extdatalock_{i}");
                let out_ident = ident!("_hsp3extdata_{i}");
                simple_load.push(quote! {
                    let mut #lock_ident = _hsp3storedctx.get_ext_data::<#ty>()?;
                    let mut #out_ident = #lock_ident.borrow_mut();
                });
                param_args.push(quote! { &mut *#out_ident });
            }
        }
    }

    let item_name = &item.sig.ident;
    let mut core_func = quote! {
        #root::HspReturnTy::into_result(#item_name(#(#param_args,)*))
    };
    for (in_ident, out_ident, ty, is_mut) in load_ref {
        let func = if !is_mut {
            ident!("from_hsp_param_ref")
        } else {
            ident!("from_hsp_param_mut")
        };
        core_func = quote! {
            <#ty as #root::VarTypeSealed>::#func(#in_ident, |#out_ident| #core_func)
        }
    }

    let item_link_name = format!("__rhsp3_dylib_fn__{}", item.sig.ident.to_string());

    let id = get_ident_id();
    let shim_impl_name = ident!("__rhsp3_dylib_shim_impl_{id}");
    let shim_name = ident!("__rhsp3_dylib_shim_{id}");
    let inner_shim_name = ident!("__rhsp3_dylib_inner_shim_{id}");
    Ok(quote! {
        unsafe fn #shim_impl_name(#(#param_names: #param_types,)*) -> i32 {
            unsafe fn #inner_shim_name(
                _hsp3storedctx: &mut #root::dylib::DylibContext,
                #(#param_names: #param_types,)*
            ) -> #root::Result<i32> {
                #(#simple_load)*
                #core_func
            }
            #root::dylib::check_error(|| {
                #root::dylib::with_active_ctx(|_hsp3storedctx| {
                    #inner_shim_name(_hsp3storedctx, #(#param_names,)*)
                })
            })
        }

        #[cfg(windows)]
        #[export_name = #item_link_name]
        pub unsafe extern "stdcall-unwind" fn #shim_name(
            #(#param_names: #param_types,)*
        ) -> i32 {
            #shim_impl_name(#(#param_names,)*)
        }

        #[cfg(not(windows))]
        #[export_name = #item_link_name]
        pub unsafe extern "C-unwind" fn #shim_name(
            #(#param_names: #param_types,)*
        ) -> i32 {
            #shim_impl_name(#(#param_names,)*)
        }

        #[cfg(windows)]
        type FuncPtr = unsafe extern "stdcall-unwind" fn(#(#param_types,)*) -> i32;

        #[cfg(not(windows))]
        type FuncPtr = unsafe extern "C-unwind" fn(#(#param_types,)*) -> i32;
    })
}

fn register_prototypes(
    root: &TokenStream,
    item: &ItemFn,
    args: &[HspParamType],
) -> Result<TokenStream, Error> {
    let plugin = quote! { #root::reexport::rhsp3_internal_common::plugin };

    let mut args_ast = Vec::new();
    for arg in args {
        match arg {
            HspParamType::ImplContext => {}
            HspParamType::ImplVar(ty) => {
                args_ast.push(quote! { <#ty as #root::VarTypeOwnedSealed>::VAR_PARAM_TYPE });
            }
            HspParamType::OwnedVal(ty) | HspParamType::RefVal(ty, _) => {
                args_ast.push(quote! { <#ty as #root::VarTypeSealed>::PARAM_TYPE });
            }
            HspParamType::ExtData(_, _) => {}
        }
    }
    if args_ast.len() > 4 {
        return Err(Error::new(
            item.sig.inputs.span(),
            "Functions cannot take more than 4 parameters exposed to HSP.",
        ));
    }

    let reg_id = get_registration_id();
    let item_name = item.sig.ident.to_string();
    let item_link_name = format!("__rhsp3_dylib_fn__{}", item.sig.ident.to_string());
    Ok(quote! {
        impl<'a> #root::registration::Registration<#reg_id>
            for crate::__rhsp3_root::GatherPrototypes<'a>
        {
            #[inline(always)]
            fn run_chain(&self) {
                self.0.borrow_mut().push(#plugin::HspFunctionPrototype {
                    name: #item_name.into(),
                    link_name: #item_link_name.into(),
                    params: (&[#(#args_ast,)*] as &[#plugin::HspParamType]).into(),
                });

                use #root::registration::{DerefRampChainA, DerefRampChainB};
                let helper = #root::registration::DerefRamp::<{ #reg_id + 1 }, _>(self);
                (&helper).run_chain();
            }
        }
    })
}

pub fn hsp_export(attr: TokenStream, item: TokenStream) -> Result<TokenStream, Error> {
    let mut func = parse2::<ItemFn>(item)?;
    let root = crate_root(attr.span(), "rhsp3_plugsdk")?;

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

        #[cfg(feature = "plugsdk_cdylib")]
        let dll_shim = make_dylib_shim(&root, &func, &args)?;
        #[cfg(not(feature = "plugsdk_cdylib"))]
        let dll_shim = quote! {};

        let register_prototypes = register_prototypes(&root, &func, &args)?;

        //Err(Error::new(func.span(), "Not yet implemented."))
        Ok(quote! {
            #func

            #[allow(deprecated)]
            const _: () = {
                #dll_shim
                #register_prototypes
                ()
            };
        })
    }
}
