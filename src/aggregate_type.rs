#[derive(Clone, Copy, Debug)]
pub enum AggregateType {
    Total,
    Count,
    Set,
    NeighbourCount,
}

use self::AggregateType::*;

pub const ALL: &[AggregateType] = &[
    Total,
    Count,
    Set,
    NeighbourCount,
];

impl AggregateType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "total" => Some(Total),
            "count" => Some(Count),
            "set" => Some(Set),
            "neighbour_count" => Some(NeighbourCount),
            _ => None,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Total => "total",
            Count => "count",
            Set => "set",
            NeighbourCount => "neighbour_count",
        }
    }

    pub fn requires_storage(self) -> bool {
        match self {
            // we'll need to look up the storage when the total changes
            Total => true,
            // we'll need to check if this component is present
            NeighbourCount => true,
            _ => false,
        }
    }

    pub fn to_type(self, component_type: Option<&String>, field_type: Option<&String>) -> String {
        match self {
            // totals are aggregated into the component's type
            Total => component_type.unwrap().clone(),
            Count => field_type.cloned().unwrap_or_else(|| "usize".to_string()),
            Set => field_type.cloned().unwrap_or_else(|| "::std::collections::HashSet<super::EntityId>".to_string()),
            NeighbourCount => "::entity_store_helpers::NeighbourCount".to_string(),
        }
    }

    pub fn to_lookup(self) -> Option<&'static str> {
        match self {
            Total => Some("get"),
            Count => Some("contains"),
            Set => Some("contains"),
            NeighbourCount => None,
        }
    }
}
