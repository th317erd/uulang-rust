#[macro_export]
macro_rules! StringLiteral {
  ($name:expr) => {
    adextopa_core::Sequence!($name; "\"", "\"", "\\")
  };

  () => {
    adextopa_core::Sequence!("StringLiteral"; "\"", "\"", "\\")
  };
}

#[cfg(test)]
mod tests {
  use crate::{
    adextopa_core::matcher::MatcherFailure, adextopa_core::parser::Parser,
    adextopa_core::parser_context::ParserContext, adextopa_core::source_range::SourceRange,
  };

  #[test]
  fn it_works1() {
    let parser = Parser::new("\"A \\\"test\\\" string!\" after string");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = StringLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "StringLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(1, 19));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 20));
      assert_eq!(token.get_value(), "A \"test\" string!");
      assert_eq!(token.get_matched_value(), "\"A \\\"test\\\" string!\"");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_fails1() {
    let parser = Parser::new("Testing");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = StringLiteral!();

    if let Err(MatcherFailure::Fail) = ParserContext::tokenize(parser_context, matcher) {
    } else {
      unreachable!("Test failed!");
    };
  }
}
