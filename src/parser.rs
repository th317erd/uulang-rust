#[macro_export]
macro_rules! UULang {
  () => {{
    adextopa_core::Loop!("Program"; $crate::AssignmentExpression!())
  }};
}
