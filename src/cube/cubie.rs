use std::convert;

macro_rules! index_enum {
    ($name:ident) => {
        impl $name {
            #[inline(always)]
            pub const fn idx(self) -> usize {
                self as usize
            }
        }
    };
}



#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Color {
    White,
    Orange,
    Green,
    Red,
    Blue,
    Yellow,
}

impl Color {
    pub fn to_char(&self) -> char {
        match self {
            Color::White => 'W',
            Color::Orange => 'O',
            Color::Green => 'G',
            Color::Red => 'R',
            Color::Blue => 'B',
            Color::Yellow => 'Y',
        }
    }
}

// Important: If the ordering of the edges are changed, then the look up table for the heuristic will not work.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EdgeId {
    WB, WR, WG, WO, BO, BR, GR, GO, YG, YR, YB, YO
}
index_enum!(EdgeId);

impl EdgeId {
    pub fn colors(&self) -> (Color, Color) {
        use Color::*;
        use EdgeId::*;
        match self {
            WB => (White, Blue),
            WR => (White, Red),
            WG => (White, Green),
            WO => (White, Orange),
            BO => (Blue, Orange),
            BR => (Blue, Red),
            GR => (Green, Red),
            GO => (Green, Orange),
            YG => (Yellow, Green),
            YR => (Yellow, Red),
            YB => (Yellow, Blue),
            YO => (Yellow, Orange),
        }
    }
}

// Important: If the ordering of the edges are changed, then the look up table for the heuristic will not work.
#[derive(Copy, Clone)]
pub enum EdgePos {
    UB, UR, UF, UL, BL, BR, FR, FL, DF, DR, DB, DL
}

index_enum!(EdgePos);

// Important: If the ordering of the corners are changed, then the look up table for the heuristic will not work.
#[derive(Copy, Clone)]
pub enum CornerPos {
    UBL, UBR, UFR, UFL, DFL, DFR, DBR, DBL
}
index_enum!(CornerPos);

// Important: If the ordering of the corners are changed, then the look up table for the heuristic will not work.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CornerId {
    WBO, WBR, WGR, WGO, YGO, YGR, YBR, YBO
}
index_enum!(CornerId);


impl CornerId {
    // returns colors starting from white / yellow and going clockwise
    pub fn colors(&self) -> (Color, Color, Color) {
        use Color::*;
        use CornerId::*;
        match self {
            WBO => (White, Orange, Blue),
            WBR => (White, Blue, Red),
            WGR => (White, Red, Green),
            WGO => (White, Green, Orange),
            YGO => (Yellow, Orange, Green),
            YGR => (Yellow, Green, Red),
            YBR => (Yellow, Red, Blue),
            YBO => (Yellow, Blue, Orange),
        }
    }
} 

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Edge {
    pub id: EdgeId,
    pub flipped: bool,
}

impl Edge {
    pub fn flip(&mut self) {
        self.flipped = !self.flipped;
    }
}


// corner orientation is based on the white or yellow face being on top / bottom, one being a clockwise twist from that, two being 2 clockwise twists or one counterclockwise
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CornerOrientation {
    Zero = 0,
    One = 1,
    Two = 2,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Corner {
    pub id: CornerId,
    pub orientation: CornerOrientation,
}

impl Corner {
    pub fn twist_clockwise(&mut self) {
        self.orientation = match self.orientation {
            CornerOrientation::Zero => CornerOrientation::One,
            CornerOrientation::One => CornerOrientation::Two,
            CornerOrientation::Two => CornerOrientation::Zero,
        }
    }

    pub fn twist_counterclockwise(&mut self) {
        self.orientation = match self.orientation {
            CornerOrientation::Zero => CornerOrientation::Two,
            CornerOrientation::One => CornerOrientation::Zero,
            CornerOrientation::Two => CornerOrientation::One,
        }
    }
}

// impl convert::Into<usize> for Corner {
//     fn into(self) -> usize {
//         self.id as usize
//     }
// }
