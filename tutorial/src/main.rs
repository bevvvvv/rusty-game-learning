use rusty_engine::prelude::*;

struct GameState {
    // high_score: u32,
    current_score: u32,
    barrier_index: i32,
    // spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            // high_score: 0,
            current_score: 0,
            barrier_index: 0,
            // spawn_timer: Timer::from_seconds(1.0, false),
        }
    }
}

fn main() {
    let mut game = Game::new();

    // game setup
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = SOUTH_WEST;
    player.scale = 1.0;
    player.layer = 1.0;
    player.collision = true;

    // can add multiple game logic functions, ran in order added
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // engine.show_colliders = true;
    for event in engine.collision_events.drain(..) {
        // println!("{:?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1].iter() {
                if label != "player" {
                    engine.sprites.remove(label);
                }
            }
            game_state.current_score += 1;
            println!("Score: {}", game_state.current_score);
        }
    }

    // player movement
    let player = engine.sprites.get_mut("player").unwrap();
    // player.translation.x += 100.0 * engine.delta_f32;
    const MOVE_SPEED: f32 = 100.0;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        player.translation.y += MOVE_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        player.translation.y -= MOVE_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        player.translation.x += MOVE_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        player.translation.x -= MOVE_SPEED * engine.delta_f32;
    }

    // barrier placement
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("barrier{}", game_state.barrier_index);
            game_state.barrier_index += 1;
            let barrieri = engine.add_sprite(label, "sprite/racing/barrier_white.png");
            barrieri.translation = mouse_location;
            barrieri.collision = true;
        }
    }
}
