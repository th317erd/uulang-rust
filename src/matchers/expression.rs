#[macro_export]
macro_rules! Expression {
  () => {
    adextopa_core::Switch!($crate::AssignmentExpression!(), $crate::Literal!())
  };
}
