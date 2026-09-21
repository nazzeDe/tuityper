#[derive(Debug, PartialEq, Eq)]
pub struct Article {
    title: String,
    content: Vec<char>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ArticleError {
    #[error("title must not be empty")]
    EmptyTitle,
    #[error("content must not be empty")]
    EmptyContent,
}

impl Article {
    pub fn new(title: &str, content: &str) -> Result<Self, ArticleError> {
        if title.trim().is_empty() {
            return Err(ArticleError::EmptyTitle);
        }
        if content.trim().is_empty() {
            return Err(ArticleError::EmptyContent);
        }
        let chars: Vec<char> = content.chars().collect();
        Ok(Self {
            title: title.to_string(),
            content: chars,
        })
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn content(&self) -> &[char] {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_title_is_reject() {
        let result = Article::new("", "have content");
        assert_eq!(result, Err(ArticleError::EmptyTitle));
    }
    #[test]
    fn empty_content_is_reject() {
        for bad in ["", "   ", "\n\t"] {
            let result = Article::new("have title", bad);
            assert_eq!(result, Err(ArticleError::EmptyContent), "input: {bad:?}");
        }
    }
    #[test]
    fn valid_input_constructs_article() {
        let article = Article::new("Rust", "abc").unwrap();
        assert_eq!(article.title(), "Rust");
        assert_eq!(article.content(), &['a', 'b', 'c']);
    }
}
