use crate::interpreter::{BuiltinFun, Interpretation};
use crate::type_error::RuntimeError;
use crate::type_values::Type;
impl Interpretation {
    pub(super) fn print_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "print",
            args: None,
            func: |value, io, _span| {
                for v in value {
                    io.print(&v.to_string());
                }
                io.print("\n");
                Ok(Type::Int(0))
            },
        }
    }

    pub(super) fn len_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "len",
            args: Some(1),
            func: |args, io, span| {
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

    pub(super) fn input_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "input",
            args: None,
            func: |args_t, io, span| {
                let prompt = match args_t.as_slice() {
                    [] => "",
                    [Type::Str(s)] => s,
                    _ => {
                        return Err(RuntimeError::ArgumentMismatch {
                            expected: 1,
                            got: args_t.len(),
                        });
                    }
                };
                let result = io.input(prompt).map_err(|_| RuntimeError::IoError {
                    name: "input".to_string(),
                    span,
                })?;

                Ok(Type::Str(result))
            },
        }
    }
}
