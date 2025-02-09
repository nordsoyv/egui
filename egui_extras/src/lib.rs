//! This is a crate that adds some features on top top of [`egui`](https://github.com/emilk/egui). This crate are for experimental features, and features that require big dependencies that does not belong in `egui`.

#![allow(clippy::float_cmp)]
#![allow(clippy::manual_range_contains)]

#[cfg(feature = "chrono")]
mod datepicker;

pub mod image;
mod layout;
mod sizing;
mod strip;
mod table;

#[cfg(feature = "chrono")]
pub use crate::datepicker::DatePickerButton;

pub use crate::image::RetainedImage;
pub(crate) use crate::layout::StripLayout;
pub use crate::sizing::Size;
pub use crate::strip::*;
pub use crate::table::*;

/// Log an error with either `tracing` or `eprintln`
#[doc(hidden)]
#[macro_export]
macro_rules! log_err {
    ($fmt: literal, $($arg: tt)*) => {{
        #[cfg(feature = "tracing")]
        tracing::error!($fmt, $($arg)*);

        #[cfg(not(feature = "tracing"))]
        eprintln!(
            concat!("egui_extras: ", $fmt), $($arg)*
        );
    }};
}

/// Panic in debug builds, log otherwise.
#[doc(hidden)]
#[macro_export]
macro_rules! log_or_panic {
    ($fmt: literal, $($arg: tt)*) => {{
        if cfg!(debug_assertions) {
            panic!($fmt, $($arg)*);
        } else {
            $crate::log_err!($fmt, $($arg)*);
        }
    }};
}
