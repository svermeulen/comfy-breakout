use comfy::*;

mod assets;
mod math;

pub use crate::math::*;

simple_game!("Breakout", GameState, config, setup, update);

static WORLD_WIDTH: f32 = 30.0;
static WORLD_HEIGHT: f32 = WORLD_WIDTH * 16.0 / 9.0;

static PADDLE_WIDTH: f32 = 5.0;
static PADDLE_HEIGHT: f32 = 1.5;
static PADDLE_MOVE_SPEED: f32 = 40.0;
static PADDLE_BOTTOM_MARGIN: f32 = 3.0;

static BALL_MOVE_SPEED: f32 = 35.0;
static BALL_RADIUS: f32 = 0.5;

static BLOCK_WIDTH: f32 = 3.0;
static BLOCK_HEIGHT: f32 = 1.5;

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        bloom_enabled: true,
        resolution: ResolutionConfig::Physical(500, 500 * 16 / 9),
        min_resolution: ResolutionConfig::Physical(100, 100 * 16 / 9),
        ..config
    }
}

pub enum State {
    Game,
    Menu,
}

pub struct GameState {
    pub ball: Ball,
    pub paddle: Paddle,
    pub level: Level,
    pub state: State,
}

fn generate_level() -> Level {
    let mut blocks = vec![];

    let gap = 0.4;
    let num_blocks_per_row = 4;
    let num_blocks_per_col = 8;
    let cell_size = vec2(BLOCK_WIDTH + BLOCK_HEIGHT + gap, BLOCK_HEIGHT + gap);
    let row_size = num_blocks_per_row as f32 * cell_size.x - gap;
    let col_size = num_blocks_per_col as f32 * cell_size.y - gap;
    let top_margin = 5.0;

    let start = vec2(
        -WORLD_WIDTH * 0.5 + (WORLD_WIDTH - row_size) * 0.5,
        WORLD_HEIGHT * 0.5 - top_margin - col_size,
    );

    for y in 0..num_blocks_per_col {
        for x in 0..num_blocks_per_row {
            blocks.push(Block {
                center: start
                    + vec2(
                        BLOCK_HEIGHT * 0.5 + BLOCK_WIDTH * 0.5 + (x as f32) * cell_size.x,
                        BLOCK_HEIGHT * 0.5 + (y as f32) * cell_size.y,
                    ),
            });
        }
    }

    return Level { blocks };
}

impl GameState {
    pub fn new(_c: &mut EngineContext) -> Self {
        Self {
            state: State::Menu,
            ball: Ball {
                center: Vec2::new(0.0, 0.0),
                velocity: Vec2::new(BALL_MOVE_SPEED, BALL_MOVE_SPEED),
            },
            paddle: Paddle {
                center: Vec2::new(
                    0.0,
                    -WORLD_HEIGHT * 0.5 + 0.5 * PADDLE_HEIGHT + PADDLE_BOTTOM_MARGIN,
                ),
            },
            level: generate_level(),
        }
    }
}

pub struct Ball {
    pub center: Vec2,
    pub velocity: Vec2,
}

pub struct Block {
    pub center: Vec2,
}

pub struct Level {
    pub blocks: Vec<Block>,
}

pub struct Paddle {
    pub center: Vec2,
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {
    crate::assets::load_assets();
}

fn move_paddle(state: &mut GameState, time_delta: f32) {
    let move_dir: f32;

    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        move_dir = -1.0;
    } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        move_dir = 1.0;
    } else {
        return;
    }

    let move_jump = PADDLE_MOVE_SPEED * time_delta * move_dir;
    let mut new_pos = state.paddle.center.x + move_jump;

    let min_x = -WORLD_WIDTH * 0.5 - PADDLE_WIDTH * 0.5 - PADDLE_HEIGHT * 0.25;
    let max_x = WORLD_WIDTH * 0.5 + PADDLE_WIDTH * 0.5 + PADDLE_HEIGHT * 0.25;

    if new_pos < min_x {
        new_pos = min_x
    }

    if new_pos > max_x {
        new_pos = max_x
    }

    state.paddle.center.x = new_pos;
}

fn draw_capsule(color: Color, z_index: i32, position: Vec2, size: Vec2) {
    draw_rect(position, size, color, z_index);

    let radius = size.y * 0.5;
    let extent = vec2(1.0, 0.0) * size.x * 0.5;
    draw_arc(
        position - extent,
        radius,
        PI * 0.5,
        3.0 * PI / 2.0,
        color,
        z_index,
    );
    draw_arc(
        position + extent,
        radius,
        -PI * 0.5,
        PI * 0.5,
        color,
        z_index,
    );
}

fn draw_level(state: &mut GameState) {
    let color = Color::rgb8(50, 50, 250);

    for block in state.level.blocks.iter() {
        draw_capsule(color, 1, block.center, Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT));
    }
}

fn draw_paddle(state: &mut GameState) {
    let color = Color::rgb8(50, 250, 50);
    draw_capsule(
        color,
        1,
        state.paddle.center,
        Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
    );
}

