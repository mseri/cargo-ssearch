#![allow(unused_must_use, unused_imports)]
// required for write! and the Write trait
use std::io::prelude::*;

#[macro_export]
macro_rules! if_some {
  ($con:expr, $fun:expr) => ({
    if $con.is_some() {
      $fun;
    }
  });
}


#[macro_export]
macro_rules! p_red {
  ($dst:expr, $fmt:expr) => ({
    $dst.fg(term::color::RED).unwrap();
    write!($dst, $fmt)
  });
  ($dst:expr, $fmt:expr, $($arg:tt)*) => ({
    $dst.fg(term::color::RED).unwrap();
    write!($dst, $fmt, $($arg)*)
  });
}

#[macro_export]
macro_rules! p_green {
  ($dst:expr, $fmt:expr) => ({
    $dst.fg(term::color::GREEN).unwrap();
    write!($dst, $fmt)
  });
  ($dst:expr, $fmt:expr, $($arg:tt)*) => ({
    $dst.fg(term::color::GREEN).unwrap();
    write!($dst, $fmt, $($arg)*)
  });
}

#[macro_export]
macro_rules! p_yellow {
  ($dst:expr, $fmt:expr) => ({
    $dst.fg(term::color::YELLOW).unwrap();
    write!($dst, $fmt)
  });
  ($dst:expr, $fmt:expr, $($arg:tt)*) => ({
    $dst.fg(term::color::YELLOW).unwrap();
    write!($dst, $fmt, $($arg)*)
  });
}

#[macro_export]
macro_rules! p_white {
  ($dst:expr, $fmt:expr) => ({
    $dst.fg(term::color::WHITE).unwrap();
    write!($dst, $fmt)
  });
  ($dst:expr, $fmt:expr, $($arg:tt)*) => ({
    $dst.fg(term::color::WHITE).unwrap();
    write!($dst, $fmt, $($arg)*)
  });
}
