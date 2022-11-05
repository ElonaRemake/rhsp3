use std::borrow::Cow;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum HspParamType {
    Int,
    Var,
    Str,
    WStr,
    SPtr,
    WPtr,
    Double,
    Float,
    PVal,
    ComObj,
    BmScr,
    PRefStr,
    PExInfo,
    NullPtr,
}

pub struct HspFunctionPrototype {
    pub name: Cow<'static, str>,
    pub link_name: Cow<'static, str>,
    pub params: Cow<'static, [HspParamType]>,
}

pub trait HspPluginSealed {
    fn get_prototypes() -> Vec<HspFunctionPrototype>;
    fn dylib_init_link_name() -> &'static str;
}

/// A trait for types that can be used as a plugin for rhsp3.
///
/// For custom plugins, this must be implemented using the `rhsp3_plugsdk` crate rather than
/// created manually.
pub trait HspPlugin: HspPluginSealed {}
