use std::fmt;

/// Formatting tokens used in the Harmony encoding system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FormattingToken {
    /// Regular text content
    Text,
    /// Special meta separator token
    MetaSep,
    /// Special meta end token
    MetaEnd,
    /// Channel separator
    Channel,
    /// System message marker
    System,
    /// User message marker
    User,
    /// Assistant message marker
    Assistant,
}

impl fmt::Display for FormattingToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattingToken::Text => write!(f, "text"),
            FormattingToken::MetaSep => write!(f, "<|meta_sep|>"),
            FormattingToken::MetaEnd => write!(f, "<|meta_end|>"),
            FormattingToken::Channel => write!(f, "<|channel|>"),
            FormattingToken::System => write!(f, "<|system|>"),
            FormattingToken::User => write!(f, "<|user|>"),
            FormattingToken::Assistant => write!(f, "<|assistant|>"),
        }
    }
}

impl FormattingToken {
    /// Get the string representation of this token
    pub fn as_str(&self) -> &'static str {
        match self {
            FormattingToken::Text => "text",
            FormattingToken::MetaSep => "<|meta_sep|>",
            FormattingToken::MetaEnd => "<|meta_end|>",
            FormattingToken::Channel => "<|channel|>",
            FormattingToken::System => "<|system|>",
            FormattingToken::User => "<|user|>",
            FormattingToken::Assistant => "<|assistant|>",
        }
    }
}

/// Encoding configuration for the Harmony system
#[derive(Debug, Clone)]
pub struct EncodingConfig {
    /// Whether to use meta tokens
    pub use_meta_tokens: bool,
    /// Maximum sequence length
    pub max_seq_len: usize,
    /// Padding character
    pub pad_token: String,
}

impl Default for EncodingConfig {
    fn default() -> Self {
        Self {
            use_meta_tokens: true,
            max_seq_len: 4096,
            pad_token: "[PAD]".to_string(),
        }
    }
}
