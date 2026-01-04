//! Cognitive Drives - Drive System Upgrade
//!
//! Implements internal motivations and goals:
//! - Curiosity: Intrinsic motivation to reduce uncertainty (ICM)
//! - Free Energy: Active Inference based goal seeking (EFE)

pub mod curiosity;
pub mod free_energy;

pub use curiosity::{CuriosityConfig, CuriosityModule};
pub use free_energy::{FreeEnergyConfig, FreeEnergyModule};
