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
      assert_eq!(*token.get_captured_range(), SourceRange::new(0, 130));
      assert_eq!(*token.get_matched_range(), SourceRange::new(0, 131));
      assert_eq!(token.get_value(), parser.borrow().get_source().trim());
      assert_eq!(token.get_matched_value(), parser.borrow().get_source());
      assert_eq!(token.get_children().len(), 1);

      let first = token.get_children()[0].borrow();
      assert_eq!(first.get_name(), "Scope");
      assert_eq!(*first.get_captured_range(), SourceRange::new(0, 130));
      assert_eq!(*first.get_matched_range(), SourceRange::new(0, 131));
      assert_eq!(first.get_value(), parser.borrow().get_source().trim());
      assert_eq!(first.get_matched_value(), parser.borrow().get_source());
      assert_eq!(first.get_children().len(), 8);

      let first_child = first.get_children()[0].borrow();
      assert_eq!(first_child.get_name(), "AssignmentExpression");
      assert_eq!(*first_child.get_captured_range(), SourceRange::new(0, 13));
      assert_eq!(*first_child.get_matched_range(), SourceRange::new(0, 13));
      assert_eq!(first_child.get_value(), "name1 = test;");
      assert_eq!(first_child.get_matched_value(), "name1 = test;");
      assert_eq!(first_child.get_children().len(), 3);
      assert_eq!(
        first_child.get_children()[1].borrow().get_name(),
        "Identifier"
      );

      let second_child = first.get_children()[1].borrow();
      assert_eq!(second_child.get_name(), "AssignmentExpression");
      assert_eq!(*second_child.get_captured_range(), SourceRange::new(14, 25));
      assert_eq!(*second_child.get_matched_range(), SourceRange::new(14, 25));
      assert_eq!(second_child.get_value(), "_name2=123;");
      assert_eq!(second_child.get_matched_value(), "_name2=123;");
      assert_eq!(second_child.get_children().len(), 3);
      assert_eq!(
        second_child.get_children()[1].borrow().get_name(),
        "IntegerLiteral"
      );

      let third_child = first.get_children()[2].borrow();
      assert_eq!(third_child.get_name(), "AssignmentExpression");
      assert_eq!(*third_child.get_captured_range(), SourceRange::new(26, 50));
      assert_eq!(*third_child.get_matched_range(), SourceRange::new(26, 50));
      assert_eq!(third_child.get_value(), "$name3\n=\n\"hello world!\";");
      assert_eq!(
        third_child.get_matched_value(),
        "$name3\n=\n\"hello world!\";"
      );
      assert_eq!(third_child.get_children().len(), 3);
      assert_eq!(
        third_child.get_children()[1].borrow().get_name(),
        "StringLiteral"
      );

      let forth_child = first.get_children()[3].borrow();
      assert_eq!(forth_child.get_name(), "AssignmentExpression");
      assert_eq!(*forth_child.get_captured_range(), SourceRange::new(51, 65));
      assert_eq!(*forth_child.get_matched_range(), SourceRange::new(51, 65));
      assert_eq!(forth_child.get_value(), "name4$ = true;");
      assert_eq!(forth_child.get_matched_value(), "name4$ = true;");
      assert_eq!(forth_child.get_children().len(), 3);
      assert_eq!(
        forth_child.get_children()[1].borrow().get_name(),
        "BooleanLiteral"
      );

      let fifth_child = first.get_children()[4].borrow();
      assert_eq!(fifth_child.get_name(), "AssignmentExpression");
      assert_eq!(*fifth_child.get_captured_range(), SourceRange::new(66, 83));
      assert_eq!(*fifth_child.get_matched_range(), SourceRange::new(66, 83));
      assert_eq!(fifth_child.get_value(), "name5_    =false;");
      assert_eq!(fifth_child.get_matched_value(), "name5_    =false;");
      assert_eq!(fifth_child.get_children().len(), 3);
      assert_eq!(
        fifth_child.get_children()[1].borrow().get_name(),
        "BooleanLiteral"
      );

      let sixth_child = first.get_children()[5].borrow();
      assert_eq!(sixth_child.get_name(), "AssignmentExpression");
      assert_eq!(*sixth_child.get_captured_range(), SourceRange::new(84, 97));
      assert_eq!(*sixth_child.get_matched_range(), SourceRange::new(84, 97));
      assert_eq!(sixth_child.get_value(), "name_6=12.43;");
      assert_eq!(sixth_child.get_matched_value(), "name_6=12.43;");
      assert_eq!(sixth_child.get_children().len(), 3);
      assert_eq!(
        sixth_child.get_children()[1].borrow().get_name(),
        "FloatLiteral"
      );

      let seventh_child = first.get_children()[6].borrow();
      assert_eq!(seventh_child.get_name(), "AssignmentExpression");
      assert_eq!(
        *seventh_child.get_captured_range(),
        SourceRange::new(98, 114)
      );
      assert_eq!(
        *seventh_child.get_matched_range(),
        SourceRange::new(98, 114)
      );
      assert_eq!(seventh_child.get_value(), "name7=12.43e-10;");
      assert_eq!(seventh_child.get_matched_value(), "name7=12.43e-10;");
      assert_eq!(seventh_child.get_children().len(), 3);
      assert_eq!(
        seventh_child.get_children()[1].borrow().get_name(),
        "FloatLiteral"
      );

      let eigth_child = first.get_children()[7].borrow();
      assert_eq!(eigth_child.get_name(), "AssignmentExpression");
      assert_eq!(
        *eigth_child.get_captured_range(),
        SourceRange::new(115, 130)
      );
      assert_eq!(*eigth_child.get_matched_range(), SourceRange::new(115, 130));
      assert_eq!(eigth_child.get_value(), "name8=/hello/i;");
      assert_eq!(eigth_child.get_matched_value(), "name8=/hello/i;");
      assert_eq!(eigth_child.get_children().len(), 3);
      assert_eq!(
        eigth_child.get_children()[1].borrow().get_name(),
        "RegexLiteral"
      );
    };
  }
}
