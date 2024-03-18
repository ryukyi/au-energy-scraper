use serde::Deserialize;
use std::fmt;

pub trait InformationRowTrait: for<'de> Deserialize<'de> + fmt::Debug {}
pub trait DataRowTrait: for<'de> Deserialize<'de> + fmt::Debug {}