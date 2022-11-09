use rhsp3_internal_common::plugin::*;

const MARKER_STR: &str = "211b4f49-22e6-4ebf-bad3-caa2ba705e8f:";

pub fn make_hpi_headers<T: HspPlugin>(dylib_name: &str) -> String {
    let ident_name = format!("__rhsp3_{}", dylib_name.replace(".hpi", "").replace(".", "_"));

    let mut accum = String::new();
    accum.push_str(&format!("; Automatically generated bindings for {dylib_name}.\n"));
    accum.push_str(&format!("; Please do not edit manually.\n"));
    accum.push_str("\n");
    accum.push_str(&format!("#uselib \"{dylib_name}\"\n"));
    accum.push_str(&format!("#regcmd \"{}\",\"{dylib_name}\"\n", T::dylib_init_link_name()));

    let mut requires_strapi = false;

    let mut function_defs = Vec::new();
    for func in T::get_prototypes() {
        let mut params = Vec::new();
        let mut true_params = Vec::new();
        let mut shim_params = Vec::new();
        let mut shim_param_names = Vec::new();

        let mut requires_shim = false;
        let mut shim_locals = Vec::new();
        let mut shim_prel = Vec::new();
        let mut shim_epil = Vec::new();

        for (i, param) in func.params.iter().enumerate() {
            let shim_name = format!("{ident_name}_{i}");
            macro_rules! push_both {
                ($param_ty:literal, $shim_ty:literal) => {{
                    params.push($param_ty);
                    shim_params.push(format!(concat!($shim_ty, " {}"), shim_name));
                    shim_param_names.push(shim_name.clone());
                }};
            }
            match param {
                HspParamType::Int => push_both!("int", "int"),
                HspParamType::Var => push_both!("var", "var"),
                HspParamType::Str => push_both!("str", "str"),
                HspParamType::WStr => push_both!("wstr", "var"),
                HspParamType::SPtr => push_both!("sptr", "var"),
                HspParamType::WPtr => push_both!("wptr", "var"),
                HspParamType::Double => push_both!("double", "double"),
                HspParamType::Float => push_both!("float", "double"),
                HspParamType::PVal => push_both!("pval", "var"),
                HspParamType::ComObj => push_both!("comobj", "var"),
                HspParamType::BmScr => params.push("bmscr"),
                HspParamType::PRefStr => params.push("prefstr"),
                HspParamType::PExInfo => params.push("pexinfo"),
                HspParamType::NullPtr => params.push("nullptr"),
                HspParamType::VarAsPVal => {
                    push_both!("pval", "var");
                    requires_shim = true;
                }
                HspParamType::StrAsHandle => {
                    params.push("int");
                    shim_params.push(format!("var {shim_name}"));

                    requires_shim = true;
                    requires_strapi = true;

                    let local_str = format!("{ident_name}_{i}h");
                    let local_len = format!("{ident_name}_{i}l");
                    let local_out = format!("{ident_name}_{i}o");
                    shim_locals.push(format!("local {local_str}"));
                    shim_locals.push(format!("local {local_len}"));
                    shim_locals.push(format!("local {local_out}"));

                    shim_prel.push(format!("{ident_name}_make_strctx {local_str}, {shim_name}\n"));
                    shim_param_names.push(local_str.clone());
                    shim_epil.push(format!("{ident_name}_strctx_len {local_str}, {local_len}\n"));
                    shim_epil.push(format!("sdim {local_out}, {local_len}\n"));
                    shim_epil.push(format!("{ident_name}_strctx_out {local_str}, {local_out}\n"));
                    shim_epil.push(format!("{shim_name} = {local_out}\n"));
                }
            }
            true_params.push(format!("{:?}", param));
        }

        let native_name = if requires_shim {
            format!("{ident_name}__{}", func.name)
        } else {
            func.name.to_string()
        };
        let full_params = params.join(",");
        accum.push_str(&format!("#func {native_name} \"{}\" {full_params}\n", func.link_name));

        let func_info = format!("{}:{}:{}", func.name, func.link_name, true_params.join(","));
        function_defs.push(format!("\t{ident_name}_x = \"func:{func_info}\"\n"));
        if requires_shim {
            let mut shim = String::new();
            shim_params.extend(shim_locals);
            shim.push_str(&format!("\t#deffunc {} {}\n", func.name, shim_params.join(", ")));
            for prel in shim_prel {
                shim.push_str(&format!("\t\t{prel}"));
            }
            shim.push_str(&format!("\t\t{native_name} {}\n", shim_param_names.join(", ")));
            for epil in shim_epil {
                shim.push_str(&format!("\t\t{epil}"));
            }
            shim.push_str("\treturn\n");
            function_defs.push(shim);
        }
    }

    if requires_strapi {
        accum.push_str(&format!(
            "#func {ident_name}_make_strctx \"__rhsp3_plugsdk__dylib_make_strctx\" var,str\n\
             #func {ident_name}_strctx_len \"__rhsp3_plugsdk__dylib_strctx_len\" int,var\n\
             #func {ident_name}_strctx_out \"__rhsp3_plugsdk__dylib_strctx_output\" int,pval\n"
        ));
    }
    if !function_defs.is_empty() {
        accum.push_str(&format!("goto *{ident_name}_skip\n"));
        accum.push_str(&format!("\t{ident_name}_x = \"{MARKER_STR}init:v1\"\n"));
        for def in function_defs {
            accum.push_str(&def);
        }
        accum.push_str(&format!("\t{ident_name}_x = \"{MARKER_STR}end\"\n"));
        accum.push_str(&format!("*{ident_name}_skip\n"));
    }

    accum
}
