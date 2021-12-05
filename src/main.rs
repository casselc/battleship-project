mod utils;
// use cursive::view::Margins;
// use cursive::views::{Dialog, EditView, LinearLayout, Panel, TextView};
// use cursive::{immut2, Cursive};
use piston_window::color::GREEN;
use rand::Rng;
use std::ops::Not;
use utils::{draw_block, draw_rectange};
extern crate find_folder;
use piston_window::*;

const BACK_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
// set the colors representing different statesx`

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

    pub fn from_board_coords(coords: &str) -> Self {
        // TODO: translate A1, etc to 0,0
        Position::random()
    }

    pub fn overlaps(&self, positions: &Vec<Position>) -> bool {
        positions.contains(self)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum ShipOrientation {
    Horizontal,
    Vertical,
}

impl ShipOrientation {
    pub fn random() -> Self {
        let mut r = rand::thread_rng();
        if r.gen_bool(0.5) {
            ShipOrientation::Horizontal
        } else {
            ShipOrientation::Vertical
        }
    }
}

impl Not for ShipOrientation {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            ShipOrientation::Horizontal => ShipOrientation::Vertical,
            ShipOrientation::Vertical => ShipOrientation::Horizontal,
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

    pub fn board_string(&self) -> String {
        let mut out = String::new();
        out.push_str("   1 2 3 4 5 6 7 8 9 10\n");
        for row in 0usize..10 {
            out.push(('A' as u8 + row as u8) as char);
            out.push(' ');
            for col in 0usize..10 {
                match self.cells[row][col] {
                    BoardCell::Empty => out.push_str("🟦"),
                    BoardCell::Ship => out.push_str("🟩"),
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

    pub fn render_board(
        &self,
        con: &Context,
        g: &mut G2d,
        x_offset: i32,
        y_offset: i32,
    ) {
        // for row in 0..10 {
        //     for col in 0..10 {
        //         draw_block(color::GREEN, row + x_offset, col + y_offset, con, g);
        //     }
        // }
        for row in 0i32..10 {
            for col in 0i32..10 {
                match self.cells[row as usize][col as usize] {
                    BoardCell::Empty => {
                        draw_block(color::BLUE, row + x_offset, col + y_offset, con, g)
                    }
                    BoardCell::Ship => {
                        draw_block(color::LIME, row + x_offset, col + y_offset, con, g)
                    }
                    BoardCell::DamagedShip => {
                        draw_block(color::YELLOW, row + x_offset, col + y_offset, con, g)
                    }
                    BoardCell::DestroyedShip => {
                        draw_block(color::RED, row + x_offset, col + y_offset, con, g)
                    }
                    BoardCell::FailedAttack => {
                        draw_block(color::NAVY, row + x_offset, col + y_offset, con, g)
                    }
                    BoardCell::SuccessfulAttack => {
                        draw_block(color::GREEN, row + x_offset, col + y_offset, con, g)
                    }
                }
            }
        }
    }

    pub fn set_cell(&mut self, pos: Position, value: BoardCell) {
        self.cells[pos.x as usize][pos.y as usize] = value;
    }

    pub fn get_cell_value(&self, pos: Position) -> BoardCell {
        self.cells[pos.x as usize][pos.y as usize].clone()
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Ship {
    kind: ShipKind,
    size: u8,
    position: Vec<Position>,
    status: ShipStatus,
}

impl Ship {
    pub fn new(kind: ShipKind) -> Self {
        match kind {
            ShipKind::Patrol => Self {
                kind,
                size: 2,
                position: vec![],
                status: ShipStatus::Undamaged,
            },
            ShipKind::Submarine => Self {
                kind,
                size: 3,
                position: vec![],
                status: ShipStatus::Undamaged,
            },
            ShipKind::Destroyer => Self {
                kind,
                size: 3,
                position: vec![],
                status: ShipStatus::Undamaged,
            },
            ShipKind::Battleship => Self {
                kind,
                size: 4,
                position: vec![],
                status: ShipStatus::Undamaged,
            },
            ShipKind::Carrier => Self {
                kind,
                size: 5,
                position: vec![],
                status: ShipStatus::Undamaged,
            },
        }
    }

    pub fn place_at(&mut self, pos: Position, orient: ShipOrientation) -> &mut Self {
        self.position.clear();
        self.position = (0..self.size)
            .into_iter()
            .map(|i| match orient {
                ShipOrientation::Horizontal => Position {
                    x: pos.x + i,
                    y: pos.y,
                },
                ShipOrientation::Vertical => Position {
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
    pub fn intersects(&self, positions: &Vec<Position>) -> bool {
        self.position.iter().any(|&p| p.overlaps(positions))
    }
}

struct Player {
    ships: Vec<Ship>,
    attacks_made: Vec<Position>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            ships: vec![],
            attacks_made: vec![],
        }
    }

    pub fn all_ship_positions(&self) -> Vec<Position> {
        self.ships()
            .into_iter()
            .flat_map(|s| s.positions())
            .collect()
    }

    pub fn add_ship(&mut self, ship: Ship) {
        self.ships.push(ship)
    }

    pub fn ships(&self) -> &Vec<Ship> {
        &self.ships
    }
}

enum GameStatus {
    NotStarted,
    InProgress,
    Complete(PlayerID),
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum PlayerID {
    P1,
    P2,
}

struct GameState {
    players: Vec<Player>,
    ships: Vec<Board>,
    attacks: Vec<Board>,
    // p1: Player,
    // p2: Player,
    // p1_attack_board: Board,
    // p1_ship_board: Board,
    // p2_attack_board: Board,
    // p2_ship_board: Board,
    status: GameStatus,
}

impl GameState {
    pub fn initialize() -> Self {
        GameState {
            players: vec![Player::new(), Player::new()],
            attacks: vec![Board::new(), Board::new()],
            ships: vec![Board::new(), Board::new()],
            status: GameStatus::NotStarted,
        }
    }

    fn place_randomly(&mut self, player: PlayerID) {
        for kind in vec![
            ShipKind::Submarine,
            ShipKind::Patrol,
            ShipKind::Destroyer,
            ShipKind::Battleship,
            ShipKind::Carrier,
        ] {
            let mut placed = false;

            while !placed {
                let p = Position::random();
                let o = ShipOrientation::random();
                placed = self.try_place_ship(&player, kind, p, o);
            }
        }
    }

    fn try_place_ship(
        &mut self,
        player: &PlayerID,
        kind: ShipKind,
        pos: Position,
        orient: ShipOrientation,
    ) -> bool {
        let p = match player {
            PlayerID::P1 => &mut self.players[0],
            PlayerID::P2 => &mut self.players[1],
        };

        let b = match player {
            PlayerID::P1 => &mut self.ships[0],
            PlayerID::P2 => &mut self.ships[1],
        };
        let mut ship = Ship::new(kind);
        ship.place_at(pos, orient);

        let out_of_bounds = ship.positions().into_iter().any(|p| p.x > 9 || p.y > 9);
        let overlaps = ship.intersects(&p.all_ship_positions());

        let valid = !(overlaps || out_of_bounds);

        if valid {
            p.add_ship(ship);
            for pos in p.all_ship_positions() {
                b.set_cell(pos, BoardCell::Ship)
            }
        }

        valid
    }

    fn do_attack(attack_board: &mut Board, target_board: &mut Board, attack_at: Position) -> bool {
        let target_cell = target_board.get_cell_value(attack_at);

        if target_cell == BoardCell::Ship {
            attack_board.set_cell(attack_at, BoardCell::SuccessfulAttack);
            target_board.set_cell(attack_at, BoardCell::DamagedShip);
            true
        } else {
            attack_board.set_cell(attack_at, BoardCell::FailedAttack);
            false
        }
    }

    pub fn attack(&mut self, player: PlayerID, pos: Position) -> bool {
        match player {
            PlayerID::P1 => GameState::do_attack(&mut self.attacks[0], &mut self.ships[1], pos),
            PlayerID::P2 => GameState::do_attack(&mut self.attacks[1], &mut self.ships[0], pos),
        }
    }

    pub fn start(&mut self) {
        self.status = GameStatus::InProgress;
        self.place_randomly(PlayerID::P1);
        self.place_randomly(PlayerID::P2);
    }
}

fn game_over( winner: PlayerID) {
    let winner_text = match winner {
        PlayerID::P1 => "Player 1 Wins!",
        PlayerID::P2 => "Player 2 Wins!",
    };
    // s.pop_layer();
    // s.add_layer(
    //     Dialog::text(winner_text)
    //         .title("Game Over")
    //         .button("OK", |s| choose_game_type(s)),
    // )
}

fn single_player_loop(game: &mut GameState) {
    // s.pop_layer();
    match game.status {
        GameStatus::InProgress => {
            // let next_attack = EditView::new();
            // s.add_layer(
            //     Dialog::around(
            //         LinearLayout::vertical()
            //             .child(
            //                 LinearLayout::horizontal()
            //                     .child(
            //                         Panel::new(TextView::new(game.ships[0].board_string()))
            //                             .title("Ships"),
            //                     )
            //                     .child(
            //                         Panel::new(TextView::new(game.attacks[0].board_string()))
            //                             .title("Attacks"),
            //                     ),
            //             )
            //             .child(next_attack),
            //     )
            //     .padding(Margins::lrtb(1, 1, 1, 1))
            //     .title("Enter Attack Location")
            //     .button("Quit", |s| s.quit())
            //     .button("Restart", |s| choose_game_type(s)),
            // )
        }
        GameStatus::Complete(winner) => game_over(winner),
        GameStatus::NotStarted => {
            game.start();
            single_player_loop(game)
        }
    }
}

fn start_single_player() {
    let mut game = GameState::initialize();
    single_player_loop(&mut game)
}

// fn choose_game_type(s: &mut Cursive) {
//     s.pop_layer();
//     s.add_layer(
//         Dialog::text("Start a new game\nHow many players?")
//             .title("Battleship")
//             .button("1 Player", |s| start_single_player(s))
//             .button("2 Players", |s| {
//                 s.add_layer(Dialog::info("Not implemented."))
//             })
//             .button("Quit", |s| s.quit()),
//     );
// }

// helper method to render a game state
fn render(con: &Context, g: &mut G2d, glyphs: &mut Glyphs, game: &mut GameState) {
    // draw the grid
    game.ships[0].render_board(con, g, 1, 1);
    game.attacks[0].render_board(con, g, 20, 1);
    let transform = con.transform.trans(25.0 * 13.0, 25.0);
    // paint the text
    text::Text::new_color(color::WHITE, 20)
        .draw("Battle ship game", glyphs, &con.draw_state, transform, g)
        .unwrap();
}
fn main() {
    let (width, height) = (40, 20);

    let mut window: PistonWindow = WindowSettings::new(
        "CMSC388Z Snake Game",
        [
            ((width as f64) * 25.0) as u32,
            ((height as f64) * 25.0) as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut mouse = [0.0, 0.0];
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();
    let mut game = GameState::initialize();
    game.start();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // press key reaction for the board?
            // game.key_pressed(key);
        }

        // clear the window
        // custom draw method to rerender everything
        // the draw becoems the render method
        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            // draw the board and character replace with render method with things passed in
            render(&c, g, &mut glyphs, &mut game);
            glyphs.factory.encoder.flush(device);
        });

        if let Some(pos) = event.mouse_cursor_args() {
            mouse = pos;
        }
        if let Some(button) = event.press_args() {
            // Find coordinates relative to upper left corner.
            //let x = event.cursor_pos[0] - pos[0];
            //let y = event.cursor_pos[1] - pos[1];
            // Check that coordinates are inside board boundaries.
            if button == Button::Mouse(MouseButton::Left) {
                // calculate if we are at a board location
                println!(
                    "cell location is {} {}",
                    (mouse[0] / 25.0).floor(),
                    (mouse[1] / 25.0).floor()
                );
            }
        }
        // update the arguments
        // event.update(|arg| {
        //     game.update(arg.dt);
        // });
    }
    // let mut siv = cursive::default();
    // siv.add_global_callback('q', |s| s.quit());

    // // let mut game = GameState::initialize();

    // // single_player_loop(&mut siv, &mut game);
    // choose_game_type(&mut siv);
    // siv.run();
}
