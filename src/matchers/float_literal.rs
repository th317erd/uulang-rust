#[macro_export]
macro_rules! FloatLiteral {
  ($name:expr) => {
    adextopa_core::Matches!($name; r"[+-]?\d+\.\d+(e[+-]\d+)?")
  };

  () => {
    adextopa_core::Matches!("FloatLiteral"; r"[+-]?\d+\.\d+(e[+-]\d+)?")
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
    let parser = Parser::new("1234.5678");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = FloatLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "FloatLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 9));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 9));
      assert_eq!(token.get_value(), "1234.5678");
      assert_eq!(token.get_matched_value(), "1234.5678");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_works_with_positive_numbers2() {
    let parser = Parser::new("+1234.5678");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = FloatLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "FloatLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 10));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 10));
      assert_eq!(token.get_value(), "+1234.5678");
      assert_eq!(token.get_matched_value(), "+1234.5678");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_works_with_positive_numbers3() {
    let parser = Parser::new("+0.5678e-10");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = FloatLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "FloatLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 11));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 11));
      assert_eq!(token.get_value(), "+0.5678e-10");
      assert_eq!(token.get_matched_value(), "+0.5678e-10");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_works_with_positive_numbers4() {
    let parser = Parser::new("0.5678e+10");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = FloatLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "FloatLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 10));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 10));
      assert_eq!(token.get_value(), "0.5678e+10");
      assert_eq!(token.get_matched_value(), "0.5678e+10");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_works_with_negative_numbers1() {
    let parser = Parser::new("-1.0");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = FloatLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "FloatLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 4));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 4));
      assert_eq!(token.get_value(), "-1.0");
      assert_eq!(token.get_matched_value(), "-1.0");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_works_with_negative_numbers2() {
    let parser = Parser::new("-1.0e-2");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = FloatLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "FloatLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 7));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 7));
      assert_eq!(token.get_value(), "-1.0e-2");
      assert_eq!(token.get_matched_value(), "-1.0e-2");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_works_with_negative_numbers3() {
    let parser = Parser::new("-1.0e+2");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = FloatLiteral!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "FloatLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 7));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 7));
      assert_eq!(token.get_value(), "-1.0e+2");
      assert_eq!(token.get_matched_value(), "-1.0e+2");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_fails1() {
    let parser = Parser::new("derp");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = FloatLiteral!();

    if let Err(MatcherFailure::Fail) = ParserContext::tokenize(parser_context, matcher) {
    } else {
      unreachable!("Test failed!");
    };
  }
}
