/* automatically generated by rust-bindgen 0.61.0 */

pub type PDAT = *mut ::std::os::raw::c_void;
pub type APTR = ::std::os::raw::c_int;
pub const CALCCODE_ADD: _bindgen_ty_1 = 0;
pub const CALCCODE_SUB: _bindgen_ty_1 = 1;
pub const CALCCODE_MUL: _bindgen_ty_1 = 2;
pub const CALCCODE_DIV: _bindgen_ty_1 = 3;
pub const CALCCODE_MOD: _bindgen_ty_1 = 4;
pub const CALCCODE_AND: _bindgen_ty_1 = 5;
pub const CALCCODE_OR: _bindgen_ty_1 = 6;
pub const CALCCODE_XOR: _bindgen_ty_1 = 7;
pub const CALCCODE_EQ: _bindgen_ty_1 = 8;
pub const CALCCODE_NE: _bindgen_ty_1 = 9;
pub const CALCCODE_GT: _bindgen_ty_1 = 10;
pub const CALCCODE_LT: _bindgen_ty_1 = 11;
pub const CALCCODE_GTEQ: _bindgen_ty_1 = 12;
pub const CALCCODE_LTEQ: _bindgen_ty_1 = 13;
pub const CALCCODE_RR: _bindgen_ty_1 = 14;
pub const CALCCODE_LR: _bindgen_ty_1 = 15;
pub const CALCCODE_MAX: _bindgen_ty_1 = 16;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct PVal {
    pub flag: ::std::os::raw::c_short,
    pub mode: ::std::os::raw::c_short,
    pub len: [::std::os::raw::c_int; 5usize],
    pub size: ::std::os::raw::c_int,
    pub pt: *mut ::std::os::raw::c_char,
    pub master: *mut ::std::os::raw::c_void,
    pub support: ::std::os::raw::c_ushort,
    pub arraycnt: ::std::os::raw::c_short,
    pub offset: ::std::os::raw::c_int,
    pub arraymul: ::std::os::raw::c_int,
}
impl Default for PVal {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct HspVarProc {
    pub flag: ::std::os::raw::c_short,
    pub aftertype: ::std::os::raw::c_short,
    pub version: ::std::os::raw::c_short,
    pub support: ::std::os::raw::c_ushort,
    pub basesize: ::std::os::raw::c_short,
    pub opt: ::std::os::raw::c_short,
    pub vartype_name: *mut ::std::os::raw::c_char,
    pub user: *mut ::std::os::raw::c_char,
    pub Cnv: ::std::option::Option<
        unsafe extern "C" fn(
            buffer: *const ::std::os::raw::c_void,
            flag: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub CnvCustom: ::std::option::Option<
        unsafe extern "C" fn(
            buffer: *const ::std::os::raw::c_void,
            flag: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetPtr: ::std::option::Option<unsafe extern "C" fn(pval: *mut PVal) -> *mut PDAT>,
    pub ArrayObjectRead: ::std::option::Option<
        unsafe extern "C" fn(
            pval: *mut PVal,
            mptype: *mut ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub ArrayObject: ::std::option::Option<unsafe extern "C" fn(pval: *mut PVal)>,
    pub ObjectWrite: ::std::option::Option<
        unsafe extern "C" fn(
            pval: *mut PVal,
            data: *mut ::std::os::raw::c_void,
            type_: ::std::os::raw::c_int,
        ),
    >,
    pub ObjectMethod: ::std::option::Option<unsafe extern "C" fn(pval: *mut PVal)>,
    pub Alloc: ::std::option::Option<unsafe extern "C" fn(pval: *mut PVal, pval2: *const PVal)>,
    pub Free: ::std::option::Option<unsafe extern "C" fn(pval: *mut PVal)>,
    pub GetSize:
        ::std::option::Option<unsafe extern "C" fn(pdat: *const PDAT) -> ::std::os::raw::c_int>,
    pub GetUsing:
        ::std::option::Option<unsafe extern "C" fn(pdat: *const PDAT) -> ::std::os::raw::c_int>,
    pub GetBlockSize: ::std::option::Option<
        unsafe extern "C" fn(
            pval: *mut PVal,
            pdat: *mut PDAT,
            size: *mut ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub AllocBlock: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PVal, pdat: *mut PDAT, size: ::std::os::raw::c_int),
    >,
    pub Set: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PVal, pdat: *mut PDAT, in_: *const ::std::os::raw::c_void),
    >,
    pub AddI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub SubI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub MulI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub DivI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub ModI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub AndI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub OrI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub XorI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub EqI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub NeI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub GtI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub LtI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub GtEqI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub LtEqI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub RrI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
    pub LrI: ::std::option::Option<
        unsafe extern "C" fn(pval: *mut PDAT, val: *const ::std::os::raw::c_void),
    >,
}
impl Default for HspVarProc {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type HSPVAR_COREFUNC = ::std::option::Option<unsafe extern "C" fn(arg1: *mut HspVarProc)>;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct FlexValue {
    pub type_: ::std::os::raw::c_short,
    pub myid: ::std::os::raw::c_short,
    pub customid: ::std::os::raw::c_short,
    pub clonetype: ::std::os::raw::c_short,
    pub size: ::std::os::raw::c_int,
    pub ptr: *mut ::std::os::raw::c_void,
}
impl Default for FlexValue {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const HSPERROR_HSPERR_NONE: HSPERROR = 0;
pub const HSPERROR_HSPERR_UNKNOWN_CODE: HSPERROR = 1;
pub const HSPERROR_HSPERR_SYNTAX: HSPERROR = 2;
pub const HSPERROR_HSPERR_ILLEGAL_FUNCTION: HSPERROR = 3;
pub const HSPERROR_HSPERR_WRONG_EXPRESSION: HSPERROR = 4;
pub const HSPERROR_HSPERR_NO_DEFAULT: HSPERROR = 5;
pub const HSPERROR_HSPERR_TYPE_MISMATCH: HSPERROR = 6;
pub const HSPERROR_HSPERR_ARRAY_OVERFLOW: HSPERROR = 7;
pub const HSPERROR_HSPERR_LABEL_REQUIRED: HSPERROR = 8;
pub const HSPERROR_HSPERR_TOO_MANY_NEST: HSPERROR = 9;
pub const HSPERROR_HSPERR_RETURN_WITHOUT_GOSUB: HSPERROR = 10;
pub const HSPERROR_HSPERR_LOOP_WITHOUT_REPEAT: HSPERROR = 11;
pub const HSPERROR_HSPERR_FILE_IO: HSPERROR = 12;
pub const HSPERROR_HSPERR_PICTURE_MISSING: HSPERROR = 13;
pub const HSPERROR_HSPERR_EXTERNAL_EXECUTE: HSPERROR = 14;
pub const HSPERROR_HSPERR_PRIORITY: HSPERROR = 15;
pub const HSPERROR_HSPERR_TOO_MANY_PARAMETERS: HSPERROR = 16;
pub const HSPERROR_HSPERR_TEMP_BUFFER_OVERFLOW: HSPERROR = 17;
pub const HSPERROR_HSPERR_WRONG_NAME: HSPERROR = 18;
pub const HSPERROR_HSPERR_DIVIDED_BY_ZERO: HSPERROR = 19;
pub const HSPERROR_HSPERR_BUFFER_OVERFLOW: HSPERROR = 20;
pub const HSPERROR_HSPERR_UNSUPPORTED_FUNCTION: HSPERROR = 21;
pub const HSPERROR_HSPERR_EXPRESSION_COMPLEX: HSPERROR = 22;
pub const HSPERROR_HSPERR_VARIABLE_REQUIRED: HSPERROR = 23;
pub const HSPERROR_HSPERR_INTEGER_REQUIRED: HSPERROR = 24;
pub const HSPERROR_HSPERR_BAD_ARRAY_EXPRESSION: HSPERROR = 25;
pub const HSPERROR_HSPERR_OUT_OF_MEMORY: HSPERROR = 26;
pub const HSPERROR_HSPERR_TYPE_INITALIZATION_FAILED: HSPERROR = 27;
pub const HSPERROR_HSPERR_NO_FUNCTION_PARAMETERS: HSPERROR = 28;
pub const HSPERROR_HSPERR_STACK_OVERFLOW: HSPERROR = 29;
pub const HSPERROR_HSPERR_INVALID_PARAMETER: HSPERROR = 30;
pub const HSPERROR_HSPERR_INVALID_ARRAYSTORE: HSPERROR = 31;
pub const HSPERROR_HSPERR_INVALID_FUNCPARAM: HSPERROR = 32;
pub const HSPERROR_HSPERR_WINDOW_OBJECT_FULL: HSPERROR = 33;
pub const HSPERROR_HSPERR_INVALID_ARRAY: HSPERROR = 34;
pub const HSPERROR_HSPERR_STRUCT_REQUIRED: HSPERROR = 35;
pub const HSPERROR_HSPERR_INVALID_STRUCT_SOURCE: HSPERROR = 36;
pub const HSPERROR_HSPERR_INVALID_TYPE: HSPERROR = 37;
pub const HSPERROR_HSPERR_DLL_ERROR: HSPERROR = 38;
pub const HSPERROR_HSPERR_COMDLL_ERROR: HSPERROR = 39;
pub const HSPERROR_HSPERR_NORETVAL: HSPERROR = 40;
pub const HSPERROR_HSPERR_FUNCTION_SYNTAX: HSPERROR = 41;
pub const HSPERROR_HSPERR_INVALID_CALLBACK: HSPERROR = 42;
pub const HSPERROR_HSPERR_INTJUMP: HSPERROR = 43;
pub const HSPERROR_HSPERR_EXITRUN: HSPERROR = 44;
pub const HSPERROR_HSPERR_MAX: HSPERROR = 45;
pub type HSPERROR = ::std::os::raw::c_uint;
pub const DEBUGINFO_GENERAL: _bindgen_ty_2 = 0;
pub const DEBUGINFO_VARNAME: _bindgen_ty_2 = 1;
pub const DEBUGINFO_INTINFO: _bindgen_ty_2 = 2;
pub const DEBUGINFO_GRINFO: _bindgen_ty_2 = 3;
pub const DEBUGINFO_MMINFO: _bindgen_ty_2 = 4;
pub const DEBUGINFO_MAX: _bindgen_ty_2 = 5;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
pub const HSPDEBUG_NONE: _bindgen_ty_3 = 0;
pub const HSPDEBUG_RUN: _bindgen_ty_3 = 1;
pub const HSPDEBUG_STOP: _bindgen_ty_3 = 2;
pub const HSPDEBUG_STEPIN: _bindgen_ty_3 = 3;
pub const HSPDEBUG_STEPOVER: _bindgen_ty_3 = 4;
pub const HSPDEBUG_MAX: _bindgen_ty_3 = 5;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct HSP3DEBUG {
    pub flag: ::std::os::raw::c_int,
    pub line: ::std::os::raw::c_int,
    pub fname: *mut ::std::os::raw::c_char,
    pub dbgwin: *mut ::std::os::raw::c_void,
    pub dbgval: *mut ::std::os::raw::c_char,
    pub hspctx: *mut HSPCTX,
    pub get_value: ::std::option::Option<
        unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char,
    >,
    pub get_varinf: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub dbg_close: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_char)>,
    pub dbg_curinf: ::std::option::Option<unsafe extern "C" fn()>,
    pub dbg_set: ::std::option::Option<
        unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub dbg_callstack: ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_char>,
}
impl Default for HSP3DEBUG {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct HSPHED {
    pub h1: ::std::os::raw::c_char,
    pub h2: ::std::os::raw::c_char,
    pub h3: ::std::os::raw::c_char,
    pub h4: ::std::os::raw::c_char,
    pub version: ::std::os::raw::c_int,
    pub max_val: ::std::os::raw::c_int,
    pub allsize: ::std::os::raw::c_int,
    pub pt_cs: ::std::os::raw::c_int,
    pub max_cs: ::std::os::raw::c_int,
    pub pt_ds: ::std::os::raw::c_int,
    pub max_ds: ::std::os::raw::c_int,
    pub pt_ot: ::std::os::raw::c_int,
    pub max_ot: ::std::os::raw::c_int,
    pub pt_dinfo: ::std::os::raw::c_int,
    pub max_dinfo: ::std::os::raw::c_int,
    pub pt_linfo: ::std::os::raw::c_int,
    pub max_linfo: ::std::os::raw::c_int,
    pub pt_finfo: ::std::os::raw::c_int,
    pub max_finfo: ::std::os::raw::c_int,
    pub pt_minfo: ::std::os::raw::c_int,
    pub max_minfo: ::std::os::raw::c_int,
    pub pt_finfo2: ::std::os::raw::c_int,
    pub max_finfo2: ::std::os::raw::c_int,
    pub pt_hpidat: ::std::os::raw::c_int,
    pub max_hpi: ::std::os::raw::c_short,
    pub max_varhpi: ::std::os::raw::c_short,
    pub bootoption: ::std::os::raw::c_int,
    pub runtime: ::std::os::raw::c_int,
    pub pt_sr: ::std::os::raw::c_int,
    pub max_sr: ::std::os::raw::c_int,
    pub pt_exopt: ::std::os::raw::c_int,
    pub max_exopt: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct MEM_HPIDAT {
    pub flag: ::std::os::raw::c_short,
    pub option: ::std::os::raw::c_short,
    pub libname: ::std::os::raw::c_int,
    pub funcname: ::std::os::raw::c_int,
    pub libptr: *mut ::std::os::raw::c_void,
}
impl Default for MEM_HPIDAT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type HPIDAT = MEM_HPIDAT;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct LIBDAT {
    pub flag: ::std::os::raw::c_int,
    pub nameidx: ::std::os::raw::c_int,
    pub hlib: *mut ::std::os::raw::c_void,
    pub clsid: ::std::os::raw::c_int,
}
impl Default for LIBDAT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type HED_LIBDAT = LIBDAT;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct STRUCTPRM {
    pub mptype: ::std::os::raw::c_short,
    pub subid: ::std::os::raw::c_short,
    pub offset: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct STRUCTDAT {
    pub index: ::std::os::raw::c_short,
    pub subid: ::std::os::raw::c_short,
    pub prmindex: ::std::os::raw::c_int,
    pub prmmax: ::std::os::raw::c_int,
    pub nameidx: ::std::os::raw::c_int,
    pub size: ::std::os::raw::c_int,
    pub otindex: ::std::os::raw::c_int,
    pub __bindgen_anon_1: STRUCTDAT__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union STRUCTDAT__bindgen_ty_1 {
    pub proc_: *mut ::std::os::raw::c_void,
    pub funcflag: ::std::os::raw::c_int,
}
impl Default for STRUCTDAT__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for STRUCTDAT__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "STRUCTDAT__bindgen_ty_1 {{ union }}")
    }
}
impl Default for STRUCTDAT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for STRUCTDAT {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! (f , "STRUCTDAT {{ index: {:?}, subid: {:?}, prmindex: {:?}, prmmax: {:?}, nameidx: {:?}, size: {:?}, otindex: {:?}, __bindgen_anon_1: {:?} }}" , self . index , self . subid , self . prmindex , self . prmmax , self . nameidx , self . size , self . otindex , self . __bindgen_anon_1)
    }
}
pub type HED_STRUCTDAT = STRUCTDAT;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct MPVarData {
    pub pval: *mut PVal,
    pub aptr: APTR,
}
impl Default for MPVarData {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct MPModVarData {
    pub subid: ::std::os::raw::c_short,
    pub magic: ::std::os::raw::c_short,
    pub pval: *mut PVal,
    pub aptr: APTR,
}
impl Default for MPModVarData {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct IRQDAT {
    pub flag: ::std::os::raw::c_short,
    pub opt: ::std::os::raw::c_short,
    pub custom: ::std::os::raw::c_int,
    pub custom2: ::std::os::raw::c_int,
    pub iparam: ::std::os::raw::c_int,
    pub ptr: *mut ::std::os::raw::c_ushort,
    pub callback: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut IRQDAT,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >,
}
impl Default for IRQDAT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct HSPEXINFO30 {
    pub ver: ::std::os::raw::c_short,
    pub min: ::std::os::raw::c_short,
    pub er: *mut ::std::os::raw::c_int,
    pub pstr: *mut ::std::os::raw::c_char,
    pub stmp: *mut ::std::os::raw::c_char,
    pub mpval: *mut *mut PVal,
    pub actscr: *mut ::std::os::raw::c_int,
    pub nptype: *mut ::std::os::raw::c_int,
    pub npval: *mut ::std::os::raw::c_int,
    pub strsize: *mut ::std::os::raw::c_int,
    pub refstr: *mut ::std::os::raw::c_char,
    pub HspFunc_prm_getv:
        ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_void>,
    pub HspFunc_prm_geti: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    pub HspFunc_prm_getdi: ::std::option::Option<
        unsafe extern "C" fn(defval: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_prm_gets:
        ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_char>,
    pub HspFunc_prm_getds: ::std::option::Option<
        unsafe extern "C" fn(defstr: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
    >,
    pub HspFunc_val_realloc: ::std::option::Option<
        unsafe extern "C" fn(
            pv: *mut PVal,
            size: ::std::os::raw::c_int,
            mode: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_fread: ::std::option::Option<
        unsafe extern "C" fn(
            fname: *mut ::std::os::raw::c_char,
            readmem: *mut ::std::os::raw::c_void,
            rlen: ::std::os::raw::c_int,
            seekofs: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_fsize: ::std::option::Option<
        unsafe extern "C" fn(fname: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_getbmscr: ::std::option::Option<
        unsafe extern "C" fn(wid: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void,
    >,
    pub HspFunc_getobj: ::std::option::Option<
        unsafe extern "C" fn(
            wid: ::std::os::raw::c_int,
            id: ::std::os::raw::c_int,
            inf: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_setobj: ::std::option::Option<
        unsafe extern "C" fn(
            wid: ::std::os::raw::c_int,
            id: ::std::os::raw::c_int,
            inf: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub npexflg: *mut ::std::os::raw::c_int,
    pub hspctx: *mut HSPCTX,
    pub HspFunc_addobj: ::std::option::Option<
        unsafe extern "C" fn(wid: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_puterror: ::std::option::Option<unsafe extern "C" fn(error: HSPERROR)>,
    pub HspFunc_getproc: ::std::option::Option<
        unsafe extern "C" fn(type_: ::std::os::raw::c_int) -> *mut HspVarProc,
    >,
    pub HspFunc_seekproc: ::std::option::Option<
        unsafe extern "C" fn(name: *const ::std::os::raw::c_char) -> *mut HspVarProc,
    >,
    pub HspFunc_prm_next: ::std::option::Option<unsafe extern "C" fn()>,
    pub HspFunc_prm_get: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    pub HspFunc_prm_getd: ::std::option::Option<unsafe extern "C" fn() -> f64>,
    pub HspFunc_prm_getdd: ::std::option::Option<unsafe extern "C" fn(defval: f64) -> f64>,
    pub HspFunc_prm_getlb:
        ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_ushort>,
    pub HspFunc_prm_getpval: ::std::option::Option<unsafe extern "C" fn() -> *mut PVal>,
    pub HspFunc_prm_getva:
        ::std::option::Option<unsafe extern "C" fn(pval: *mut *mut PVal) -> APTR>,
    pub HspFunc_prm_setva: ::std::option::Option<
        unsafe extern "C" fn(
            pval: *mut PVal,
            aptr: APTR,
            type_: ::std::os::raw::c_int,
            ptr: *const ::std::os::raw::c_void,
        ),
    >,
    pub HspFunc_malloc: ::std::option::Option<
        unsafe extern "C" fn(size: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char,
    >,
    pub HspFunc_free: ::std::option::Option<unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_void)>,
    pub HspFunc_expand: ::std::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::std::os::raw::c_char,
            size: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub HspFunc_addirq: ::std::option::Option<unsafe extern "C" fn() -> *mut IRQDAT>,
    pub HspFunc_hspevent: ::std::option::Option<
        unsafe extern "C" fn(
            event: ::std::os::raw::c_int,
            prm1: ::std::os::raw::c_int,
            prm2: ::std::os::raw::c_int,
            prm3: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_registvar: ::std::option::Option<
        unsafe extern "C" fn(flag: ::std::os::raw::c_int, func: HSPVAR_COREFUNC),
    >,
    pub HspFunc_setpc:
        ::std::option::Option<unsafe extern "C" fn(pc: *const ::std::os::raw::c_ushort)>,
    pub HspFunc_call:
        ::std::option::Option<unsafe extern "C" fn(pc: *const ::std::os::raw::c_ushort)>,
    pub HspFunc_mref:
        ::std::option::Option<unsafe extern "C" fn(pval: *mut PVal, prm: ::std::os::raw::c_int)>,
    pub HspFunc_dim: ::std::option::Option<
        unsafe extern "C" fn(
            pval: *mut PVal,
            flag: ::std::os::raw::c_int,
            len0: ::std::os::raw::c_int,
            len1: ::std::os::raw::c_int,
            len2: ::std::os::raw::c_int,
            len3: ::std::os::raw::c_int,
            len4: ::std::os::raw::c_int,
        ),
    >,
    pub HspFunc_redim: ::std::option::Option<
        unsafe extern "C" fn(
            pval: *mut PVal,
            lenid: ::std::os::raw::c_int,
            len: ::std::os::raw::c_int,
        ),
    >,
    pub HspFunc_array:
        ::std::option::Option<unsafe extern "C" fn(pval: *mut PVal, offset: ::std::os::raw::c_int)>,
}
impl Default for HSPEXINFO30 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct HSPEXINFO {
    pub ver: ::std::os::raw::c_short,
    pub min: ::std::os::raw::c_short,
    pub er: *mut ::std::os::raw::c_int,
    pub pstr: *mut ::std::os::raw::c_char,
    pub stmp: *mut ::std::os::raw::c_char,
    pub mpval: *mut *mut PVal,
    pub actscr: *mut ::std::os::raw::c_int,
    pub nptype: *mut ::std::os::raw::c_int,
    pub npval: *mut ::std::os::raw::c_int,
    pub strsize: *mut ::std::os::raw::c_int,
    pub refstr: *mut ::std::os::raw::c_char,
    pub HspFunc_prm_getv:
        ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_void>,
    pub HspFunc_prm_geti: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    pub HspFunc_prm_getdi: ::std::option::Option<
        unsafe extern "C" fn(defval: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_prm_gets:
        ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_char>,
    pub HspFunc_prm_getds: ::std::option::Option<
        unsafe extern "C" fn(defstr: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
    >,
    pub HspFunc_val_realloc: ::std::option::Option<
        unsafe extern "C" fn(
            pv: *mut PVal,
            size: ::std::os::raw::c_int,
            mode: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_fread: ::std::option::Option<
        unsafe extern "C" fn(
            fname: *mut ::std::os::raw::c_char,
            readmem: *mut ::std::os::raw::c_void,
            rlen: ::std::os::raw::c_int,
            seekofs: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_fsize: ::std::option::Option<
        unsafe extern "C" fn(fname: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_getbmscr: ::std::option::Option<
        unsafe extern "C" fn(wid: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void,
    >,
    pub HspFunc_getobj: ::std::option::Option<
        unsafe extern "C" fn(
            wid: ::std::os::raw::c_int,
            id: ::std::os::raw::c_int,
            inf: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_setobj: ::std::option::Option<
        unsafe extern "C" fn(
            wid: ::std::os::raw::c_int,
            id: ::std::os::raw::c_int,
            inf: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub npexflg: *mut ::std::os::raw::c_int,
    pub hspctx: *mut HSPCTX,
    pub HspFunc_addobj: ::std::option::Option<
        unsafe extern "C" fn(wid: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_puterror: ::std::option::Option<unsafe extern "C" fn(error: HSPERROR)>,
    pub HspFunc_getproc: ::std::option::Option<
        unsafe extern "C" fn(type_: ::std::os::raw::c_int) -> *mut HspVarProc,
    >,
    pub HspFunc_seekproc: ::std::option::Option<
        unsafe extern "C" fn(name: *const ::std::os::raw::c_char) -> *mut HspVarProc,
    >,
    pub HspFunc_prm_next: ::std::option::Option<unsafe extern "C" fn()>,
    pub HspFunc_prm_get: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    pub HspFunc_prm_getd: ::std::option::Option<unsafe extern "C" fn() -> f64>,
    pub HspFunc_prm_getdd: ::std::option::Option<unsafe extern "C" fn(defval: f64) -> f64>,
    pub HspFunc_prm_getlb:
        ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_ushort>,
    pub HspFunc_prm_getpval: ::std::option::Option<unsafe extern "C" fn() -> *mut PVal>,
    pub HspFunc_prm_getva:
        ::std::option::Option<unsafe extern "C" fn(pval: *mut *mut PVal) -> APTR>,
    pub HspFunc_prm_setva: ::std::option::Option<
        unsafe extern "C" fn(
            pval: *mut PVal,
            aptr: APTR,
            type_: ::std::os::raw::c_int,
            ptr: *const ::std::os::raw::c_void,
        ),
    >,
    pub HspFunc_malloc: ::std::option::Option<
        unsafe extern "C" fn(size: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char,
    >,
    pub HspFunc_free: ::std::option::Option<unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_void)>,
    pub HspFunc_expand: ::std::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::std::os::raw::c_char,
            size: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub HspFunc_addirq: ::std::option::Option<unsafe extern "C" fn() -> *mut IRQDAT>,
    pub HspFunc_hspevent: ::std::option::Option<
        unsafe extern "C" fn(
            event: ::std::os::raw::c_int,
            prm1: ::std::os::raw::c_int,
            prm2: ::std::os::raw::c_int,
            prm3: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_registvar: ::std::option::Option<
        unsafe extern "C" fn(flag: ::std::os::raw::c_int, func: HSPVAR_COREFUNC),
    >,
    pub HspFunc_setpc:
        ::std::option::Option<unsafe extern "C" fn(pc: *const ::std::os::raw::c_ushort)>,
    pub HspFunc_call:
        ::std::option::Option<unsafe extern "C" fn(pc: *const ::std::os::raw::c_ushort)>,
    pub HspFunc_mref:
        ::std::option::Option<unsafe extern "C" fn(pval: *mut PVal, prm: ::std::os::raw::c_int)>,
    pub HspFunc_dim: ::std::option::Option<
        unsafe extern "C" fn(
            pval: *mut PVal,
            flag: ::std::os::raw::c_int,
            len0: ::std::os::raw::c_int,
            len1: ::std::os::raw::c_int,
            len2: ::std::os::raw::c_int,
            len3: ::std::os::raw::c_int,
            len4: ::std::os::raw::c_int,
        ),
    >,
    pub HspFunc_redim: ::std::option::Option<
        unsafe extern "C" fn(
            pval: *mut PVal,
            lenid: ::std::os::raw::c_int,
            len: ::std::os::raw::c_int,
        ),
    >,
    pub HspFunc_array:
        ::std::option::Option<unsafe extern "C" fn(pval: *mut PVal, offset: ::std::os::raw::c_int)>,
    pub HspFunc_varname: ::std::option::Option<
        unsafe extern "C" fn(id: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char,
    >,
    pub HspFunc_seekvar: ::std::option::Option<
        unsafe extern "C" fn(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub HspFunc_prm_getns:
        ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_char>,
    pub HspFunc_prm_getnds: ::std::option::Option<
        unsafe extern "C" fn(defstr: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
    >,
}
impl Default for HSPEXINFO {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct LOOPDAT {
    pub time: ::std::os::raw::c_int,
    pub cnt: ::std::os::raw::c_int,
    pub step: ::std::os::raw::c_int,
    pub pt: *mut ::std::os::raw::c_ushort,
}
impl Default for LOOPDAT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const RUNMODE_RUN: _bindgen_ty_4 = 0;
pub const RUNMODE_WAIT: _bindgen_ty_4 = 1;
pub const RUNMODE_AWAIT: _bindgen_ty_4 = 2;
pub const RUNMODE_STOP: _bindgen_ty_4 = 3;
pub const RUNMODE_END: _bindgen_ty_4 = 4;
pub const RUNMODE_ERROR: _bindgen_ty_4 = 5;
pub const RUNMODE_RETURN: _bindgen_ty_4 = 6;
pub const RUNMODE_INTJUMP: _bindgen_ty_4 = 7;
pub const RUNMODE_ASSERT: _bindgen_ty_4 = 8;
pub const RUNMODE_LOGMES: _bindgen_ty_4 = 9;
pub const RUNMODE_EXITRUN: _bindgen_ty_4 = 10;
pub const RUNMODE_RESTART: _bindgen_ty_4 = 11;
pub const RUNMODE_MAX: _bindgen_ty_4 = 12;
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct HSPCTX {
    pub hsphed: *mut HSPHED,
    pub mcs: *mut ::std::os::raw::c_ushort,
    pub mem_mcs: *mut ::std::os::raw::c_ushort,
    pub mem_mds: *mut ::std::os::raw::c_char,
    pub mem_di: *mut ::std::os::raw::c_uchar,
    pub mem_ot: *mut ::std::os::raw::c_int,
    pub mem_irq: *mut IRQDAT,
    pub irqmax: ::std::os::raw::c_int,
    pub iparam: ::std::os::raw::c_int,
    pub wparam: ::std::os::raw::c_int,
    pub lparam: ::std::os::raw::c_int,
    pub mem_var: *mut PVal,
    pub exinfo: HSPEXINFO30,
    pub runmode: ::std::os::raw::c_int,
    pub waitcount: ::std::os::raw::c_int,
    pub waitbase: ::std::os::raw::c_int,
    pub waittick: ::std::os::raw::c_int,
    pub lasttick: ::std::os::raw::c_int,
    pub sublev: ::std::os::raw::c_int,
    pub mem_loop: [LOOPDAT; 32usize],
    pub looplev: ::std::os::raw::c_int,
    pub err: HSPERROR,
    pub hspstat: ::std::os::raw::c_int,
    pub stat: ::std::os::raw::c_int,
    pub strsize: ::std::os::raw::c_int,
    pub refstr: *mut ::std::os::raw::c_char,
    pub fnbuffer: *mut ::std::os::raw::c_char,
    pub instance: *mut ::std::os::raw::c_void,
    pub intwnd_id: ::std::os::raw::c_int,
    pub note_pval: *mut PVal,
    pub note_aptr: APTR,
    pub notep_pval: *mut PVal,
    pub notep_aptr: APTR,
    pub stmp: *mut ::std::os::raw::c_char,
    pub prmstack: *mut ::std::os::raw::c_void,
    pub mem_linfo: *mut LIBDAT,
    pub mem_minfo: *mut STRUCTPRM,
    pub mem_finfo: *mut STRUCTDAT,
    pub retval_level: ::std::os::raw::c_int,
    pub endcode: ::std::os::raw::c_int,
    pub msgfunc: ::std::option::Option<unsafe extern "C" fn(arg1: *mut HSPCTX)>,
    pub wnd_parent: *mut ::std::os::raw::c_void,
    pub refdval: f64,
    pub cmdline: *mut ::std::os::raw::c_char,
    pub exinfo2: *mut HSPEXINFO,
    pub prmstack_max: ::std::os::raw::c_int,
    pub dsindex: *mut ::std::os::raw::c_int,
    pub dsindex_size: ::std::os::raw::c_int,
    pub language: ::std::os::raw::c_int,
    pub callback_flag: ::std::os::raw::c_int,
    pub modfilename: *mut ::std::os::raw::c_char,
    pub tvfoldername: *mut ::std::os::raw::c_char,
    pub homefoldername: *mut ::std::os::raw::c_char,
    pub langcode: [::std::os::raw::c_char; 4usize],
}
impl Default for HSPCTX {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct HSPROUTINE {
    pub stacklev: ::std::os::raw::c_int,
    pub mcsret: *mut ::std::os::raw::c_ushort,
    pub param: *mut STRUCTDAT,
    pub oldtack: *mut ::std::os::raw::c_void,
    pub oldlev: ::std::os::raw::c_int,
}
impl Default for HSPROUTINE {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type HSP3_CMDFUNC = ::std::option::Option<
    unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
>;
pub type HSP3_REFFUNC = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type HSP3_TERMFUNC = ::std::option::Option<
    unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
>;
pub type HSP3_MSGFUNC = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type HSP3_EVENTFUNC = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct HSP3TYPEINFO {
    pub type_: ::std::os::raw::c_short,
    pub option: ::std::os::raw::c_short,
    pub hspctx: *mut HSPCTX,
    pub hspexinfo: *mut HSPEXINFO,
    pub cmdfunc: ::std::option::Option<
        unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub reffunc: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_int,
            arg2: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub termfunc: ::std::option::Option<
        unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub msgfunc: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub eventfunc: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
}
impl Default for HSP3TYPEINFO {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const HSPIRQ_ONEXIT: _bindgen_ty_5 = 0;
pub const HSPIRQ_ONERROR: _bindgen_ty_5 = 1;
pub const HSPIRQ_ONKEY: _bindgen_ty_5 = 2;
pub const HSPIRQ_ONCLICK: _bindgen_ty_5 = 3;
pub const HSPIRQ_USERDEF: _bindgen_ty_5 = 4;
pub const HSPIRQ_MAX: _bindgen_ty_5 = 5;
pub type _bindgen_ty_5 = ::std::os::raw::c_uint;
pub const HSPEVENT_NONE: _bindgen_ty_6 = 0;
pub const HSPEVENT_COMMAND: _bindgen_ty_6 = 1;
pub const HSPEVENT_HSPIRQ: _bindgen_ty_6 = 2;
pub const HSPEVENT_GETKEY: _bindgen_ty_6 = 3;
pub const HSPEVENT_STICK: _bindgen_ty_6 = 4;
pub const HSPEVENT_FNAME: _bindgen_ty_6 = 5;
pub const HSPEVENT_FREAD: _bindgen_ty_6 = 6;
pub const HSPEVENT_FWRITE: _bindgen_ty_6 = 7;
pub const HSPEVENT_FEXIST: _bindgen_ty_6 = 8;
pub const HSPEVENT_FDELETE: _bindgen_ty_6 = 9;
pub const HSPEVENT_FMKDIR: _bindgen_ty_6 = 10;
pub const HSPEVENT_FCHDIR: _bindgen_ty_6 = 11;
pub const HSPEVENT_FCOPY: _bindgen_ty_6 = 12;
pub const HSPEVENT_FDIRLIST1: _bindgen_ty_6 = 13;
pub const HSPEVENT_FDIRLIST2: _bindgen_ty_6 = 14;
pub const HSPEVENT_GETPICSIZE: _bindgen_ty_6 = 15;
pub const HSPEVENT_PICLOAD: _bindgen_ty_6 = 16;
pub const HSPEVENT_MAX: _bindgen_ty_6 = 17;
pub type _bindgen_ty_6 = ::std::os::raw::c_uint;
