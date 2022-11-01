use crate::prelude::*;
use crate::utils::{map, XTakeVal};
use std::collections::BTreeMap;

// Marker traits for types that can be used for query
pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}
pub trait Filterable: Into<Value> {}

pub struct Store {
    ds:
}

impl Store