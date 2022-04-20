#[macro_export]
macro_rules! Scope {
  () => {
    adextopa_core::Loop!("Scope";
      $crate::WSN0!(?),
      adextopa_core::ExpandRange!(
        adextopa_core::Switch!(
          adextopa_core::Ref!("Body"),
          $crate::Expression!(),
        )
      ),
      $crate::WSN0!(?),
    )
  };
}

#[cfg(test)]
mod tests {
  use adextopa_core::parser_context::ParserContextRef;

  use crate::{
    adextopa_core::matcher::MatcherFailure, adextopa_core::parser::Parser,
    adextopa_core::parser_context::ParserContext, adextopa_core::source_range::SourceRange, Body,
  };

  fn register_matchers(context: ParserContextRef) {
    context.borrow_mut().register_matcher(Body!());
  }

  #[test]
  fn it_works1() {
    let parser = Parser::new(" hello = world; { name = true; } ");
    let parser_context = ParserContext::new(&parser, "Test");

    register_matchers(parser_context.clone());

    let matcher = Scope!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "Scope");
      assert_eq!(*token.get_captured_range(), SourceRange::new(1, 32));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 33));
      assert_eq!(token.get_value(), "hello = world; { name = true; }");
      assert_eq!(
        token.get_matched_value(),
        " hello = world; { name = true; } "
      );
      assert_eq!(token.get_children().len(), 2);

      let first = token.get_children()[0].borrow();
      assert_eq!(first.get_name(), "AssignmentExpression");
      assert_eq!(*first.get_captured_range(), SourceRange::new(1, 15));
      assert_eq!(*first.get_matched_range(), SourceRange::new(1, 15));
      assert_eq!(first.get_value(), "hello = world;");
      assert_eq!(first.get_matched_value(), "hello = world;");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_parses_a_body_in_a_body() {
    let parser = Parser::new("{ { name = true;}\n }");
    let parser_context = ParserContext::new(&parser, "Test");

    register_matchers(parser_context.clone());

    let matcher = Scope!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "Scope");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 20));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 20));
      assert_eq!(token.get_value(), "{ { name = true;}\n }");
      assert_eq!(token.get_matched_value(), "{ { name = true;}\n }");
      assert_eq!(token.get_children().len(), 1);

      let first = token.get_children()[0].borrow();
      assert_eq!(first.get_name(), "Body");
      assert_eq!(*first.get_captured_range(), SourceRange::new(2, 17));
      assert_eq!(*first.get_matched_range(), SourceRange::new(0, 20));
      assert_eq!(first.get_value(), "{ name = true;}");
      assert_eq!(first.get_matched_value(), "{ { name = true;}\n }");
      assert_eq!(first.get_children().len(), 1);

      let first_child = first.get_children()[0].borrow();
      assert_eq!(first_child.get_name(), "Scope");
      assert_eq!(*first_child.get_captured_range(), SourceRange::new(2, 17));
      assert_eq!(*first_child.get_matched_range(), SourceRange::new(1, 19));
      assert_eq!(first_child.get_value(), "{ name = true;}");
      assert_eq!(first_child.get_matched_value(), " { name = true;}\n ");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_fails1() {
    let parser = Parser::new("derp");
    let parser_context = ParserContext::new(&parser, "Test");

    register_matchers(parser_context.clone());

    let matcher = Scope!();

    if let Err(MatcherFailure::Fail) = ParserContext::tokenize(parser_context, matcher) {
    } else {
      unreachable!("Test failed!");
    };
  }
}
