#[macro_export]
macro_rules! AssignmentExpression {
  () => {{
    adextopa_core::Program!(
      "AssignmentExpression";
      $crate::Identifier!(),
      $crate::WSN0!(?),
      adextopa_core::Discard!(adextopa_core::Equals!("=")),
      $crate::WSN0!(?),
      adextopa_core::Switch!(
        $crate::Literal!(),
        $crate::Identifier!(),
      ),
      $crate::WSN0!(?),
      $crate::EndOfStatement!(),
    )
  }};
}
