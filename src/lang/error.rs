use std::fmt::{Display, Formatter};
use crate::lang::compiler::CompilationDetail;
use crate::lang::stack_trace::StackTrace;


#[derive(Debug)]
#[allow(dead_code)]
pub struct CompilationError {
    pub error_type: CompilationErrorType,
    pub message: String,
    pub details: CompilationDetail,
}


#[derive(Debug)]
pub enum CompilationErrorType {
    Generic,
    UndefinedVariable,
    UndefinedFunction,
    FunctionAlreadyDefined,
    ClassAlreadyDefined,
    NativeAlreadyDefined,
    Type,
    LabelNotInMain,
    UndefinedLabel,
    NativeArgumentCount,
}

impl CompilationError {
    pub fn message(&self) -> String {
        format!("{}", self)
    }
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message).unwrap();
        write!(f, "{}", self.details)
    }
}


#[derive(Debug)]
pub enum RuntimeErrorType {
    Internal,
    Execution,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub source: CompilationDetail,
    pub error_type: RuntimeErrorType,
    pub stack_traces: Vec<StackTrace>,
}

#[derive(Debug)]
pub struct TemporaryRuntimeError {
    pub message: String,
}

impl TemporaryRuntimeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string()
        }
    }
    pub fn new_string(message: String) -> Self {
        Self {
            message
        }
    }
}

impl RuntimeError {
    pub fn new(source: CompilationDetail, stack_traces: Vec<StackTrace>, message: &str) -> Self {
        Self {
            source,
            message: message.to_string(),
            stack_traces,
            error_type: RuntimeErrorType::Execution,
        }
    }
    pub fn new_with_type(source: CompilationDetail, stack_traces: Vec<StackTrace>, error_type: RuntimeErrorType, message: &str) -> Self {
        Self {
            source,
            message: message.to_string(),
            stack_traces,
            error_type,
        }
    }
    pub fn new_string(source: CompilationDetail, stack_traces: Vec<StackTrace>, message: String) -> Self {
        Self {
            source,
            message,
            stack_traces,
            error_type: RuntimeErrorType::Execution,
        }
    }
    pub fn new_internal(message: String) -> Self {
        Self {
            message,
            source: CompilationDetail::new_empty(),
            error_type: RuntimeErrorType::Internal,
            stack_traces: vec![],
        }
    }
    pub fn from_temporary(source: CompilationDetail, stack_traces: Vec<StackTrace>, temporary: TemporaryRuntimeError) -> Self {
        Self {
            message: temporary.message,
            source,
            error_type: RuntimeErrorType::Execution,
            stack_traces,
        }
    }
    pub fn from_temporary_and_message(source: CompilationDetail, stack_traces: Vec<StackTrace>, temporary: TemporaryRuntimeError, message: &str) -> Self {
        Self {
            message: format!("{}. {}", message, temporary.message),
            source,
            error_type: RuntimeErrorType::Execution,
            stack_traces,
        }
    }
    pub fn message(&self) -> String {
        format!("{}", self)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message).unwrap();
        writeln!(f, "{}", self.source).unwrap();
        for (i, trace) in self.stack_traces.iter().enumerate() {
            writeln!(f, "{}: {}", i, trace).unwrap();
        }
        write!(f, "")
    }
}
