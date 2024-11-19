use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PositionAtEdge {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

impl PositionAtEdge {
    pub fn from_string(edge: String) -> PositionAtEdge {
        match edge.to_lowercase().as_str() {
            "top" => PositionAtEdge::Top,
            "bottom" => PositionAtEdge::Bottom,
            "left" => PositionAtEdge::Left,
            "right" => PositionAtEdge::Right,
            "none" => PositionAtEdge::None,
            _ => PositionAtEdge::None,
        }
    }
}

impl fmt::Display for PositionAtEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PositionAtEdge::Bottom => write!(f, "bottom"),
            PositionAtEdge::Top => write!(f, "top"),
            PositionAtEdge::Left => write!(f, "left"),
            PositionAtEdge::Right => write!(f, "right"),
            PositionAtEdge::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScreenMapperController {
    ScreenNumber,
}

impl fmt::Display for ScreenMapperController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScreenMapperController::ScreenNumber => write!(f, "SCREEN_NUMBER"),
        }
    }
}
