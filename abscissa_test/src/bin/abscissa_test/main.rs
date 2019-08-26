//! Main entry point for Abscissa Test

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_test::application::APPLICATION;

/// Boot Abscissa Test
fn main() {
    abscissa_core::boot(&APPLICATION);
}
