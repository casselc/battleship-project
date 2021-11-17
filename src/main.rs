use cursive::views::{Dialog, LinearLayout, TextContent, TextView};
use cursive::Cursive;
use rand::Rng;
use std::ops::Not;
use std::process::Output;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    pub fn random() -> Self {
        let mut r = rand::thread_rng();
        Position {
            x: r.gen_range(0..10),
            y: r.gen_range(0..10),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn random() -> Self {
        let mut r = rand::thread_rng();
        if r.gen_bool(0.5) {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        }
    }
}

impl Not for Orientation {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Orientation::Horizontal => Orientation::Vertical,
            Orientation::Vertical => Orientation::Horizontal,
        }
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
enum ShipStatus {
    Undamaged,
    Damaged(u8),
    Destroyed,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum ShipKind {
    Patrol,
    Submarine,
    Destroyer,
    Battleship,
    Carrier,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum BoardCell {
    Empty,
    Ship,
    DamagedShip,
    DestroyedShip,
    FailedAttack,
    SuccessfulAttack,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Board {
    cells: [[BoardCell; 10]; 10],
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[BoardCell::Empty; 10]; 10],
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Ship {
    name: &'static str,
    size: u8,
    orientation: Orientation,
    position: Vec<Position>,
    status: ShipStatus,
}

impl Ship {
    pub fn new(kind: ShipKind) -> Self {
        match kind {
            ShipKind::Patrol => Self {
                name: "Patrol Boat",
                size: 2,
                orientation: Orientation::Horizontal,
                position: vec![],
                status: ShipStatus::Undamaged,
            },
            ShipKind::Submarine => Self {
                name: "Submarine",
                size: 3,
                orientation: Orientation::Horizontal,
                position: vec![],
                status: ShipStatus::Undamaged,
            },
            ShipKind::Destroyer => Self {
                name: "Destroyer",
                size: 3,
                orientation: Orientation::Horizontal,
                position: vec![],
                status: ShipStatus::Undamaged,
            },
            ShipKind::Battleship => Self {
                name: "Battleship",
                size: 4,
                orientation: Orientation::Horizontal,
                position: vec![],
                status: ShipStatus::Undamaged,
            },
            ShipKind::Carrier => Self {
                name: "Aircraft Carrier",
                size: 5,
                orientation: Orientation::Horizontal,
                position: vec![],
                status: ShipStatus::Undamaged,
            },
        }
    }

    pub fn place_at(&mut self, pos: Position, orient: Orientation) -> &mut Self {
        self.orientation = orient;
        self.position.clear();
        self.position = (0..self.size)
            .into_iter()
            .map(|i| match self.orientation {
                Orientation::Horizontal => Position {
                    x: pos.x + i,
                    y: pos.y,
                },
                Orientation::Vertical => Position {
                    x: pos.x,
                    y: pos.y + i,
                },
            })
            .collect();

        self
    }

    pub fn positions(&self) -> Vec<Position> {
        self.position.clone()
    }
    pub fn overlaps(&self, pos: Position) -> bool {
        self.position.iter().any(|&p| p == pos)
    }
}

struct Player {
    ships: Vec<Ship>,
    attacks_made: Vec<Position>,
    attacks_seen: Vec<Position>,
}

impl Player {
    pub fn new() -> Self {
        let mut p = Player {
            ships: vec![
                Ship::new(ShipKind::Patrol),
                Ship::new(ShipKind::Submarine),
                Ship::new(ShipKind::Destroyer),
                Ship::new(ShipKind::Battleship),
                Ship::new(ShipKind::Carrier),
            ],
            attacks_made: vec![],
            attacks_seen: vec![],
        };

        p.place_randomly();
        p
    }

    fn place_randomly(&mut self) {
        let ships = self.ships.clone();
        for mut s in ships {
            let mut placed = false;

            while !placed {
                let p = Position::random();
                let o = Orientation::random();
                placed = self.place_ship(&mut s, p, o) || self.place_ship(&mut s, p, !o)
            }
        }
    }

    fn place_ship(&mut self, ship: &mut Ship, pos: Position, orient: Orientation) -> bool {
        ship.place_at(pos, orient);
        let ref ships = self.ships;
        let out_of_bounds = ship.positions().into_iter().any(|p| p.x > 9 || p.y > 9);
        let overlaps = ships
            .into_iter()
            .filter(|&s| !s.name.eq(ship.name))
            .flat_map(|s| s.positions())
            .any(|p| ship.overlaps(p));

        !(overlaps || out_of_bounds)
    }

    pub fn ships(&self) -> &Vec<Ship> {
        &self.ships
    }
}

enum GameStatus {
    NotStarted,
    InProgress,
    Complete,
}

struct GameState {
    p1: Player,
    p2: Player,

    pub p1_attack_board: Board,
    p1_ship_board: Board,
    p2_attack_board: Board,
    p2_ship_board: Board,

    status: GameStatus,
}

impl GameState {
    pub fn player_one(&mut self) -> &mut Player {
        &mut self.p1
    }
    pub fn player_two(&mut self) -> &mut Player {
        &mut self.p2
    }
    pub fn initialize() -> Self {
        let state = GameState {
            p1: Player::new(),
            p1_attack_board: Board::new(),
            p1_ship_board: Board::new(),
            p2: Player::new(),
            p2_attack_board: Board::new(),
            p2_ship_board: Board::new(),
            status: GameStatus::NotStarted,
        };
        state
    }

    pub fn p1_attack(&mut self, pos: Position) -> bool {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let cell = self.p2_ship_board.cells[x][y];
        match cell {
            BoardCell::Ship => {
                self.p2_ship_board.cells[x][y] = BoardCell::DamagedShip;
                self.p1_attack_board.cells[x][y] = BoardCell::SuccessfulAttack;
                true
            }
            BoardCell::DamagedShip => true,
            _ => {
                self.p1_attack_board.cells[x][y] = BoardCell::FailedAttack;
                false
            }
        }
    }

    pub fn p2_attack(&mut self, pos: Position) -> bool {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let cell = self.p1_ship_board.cells[x][y];
        match cell {
            BoardCell::Ship => {
                self.p1_ship_board.cells[x][y] = BoardCell::DamagedShip;
                self.p2_attack_board.cells[x][y] = BoardCell::SuccessfulAttack;
                true
            }
            BoardCell::DamagedShip => true,
            _ => {
                self.p2_attack_board.cells[x][y] = BoardCell::FailedAttack;
                false
            }
        }
    }
    pub fn draw_p1_attacks(&self) -> String {
        GameState::board_string(self.p1_attack_board)
    }

     fn board_string(board: Board) -> String {
        let mut out = String::new();
        for row in 0usize..10 {
            for col in 0usize..10 {
                match board.cells[row][col] {
                    BoardCell::Empty => out.push_str("🟦"),
                    BoardCell::Ship =>out.push_str("🟩"),
                    BoardCell::DamagedShip => out.push_str("🟥"),
                    BoardCell::DestroyedShip => out.push_str("🟥"),
                    BoardCell::FailedAttack => out.push_str("🟪"),
                    BoardCell::SuccessfulAttack => out.push_str("🟥"),
                }
            }
            out.push_str("\n");
        }
        out

}

fn game_over(s: &mut Cursive) {}
fn single_player_loop(s: &mut Cursive, game: &mut GameState) {
    s.pop_layer();
    match game.status {
        GameStatus::InProgress => s.add_layer(
            Dialog::around(LinearLayout::vertical().child(TextView::new(TextContent::new(game.draw_p1_attacks()))))
                .title("Next Attack")
                .button("Done", |s| s.quit())
                .button("Cancel", |s| choose_game_type(s)),
        ),
        GameStatus::Complete => game_over(s),
        GameStatus::NotStarted => game.status = GameStatus::InProgress,
    }
}
fn start_single_player(s: &mut Cursive) {
    let mut game = GameState::initialize();
    single_player_loop(s, &mut game)
}

fn choose_game_type(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Start a new game\nHow many players?")
            .title("Battleship")
            .button("1 Player", |s| start_single_player(s))
            .button("2 Players", |s| {
                s.add_layer(Dialog::info("Not implemented."))
            }),
    );
}
fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    choose_game_type(&mut siv);
    siv.run();
}
