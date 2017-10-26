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
}
