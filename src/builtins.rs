use crate::interpreter::{BuiltinFun, Interpretation};
use crate::type_values::Type;
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
}
