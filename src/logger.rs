use web_sys::console;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into())
  }
}

// A macro to provide `println!(..)`-style syntax for `console.info` logging.
macro_rules! info {
  ( $( $t:tt )* ) => {
      web_sys::console::info_1(&format!( $( $t )* ).into())
  }
}

// A macro to provide `println!(..)`-style syntax for `console.info` logging.
macro_rules! warn {
  ( $( $t:tt )* ) => {
      web_sys::console::warn_1(&format!( $( $t )* ).into())
  }
}

// A macro to provide `println!(..)`-style syntax for `console.info` logging.
macro_rules! error {
  ( $( $t:tt )* ) => {
      web_sys::console::error_1(&format!( $( $t )* ).into())
  }
}