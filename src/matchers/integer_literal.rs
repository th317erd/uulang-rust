#[macro_export]
macro_rules! IntegerLiteral {
  ($name:expr) => {
    adextopa_core::Matches!($name; r"[+-]?\d+")
  };

  () => {
    adextopa_core::Matches!("IntegerLiteral"; r"[+-]?\d+")
  };
}

#[cfg(test)]
mod tests {
  use crate::{
    adextopa_core::matcher::MatcherFailure, adextopa_core::parser::Parser,
    adextopa_core::parser_context::ParserContext, adextopa_core::source_range::SourceRange,
  };

  #[test]
  fn it_works_with_positive_numbers1() {
    let parser = Parser::new("1234");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = IntegerLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "IntegerLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 4));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 4));
      assert_eq!(token.get_value(), "1234");
      assert_eq!(token.get_matched_value(), "1234");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_works_with_positive_numbers2() {
    let parser = Parser::new("+1234");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = IntegerLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "IntegerLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 5));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 5));
      assert_eq!(token.get_value(), "+1234");
      assert_eq!(token.get_matched_value(), "+1234");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_works_with_negative_numbers1() {
    let parser = Parser::new("-1234");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = IntegerLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "IntegerLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 5));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 5));
      assert_eq!(token.get_value(), "-1234");
      assert_eq!(token.get_matched_value(), "-1234");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_fails1() {
    let parser = Parser::new("derp");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = IntegerLiteral!();

    if let Err(MatcherFailure::Fail) = ParserContext::tokenize(parser_context, matcher) {
    } else {
      unreachable!("Test failed!");
    };
  }
}
