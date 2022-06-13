use std::fmt::{Display, Formatter};
use crate::lang::compiler::CompilationDetail;

#[derive(Clone, Debug)]
pub struct StackTrace {
    pub file_name: String,
    pub callframe_name: String,
    pub class_name: String,
    pub line_number: usize,
}

impl StackTrace {
    pub fn from_compilation_detail(compilation_detail: &CompilationDetail, callframe_name: String, class_name: String) -> Self{
        Self {
            file_name: compilation_detail.file_name.clone(),
            callframe_name,
            class_name,
            line_number: compilation_detail.start_line.clone(),
        }
    }
}

impl Display for StackTrace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n\tat {}({}:{})", self.callframe_name, self.file_name, self.class_name,self.line_number)
    }
}