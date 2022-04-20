#[cfg(test)]
mod tests {
  use adextopa_core::{parser::Parser, parser_context::ParserContext, source_range::SourceRange};
  use uulang_rust::*;

  #[test]
  fn it_works1() {
    let parser = Parser::new_from_file("./tests/uulang/assignment_expression_test01.uu").unwrap();
    let parser_context = ParserContext::new(&parser, "Test");
    let matcher = UULang!();

    if let Ok(token) = ParserContext::tokenize(parser_context.clone(), matcher) {
      let token = token.borrow();
      assert_eq!(token.get_name(), "Program");
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 24));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 24));
      assert_eq!(token.get_value(), r"test\/[chars/\]]stuff");
      assert_eq!(token.get_matched_value(), r"/test\/[chars/\]]stuff/i");
      assert_eq!(token.get_children().len(), 7);
    };
  }
}
