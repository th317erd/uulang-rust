#[macro_export]
macro_rules! WS0 {
  (?) => {
    adextopa_core::Discard!($crate::WS0!())
  };

  () => {
    adextopa_core::Optional!(
      adextopa_core::Loop!(1..; "Whitespace";
        adextopa_core::Switch!(
          adextopa_core::Matches!("Whitespace"; r"[^\S\n\r]*"),
          $crate::MultilineComment!(),
        )
      )
    )
  };
}

#[macro_export]
macro_rules! WS1 {
  (?) => {
    adextopa_core::Discard!($crate::WS1!())
  };

  () => {
    adextopa_core::Optional!(
      adextopa_core::Loop!(1..; "Whitespace";
        adextopa_core::Switch!(
          adextopa_core::Matches!("Whitespace"; r"[^\S\n\r]+"),
          $crate::MultilineComment!(),
        )
      )
    )
  };
}

#[macro_export]
macro_rules! WSN0 {
  (?) => {
    adextopa_core::Discard!($crate::WSN0!())
  };

  () => {
    adextopa_core::Optional!(
      adextopa_core::Loop!(1..; "Whitespace";
        adextopa_core::Switch!(
          adextopa_core::Matches!("Whitespace"; r"\s*"),
          $crate::Comment!(),
        )
      )
    )
  };
}

#[macro_export]
macro_rules! WSN1 {
  (?) => {
    adextopa_core::Discard!($crate::WSN1!())
  };

  () => {
    adextopa_core::Loop!(1..; "Whitespace";
      adextopa_core::Switch!(
        adextopa_core::Matches!("Whitespace"; r"\s+"),
        $crate::Comment!(),
      )
    )
  };
}

#[cfg(test)]
mod tests {
  use crate::{
    adextopa_core::matcher::{MatcherFailure, MatcherSuccess},
    adextopa_core::parser::Parser,
    adextopa_core::parser_context::ParserContext,
    adextopa_core::source_range::SourceRange,
  };

  #[test]
  fn it_matches_against_zero_or_more() {
    let parser = Parser::new("");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = WS0!();

    assert_eq!(
      Ok(MatcherSuccess::Skip(0)),
      matcher.borrow().exec(
        matcher.clone(),
        parser_context.clone(),
        parser_context.borrow().get_scope(),
      )
    );
  }

  #[test]
  fn it_matches_against_one_or_more() {
    let parser = Parser::new("  ");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = WS0!();

    let result = ParserContext::tokenize(parser_context, matcher);

    if let Ok(token) = result {
      let token = token.borrow();
      assert_eq!(token.get_name(), "Whitespace");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 2));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 2));
      assert_eq!(token.get_value(), "  ");
      assert_eq!(token.get_matched_value(), "  ");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_will_not_match_against_new_lines() {
    let parser = Parser::new("  \n \t\r\n");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = WS1!();

    let result = ParserContext::tokenize(parser_context, matcher);

    if let Ok(token) = result {
      let token = token.borrow();
      assert_eq!(token.get_name(), "Whitespace");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 2));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 2));
      assert_eq!(token.get_value(), "  ");
      assert_eq!(token.get_matched_value(), "  ");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_will_match_against_newlines() {
    let parser = Parser::new("\r\n\r  \n");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = WSN1!(?);

    assert_eq!(
      Ok(MatcherSuccess::Skip(6)),
      matcher.borrow().exec(
        matcher.clone(),
        parser_context.clone(),
        parser_context.borrow().get_scope(),
      )
    );
  }

  #[test]
  fn it_will_succeed_with_comments1() {
    let parser = Parser::new("\r\n\r /* Test */  \n");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = WSN1!(?);

    assert_eq!(
      Ok(MatcherSuccess::Skip(17)),
      matcher.borrow().exec(
        matcher.clone(),
        parser_context.clone(),
        parser_context.borrow().get_scope(),
      )
    );
  }

  #[test]
  fn it_will_succeed_with_comments2() {
    let parser = Parser::new("    // Test");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = WS1!(?);

    assert_eq!(
      Ok(MatcherSuccess::Skip(4)),
      matcher.borrow().exec(
        matcher.clone(),
        parser_context.clone(),
        parser_context.borrow().get_scope(),
      )
    );
  }

  #[test]
  fn it_will_succeed_with_comments3() {
    let parser = Parser::new("\r\n\r /* Test */  \n // Hell0 world \n\n");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = WSN1!(?);

    assert_eq!(
      Ok(MatcherSuccess::Skip(35)),
      matcher.borrow().exec(
        matcher.clone(),
        parser_context.clone(),
        parser_context.borrow().get_scope(),
      )
    );
  }

  #[test]
  fn it_fails1() {
    let parser = Parser::new("Testing");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = WS0!();

    assert_eq!(
      Ok(MatcherSuccess::Skip(0)),
      matcher.borrow().exec(
        matcher.clone(),
        parser_context.clone(),
        parser_context.borrow().get_scope(),
      )
    );
  }

  #[test]
  fn it_fails2() {
    let parser = Parser::new("Testing");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = WS1!();

    assert_eq!(
      Err(MatcherFailure::Fail),
      ParserContext::tokenize(parser_context, matcher)
    );
  }
}
