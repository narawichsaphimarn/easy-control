use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PositionAtEdge {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

pub fn map_from_string(edge: String) -> PositionAtEdge {
    if PositionAtEdge::Bottom
        .to_string()
        .eq_ignore_ascii_case(&edge)
    {
        PositionAtEdge::Bottom
    } else if PositionAtEdge::Left.to_string().eq_ignore_ascii_case(&edge) {
        PositionAtEdge::Left
    } else if PositionAtEdge::Right
        .to_string()
        .eq_ignore_ascii_case(&edge)
    {
        PositionAtEdge::Right
    } else if PositionAtEdge::Top.to_string().eq_ignore_ascii_case(&edge) {
        PositionAtEdge::Top
    } else {
        PositionAtEdge::None
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
