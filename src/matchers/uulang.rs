#[macro_export]
macro_rules! UULang {
  () => {{
    adextopa_core::Program!("Program";
      adextopa_core::Register!(
        $crate::Body!(),
      ),
      $crate::Scope!()
    )
  }};
}
