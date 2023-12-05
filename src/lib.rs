// Copied from https://github.com/knurling-rs/defmt/blob/main/firmware/defmt-test/src/lib.rs

#![feature(trait_alias)]
#![cfg_attr(target_os ="none", no_std)]


cfg_if::cfg_if! {
    if #[cfg(target_os ="none")]
    {


pub use embedded_test_macros::tests;

#[cfg(feature = "defmt")]
pub trait FormatOrDebug = defmt::Format;
#[cfg(feature = "log")]
pub trait FormatOrDebug = core::fmt::Debug;


/// Private implementation details used by the proc macro.
#[doc(hidden)]
pub mod export;

mod sealed {
    pub trait Sealed {}
    impl Sealed for () {}
    impl<T, E> Sealed for Result<T, E> {}
}

/// Indicates whether a test succeeded or failed.
///
/// This is comparable to the `Termination` trait in libstd, except stable and tailored towards the
/// needs of embedded-test. It is implemented for `()`, which always indicates success, and `Result`,
/// where `Ok` indicates success.
pub trait TestOutcome: FormatOrDebug + sealed::Sealed {
    fn is_success(&self) -> bool;
}

impl TestOutcome for () {
    fn is_success(&self) -> bool {
        true
    }
}

impl<T: FormatOrDebug, E: FormatOrDebug> TestOutcome for Result<T, E> {
    fn is_success(&self) -> bool {
        self.is_ok()
    }
}

}
}
