use crate::parser::{SpanData, Token};

pub struct AstParser {
  tokens: Vec<SpanData<Token>>,
  index: usize,
}

impl AstParser {
  pub fn new(tokens: Vec<SpanData<Token>>) -> AstParser {
    AstParser {
      tokens,
      index: 0,
    }
  }

  fn get_token(&self) -> Option<&SpanData<Token>> {
    self.tokens.get(self.index)
  }

  fn next_token(&mut self) -> Option<SpanData<Token>> {
    let token = self.get_token().cloned();
    self.index += 1;
    token
  }
}