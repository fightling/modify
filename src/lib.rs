//! Adds a modify flag to an object.

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Interface for modifiable objects.
pub trait Modifiable {
    /// Return `true` if the object was modified.
    fn is_modified(&self) -> bool;
}

/// Modifiable object
/// Implements `Deref` and `DerefMut` to access the value and
/// adds a flag to indicate if the object was modified.
/// # Example
/// ```
/// use crate::modify::*;
/// let mut value = Modify::new(42);
/// assert_eq!(value.is_modified(), false);
/// *value = 43;
/// assert_eq!(value.is_modified(), true);
/// value.saved();
/// assert_eq!(value.is_modified(), false);
/// ```
#[derive(Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Modify<T> {
    /// Value of the object
    #[cfg_attr(feature = "serde", serde(flatten))]
    value: T,
    /// Flag that is `true` if the object was modified.
    #[cfg_attr(feature = "serde", serde(skip))]
    modified: bool,
}

impl<T> Modify<T> {
    /// Create a new modifiable object.
    pub fn new(value: T) -> Self {
        Self {
            value,
            modified: false,
        }
    }
    /// Clear the modified flag.
    pub fn saved(&mut self) {
        self.modified = false;
    }
}

impl<T> Modifiable for Modify<T> {
    fn is_modified(&self) -> bool {
        self.modified
    }
}

impl<T> Display for Modify<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> From<T> for Modify<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Deref for Modify<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Modify<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.modified = true;
        &mut self.value
    }
}
