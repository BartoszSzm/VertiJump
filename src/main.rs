use coroutines::wait_seconds;
use macroquad::prelude::*;
use miniquad::window::order_quit;

pub mod config;
pub mod extensions;
pub mod player;
pub mod world;

fn window_conf() -> Conf {
    let game_config = config::get_config("src/config.toml");
    Conf {
        window_title: game_config.window_title,
        fullscreen: game_config.fullscreen,
        window_resizable: game_config.window_resizable,
        window_width: game_config.window_dims.width,
        window_height: game_config.window_dims.height,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Config
    let game_config = config::get_config("src/config.toml");

    // World
    let world = world::World::new(&game_config).await;

    // Player
    let mut player = player::Player::new(
        &game_config.paths.player_texture_filepath,
        vec2(2., 2.),
        &game_config.player_tile_dims,
        &vec2(game_config.spawn_point_x, game_config.spawn_point_y),
        world.clone(),
    )
    .await;

    let target_frame_time = 1. / game_config.max_fps as f32;

    loop {
        //
        // ********************** Calculations ****************************** //
        //
        let frame_time = get_frame_time();

        player._move(player::Direction::None);

        if is_key_down(KeyCode::A) {
            player._move(player::Direction::Left)
        }

        if is_key_down(KeyCode::D) {
            player._move(player::Direction::Right)
        }

        if is_key_pressed(KeyCode::Space) {
            player.jump(10.)
        }

        if is_key_down(KeyCode::Escape) {
            order_quit()
        }

        set_camera(&Camera2D {
            target: player.rect.center(),
            zoom: vec2(
                game_config.camera_zoom_y as f32 / screen_width(),
                game_config.camera_zoom_x as f32 / screen_height(),
            ),
            ..Default::default()
        });

        player.update();
        player.world.update();

        //
        // ************************* Drawing ******************************** //
        //

        clear_background(BLACK);

        draw_texture_ex(
            &player
                .world
                .texture_layers
                .get(&world::MapLayers::Terrain)
                .unwrap(),
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );

        // TODO offset values should be removed, player should
        // have its own collisions defined in Tiled
        draw_texture_ex(
            &player.texture,
            player.rect.x - 12.,
            player.rect.y - 12.,
            WHITE,
            DrawTextureParams {
                source: Some(player.sprite.frame().source_rect),
                dest_size: Some(player.sprite.frame().dest_size * 1.7),
                ..Default::default()
            },
        );

        // Debug only
        // draw_text(
        //     &format!(
        //         "{:?}, v_y:{:?}, v_x:{:?}",
        //         player.rect.point(),
        //         player.speed.y,
        //         player.speed.x
        //     ),
        //     player.rect.x + 10.,
        //     player.rect.y,
        //     18.,
        //     WHITE,
        // );

        // draw_rectangle_lines(
        //     player.rect.x,
        //     player.rect.y,
        //     player.rect.w,
        //     player.rect.h,
        //     3.,
        //     RED,
        // );

        if frame_time < target_frame_time {
            wait_seconds(frame_time - target_frame_time).await;
        }
        next_frame().await;
    }
}
