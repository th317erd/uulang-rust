#[macro_export]
macro_rules! Body {
  () => {
    adextopa_core::Loop!("Body";
      adextopa_core::Discard!(adextopa_core::Equals!("BodyStart"; "{")),
      $crate::Scope!(),
      adextopa_core::Discard!(adextopa_core::Equals!("BodyEnd"; "}")),
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
  fn it_works1() {
    let parser = Parser::new("{ name = true; }");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = Body!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "Body");
      assert_eq!(*token.get_captured_range(), SourceRange::new(2, 14));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 16));
      assert_eq!(token.get_value(), "name = true;");
      assert_eq!(token.get_matched_value(), "{ name = true; }");
      assert_eq!(token.get_children().len(), 1);

      let first = token.get_children()[0].borrow();
      assert_eq!(first.get_name(), "Scope");
      assert_eq!(*first.get_captured_range(), SourceRange::new(2, 14));
      assert_eq!(*first.get_matched_range(), SourceRange::new(1, 15));
      assert_eq!(first.get_value(), "name = true;");
      assert_eq!(first.get_matched_value(), " name = true; ");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_parses_a_body_in_a_body() {
    let parser = Parser::new("{ { name = true;}\n }");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = Body!();

    if let Ok(token) = ParserContext::tokenize(parser_context, matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "Body");
      assert_eq!(*token.get_captured_range(), SourceRange::new(2, 17));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 20));
      assert_eq!(token.get_value(), "{ name = true;}");
      assert_eq!(token.get_matched_value(), "{ { name = true;}\n }");
      assert_eq!(token.get_children().len(), 1);

      let first = token.get_children()[0].borrow();
      assert_eq!(first.get_name(), "Scope");
      assert_eq!(*first.get_captured_range(), SourceRange::new(2, 17));
      assert_eq!(*first.get_matched_range(), SourceRange::new(1, 19));
      assert_eq!(first.get_value(), "{ name = true;}");
      assert_eq!(first.get_matched_value(), " { name = true;}\n ");
      assert_eq!(first.get_children().len(), 1);

      let first_child = first.get_children()[0].borrow();
      assert_eq!(first_child.get_name(), "Body");
      assert_eq!(*first_child.get_captured_range(), SourceRange::new(4, 16));
      assert_eq!(*first_child.get_matched_range(), SourceRange::new(2, 17));
      assert_eq!(first_child.get_value(), "name = true;");
      assert_eq!(first_child.get_matched_value(), "{ name = true;}");
      assert_eq!(first_child.get_children().len(), 1);

      let first_grandchild = first_child.get_children()[0].borrow();
      assert_eq!(first_grandchild.get_name(), "Scope");
      assert_eq!(
        *first_grandchild.get_captured_range(),
        SourceRange::new(4, 16)
      );
      assert_eq!(
        *first_grandchild.get_matched_range(),
        SourceRange::new(3, 16)
      );
      assert_eq!(first_grandchild.get_value(), "name = true;");
      assert_eq!(first_grandchild.get_matched_value(), " name = true;");

      let first_grandgrandchild = first_grandchild.get_children()[0].borrow();
      assert_eq!(first_grandgrandchild.get_name(), "AssignmentExpression");
      assert_eq!(
        *first_grandgrandchild.get_captured_range(),
        SourceRange::new(4, 16)
      );
      assert_eq!(
        *first_grandgrandchild.get_matched_range(),
        SourceRange::new(4, 16)
      );
      assert_eq!(first_grandgrandchild.get_value(), "name = true;");
      assert_eq!(first_grandgrandchild.get_matched_value(), "name = true;");
    } else {
      unreachable!("Test failed!");
    };
  }

  #[test]
  fn it_fails1() {
    let parser = Parser::new("derp");
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = Body!();

    if let Err(MatcherFailure::Fail) = ParserContext::tokenize(parser_context, matcher) {
    } else {
      unreachable!("Test failed!");
    };
  }
}