fn draw_game(state: &mut GameState) {
    draw_sprite_ex(
        texture_id("ball1"),
        state.ball.center,
        WHITE,
        2,
        DrawTextureParams {
            dest_size: Some(splat(2.0 * BALL_RADIUS).as_world_size()),
            ..Default::default()
        },
    );

    draw_paddle(state);
    draw_level(state);
}

fn collide_ball_with_capsule(
    capsule_start: Vec2,
    capsule_end: Vec2,
    capsule_radius: f32,
    ball: &mut Ball,
    ball_new_pos: &mut Vec2,
) -> bool {
    let (p1, p2) =
        closest_points_on_segments(ball.center, *ball_new_pos, capsule_start, capsule_end);

    let line_distance = (p1 - p2).length();

    if line_distance >= BALL_RADIUS + capsule_radius {
        return false;
    }

    let normal: Vec2;

    if line_distance < 1e-8 {
        // lines intersect
        // TODO: This only works because the capsules are horizontal
        if ball.center.y > capsule_start.y {
            normal = vec2(0.0, -1.0);
        } else {
            normal = vec2(0.0, 1.0);
        }
    } else {
        normal = (p1 - p2).normalize();
    }

    if ball.velocity.dot(normal) > 0.0 {
        return false;
    }

    *ball_new_pos = p2 + normal * (BALL_RADIUS + capsule_radius);

    let mut ball_velocity = ball.velocity;
    let dot = ball_velocity.dot(normal);
    ball_velocity = ball_velocity - 2.0 * dot * normal;

    ball.velocity = ball_velocity;
    play_sound("ball-hit");
    return true;
}

fn collide_ball_with_paddle(state: &mut GameState, new_pos: &mut Vec2) {
    let paddle_line_start = state.paddle.center - vec2(PADDLE_WIDTH * 0.5, 0.0);
    let paddle_line_end = state.paddle.center + vec2(PADDLE_WIDTH * 0.5, 0.0);
    let paddle_radius = PADDLE_HEIGHT * 0.5;

    collide_ball_with_capsule(
        paddle_line_start,
        paddle_line_end,
        paddle_radius,
        &mut state.ball,
        new_pos,
    );
}

fn collide_ball_with_level(state: &mut GameState, new_pos: &mut Vec2) {
    let mut destroyed_blocks: Vec<usize> = vec![];

    for (i, block) in state.level.blocks.iter().enumerate().rev() {
        let line_start = block.center - vec2(BLOCK_WIDTH * 0.5, 0.0);
        let line_end = block.center + vec2(BLOCK_WIDTH * 0.5, 0.0);
        let radius = BLOCK_HEIGHT * 0.5;

        if collide_ball_with_capsule(line_start, line_end, radius, &mut state.ball, new_pos) {
            destroyed_blocks.push(i);
        }
    }

    for i in destroyed_blocks {
        state.level.blocks.remove(i);
    }
}

fn collide_ball_with_wall(state: &mut GameState, new_pos: &mut Vec2) {
    let min_x = -WORLD_WIDTH * 0.5 + BALL_RADIUS;
    let max_x = WORLD_WIDTH * 0.5 - BALL_RADIUS;
    let min_y = -WORLD_HEIGHT * 0.5 + BALL_RADIUS;
    let max_y = WORLD_HEIGHT * 0.5 - BALL_RADIUS;
    let mut was_hit = false;

    if new_pos.x < min_x {
        new_pos.x = min_x;
        state.ball.velocity.x = -state.ball.velocity.x;
        was_hit = true;
    }

    if new_pos.x > max_x {
        new_pos.x = max_x;
        state.ball.velocity.x = -state.ball.velocity.x;
        was_hit = true;
    }

    if new_pos.y < min_y {
        new_pos.y = min_y;
        state.ball.velocity.y = -state.ball.velocity.y;
        was_hit = true;
    }

    if new_pos.y > max_y {
        new_pos.y = max_y;
        state.ball.velocity.y = -state.ball.velocity.y;
        was_hit = true;
    }

    if was_hit {
        play_sound("ball-hit");
    }
}

fn move_ball(state: &mut GameState, time_delta: f32) {
    let mut new_pos = state.ball.center + state.ball.velocity * time_delta;

    collide_ball_with_level(state, &mut new_pos);
    collide_ball_with_wall(state, &mut new_pos);
    collide_ball_with_paddle(state, &mut new_pos);

    state.ball.center = new_pos;
}

fn run_menu(state: &mut GameState) {
    egui::Window::new("")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 200.0))
        .collapsible(false)
        .title_bar(false)
        .resizable(false)
        .show(egui(), |ui| {
            draw_text("Breakout", vec2(0.0, -10.0), WHITE, TextAlign::Center);

            if ui.button("Start Game").clicked() {
                state.state = State::Game;
            }
        });

    if is_key_pressed(KeyCode::Return) {
        state.state = State::Game;
    }
}

fn run_game(state: &mut GameState) {
    let time_delta = delta();

    move_paddle(state, time_delta);
    move_ball(state, time_delta);
}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    clear_background(Color::rgb8(25, 10, 10));

    match state.state {
        State::Game => {
            run_game(state);
        }
        State::Menu => {
            run_menu(state);
        }
    }

    draw_game(state);
}
