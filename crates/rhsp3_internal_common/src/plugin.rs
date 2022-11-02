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

pub trait HspPluginSealed {}

/// A trait for types that can be used as a plugin for rhsp3.
///
/// For custom plugins, this must be implemented using the `rhsp3_plugsdk` crate rather than
/// created manually.
pub trait HspPlugin: HspPluginSealed {}
