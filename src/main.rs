mod utils;
// use cursive::view::Margins;
// use cursive::views::{Dialog, EditView, LinearLayout, Panel, TextView};
// use cursive::{immut2, Cursive};
use rand::Rng;
use std::ops::Not;
use std::time::{SystemTime, UNIX_EPOCH};
use utils::{draw_block, draw_circle};
extern crate find_folder;
use piston_window::*;

const BACK_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
const OWN_OFFSET_X: i32 = 3;
const OWN_OFFSET_Y: i32 = 3;
const ENEMY_OFFSET_X: i32 = 18;
const ENEMY_OFFSET_Y: i32 = 3;
const BLOCK_SIZE: f64 = 25.0;
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

    pub fn render_board(&self, con: &Context, g: &mut G2d, x_offset: i32, y_offset: i32) {
        // for row in 0..10 {
        //     for col in 0..10 {
        //         draw_block(color::GREEN, row + x_offset, col + y_offset, con, g);
        //     }
        // }
        for row in 0i32..10 {
            for col in 0i32..10 {
                match self.cells[row as usize][col as usize] {
                    BoardCell::Empty => {
                        draw_block(color::CYAN, row + x_offset, col + y_offset, con, g)
                    }
                    BoardCell::Ship => {
                        draw_block(color::LIME, row + x_offset, col + y_offset, con, g)
                    }
                    BoardCell::DamagedShip => {
                        draw_block(color::RED, row + x_offset, col + y_offset, con, g)
                    }
                    BoardCell::DestroyedShip => {
                        draw_block(color::RED, row + x_offset, col + y_offset, con, g)
                    }
                    BoardCell::FailedAttack => {
                        draw_block(color::NAVY, row + x_offset, col + y_offset, con, g)
                    }
                    BoardCell::SuccessfulAttack => {
                        draw_block(color::RED, row + x_offset, col + y_offset, con, g)
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

#[derive(PartialEq)]
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

    // check winner by looking at total ships remaining
    fn check_winner(&mut self) {
        let mut p1_count = 0;
        let mut p2_count = 0;
        // go over p2 board to check whether they are done
        for x in 0..10 {
            for y in 0..10 {
                if self.ships[1].get_cell_value(Position { x, y }) == BoardCell::Ship {
                    p2_count += 1;
                }
                if self.ships[0].get_cell_value(Position { x, y }) == BoardCell::Ship {
                    p1_count += 1;
                }
            }
        }

        if p2_count == 0 {
            self.status = GameStatus::Complete(PlayerID::P1);
        } else if p1_count == 0 {
            self.status = GameStatus::Complete(PlayerID::P2);
        }
    }

    fn do_attack(
        attack_board: &mut Board,
        target_board: &mut Board,
        attack_at: Position,
        animations: &mut Vec<Animation>,
    ) -> bool {
        let target_cell = target_board.get_cell_value(attack_at);
        animations.push(Animation {
            time_remaining: 500.0,
            position: attack_at,
        });
        if target_cell == BoardCell::Ship {
            attack_board.set_cell(attack_at, BoardCell::SuccessfulAttack);
            target_board.set_cell(attack_at, BoardCell::DamagedShip);
            true
        } else {
            attack_board.set_cell(attack_at, BoardCell::FailedAttack);
            false
        }
    }

    pub fn attack(
        &mut self,
        player: PlayerID,
        pos: Position,
        animations: &mut Vec<Animation>,
    ) -> bool {
        match player {
            PlayerID::P1 => {
                GameState::do_attack(&mut self.attacks[0], &mut self.ships[1], pos, animations)
            }
            PlayerID::P2 => {
                GameState::do_attack(&mut self.attacks[1], &mut self.ships[0], pos, animations)
            }
        }
    }

    pub fn randomly_attack(&mut self, player: PlayerID, animations: &mut Vec<Animation>) {
        let mut random_pos = Position::random();
        // check if random position is taken
        match player {
            PlayerID::P1 => {
                // check and only allow attacks on empty cell
                while self.attacks[0].get_cell_value(random_pos) != BoardCell::Empty {
                    random_pos = Position::random();
                }
                self.attack(player, random_pos, animations);
            }
            PlayerID::P2 => {
                while self.attacks[1].get_cell_value(random_pos) != BoardCell::Empty {
                    random_pos = Position::random();
                }
                self.attack(player, random_pos, animations);
            }
        }
    }

    pub fn start(&mut self) {
        self.status = GameStatus::InProgress;
        self.place_randomly(PlayerID::P1);
        self.place_randomly(PlayerID::P2);
    }
}

// fn game_over( winner: PlayerID) {
//     let winner_text = match winner {
//         PlayerID::P1 => "Player 1 Wins!",
//         PlayerID::P2 => "Player 2 Wins!",
//     };
//     // s.pop_layer();
//     // s.add_layer(
//     //     Dialog::text(winner_text)
//     //         .title("Game Over")
//     //         .button("OK", |s| choose_game_type(s)),
//     // )
// }

// helper method to render a game state
fn render(con: &Context, g: &mut G2d, glyphs: &mut Glyphs, game: &mut GameState) {
    // draw the grid

    
    game.ships[0].render_board(con, g, OWN_OFFSET_X, OWN_OFFSET_Y);
    game.attacks[0].render_board(con, g, ENEMY_OFFSET_X, ENEMY_OFFSET_Y);
    // render text for the boards
    let mut transform = con.transform.trans(BLOCK_SIZE * 5.5, BLOCK_SIZE * 16.0 );
    text::Text::new_color(color::GRAY, 20)
        .draw("Your board", glyphs, &con.draw_state, transform, g)
        .unwrap();
    transform = con.transform.trans(BLOCK_SIZE * 20.5, BLOCK_SIZE * 16.0);
    text::Text::new_color(color::GRAY, 20)
        .draw("Enemy board", glyphs, &con.draw_state, transform, g)
        .unwrap();
    transform = con.transform.trans(BLOCK_SIZE * 11.0, BLOCK_SIZE * 2.0);
    text::Text::new_color(color::WHITE, 32)
        .draw("Battle ship game", glyphs, &con.draw_state, transform, g)
        .unwrap();
    transform = con.transform.trans(BLOCK_SIZE * 2.0, BLOCK_SIZE * 18.0);
    // paint the text
    text::Text::new_color(color::WHITE,15)
        .draw("* Click on enemy board's grid to attack", glyphs, &con.draw_state, transform, g)
        .unwrap();
}

// render the animations for dropping attack
fn render_animations(
    con: &Context,
    g: &mut G2d,
    animations: &mut Vec<Animation>,
    offset_x: i32,
    offset_y: i32,
) {
    for &animation in animations.clone().iter() {
        let color = [0.0, 0.0, 0.0, (500.0 - animation.time_remaining) / 500.0];
        draw_circle(
            color,
            animation.position.x as i32 + offset_x,
            animation.position.y as i32 + offset_y,
            con,
            g,
        );
    }
}

fn render_winning_screen(con: &Context, g: &mut G2d, glyphs: &mut Glyphs, winner: PlayerID) {
    let message = match winner {
        PlayerID::P1 => "You won the game! :)",
        PlayerID::P2 => "You lost the game :(",
    };
    let mut transform = con.transform.trans(BLOCK_SIZE * 9.0, BLOCK_SIZE * 8.0);
    text::Text::new_color(color::WHITE, 30)
        .draw(message, glyphs, &con.draw_state, transform, g)
        .unwrap();
    transform = con.transform.trans(BLOCK_SIZE * 9.0, BLOCK_SIZE * 11.0);
    text::Text::new_color(color::WHITE, 30)
        .draw(
            "Click anywhere to restart",
            glyphs,
            &con.draw_state,
            transform,
            g,
        )
        .unwrap();
}

#[derive(Copy, Clone)]
struct Animation {
    time_remaining: f32,
    position: Position,
}

fn main() {
    let (width, height) = (30, 20);
    let mut last_time = SystemTime::now();

    let mut window: PistonWindow = WindowSettings::new(
        "Battleship game",
        [
            ((width as f64) * BLOCK_SIZE) as u32,
            ((height as f64) * BLOCK_SIZE) as u32,
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
    // instantiate vecotr for storing animation of dropping
    let mut own_board_animations: Vec<Animation> = Vec::new();
    let mut enemy_board_animations: Vec<Animation> = Vec::new();

    while let Some(event) = window.next() {
        let current_time = SystemTime::now();
        let duration_passed = current_time
            .duration_since(last_time)
            .expect("Time went backwards")
            .as_millis();
        last_time = current_time;

        // clear the window
        // custom draw method to rerender everything
        // the draw becoems the render method

        if let Some(pos) = event.mouse_cursor_args() {
            mouse = pos;
        }
        if let Some(button) = event.press_args() {
            // Check that coordinates are inside board boundaries.
            if button == Button::Mouse(MouseButton::Left) {
                if game.status == GameStatus::InProgress {
                    // calculate if we are at a board location
                    // check if it's on enemy board
                    let x_grid = (mouse[0] / BLOCK_SIZE).floor() - ENEMY_OFFSET_X as f64;
                    let y_grid = (mouse[1] / BLOCK_SIZE).floor() - ENEMY_OFFSET_Y as f64;
                    if x_grid >= 0.0 && x_grid >= 0.0 && x_grid < 10.0 && y_grid < 10.0 {
                        // check whether the placed was already attacked
                        if game.attacks[0].get_cell_value(Position {
                            x: x_grid as u8,
                            y: y_grid as u8,
                        }) == BoardCell::Empty
                        {
                            game.attack(
                                PlayerID::P1,
                                Position {
                                    x: x_grid as u8,
                                    y: y_grid as u8,
                                },
                                &mut enemy_board_animations,
                            );

                            // also have reaction for the attack
                            game.randomly_attack(PlayerID::P2, &mut own_board_animations);
                            // after attack check if game is over check p2 first then p1 since p1 attacks first
                            game.check_winner();
                        } else {
                            // show already attacked
                            println!("already attacked");
                        }
                    } 
                } else {
                    // restart the game
                    game = GameState::initialize();
                    game.start();
                }
                // check if the game has ended which means either side has no ship left
            }
        }
        // // update the animation time
        for animation in &mut own_board_animations {
            animation.time_remaining -= duration_passed as f32;
        }
        for animation in &mut enemy_board_animations {
            animation.time_remaining -= duration_passed as f32;
        }
        // filter out ones that are below 0
        own_board_animations.retain(|animation| animation.time_remaining > 0.0);
        enemy_board_animations.retain(|animation| animation.time_remaining > 0.0);

        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            // check the current game state and render accordingly
            match game.status {
                GameStatus::InProgress => {
                    render(&c, g, &mut glyphs, &mut game);
                    render_animations(&c, g, &mut own_board_animations, OWN_OFFSET_X, OWN_OFFSET_Y);
                    render_animations(
                        &c,
                        g,
                        &mut enemy_board_animations,
                        ENEMY_OFFSET_X,
                        ENEMY_OFFSET_Y,
                    );
                }
                GameStatus::Complete(winner) => {
                    render_winning_screen(&c, g, &mut glyphs, winner);
                }
                GameStatus::NotStarted => {
                    // should be ignored but render board anyways
                    render(&c, g, &mut glyphs, &mut game);
                }
            }
            glyphs.factory.encoder.flush(device);
        });
        // print time
    }
    // let mut siv = cursive::default();
    // siv.add_global_callback('q', |s| s.quit());

    // // let mut game = GameState::initialize();

    // // single_player_loop(&mut siv, &mut game);
    // choose_game_type(&mut siv);
    // siv.run();
}
