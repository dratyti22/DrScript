use crate::interpreter::{BuiltinFun, Interpretation};
use crate::type_error::RuntimeError;
use crate::type_values::Type;

impl Interpretation {
    pub(crate) fn len_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "len",
            args: Some(1),
            func: |args, _io, span| {
                let arg = match &args[0] {
                    Type::Str(s) => s,
                    _ => {
                        return Err(RuntimeError::IoError {
                            name: "len".to_string(),
                            span,
                        });
                    }
                };
                let l = arg.len();
                Ok(Type::Int(l as i64))
            },
        }
    }
    pub(crate) fn chars_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "chars",
            args: Some(1),
            func: |args, _io, span| {
                let arg = match &args[0] {
                    Type::Str(s) => s,
                    _ => {
                        return Err(RuntimeError::IoError {
                            name: "chars".to_string(),
                            span,
                        });
                    }
                };
                let l = arg.chars().count();
                Ok(Type::Int(l as i64))
            },
        }
    }
    pub(crate) fn str_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "str",
            args: Some(1),
            func: |args, _io, _span| {
                let arg = &args[0];
                let s = arg.to_string();
                Ok(Type::Str(s))
            },
        }
    }
}
