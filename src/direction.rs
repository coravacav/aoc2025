use crate::grid::Coord;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    None,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "⬆️"),
            Direction::Down => write!(f, "⬇️"),
            Direction::Left => write!(f, "⬅️"),
            Direction::Right => write!(f, "➡️"),
            Direction::UpRight => write!(f, "↗️"),
            Direction::UpLeft => write!(f, "↖️"),
            Direction::DownRight => write!(f, "↘️"),
            Direction::DownLeft => write!(f, "↙️"),
            Direction::None => write!(f, "⍰"),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid char {c}"),
        }
    }
}

impl Direction {
    #[inline(always)]
    pub fn rotate_right_quad(self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::UpRight => Self::DownRight,
            Self::UpLeft => Self::UpRight,
            Self::DownRight => Self::DownLeft,
            Self::DownLeft => Self::UpLeft,
            Self::None => panic!("Cannot rotate None"),
        }
    }

    #[inline(always)]
    pub fn rotate_right(self) -> Direction {
        match self {
            Direction::Up => Direction::UpRight,
            Direction::UpRight => Direction::Right,
            Direction::Right => Direction::DownRight,
            Direction::DownRight => Direction::Down,
            Direction::Down => Direction::DownLeft,
            Direction::DownLeft => Direction::Left,
            Direction::Left => Direction::UpLeft,
            Direction::UpLeft => Direction::Up,
            Self::None => panic!("Cannot rotate None"),
        }
    }

    #[inline(always)]
    pub fn to_coord_offset(self) -> Coord {
        match self {
            Self::Up => Coord::new(-1, 0),
            Self::UpRight => Coord::new(-1, 1),
            Self::UpLeft => Coord::new(-1, -1),
            Self::Down => Coord::new(1, 0),
            Self::DownRight => Coord::new(1, 1),
            Self::DownLeft => Coord::new(1, -1),
            Self::Left => Coord::new(0, -1),
            Self::Right => Coord::new(0, 1),
            Self::None => panic!("Cannot get offset of None"),
        }
    }

    pub fn orthogonal_directions(&self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
            Direction::UpLeft | Direction::DownRight => [Direction::UpRight, Direction::DownLeft],
            Direction::UpRight | Direction::DownLeft => [Direction::UpLeft, Direction::DownRight],
            Direction::None => panic!("Cannot get orthogonal directions of None"),
        }
    }
}

impl std::ops::Add<Coord> for Direction {
    type Output = Coord;

    fn add(self, other: Coord) -> Self::Output {
        other + self.to_coord_offset()
    }
}

impl std::ops::Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, other: Direction) -> Self::Output {
        self + other.to_coord_offset()
    }
}

impl std::ops::AddAssign<Direction> for Coord {
    fn add_assign(&mut self, other: Direction) {
        *self += other.to_coord_offset();
    }
}

impl std::ops::Sub<Direction> for Coord {
    type Output = Coord;

    fn sub(self, other: Direction) -> Self::Output {
        self - other.to_coord_offset()
    }
}

impl std::ops::SubAssign<Direction> for Coord {
    fn sub_assign(&mut self, other: Direction) {
        *self -= other.to_coord_offset();
    }
}
