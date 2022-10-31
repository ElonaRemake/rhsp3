use rhsp3_internal_abi::hsp3struct;

macro_rules! gen_error_type {
    ($($(#[$meta:meta])* $variant:ident $hsp_ty:ident,)*) => {
        /// Represents an HSP error code.
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        #[non_exhaustive]
        pub enum ErrorCode {
            $($(#[$meta])* $variant,)*
            /// Used to represent unknown error codes, when they are received from HSP.
            Unknown,
        }

        /// Converts an rhsp3 error code to an HSP error code.
        pub fn to_hsp_error(error: ErrorCode) -> hsp3struct::HSPERROR {
            match error {
                $(ErrorCode::$variant => hsp3struct::$hsp_ty,)*
                _ => hsp3struct::HSPERROR_HSPERR_UNKNOWN_CODE,
            }
        }

        /// Converts an HSP error code to an rhsp3 error code.
        pub fn from_hsp_error(error: hsp3struct::HSPERROR) -> ErrorCode {
            match error {
                $(hsp3struct::$hsp_ty => ErrorCode::$variant,)*
                _ => ErrorCode::Unknown,
            }
        }

    };
}
gen_error_type!(
    GenericError HSPERROR_HSPERR_UNKNOWN_CODE,
    SyntaxError HSPERROR_HSPERR_SYNTAX,
    IllegalParameter HSPERROR_HSPERR_ILLEGAL_FUNCTION,
    SyntaxInvalidExpression HSPERROR_HSPERR_WRONG_EXPRESSION,
    RequiredParameterMissing HSPERROR_HSPERR_NO_DEFAULT,
    TypeMismatch HSPERROR_HSPERR_TYPE_MISMATCH,
    InvalidArrayLength HSPERROR_HSPERR_ARRAY_OVERFLOW,
    SyntaxInvalidLabel HSPERROR_HSPERR_LABEL_REQUIRED,
    SubroutineStackOverflow HSPERROR_HSPERR_TOO_MANY_NEST,
    ReturnOutsideSubroutine HSPERROR_HSPERR_RETURN_WITHOUT_GOSUB,
    SyntaxLoopWithoutRepeat HSPERROR_HSPERR_LOOP_WITHOUT_REPEAT,
    FileIoError HSPERROR_HSPERR_FILE_IO,
    ImageNotFound HSPERROR_HSPERR_PICTURE_MISSING,
    ExternalCommandError HSPERROR_HSPERR_EXTERNAL_EXECUTE,
    SyntaxParenthesisMismatch HSPERROR_HSPERR_PRIORITY,
    TooManyParameters HSPERROR_HSPERR_TOO_MANY_PARAMETERS,
    TempStringOverflow HSPERROR_HSPERR_TEMP_BUFFER_OVERFLOW,
    SyntaxInvalidVariableName HSPERROR_HSPERR_WRONG_NAME,
    DivideByZero HSPERROR_HSPERR_DIVIDED_BY_ZERO,
    OutOfBoundsAccess HSPERROR_HSPERR_BUFFER_OVERFLOW,
    UnsupportedFunction HSPERROR_HSPERR_UNSUPPORTED_FUNCTION,
    SyntaxExpressionTooComplex HSPERROR_HSPERR_EXPRESSION_COMPLEX,
    SyntaxVariableRequired HSPERROR_HSPERR_VARIABLE_REQUIRED,
    IntegerRequired HSPERROR_HSPERR_INTEGER_REQUIRED,
    SyntaxBadArrayExpression HSPERROR_HSPERR_BAD_ARRAY_EXPRESSION,
    OutOfMemory HSPERROR_HSPERR_OUT_OF_MEMORY,
    TypeInitializationFailed HSPERROR_HSPERR_TYPE_INITALIZATION_FAILED,
    NoFunctionParameters HSPERROR_HSPERR_NO_FUNCTION_PARAMETERS,
    StackOverflow HSPERROR_HSPERR_STACK_OVERFLOW,
    InvalidParameter HSPERROR_HSPERR_INVALID_PARAMETER,
    ArrayTypeMismatch HSPERROR_HSPERR_INVALID_ARRAYSTORE,
    InvalidFunctionDescriptor HSPERROR_HSPERR_INVALID_FUNCPARAM,
    TooManyObjects HSPERROR_HSPERR_WINDOW_OBJECT_FULL,
    SyntaxNotAnArray HSPERROR_HSPERR_INVALID_ARRAY,
    SyntaxModuleRequired HSPERROR_HSPERR_STRUCT_REQUIRED,
    SyntaxInvalidModule HSPERROR_HSPERR_INVALID_STRUCT_SOURCE,
    CouldNotConvertType HSPERROR_HSPERR_INVALID_TYPE,
    ExternalModuleError HSPERROR_HSPERR_DLL_ERROR,
    ExternalObjectError HSPERROR_HSPERR_COMDLL_ERROR,
    NoReturnValue HSPERROR_HSPERR_NORETVAL,
    SyntaxFunctionDefinition HSPERROR_HSPERR_FUNCTION_SYNTAX,
    InvalidCallback HSPERROR_HSPERR_INVALID_CALLBACK,
);

/// Converts an optional rhsp3 error code to an HSP error code.
pub fn opt_to_hsp_error(error: Option<ErrorCode>) -> hsp3struct::HSPERROR {
    match error {
        Some(v) => to_hsp_error(v),
        None => hsp3struct::HSPERROR_HSPERR_NONE,
    }
}
