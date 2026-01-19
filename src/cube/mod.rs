use rand::seq::IteratorRandom;

pub mod cubie;
use cubie::*;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Turn {
    U, L, F, R, B, D
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TurnDir {
    None, One, Two, Prime
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Twist {
    pub turn: Turn,
    dir: TurnDir,
}

impl Twist {
    pub const fn new(turn: Turn, dir: TurnDir) -> Self {
        Self { turn, dir }
    }

    pub fn new_random(prev_move: Option<Turn>) -> Self {
        let mut rng = rand::rng();
        Self::allowed_moves(prev_move).choose(&mut rng).unwrap()
    }

    pub fn inverse(self) -> Self {
        match self {
            Twist { turn, dir: TurnDir::None } => Twist { turn, dir: TurnDir::None },
            Twist { turn, dir: TurnDir::One } => Twist { turn, dir: TurnDir::Prime },
            Twist { turn, dir: TurnDir::Two } => Twist { turn, dir: TurnDir::Two },
            Twist { turn, dir: TurnDir::Prime } => Twist { turn, dir: TurnDir::One },
        }
    }

    pub const ALL_MOVES: [Twist; 18] = [
        Twist { turn: Turn::U, dir: TurnDir::One },
        Twist { turn: Turn::U, dir: TurnDir::Two },
        Twist { turn: Turn::U, dir: TurnDir::Prime },
        Twist { turn: Turn::D, dir: TurnDir::One },
        Twist { turn: Turn::D, dir: TurnDir::Two },
        Twist { turn: Turn::D, dir: TurnDir::Prime },
        Twist { turn: Turn::F, dir: TurnDir::One },
        Twist { turn: Turn::F, dir: TurnDir::Two },
        Twist { turn: Turn::F, dir: TurnDir::Prime },
        Twist { turn: Turn::B, dir: TurnDir::One },
        Twist { turn: Turn::B, dir: TurnDir::Two },
        Twist { turn: Turn::B, dir: TurnDir::Prime },
        Twist { turn: Turn::L, dir: TurnDir::One },
        Twist { turn: Turn::L, dir: TurnDir::Two },
        Twist { turn: Turn::L, dir: TurnDir::Prime },
        Twist { turn: Turn::R, dir: TurnDir::One },
        Twist { turn: Turn::R, dir: TurnDir::Two },
        Twist { turn: Turn::R, dir: TurnDir::Prime },
    ];

    pub fn allowed_moves(prev: Option<Turn>) -> impl Iterator<Item = Twist> {
        Self::allowed_moves_from_moveset(&Self::ALL_MOVES, prev)
    }
    pub fn allowed_moves_from_moveset(moveset: &[Twist], prev: Option<Turn>) -> impl Iterator<Item = Twist> {
        moveset.iter().filter(move |m| {
            match prev {
                None => true,
                Some(p) => match p {
                    Turn::U | Turn::R | Turn::F => m.turn != p,
                    Turn::L => {m.turn != Turn::L && m.turn != Turn::R}
                    Turn::B => {m.turn != Turn::B && m.turn != Turn::F}
                    Turn::D => {m.turn != Turn::D && m.turn != Turn::U}
                }
            }
        }).cloned()
    }
}

impl std::fmt::Display for Twist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let turn_str = match self.turn {
            Turn::U => "U",
            Turn::D => "D",
            Turn::F => "F",
            Turn::B => "B",
            Turn::L => "L",
            Turn::R => "R",
        };
        let dir_str = match self.dir {
            TurnDir::One => "",
            TurnDir::Two => "2",
            TurnDir::Prime => "'",
            TurnDir::None => "0",
        };
        write!(f, "{}{}", turn_str, dir_str)
    }
}

pub struct Algorithm {
    pub twists: Vec<Twist>,
}

