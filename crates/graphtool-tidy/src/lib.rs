//! Data import and tidyverse-style data preparation for GraphTool.
//!
//! Phase 0 skeleton. The concrete function catalogue (cleaning, mutating,
//! filtering, grouping, reshaping) is TO BE DEFINED LATER; the output
//! contract is fixed: a tidy table.
//!
//! Tidy table contract: one observation per row, one variable per column,
//! one type per column.
//!
//! NOTE: serde derives were temporarily removed for the Phase 0 sandbox
//! build (no C toolchain available there); they are restored as of this
//! revision, per the confirmed architecture (serialisable, versioned
//! ChartSpec embedded in the PPTX as a custom XML part).

use serde::{Deserialize, Serialize};


/// Column type of a tidy table.
#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq)]
pub enum ColumnType {
    /// Numeric (double precision, SI-consistent units).
    Number,
    /// Categorical (factor-like).
    Text,
    /// Temporal (date, time or date-time).
    DateTime,
}

/// A tidy table: the output contract of every data import and tidying step.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct TidyTable {
    /// Column names, in order.
    pub columns: Vec<Column>,
    /// Rows, each one observation, values in column order.
    pub rows: Vec<Vec<Value>>,
}

/// A named, typed column.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub ty: ColumnType,
    /// Optional description and unit shown in the Data step.
    pub desc: Option<String>,
    pub unit: Option<String>,
}

/// A cell value.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Text(String),
    DateTime(String),
    Missing,
}

/// One step of a tidy recipe. The operation catalogue is TO BE DEFINED LATER;
/// the variant is kept opaque (a tagged string) until then.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct TidyOp {
    pub op: String,
    pub params: Option<String>,
}

/// Apply a tidy recipe to a table. Phase 0 stub: returns the input unchanged.
pub fn apply_recipe(table: &TidyTable, _recipe: &[TidyOp]) -> TidyTable {
    table.clone()
}
