use super::Expression;
use crate::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    pub operator: Span,
    pub expr: Expression,
}

impl Attribute {
    pub fn span(&self) -> Span {
        self.operator.append(self.expr.span)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeBlock {
    pub span: Option<Span>,
    pub attributes: Vec<Attribute>,
}

#[allow(clippy::new_without_default)]
impl AttributeBlock {
    pub fn new() -> Self {
        Self {
            span: None,
            attributes: vec![],
        }
    }
}