impl Algorithm {
    pub fn new(twists: Vec<Twist>) -> Self {
        Self { twists }
    }
    pub fn append(&mut self, other: &mut Self) {
        self.twists.append(&mut other.twists);
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in &self.twists {
            write!(f, "{} ", m)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Cube {
    pub edges: [Edge; 12],
    pub corners: [Corner; 8],
}

impl Cube {
    pub const SOLVED_EDGES: [Edge; 12] = [
        Edge { id: EdgeId::WB, flipped: false }, // UB
        Edge { id: EdgeId::WR, flipped: false }, // UR
        Edge { id: EdgeId::WG, flipped: false }, // UF
        Edge { id: EdgeId::WO, flipped: false }, // UL
        Edge { id: EdgeId::BO, flipped: false }, // LB
        Edge { id: EdgeId::BR, flipped: false }, // RB
        Edge { id: EdgeId::GR, flipped: false }, // FR
        Edge { id: EdgeId::GO, flipped: false }, // FL
        Edge { id: EdgeId::YG, flipped: false }, // DF
        Edge { id: EdgeId::YR, flipped: false }, // DR
        Edge { id: EdgeId::YB, flipped: false }, // DB
        Edge { id: EdgeId::YO, flipped: false }, // DL
    ];

    pub const SOLVED_CORNERS: [Corner; 8] = [
        Corner { id: CornerId::WBO, orientation: CornerOrientation::Zero }, // UBL
        Corner { id: CornerId::WBR, orientation: CornerOrientation::Zero }, // UBR
        Corner { id: CornerId::WGR, orientation: CornerOrientation::Zero }, // UFR
        Corner { id: CornerId::WGO, orientation: CornerOrientation::Zero }, // UFL
        Corner { id: CornerId::YGO, orientation: CornerOrientation::Zero }, // DFL
        Corner { id: CornerId::YGR, orientation: CornerOrientation::Zero }, // DFR
        Corner { id: CornerId::YBR, orientation: CornerOrientation::Zero }, // DBR
        Corner { id: CornerId::YBO, orientation: CornerOrientation::Zero }, // DBL
    ];

    pub fn new_solved() -> Self {
        Self {
            edges: Self::SOLVED_EDGES,
            corners: Self::SOLVED_CORNERS,
        }
    }

    pub fn twist(&mut self, twist: Twist) {
        use EdgePos::*;
        use CornerPos::*;
        
        let (should_correct_orientation, cycle_edges, cycle_corners): (
            bool,
            fn(&mut Cube, EdgePos, EdgePos, EdgePos, EdgePos),
            fn(&mut Cube, CornerPos, CornerPos, CornerPos, CornerPos)
        ) = match twist.dir {
            TurnDir::One => (true, Cube::cycle_edges_right, Cube::cycle_corners_right),
            TurnDir::Two => (false, Cube::swap_opposite_edges, Cube::swap_opposite_corners),
            TurnDir::Prime => (true, Cube::cycle_edges_left, Cube::cycle_corners_left),
            TurnDir::None => return,
        };

        match twist {
            Twist { turn: Turn::U, .. } => {
                cycle_edges(self, UB, UR, UF, UL);
                cycle_corners(self, UBL, UBR, UFR, UFL);
            }
            Twist { turn: Turn::D, .. } => {
                cycle_edges(self, DF, DR, DB, DL);
                cycle_corners(self, DFL, DFR, DBR, DBL);
            }
            Twist { turn: Turn::F, .. } => {
                cycle_edges(self, UF, FR, DF, FL);
                cycle_corners(self, UFL, UFR, DFR, DFL);
                if should_correct_orientation {
                    self.flip_edges(UF, FR, DF, FL);
                    self.corner_correction(UFL, UFR, DFR, DFL);
                }
            }
            Twist { turn: Turn::B, .. } => {
                cycle_edges(self, UB, BL, DB, BR);
                cycle_corners(self, UBL, DBL, DBR, UBR);
                if should_correct_orientation {
                    self.flip_edges(UB, BL, DB, BR);
                    self.corner_correction(UBR, UBL, DBL, DBR);
                }
            }
            Twist { turn: Turn::L, .. } => {
                cycle_edges(self, UL, FL, DL, BL);
                cycle_corners(self, UBL, UFL, DFL, DBL);
                if should_correct_orientation {
                    self.corner_correction(UBL, UFL, DFL, DBL);
                }
            }
            Twist { turn: Turn::R, .. } => {
                cycle_edges(self, UR, BR, DR, FR);
                cycle_corners(self, UFR, UBR, DBR, DFR);
                if should_correct_orientation {
                    self.corner_correction(UFR, UBR, DBR, DFR);
                }
            }
        }
    }
    pub fn is_solved(&self) -> bool {
        self.edges == Self::SOLVED_EDGES && self.corners == Self::SOLVED_CORNERS
    }

    fn swap_edges(&mut self, a: EdgePos, b: EdgePos) {
        let tmp = self.edges[a.idx()];
        self.edges[a.idx()] = self.edges[b.idx()];
        self.edges[b.idx()] = tmp;
    }
    fn swap_corners(&mut self, a: CornerPos, b: CornerPos) {
        let tmp = self.corners[a.idx()];
        self.corners[a.idx()] = self.corners[b.idx()];
        self.corners[b.idx()] = tmp;
    }
    fn swap_opposite_edges(&mut self, a: EdgePos, b: EdgePos, c: EdgePos, d: EdgePos) {
        self.swap_edges(a, c);
        self.swap_edges(b, d);
    }
    fn swap_opposite_corners(&mut self, a: CornerPos, b: CornerPos, c: CornerPos, d: CornerPos) {
        self.swap_corners(a, c);
        self.swap_corners(b, d);
    }
    // cycles a -> b -> c -> d -> a
    fn cycle_edges_right(&mut self, a: EdgePos, b: EdgePos, c: EdgePos, d: EdgePos) {
        let tmp = self.edges[d.idx()];
        self.edges[d.idx()] = self.edges[c.idx()];
        self.edges[c.idx()] = self.edges[b.idx()];
        self.edges[b.idx()] = self.edges[a.idx()];
        self.edges[a.idx()] = tmp;
    }

    // cycles a -> b -> c -> d -> a
    fn cycle_corners_right(&mut self, a: CornerPos, b: CornerPos, c: CornerPos, d: CornerPos) {
        let tmp = self.corners[d.idx()];
        self.corners[d.idx()] = self.corners[c.idx()];
        self.corners[c.idx()] = self.corners[b.idx()];
        self.corners[b.idx()] = self.corners[a.idx()];
        self.corners[a.idx()] = tmp;
    }

    // cycles a -> d -> c -> b -> a
    fn cycle_edges_left(&mut self, a: EdgePos, b: EdgePos, c: EdgePos, d: EdgePos) {
        let tmp = self.edges[a.idx()];
        self.edges[a.idx()] = self.edges[b.idx()];
        self.edges[b.idx()] = self.edges[c.idx()];
        self.edges[c.idx()] = self.edges[d.idx()];
        self.edges[d.idx()] = tmp;
    }

    // cycles a -> d -> c -> b -> a
    fn cycle_corners_left(&mut self, a: CornerPos, b: CornerPos, c: CornerPos, d: CornerPos) {
        let tmp = self.corners[a.idx()];
        self.corners[a.idx()] = self.corners[b.idx()];
        self.corners[b.idx()] = self.corners[c.idx()];
        self.corners[c.idx()] = self.corners[d.idx()];
        self.corners[d.idx()] = tmp;
    }

    // Flips edges in the given positions
    fn flip_edges(&mut self, a: EdgePos, b: EdgePos, c: EdgePos, d: EdgePos) {
        self.edges[a.idx()].flip();
        self.edges[b.idx()].flip();
        self.edges[c.idx()].flip();
        self.edges[d.idx()].flip();
    }

    // used for F, R, B, L turns. With white on top and the twisted face in front, CornerPos a, should be top-left, b top-right, c bottom-right, d bottom-left
    // should be applied after corners are cycled
    // corner orientation is based on the white or yellow face being on top / bottom, one being a clockwise twist from that, two being 2 clockwise twists or one counterclockwise twist from that
    fn corner_correction(&mut self, a: CornerPos, b: CornerPos, c: CornerPos, d: CornerPos) {
        self.corners[a.idx()].twist_counterclockwise();
        self.corners[b.idx()].twist_clockwise();
        self.corners[c.idx()].twist_counterclockwise();
        self.corners[d.idx()].twist_clockwise();
    }

    // Turns the current state of the pieces orientation into a unique number
    // One corner and one edge is omitted, because its orientation is determined by the others
    // The specific corner piece is ignored (so colors ignored) only orientation is used
    pub fn get_orientation(&self) -> usize {
        let corner_orient = self.corners.iter().skip(1).enumerate().fold(0, |acc, (i, c)| acc + (c.orientation as usize) * 3usize.pow(i as u32));
        let edge_orient = self.edges.iter().skip(1).enumerate().fold(0, |acc, (i, c)| acc + (c.flipped as usize) * 2usize.pow(i as u32));
        corner_orient + edge_orient * 3usize.pow(7)
    }

    fn get_color(&self, face: Face, sticker: usize) -> char {
        match face {
            Face::Up => self.get_face_color(Face::Up, sticker),
            Face::Down => self.get_face_color(Face::Down, sticker),
            Face::Front => self.get_face_color(Face::Front, sticker),
            Face::Back => self.get_face_color(Face::Back, sticker),
            Face::Left => self.get_face_color(Face::Left, sticker),
            Face::Right => self.get_face_color(Face::Right, sticker),
        }
    }

    fn get_face_color(&self, face: Face, sticker: usize) -> char {
        // Sticker layout:
        // 0 1 2
        // 3 4 5
        // 6 7 8
        
        // Determine which piece and which index within that piece
        match (&face, &sticker) {
            // Up face
            // Corners
            (Face::Up, 0) => self.get_corner_sticker(CornerPos::UBL, 0),
            (Face::Up, 2) => self.get_corner_sticker(CornerPos::UBR, 0),
            (Face::Up, 6) => self.get_corner_sticker(CornerPos::UFL, 0),
            (Face::Up, 8) => self.get_corner_sticker(CornerPos::UFR, 0),
            
            // Edges
            (Face::Up, 1) => self.get_edge_sticker(EdgePos::UB, false),
            (Face::Up, 3) => self.get_edge_sticker(EdgePos::UL, false),
            (Face::Up, 5) => self.get_edge_sticker(EdgePos::UR, false),
            (Face::Up, 7) => self.get_edge_sticker(EdgePos::UF, false),

            // Left face
            // Corners
            (Face::Left, 0) => self.get_corner_sticker(CornerPos::UBL, 1),
            (Face::Left, 2) => self.get_corner_sticker(CornerPos::UFL, 2),
            (Face::Left, 6) => self.get_corner_sticker(CornerPos::DBL, 2),
            (Face::Left, 8) => self.get_corner_sticker(CornerPos::DFL, 1),
            
            // Edges
            (Face::Left, 1) => self.get_edge_sticker(EdgePos::UL, true),
            (Face::Left, 3) => self.get_edge_sticker(EdgePos::BL, true),
            (Face::Left, 5) => self.get_edge_sticker(EdgePos::FL, true),
            (Face::Left, 7) => self.get_edge_sticker(EdgePos::DL, true),

            // Front face
            // Corners
            (Face::Front, 0) => self.get_corner_sticker(CornerPos::UFL, 1),
            (Face::Front, 2) => self.get_corner_sticker(CornerPos::UFR, 2),
            (Face::Front, 6) => self.get_corner_sticker(CornerPos::DFL, 2),
            (Face::Front, 8) => self.get_corner_sticker(CornerPos::DFR, 1),
            
            // Edges
            (Face::Front, 1) => self.get_edge_sticker(EdgePos::UF, true),
            (Face::Front, 3) => self.get_edge_sticker(EdgePos::FL, false),
            (Face::Front, 5) => self.get_edge_sticker(EdgePos::FR, false),
            (Face::Front, 7) => self.get_edge_sticker(EdgePos::DF, true),

            // Right face
            // Corners
            (Face::Right, 0) => self.get_corner_sticker(CornerPos::UFR, 1),
            (Face::Right, 2) => self.get_corner_sticker(CornerPos::UBR, 2),
            (Face::Right, 6) => self.get_corner_sticker(CornerPos::DFR, 2),
            (Face::Right, 8) => self.get_corner_sticker(CornerPos::DBR, 1),
            
            // Edges
            (Face::Right, 1) => self.get_edge_sticker(EdgePos::UR, true),
            (Face::Right, 3) => self.get_edge_sticker(EdgePos::FR, true),
            (Face::Right, 5) => self.get_edge_sticker(EdgePos::BR, true),
            (Face::Right, 7) => self.get_edge_sticker(EdgePos::DR, true),

            // Back face
            // Corners
            (Face::Back, 0) => self.get_corner_sticker(CornerPos::UBR, 1),
            (Face::Back, 2) => self.get_corner_sticker(CornerPos::UBL, 2),
            (Face::Back, 6) => self.get_corner_sticker(CornerPos::DBR, 2),
            (Face::Back, 8) => self.get_corner_sticker(CornerPos::DBL, 1),
            
            // Edges
            (Face::Back, 1) => self.get_edge_sticker(EdgePos::UB, true),
            (Face::Back, 3) => self.get_edge_sticker(EdgePos::BR, false),
            (Face::Back, 5) => self.get_edge_sticker(EdgePos::BL, false),
            (Face::Back, 7) => self.get_edge_sticker(EdgePos::DB, true),
            
            // Down face
            // Corners
            (Face::Down, 0) => self.get_corner_sticker(CornerPos::DFL, 0),
            (Face::Down, 2) => self.get_corner_sticker(CornerPos::DFR, 0),
            (Face::Down, 6) => self.get_corner_sticker(CornerPos::DBL, 0),
            (Face::Down, 8) => self.get_corner_sticker(CornerPos::DBR, 0),
            
            // Edges
            (Face::Down, 1) => self.get_edge_sticker(EdgePos::DF, false),
            (Face::Down, 3) => self.get_edge_sticker(EdgePos::DL, false),
            (Face::Down, 5) => self.get_edge_sticker(EdgePos::DR, false),
            (Face::Down, 7) => self.get_edge_sticker(EdgePos::DB, false),
            
            // centers
            (face, 4) => face.face_color(),
            _ => unreachable!()
        }.to_char()
    }

    // From the specified edge and what face of the edge is wanted the color of that sticker is returnen
    // false meaning the top / bottom color or secondly the front / back color if in middle layer
    // true meaning opposite
    fn get_edge_sticker(&self, pos: EdgePos, sticker_flip: bool) -> Color {
        let edge = self.edges[pos.idx()];
        let (color1, color2) = edge.id.colors();
        
        // Determine which color based on edge orientation and which sticker we want
        if edge.flipped == sticker_flip {color1} else {color2}
    }

    // similiar to get_edge_sticker, sticker_orient is the wanted sticker location where up / down is zero, 1 is clockwise from that and 2 is counterclockwise from top / bottom
    fn get_corner_sticker(&self, pos: CornerPos, sticker_orient: usize) -> Color {
        let corner = self.corners[pos.idx()];
        let (color1, color2, color3) = corner.id.colors();
        
        // Rotate colors based on corner twist
        let colors = [color1, color2, color3];
        let twist_offset = corner.orientation as usize;
        colors[(sticker_orient + 3 - twist_offset) % 3]
    }
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..3 {
            write!(f, "    ")?;
            for col in 0..3 {
                let sticker_idx = row * 3 + col;
                let color_char = self.get_color(Face::Up, sticker_idx);
                write!(f, "{}", color_char)?;
            }
            writeln!(f)?;
        }
        for row in 0..3 {
            for face in &[Face::Left, Face::Front, Face::Right, Face::Back] {
                for col in 0..3 {
                    let sticker_idx = row * 3 + col;
                    let color_char = self.get_color(*face, sticker_idx);
                    write!(f, "{}", color_char)?;
                }
                write!(f, " ")?;
            }
            writeln!(f)?;
        }
        for row in 0..3 {
            write!(f, "    ")?;
            for col in 0..3 {
                let sticker_idx = row * 3 + col;
                let color_char = self.get_color(Face::Down, sticker_idx);
                write!(f, "{}", color_char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


#[derive(PartialEq, Eq, Copy, Clone)]
enum Face {
    Up,
    Left,
    Front,
    Right,
    Back,
    Down
}

impl Face {
    fn face_color(&self) -> Color {
        match self {
            Face::Up => Color::White,
            Face::Left => Color::Orange,
            Face::Front => Color::Green,
            Face::Right => Color::Red,
            Face::Back => Color::Blue,
            Face::Down => Color::Yellow,
        }
    }
}