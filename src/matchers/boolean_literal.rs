#[macro_export]
macro_rules! BooleanLiteral {
  ($name:expr) => {
    adextopa_core::Matches!($name; r"true|false")
  };

  () => {
    adextopa_core::Matches!("BooleanLiteral"; r"true|false")
  };
}

#[cfg(test)]
mod tests {
  use crate::{
    adextopa_core::matcher::MatcherFailure, adextopa_core::parser::Parser,
    adextopa_core::parser_context::ParserContext, adextopa_core::source_range::SourceRange,
  };

  #[test]
  fn it_matches_against_true() {
    let parser = Parser::new("true");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = BooleanLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "BooleanLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 4));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 4));
      assert_eq!(token.get_value(), "true");
      assert_eq!(token.get_matched_value(), "true");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_matches_against_false() {
    let parser = Parser::new("false");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = BooleanLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "BooleanLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 5));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 5));
      assert_eq!(token.get_value(), "false");
      assert_eq!(token.get_matched_value(), "false");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_fails1() {
    let parser = Parser::new("derp");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = BooleanLiteral!();

    if let Err(MatcherFailure::Fail) = ParserContext::tokenize(parser_context, matcher) {
    } else {
      unreachable!("Test failed!");
    };
  }
}
