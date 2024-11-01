use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PositionAtEdge {
    Top,
    Bottom,
    Left,
    Right,
    None
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
    ScreenNumber
}

impl fmt::Display for ScreenMapperController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScreenMapperController::ScreenNumber => write!(f, "SCREEN_NUMBER"),
        }
    }
}
