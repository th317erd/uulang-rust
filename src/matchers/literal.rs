#[macro_export]
macro_rules! Literal {
  () => {
    adextopa_core::Switch!(
      $crate::StringLiteral!(),
      $crate::RegexLiteral!(),
      $crate::BooleanLiteral!(),
      $crate::FloatLiteral!(),
      $crate::IntegerLiteral!(),
    )
  };
}

#[cfg(test)]
mod tests {
  use crate::{
    adextopa_core::matcher::MatcherFailure, adextopa_core::parser::Parser,
    adextopa_core::parser_context::ParserContext, adextopa_core::source_range::SourceRange,
  };

  #[test]
  fn it_works_with_strings1() {
    let parser = Parser::new("\"I am a \\\"string\\\"\"");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = Literal!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "StringLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(1, 18));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 19));
      assert_eq!(token.get_value(), "I am a \"string\"");
      assert_eq!(token.get_matched_value(), "\"I am a \\\"string\\\"\"");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_works_with_booleans1() {
    let parser = Parser::new("true");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = Literal!();

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
  fn it_works_with_floats1() {
    let parser = Parser::new("1234.5678");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = Literal!();

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
  fn it_works_with_integers1() {
    let parser = Parser::new("1234");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = Literal!();

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
  fn it_works_with_regex1() {
    let parser = Parser::new(r"/\s*hello[^\S/\\]world\s*/gim");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = Literal!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "RegexLiteral");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 29));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 29));
      assert_eq!(token.get_value(), r"\s*hello[^\S/\\]world\s*");
      assert_eq!(token.get_matched_value(), r"/\s*hello[^\S/\\]world\s*/gim");

      let flags = token.find_child("Flags").unwrap();
      let flags = flags.borrow();
      assert_eq!(flags.get_name(), "Flags");
      assert_eq!(*flags.get_captured_range(), SourceRange::new(26, 29));
      assert_eq!(*flags.get_matched_range(), SourceRange::new(26, 29));
      assert_eq!(flags.get_value(), r"gim");
      assert_eq!(flags.get_matched_value(), r"gim");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_fails1() {
    let parser = Parser::new("derp");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = Literal!();

    if let Err(MatcherFailure::Fail) = ParserContext::tokenize(parser_context, matcher) {
    } else {
      unreachable!("Test failed!");
    };
  }
}
