#![allow(dead_code)]
use crate::lexer::Token;
use std::fmt;

pub type TError<T> = Result<T, Box<ParseError>>;
#[derive(Clone, Debug, Default)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug)]
pub struct TokenPosition {
    pub token: Token,
    pub position: Span,
}
#[derive(Clone, Debug, Default)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
    pub file: String,
    pub source: String,
}

impl ParseError {
    pub fn new(
        kind: ParseErrorKind,
        span: Span,
        file: Option<String>,
        source: Option<String>,
    ) -> Self {
        Self {
            kind,
            span,
            file: file.unwrap_or_default(),
            source: source.unwrap_or_default(),
        }
    }
    pub fn get_report_string(&self) -> String {
        let mut report = String::new();

        report.push_str(&format!("ERROR: {:?}\n", self.kind.message()));

        report.push_str(&format!(
            " --> {}:{}:{}\n",
            self.file, self.span.start.line, self.span.start.column
        ));

        let line = self
            .source
            .lines()
            .nth(self.span.start.line.saturating_sub(1))
            .unwrap_or("");

        report.push_str(&format!("\n{} | {}\n", self.span.start.line, line));

        report.push_str(&format!(
            "  | {}{}\n",
            " ".repeat(self.span.start.column.saturating_sub(1)),
            "^"
        ));

        format!("REPORT_START:\n{}", report)
    }
    pub fn report(&self) {
        use colored::*;

        eprintln!("{}: {:?}", "error".red().bold(), self.kind.message().bold());
        eprintln!(
            "  --> {}:{}:{}",
            self.file, self.span.start.line, self.span.start.column
        );

        let line = self
            .source
            .lines()
            .nth(self.span.start.line.saturating_sub(1))
            .unwrap_or("");

        eprintln!("   |\n{} | {}", self.span.start.line, line);
        eprintln!(
            "   | {}{}",
            " ".repeat(self.span.start.column.saturating_sub(1)),
            "^".red().bold()
        );
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_report_string())
    }
}

// ==================== LEXER ERRORS ====================
#[derive(Debug)]
pub enum LexerError {
    InvalidCharacter(char), // встретился символ, которого язык не знает
    UnterminatedString,     // строка началась, но не закрылась "
    InvalidNumberFormat,    // число неправильное (например: 12.3.4)
}

// ==================== TOKEN ERRORS ====================
#[derive(Debug)]
pub enum TokensError {
    ExpectedToken(Token, Token), // ожидался один токен, но пришёл другой
    UnexpectedToken(Token),      // получен токен, который здесь недопустим
    UnexpectedEOF,               // файл закончился раньше времени
}

// ==================== VARIABLE / IDENTIFIER ERRORS ====================
#[derive(Debug)]
pub enum VariableError {
    ExpectedIdentifier,                    // ожидалось имя переменной
    AssignmentToImmutableVariable(String), // попытка изменить let-переменную
    InvalidAssignmentTarget, // присваивание в что-то, что нельзя присвоить (например: 5 = x)
}

// ==================== EXPRESSION ERRORS ====================
#[derive(Debug)]
pub enum ExpressionError {
    InvalidPrimary(Token), // ожидалось первичное выражение: число, идентификатор, скобки
    UnexpectedOperator(String), // встретился оператор, который здесь недопустим
    MissingOperand,        // оператор без левого/правого операнда
    InvalidPrefixOperator(String), // неправильный префиксный оператор
    InvalidPostfixOperator(String), // неправильный постфиксный оператор
    DivideByZero,          // попытка деления на 0 (runtime)
}

// ==================== FUNCTION ERRORS ====================
#[derive(Debug)]
pub enum FunctionError {
    ExpectedFunctionName,           // после fn ожидалось имя функции
    ExpectedParameterName,          // параметр без имени
    DuplicateParameterName(String), // два параметра с одинаковым именем
    MissingClosingParenInCall,      // вызов функции без закрывающей ')'
    UnexpectedCommaInArguments,     // лишняя запятая между аргументами
    TooManyArguments,               // передано слишком много аргументов
    TooFewArguments,                // передано слишком мало аргументов
}

