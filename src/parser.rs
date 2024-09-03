use crate::{
    lexer::Token,
    position::{Diagnostic, Span, WithSpan},
};

pub struct Parser<'a> {
    tokens: &'a [WithSpan<Token>],
    current: usize,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [WithSpan<Token>]) -> Self {
        Self {
            tokens,
            current: 0,
            diagnostics: Vec::new(),
        }
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn error(&mut self, message: &str, span: Span) {
        self.diagnostics.push(Diagnostic {
            message: message.to_string(),
            span,
        })
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// Retrieves interior Token from slice of WithSpan<Token>
    pub fn peek(&self) -> Option<Token> {
        // this feels weird, but I like the idea of keeping the tokens
        // as a slice since we never need to modify, and I'm pretty sure
        // the clone on the enum variant in the From Impl is very cheap
        self.tokens.get(self.current).map(Token::from)
    }

    pub fn peek_with_span(&self) -> Option<&'a WithSpan<Token>> {
        self.tokens.get(self.current)
    }

    /// Returns span of current token
    pub fn current_span(&self) -> Span {
        self.tokens
            .get(self.current)
            .map_or(Span::empty(), |t| t.span)
    }

    pub fn advance(&mut self) -> WithSpan<Token> {
        // achtung: could panic! but also I dont think so
        let token = self.tokens.get(self.current).unwrap().clone();
        if !self.is_at_end() {
            self.current += 1;
        }
        token
    }

    pub fn matches(&self, expected: Token) -> bool {
        expected == self.peek().unwrap()
        // I am a criminal --------^
    }
}
