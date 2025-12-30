use crate::interpreter::{BuiltinFun, Interpretation};
use crate::type_error::RuntimeError;
use crate::type_values::Type;
use rand::Rng;

impl Interpretation {
    pub(crate) fn random_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "random",
            args: Some(2),
            func: |args_t, _io, span| {
                let min = args_t[0]
                    .as_int()
                    .map_err(|e| RuntimeError::IoErrorMassage {
                        name: "random".to_string(),
                        span: span.clone(),
                        message: e,
                    })?;
                let max = args_t[1]
                    .as_int()
                    .map_err(|e| RuntimeError::IoErrorMassage {
                        name: "random".to_string(),
                        span,
                        message: e,
                    })?;
                let mut rng = rand::rng();
                let random = rng.random_range(min..max);
                Ok(Type::Int(random))
            },
        }
    }

    pub(crate) fn int_builtin() -> BuiltinFun {
        BuiltinFun {
            name: "int",
            args: Some(1),
            func: |args_t, _io, span| {
                let arg = args_t[0]
                    .as_int()
                    .map_err(|e| RuntimeError::IoErrorMassage {
                        name: "int".to_string(),
                        span,
                        message: e,
                    })?;
                Ok(Type::Int(arg))
            },
        }
    }
}
