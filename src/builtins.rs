use crate::interpreter::{BuiltinFun, Interpretation};
use crate::type_error::RuntimeError;
use crate::type_values::Type;
use std::io;
use std::io::Write;
impl Interpretation {
    pub(super) fn print_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "print",
            args: None,
            func: |value, output, _span| {
                for v in value {
                    output.push_str(&v.to_string());
                }
                output.push('\n');
                Ok(Type::Int(0))
            },
        }
    }

    pub(super) fn len_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "len",
            args: Some(1),
            func: |args, _output, span| {
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
}
