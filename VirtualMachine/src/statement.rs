pub struct Statement {
    pub identifier: String,
    pub value: String,
}

impl From<Option<&Statement>> for Statement {
    fn from(statement_option: Option<&Statement>) -> Self {
        match statement_option {
            Some(statement) => Statement {
                identifier: statement.identifier.clone(),
                value: statement.value.clone(),
            },
            None => Statement {
                identifier: String::new(),
                value: String::new(),
            },
        }
    }
}