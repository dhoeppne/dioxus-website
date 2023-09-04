#[macro_export]
macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into())
  }
}

// macro_rules! info {
//   ( $( $t:tt )* ) => {
//       web_sys::console::info_1(&format!( $( $t )* ).into())
//   }
// }

// macro_rules! warn {
//   ( $( $t:tt )* ) => {
//       web_sys::console::warn_1(&format!( $( $t )* ).into())
//   }
// }

macro_rules! error {
  ( $( $t:tt )* ) => {
      web_sys::console::error_1(&format!( $( $t )* ).into())
  }
}