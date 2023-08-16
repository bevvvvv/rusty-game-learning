use std::fmt::format;

use rand::prelude::*;
use rusty_engine::prelude::*;

struct GameState {
    health_amount: u8,
    lost: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            health_amount: 5,
            lost: false,
        }
    }
}

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 780.0;

fn main() {
    let mut game = Game::new();
    let initial_game_state = GameState::default();

    // game setup goes here
    // Create window
    game.window_settings(WindowDescriptor {
        title: "Rusty Engine Tutorial".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    });

    // Player creation
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation.x = -500.0;
    player1.layer = 10.0;
    player1.collision = true;

    // Start background music
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // Road lines
    for i in 0..15 {
        let line = game.add_sprite(&format!("road{}", i), SpritePreset::RacingBarrierWhite);
        line.scale = 0.1;
        line.translation.x = (-WINDOW_WIDTH / 2.0) + (i as f32 * WINDOW_WIDTH / 15.0);
        println!("{}", line.translation.x);
    }

    // Starting obstacles
    let obstacle_preset = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
    ];
    for (i, preset) in obstacle_preset.into_iter().enumerate() {
        let obstacle = game.add_sprite(&format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(WINDOW_WIDTH..WINDOW_WIDTH + 1000.0);
        obstacle.translation.y =
            thread_rng().gen_range(-WINDOW_HEIGHT / 2.0 + 100.0..WINDOW_HEIGHT / 2.0 - 100.0);
    }

    // Create health message
    let health_message = game.add_text(
        "health_message",
        format!("Health: {}", initial_game_state.health_amount),
    );
    health_message.translation = Vec2::new(WINDOW_WIDTH / 2.0 - 100.0, WINDOW_HEIGHT / 2.0 - 50.0);

    game.add_logic(game_logic);
    game.run(initial_game_state);
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Freeze if lost
    if game_state.lost {
        return;
    }

    // Collect keyboard input
    let mut direction = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    // Move player
    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;
    if player1.translation.y < -360.0 || player1.translation.y > 360.0 {
        game_state.health_amount = 0;
    }

    // Move road objects
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("road") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -engine.window_dimensions.x / 2.0 {
                sprite.translation.x = engine.window_dimensions.x / 2.0;
            }
        }
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -engine.window_dimensions.x / 2.0 - 800.0 {
                sprite.translation.x = thread_rng().gen_range(WINDOW_WIDTH..WINDOW_WIDTH + 1000.0);
                sprite.translation.y = thread_rng()
                    .gen_range(-WINDOW_HEIGHT / 2.0 + 100.0..WINDOW_HEIGHT / 2.0 - 100.0);
            }
        }
    }

    // Handle collisions
    let health_message = engine.texts.get_mut("health_message").unwrap();
    for event in engine.collision_events.drain(..) {
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.3);
        }
    }

    // Lost condition
    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game_over", "Game Over!");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}
