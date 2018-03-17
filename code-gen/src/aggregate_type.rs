#[derive(Clone, Copy, Debug)]
pub enum AggregateType {
    Total,
    Count,
    Set,
}

use self::AggregateType::*;

pub const ALL: &[AggregateType] = &[
    Total,
    Count,
    Set,
];

impl AggregateType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "total" => Some(Total),
            "count" => Some(Count),
            "set" => Some(Set),
            _ => None,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Total => "total",
            Count => "count",
            Set => "set",
        }
    }

    pub fn requires_storage(self) -> bool {
        match self {
            // we'll need to look up the storage when the total changes
            Total => true,
            // we'll need to check if this component is present
            _ => false,
        }
    }

    pub fn to_type(self, component_type: Option<&String>, field_type: Option<&String>) -> String {
        match self {
            // totals are aggregated into the component's type
            Total => component_type.unwrap().clone(),
            Count => field_type.cloned().unwrap_or_else(|| "usize".to_string()),
            Set => field_type.cloned().unwrap_or_else(|| "SpatialHashCellEntityIdSet".to_string()),
        }
    }

    pub fn to_lookup(self) -> Option<&'static str> {
        match self {
            Total => Some("get"),
            Count => Some("contains"),
            Set => Some("contains"),
        }
    }
}
