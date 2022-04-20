#[macro_export]
macro_rules! EndOfStatement {
  () => {
    adextopa_core::Equals!("EndOfStatement"; ";")
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
    let parser = Parser::new(";");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = EndOfStatement!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "EndOfStatement");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 1));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 1));
      assert_eq!(token.get_value(), ";");
      assert_eq!(token.get_matched_value(), ";");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_fails1() {
    let parser = Parser::new("derp");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = EndOfStatement!();

    if let Err(MatcherFailure::Fail) = ParserContext::tokenize(parser_context, matcher) {
    } else {
      unreachable!("Test failed!");
    };
  }
}
