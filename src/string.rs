/// Trait to simplify obtaining a symbol by index
pub trait CharAt {
    fn char_at(&self, index: usize) -> Option<char>;
}

/// CharAt implementation for String
impl CharAt for String {
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
}

/// CharAt implementation for str
impl CharAt for str {
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
}