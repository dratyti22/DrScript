use crate::interpreter::{BuiltinFun, Interpretation};
use crate::type_error::RuntimeError;
use crate::type_values::Type;

impl Interpretation {
    pub(crate) fn print_builtin() -> BuiltinFun {
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
    pub(crate) fn input_builtin() -> BuiltinFun {
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
    pub(crate) fn inputint_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "inputint",
            args: None,
            func: |args_t, io, span| {
                let promt = match args_t.as_slice() {
                    [] => "",
                    [Type::Str(s)] => s,
                    _ => {
                        return Err(RuntimeError::ArgumentMismatch {
                            expected: 1,
                            got: args_t.len(),
                        });
                    }
                };
                let result = io.input(promt).map_err(|e| RuntimeError::IoErrorMassage {
                    name: "inputint".to_string(),
                    span: span.clone(),
                    message: e.to_string(),
                })?;
                Ok(Type::Int(result.parse::<i64>().map_err(|_| {
                    RuntimeError::IoErrorMassage {
                        name: "inputint".to_string(),
                        span,
                        message:
                            "Invalid input, to convert a string to a number, turn what you type"
                                .to_string(),
                    }
                })?))
            },
        }
    }
}
