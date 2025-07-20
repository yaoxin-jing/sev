// SPDX-License-Identifier: Apache-2.0

//! Operations and types for launching on Linux
pub(crate) mod ioctl;

pub use ioctl::KvmSetMemoryAttributes;

#[cfg(feature = "sev")]
pub(crate) mod sev;

#[cfg(feature = "snp")]
pub(crate) mod snp;

#[cfg(any(feature = "sev", feature = "snp"))]
pub(crate) mod shared;
