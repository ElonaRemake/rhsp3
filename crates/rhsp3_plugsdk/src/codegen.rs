use rhsp3_internal_common::plugin::*;

pub fn make_hpi_headers<T: HspPlugin>(dylib_name: &str) -> String {
    let ident_name = format!("_rhsp3_{}", dylib_name.replace(".", "_"));

    let mut accum = String::new();
    accum.push_str(&format!("; Automatically generated bindings for {dylib_name}.\n"));
    accum.push_str(&format!("; Please do not edit manually.\n"));
    accum.push_str("\n");
    accum.push_str(&format!("#uselib \"{dylib_name}\"\n"));
    accum.push_str(&format!("#regcmd \"{}\",\"{dylib_name}\"\n", T::dylib_init_link_name()));

    let mut function_defs = Vec::new();
    for func in T::get_prototypes() {
        let mut params = Vec::new();
        let mut shim_params = Vec::new();
        let mut shim_param_names = Vec::new();

        let mut requires_shim = false;
        for (i, param) in func.params.iter().enumerate() {
            let shim_name = format!("{ident_name}_{i}");
            macro_rules! push_both {
                ($param_ty:literal, $shim_ty:literal) => {{
                    params.push($param_ty);
                    shim_params.push(format!(concat!($shim_ty, " {}"), shim_name));
                    shim_param_names.push(shim_name);
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
            }
        }

        let native_name = if requires_shim {
            format!("{ident_name}__{}", func.name)
        } else {
            func.name.to_string()
        };
        let full_params = params.join(",");
        accum.push_str(&format!("#func {native_name} \"{}\" {full_params}\n", func.link_name));

        if requires_shim {
            let mut shim = String::new();
            shim.push_str(&format!("#deffunc {} {}\n", func.name, shim_params.join(", ")));
            shim.push_str(&format!("{native_name} {}\n", shim_param_names.join(", ")));
            shim.push_str("return\n");
            function_defs.push(shim);
        }
    }

    if !function_defs.is_empty() {
        accum.push_str(&format!("goto *{ident_name}_skip_defs\n"));
        for def in function_defs {
            accum.push_str(&def);
        }
        accum.push_str(&format!("*{ident_name}_skip_defs\n"));
    }

    accum
}
