use ::rand::random_range;
use macroquad::prelude::*;

struct Rectangle {
    pos: Vec2,
    velocity: Vec2,
    color: Color,
    speed: f32,
    size: f32,
    destroy: bool,
}
impl Rectangle {
    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.size, self.size, self.color);
    }
    fn move_rectangle(&mut self) {
        self.pos += self.velocity * self.speed * get_frame_time();
    }
}

const PLAYER_SPEED: f32 = 250.0;
const PROJECTILE_SPEED: f32 = 400.0;
const ENEMY_SPEED: f32 = 200.0;

fn setup_player() -> Rectangle {
    Rectangle {
        pos: Vec2::new((screen_width() / 2.0) - 30.0, screen_height() - 100.0),
        velocity: Vec2::new(0.0, 0.0),
        color: GREEN,
        speed: PLAYER_SPEED,
        size: 60.0,
        destroy: false,
    }
}

fn setup_enemy() -> Rectangle {
    let x_start = random_range(0.0..screen_width());
    Rectangle {
        pos: Vec2::new(x_start, 0.0),
        velocity: Vec2::new(0.0, 1.0),
        color: GREEN,
        speed: ENEMY_SPEED,
        size: 30.0,
        destroy: false,
    }
}

fn spawn_projectile(player_rect: &Rectangle) -> Rectangle {
    Rectangle {
        pos: player_rect.pos + (player_rect.size / 3.0),
        velocity: Vec2::new(0.0, -1.0),
        color: YELLOW,
        speed: PROJECTILE_SPEED,
        size: 25.0,
        destroy: false,
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    // player creation
    let mut player_rect = setup_player();

    let mut score: i32 = 0;

    let mut enemy_vec: Vec<Rectangle> = Vec::new();
    let mut projectile_vec: Vec<Rectangle> = Vec::new();

    let mut spawn_timer: f32 = 1.0;
    let mut game_over: bool = false;

    loop {
        if game_over {
            clear_background(RED);
            let game_over_text = "Game Over! Press [ENTER] to continue";
            let text_size = measure_text(game_over_text, None, 40.0 as _, 1.0);
            draw_text(
                game_over_text,
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0 - text_size.height / 2.0,
                40.0,
                WHITE,
            );
            if is_key_down(KeyCode::Enter) {
                game_over = false;
                player_rect = setup_player();
                enemy_vec.clear();
                projectile_vec.clear();
                score = 0;
            }
            next_frame().await;
            continue;
        }

        clear_background(RED);

        draw_text(format!("Score: {score}").as_str(), 10.0, 30.0, 40.0, BLUE);

        // spawn enemies
        spawn_timer -= get_frame_time();
        if spawn_timer <= 0.0 {
            enemy_vec.push(setup_enemy());
            spawn_timer = 1.0;
        }

        // player input
        {
            let mut curr_vec: Vec2 = Vec2::ZERO;
            if is_key_down(KeyCode::Right) {
                curr_vec.x = 1.0;
            } else if is_key_down(KeyCode::Left) {
                curr_vec.x = -1.0;
            }
            if is_key_down(KeyCode::Up) {
                curr_vec.y = -1.0;
            } else if is_key_down(KeyCode::Down) {
                curr_vec.y = 1.0;
            }
            player_rect.velocity = curr_vec.normalize_or_zero();

            // shoot projectile
            if is_key_pressed(KeyCode::Space) {
                projectile_vec.push(spawn_projectile(&player_rect));
            }
        }

        // handle component movement
        {
            player_rect.move_rectangle();

            for projectile in projectile_vec.iter_mut() {
                projectile.move_rectangle();
            }

            for enemy in enemy_vec.iter_mut() {
                enemy.move_rectangle();

                for projectile in projectile_vec.iter_mut() {
                    if (projectile.pos - enemy.pos).length() < enemy.size {
                        enemy.destroy = true;
                        projectile.destroy = true;
                        score += 1;
                    }
                }

                // player gets hit
                if (player_rect.pos - enemy.pos).length() < player_rect.size {}

                // remove from vector if out of bounds
                if enemy.pos.y > screen_height() {
                    enemy.destroy = true;
                    game_over = true;
                }
            }
        }

        // draw all components
        {
            for enemy in enemy_vec.iter() {
                enemy.draw();
            }
            for projectile in projectile_vec.iter() {
                projectile.draw();
            }
            player_rect.draw();
        }

        // cleanup destroyed components
        enemy_vec.retain(|enemy| !enemy.destroy);
        projectile_vec.retain(|projectile| !projectile.destroy);

        next_frame().await;
    }
}
