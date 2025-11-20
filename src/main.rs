use macroquad::color::*;
use macroquad::prelude::*;

mod player;
use player::{Player, Point};

#[macroquad::main("MyGame")]
async fn main() {
    let mut player = Player::new(screen_width() / 2.0, 120.0);
    const ROTATION: f32 = 2.0;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Left) {
            //player.x -= 2.0;
            //player.rotation_deg -= 2.0;
            player.rotate(-ROTATION);
        }
        if is_key_down(KeyCode::Right) {
            //player.x += 2.0;
            //player.rotation_deg += 2.0;
            player.rotate(ROTATION);
        }
        if is_key_down(KeyCode::Up) {
            player.translate(0.0, -2.0);
        }
        if is_key_down(KeyCode::Down) {
            player.translate(0.0, 2.0);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            player.reposition(mouse_x, mouse_y);
        }

        render(&player);
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
    for i in 0..player.points.len() {
        let current = player.points[i];
        let next = player.points[(i + 1) % player.points.len()]; // Wrap around to first point
        draw_line(current.x, current.y, next.x, next.y, 2.0, YELLOW);
    }

    draw_circle(player.center.x, player.center.y, 3.0, RED);
}
