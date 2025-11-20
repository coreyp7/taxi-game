use macroquad::color::*;
use macroquad::prelude::*;

mod player;
use player::Player;

#[macroquad::main("MyGame")]
async fn main() {
    let mut player = Player {
        x: screen_width() / 2.0 - 60.0,
        y: 120.0,
        rotation_deg: 0.0,
    };

    loop {
        clear_background(BLACK);

        // Keyboard input
        if is_key_down(KeyCode::Left) {
            //player.x -= 2.0;
            player.rotation_deg -= 2.0;
        }
        if is_key_down(KeyCode::Right) {
            //player.x += 2.0;
            player.rotation_deg += 2.0;
        }
        if is_key_down(KeyCode::Up) {
            player.y -= 2.0;
        }
        if is_key_down(KeyCode::Down) {
            player.y += 2.0;
        }

        // Mouse input
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            player.x = mouse_x - 30.0; // Center the rectangle on mouse
            player.y = mouse_y - 50.0;
        }

        render(&player);
        draw_circle(player.x, player.y, 3.0, RED);
        next_frame().await
    }
}

fn render(player: &Player) {
    render_player(player);
    draw_text("Hello, taxi!", 20.0, 20.0, 30.0, DARKGRAY);
    draw_text(
        "Arrow keys to move, Space to reset, Click to teleport",
        20.0,
        50.0,
        20.0,
        WHITE,
    );
}

fn render_player(player: &Player) {
    let mut params: DrawRectangleParams = DrawRectangleParams::default();
    params.rotation = player.rotation_deg.to_radians();
    params.color = YELLOW;

    // FIXME: get rid of magic numbers; should have player w/h defined somewhere.
    let taxi_x = player.x - 30.0;
    let taxi_y = player.y - 50.0;

    draw_rectangle_ex(taxi_x, taxi_y, 60.0, 100.0, params);
}
