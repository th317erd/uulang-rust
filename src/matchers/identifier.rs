#[macro_export]
macro_rules! Identifier {
  () => {{
    adextopa_core::Matches!("Identifier"; r"[a-zA-Z$_][a-zA-Z0-9$_]*")
  }};
}
