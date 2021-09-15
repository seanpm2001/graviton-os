// SPDX-License-Identifier: MIT OR APACHE-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>
//
// Sublicensed from selected MIT to MPLv2

//! Architectual processor code.
//!
//! # Orientation
//!
//! Since arch modules are imported into generic modules using the path attribute, the path of this file is:
//!
//! crate::cpu::arch_cpu

use cortex_a::asm;

//------------------------------------------------------------------------------
// Public Code
//------------------------------------------------------------------------------

/// Pause execution on the core.
#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}