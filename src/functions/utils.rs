use crate::interpreter::{BuiltinFun, Interpretation};
use crate::type_error::RuntimeError;
use crate::type_values::Type;
use std::time::SystemTime;

impl Interpretation {
    pub(crate) fn time_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "time",
            args: None,
            func: |_, _, span| {
                let now = SystemTime::now();
                let since_the_eposh = now.duration_since(SystemTime::UNIX_EPOCH).map_err(|e| {
                    RuntimeError::IoErrorMassage {
                        name: "time".to_string(),
                        span,
                        message: format!("Error getting time: {}", e),
                    }
                })?;
                Ok(Type::Int(since_the_eposh.as_millis() as i64))
            },
        }
    }
    pub(crate) fn type_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "type",
            args: Some(1),
            func: |args_t, _, _span| {
                let arg = &args_t[0];
                let type_name = match arg {
                    Type::Int(_) => "int",
                    Type::Str(_) => "str",
                };
                Ok(Type::Str(type_name.to_string()))
            },
        }
    }
}