// ==================== BLOCK / CONTROL FLOW ERRORS ====================
#[derive(Debug)]
pub enum BlockError {
    MissingOpeningBrace,        // ожидалась {,  но её нет
    MissingClosingBrace,        // не закрыта }
    InvalidConditionExpression, // условие if/while неверного типа
    InvalidForInitializer,      // неправильная часть перед ";"
    InvalidForIncrement,        // неправильная часть после второго ";"
}

// ==================== RUNTIME ERRORS ====================
#[derive(Debug, Clone)]
pub enum RuntimeError {
    UndefinedVariable { name: String, span: Span },
    UndefinedFunction { name: String, span: Span },
    ImmutableAssignment { name: String, span: Span },
    ArgumentMismatch { expected: usize, got: usize },
    DivisionByZero,
    IoError { name: String, span: Span },
}
impl RuntimeError {
    pub fn span(&self) -> Span {
        match self {
            RuntimeError::UndefinedVariable { span, .. } => span.clone(),
            RuntimeError::UndefinedFunction { span, .. } => span.clone(),
            RuntimeError::ImmutableAssignment { span, .. } => span.clone(),
            RuntimeError::IoError { span, .. } => span.clone(),
            RuntimeError::ArgumentMismatch { .. } => Default::default(),
            RuntimeError::DivisionByZero => Default::default(),
        }
    }

    pub fn to_parse_error(&self, file: Option<String>, source: Option<String>) -> ParseError {
        ParseError::new(
            ParseErrorKind::Runtime(self.clone()),
            self.span(),
            file,
            source,
        )
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::UndefinedVariable { name, .. } => {
                write!(f, "Undefined variable: {}", name)
            }
            RuntimeError::UndefinedFunction { name, .. } => {
                write!(f, "Undefined function: {}", name)
            }
            RuntimeError::ImmutableAssignment { name, .. } => {
                write!(f, "Cannot assign to immutable variable: {}", name)
            }
            RuntimeError::ArgumentMismatch { expected, got } => {
                write!(f, "Argument mismatch: expected {}, got {}", expected, got)
            }
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
            RuntimeError::IoError { name, .. } => {
                write!(f, "no arguments were received in: {}", name)
            }
        }
    }
}

// ==================== TOP-LEVEL PARSER ERROR WRAPPER ====================

#[derive(Debug)]
pub enum ParseErrorKind {
    Lexer(LexerError),          // ошибка лексера
    Tokens(TokensError),        // ошибка токенизации / ожидания токенов
    Variable(VariableError),    // ошибка идентификаторов
    ExprError(ExpressionError), // ошибка выражений
    FunError(FunctionError),    // ошибка функций
    Block(BlockError),          // ошибка блоков / контрольных структур
    Runtime(RuntimeError),      // ошибка выполнения
    Custom(String),             // произвольная пользовательская ошибка
}
impl ParseErrorKind {
    pub fn message(&self) -> String {
        match self {
            ParseErrorKind::Runtime(err) => match err {
                RuntimeError::UndefinedVariable { name, .. } => {
                    format!("Undefined variable `{}`", name)
                }

                RuntimeError::UndefinedFunction { name, .. } => {
                    format!("Undefined function `{}`", name)
                }

                RuntimeError::ImmutableAssignment { name, .. } => {
                    format!("Cannot assign to immutable variable `{}`", name)
                }

                RuntimeError::ArgumentMismatch { expected, got } => {
                    format!("Argument mismatch: expected {}, got {}", expected, got)
                }

                RuntimeError::DivisionByZero => "Division by zero".to_string(),
                RuntimeError::IoError { name, .. } => {
                    format!("no arguments were received in: {}", name)
                }
            },

            other => format!("{:?}", other),
        }
    }
}
