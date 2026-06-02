use crate::encoding::FormattingToken;
use std::collections::HashMap;

/// Token registry for mapping formatting tokens to their string representations
pub struct TokenRegistry {
    tokens: HashMap<FormattingToken, &'static str>,
}

impl TokenRegistry {
    /// Create a new token registry
    pub fn new() -> Self {
        let mut registry = TokenRegistry {
            tokens: HashMap::new(),
        };
        registry.register_all_tokens();
        registry
    }

    /// Register all formatting tokens
    fn register_all_tokens(&mut self) {
        self.register_token(FormattingToken::Text, "text");
        self.register_token(FormattingToken::MetaSep, "<|meta_sep|>");
        self.register_token(FormattingToken::MetaEnd, "<|meta_end|>");
        self.register_token(FormattingToken::Channel, "<|channel|>");
        self.register_token(FormattingToken::System, "<|system|>");
        self.register_token(FormattingToken::User, "<|user|>");
        self.register_token(FormattingToken::Assistant, "<|assistant|>");
    }

    /// Register a single token
    pub fn register_token(&mut self, token: FormattingToken, value: &'static str) {
        self.tokens.insert(token, value);
    }

    /// Get the string representation of a token
    pub fn get_token(&self, token: &FormattingToken) -> Option<&str> {
        self.tokens.get(token).copied()
    }

    /// Get all registered tokens
    pub fn get_all_tokens(&self) -> &HashMap<FormattingToken, &'static str> {
        &self.tokens
    }

    /// Check if a token is registered
    pub fn contains(&self, token: &FormattingToken) -> bool {
        self.tokens.contains_key(token)
    }
}

impl Default for TokenRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_sep_token() {
        let registry = TokenRegistry::new();
        assert_eq!(registry.get_token(&FormattingToken::MetaSep), Some("<|meta_sep|>"));
    }

    #[test]
    fn test_meta_end_token() {
        let registry = TokenRegistry::new();
        assert_eq!(registry.get_token(&FormattingToken::MetaEnd), Some("<|meta_end|>"));
    }

    #[test]
    fn test_all_tokens_registered() {
        let registry = TokenRegistry::new();
        assert!(registry.contains(&FormattingToken::Text));
        assert!(registry.contains(&FormattingToken::MetaSep));
        assert!(registry.contains(&FormattingToken::MetaEnd));
        assert!(registry.contains(&FormattingToken::Channel));
        assert!(registry.contains(&FormattingToken::System));
        assert!(registry.contains(&FormattingToken::User));
        assert!(registry.contains(&FormattingToken::Assistant));
    }
}
