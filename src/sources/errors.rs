#[derive(Debug)]
pub enum SourceError {
    Instapaper { message: String },
}

impl std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SourceError::Instapaper;
        let output = match self {
            Instapaper { message: m } => m,
        };
        write!(f, "{output}")
    }
}
