use serde::{Deserialize, Serialize};

/// Monetary value stored as integer cents to avoid floating-point errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money(pub i64);

/// Quantity in base units (grams for solids, milliliters for liquids).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Quantity(pub u64);
