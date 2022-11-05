use rhsp3_internal_common::plugin::*;

pub fn make_hpi_headers<T: HspPlugin>(dylib_name: &str) -> String {
    let mut accum = String::new();

    accum.push_str(&format!("#uselib \"{dylib_name}\"\n"));
    accum.push_str(&format!("#regcmd \"{}\",\"{dylib_name}\"\n", T::dylib_init_link_name()));
    for func in T::get_prototypes() {
        let mut params = Vec::new();
        for param in func.params.iter() {
            match param {
                HspParamType::Int => params.push("int"),
                HspParamType::Var => params.push("var"),
                HspParamType::Str => params.push("str"),
                HspParamType::WStr => params.push("wstr"),
                HspParamType::SPtr => params.push("sptr"),
                HspParamType::WPtr => params.push("wptr"),
                HspParamType::Double => params.push("double"),
                HspParamType::Float => params.push("float"),
                HspParamType::PVal => params.push("pval"),
                HspParamType::ComObj => params.push("comobj"),
                HspParamType::BmScr => params.push("bmscr"),
                HspParamType::PRefStr => params.push("prefstr"),
                HspParamType::PExInfo => params.push("pexinfo"),
                HspParamType::NullPtr => params.push("nullptr"),
            }
        }

        let full_params = params.join(",");
        accum.push_str(&format!("#func {} \"{}\" {full_params}\n", func.name, func.link_name));
    }

    accum
}
