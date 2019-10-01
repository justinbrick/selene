use codespan_reporting::diagnostic::{
    Diagnostic as CodespanDiagnostic, Label as CodespanLabel, Severity as CodespanSeverity,
};
use serde::de::DeserializeOwned;

pub mod empty_if;

#[cfg(test)]
mod test_util;

pub trait Rule {
    type Config: DeserializeOwned;
    type Error: std::error::Error;

    fn new(config: Self::Config) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn pass(&self, ast: &full_moon::ast::Ast<'static>) -> Vec<Diagnostic>;

    fn severity(&self) -> Severity;
    fn rule_type(&self) -> RuleType;
}

pub enum RuleType {
    /// Code that does something simple but in a complex way
    Complexity,

    /// Code that is outright wrong or very very useless
    /// Should have severity "Error"
    Correctness,

    /// Code that can be written in a faster way
    Performance,

    /// Code that should be written in a more idiomatic way
    Style,
}

pub enum Severity {
    Error,
    Warning,
}

pub struct Diagnostic {
    code: &'static str,
    message: String,
    notes: Vec<String>,
    primary_label: Label,
    secondary_labels: Vec<Label>,
}

impl Diagnostic {
    pub fn new(code: &'static str, message: String, primary_label: Label) -> Self {
        Self {
            code,
            message,
            primary_label,

            notes: Vec::new(),
            secondary_labels: Vec::new(),
        }
    }

    pub fn new_complete(
        code: &'static str,
        message: String,
        primary_label: Label,
        notes: Vec<String>,
        secondary_labels: Vec<Label>,
    ) -> Self {
        Self {
            code,
            message,
            notes,
            primary_label,
            secondary_labels,
        }
    }

    pub fn into_codespan_diagnostic(
        self,
        file_id: codespan::FileId,
        severity: CodespanSeverity,
    ) -> CodespanDiagnostic {
        CodespanDiagnostic {
            severity,
            code: Some(self.code.to_owned()),
            message: self.message.to_owned(),
            primary_label: self.primary_label.codespan_label(file_id),
            notes: self.notes.to_owned(),
            secondary_labels: self
                .secondary_labels
                .iter()
                .map(|label| label.codespan_label(file_id))
                .collect(),
        }
    }
}

pub struct Label {
    message: Option<String>,
    position: (u32, u32),
}

impl Label {
    pub fn new(position: (u32, u32)) -> Label {
        Label {
            position,
            message: None,
        }
    }

    pub fn new_with_message(position: (u32, u32), message: String) -> Label {
        Label {
            position,
            message: Some(message),
        }
    }

    pub fn codespan_label(&self, file_id: codespan::FileId) -> CodespanLabel {
        CodespanLabel::new(
            file_id.to_owned(),
            codespan::Span::new(self.position.0, self.position.1),
            self.message.as_ref().unwrap_or(&"".to_owned()).to_owned(),
        )
    }
}
