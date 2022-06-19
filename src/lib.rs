#[cfg(feature = "printer")]
/// Printer utilities
/// 
/// This module contains utilities for printing to either the system or a custom stdout and stderr.
/// The utils also allow you to read from the system or a csutom stdin.
pub mod printer;

#[cfg(feature = "orochi")]
/// Orochi utilities
/// 
/// This module contains utilities for parsing metadata from an Orochi repository.
pub mod orochi;