#[cfg(target_vendor = "pc")]
pub use self::com::*;
pub use self::enumerate::*;
#[cfg(target_vendor = "uwp")]
pub use self::winrt::*;

#[cfg(target_vendor = "pc")]
mod com;
mod enumerate;
mod error;
#[cfg(target_vendor = "uwp")]
mod winrt;
