use std::fmt::Debug;
use std::io::Write;

pub trait DrScriptIo: Debug {
    fn print(&mut self, message: &str);
    fn input(&mut self, prompt: &str) -> Result<String, String>;
}

#[derive(Debug)]
pub struct IoTerminal;

impl DrScriptIo for IoTerminal {
    fn print(&mut self, message: &str) {
        print!("{}", message);
    }
    fn input(&mut self, prompt: &str) -> Result<String, String> {
        print!("{}", prompt);
        std::io::stdout().flush().map_err(|e| e.to_string())?;
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;
        Ok(input.trim_end().to_string())
    }
}
