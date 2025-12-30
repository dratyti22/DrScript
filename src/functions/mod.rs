use crate::interpreter::Interpretation;

mod io;
mod math;
mod strings;
mod utils;

impl Interpretation {
    pub(super) fn register_builtin(&mut self) {
        self.add_builtin(Self::print_builtin());
        self.add_builtin(Self::len_builtin());
        self.add_builtin(Self::input_builtin());
        self.add_builtin(Self::inputint_builtin());
        self.add_builtin(Self::random_builtin());
        self.add_builtin(Self::chars_builtin());
        self.add_builtin(Self::int_builtin());
        self.add_builtin(Self::str_builtin());
        self.add_builtin(Self::time_builtin());
        self.add_builtin(Self::type_builtin());
    }
}
