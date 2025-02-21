use futures::future;
use macroquad::prelude::*;
use std::rc::Rc;

fn game_config() -> Conf {
    Conf {
        window_title: "SpaceInvaders".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(game_config)]
async fn main() {
    let player_textures: Rc<Vec<Texture2D>> = Rc::new(
        load_textures(&[
            "assets/player/player_000.png",
            "assets/player/player_001.png",
            "assets/player/player_002.png",
            "assets/player/player_003.png",
            "assets/player/player_004.png",
            "assets/player/player_005.png",
            "assets/player/player_006.png",
            "assets/player/player_007.png",
            "assets/player/player_008.png",
            "assets/player/player_009.png",
        ])
        .await,
    );
    let player_dmg_textures: Rc<Vec<Texture2D>> =
        Rc::new(load_textures(&["assets/player/player_dmg.png"]).await);
    let player_expl_textures: Rc<Vec<Texture2D>> = Rc::new(
        load_textures(&[
            "assets/player/explosion/player_expl_000.png",
            "assets/player/explosion/player_expl_001.png",
            "assets/player/explosion/player_expl_002.png",
            "assets/player/explosion/player_expl_003.png",
            "assets/player/explosion/player_expl_004.png",
            "assets/player/explosion/player_expl_005.png",
            "assets/player/explosion/player_expl_006.png",
            "assets/player/explosion/player_expl_007.png",
        ])
        .await,
    );
    let enemy_textures: Rc<Vec<Texture2D>> = Rc::new(
        load_textures(&[
            "assets/enemy/enemy_000.png",
            "assets/enemy/enemy_001.png",
            "assets/enemy/enemy_002.png",
            "assets/enemy/enemy_003.png",
            "assets/enemy/enemy_004.png",
            "assets/enemy/enemy_005.png",
            "assets/enemy/enemy_006.png",
            "assets/enemy/enemy_007.png",
            "assets/enemy/enemy_008.png",
            "assets/enemy/enemy_009.png",
        ])
        .await,
    );
    let enemy_dmg_textures: Rc<Vec<Texture2D>> =
        Rc::new(load_textures(&["assets/enemy/enemy_dmg.png"]).await);
    let enemy_expl_textures: Rc<Vec<Texture2D>> = Rc::new(
        load_textures(&[
            "assets/enemy/explosion/enemy_expl_000.png",
            "assets/enemy/explosion/enemy_expl_001.png",
            "assets/enemy/explosion/enemy_expl_002.png",
            "assets/enemy/explosion/enemy_expl_003.png",
            "assets/enemy/explosion/enemy_expl_004.png",
            "assets/enemy/explosion/enemy_expl_005.png",
            "assets/enemy/explosion/enemy_expl_006.png",
            "assets/enemy/explosion/enemy_expl_007.png",
        ])
        .await,
    );
    let proj_textures: Rc<Vec<Texture2D>> = Rc::new(
        load_textures(&[
            "assets/projectile/proj_001.png",
            "assets/projectile/proj_002.png",
            "assets/projectile/proj_003.png",
            "assets/projectile/proj_004.png",
            "assets/projectile/proj_005.png",
        ])
        .await,
    );

    let mut player: Ship = Ship::new(
        screen_width() / 2.0,
        screen_height() - 180.0,
        0.0,
        true,
        player_textures.clone(),
        player_dmg_textures.clone(),
        player_expl_textures.clone(),
        proj_textures.clone(),
    );

    let mut enemies: Vec<Ship> = Vec::new();
    let enemy_positions = [
        screen_width() * 0.1,
        screen_width() * 0.7,
        screen_width() * 0.9,
        screen_width() * 0.5,
        screen_width() * 0.3,
        screen_width() * 0.8,
        screen_width() * 0.2,
        screen_width() * 0.6,
        screen_width() * 0.4,
        screen_width() * 0.9,
        screen_width() * 0.5,
        screen_width() * 0.3,
        screen_width() * 0.1,
        screen_width() * 0.7,
        screen_width() * 0.2,
        screen_width() * 0.6,
        screen_width() * 0.4,
        screen_width() * 0.8,
    ];
    let mut enemy_positions_idx: usize = 0;
    let mut enemy_timer: f32 = 0.0;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::A) {
            player.mov(-5.0);
        }
        if is_key_down(KeyCode::D) {
            player.mov(5.0);
        }
        if is_key_pressed(KeyCode::Space) {
            player.shoot();
        }

        enemies.retain(|enemy| !enemy.can_be_removed());
        enemy_timer += get_frame_time();
        if enemy_timer >= 2.0 {
            enemies.push(Ship::new(
                enemy_positions[enemy_positions_idx],
                -150.0,
                3.1415,
                false,
                enemy_textures.clone(),
                enemy_dmg_textures.clone(),
                enemy_expl_textures.clone(),
                proj_textures.clone(),
            ));
            enemy_positions_idx = (enemy_positions_idx + 1) % enemy_positions.len();

            enemy_timer = 0.0;
        }

        for enemy in enemies.iter_mut().filter(|enemy| enemy.health > 0) {
            let player_x = player.x + 36.0;
            let player_y = player.y + 55.0;
            let enemy_x = enemy.x + 30.0;
            let enemy_y = enemy.y + 90.0;

            let dist_x = (player_x - enemy_x).abs();
            let dist_y = (player_y - enemy_y).abs();

            if dist_x < 60.0 && dist_y < 90.0 {
                enemy.notify_hit(100);
            }
        }

        for i in 0..enemies.len() {
            for j in (i + 1)..enemies.len() {
                if enemies[i].x == enemies[j].x && (enemies[i].y - enemies[j].y).abs() <= 80.0 {
                    let mut upper_ship_idx = i;
                    let mut lower_ship_idx = j;
                    if enemies[j].y < enemies[i].y {
                        upper_ship_idx = j;
                        lower_ship_idx = i;
                    }

                    if enemies[lower_ship_idx].health > 0 {
                        enemies[upper_ship_idx].notify_hit(50);
                        enemies[lower_ship_idx].notify_hit(100);
                    }
                }
            }
        }

        for proj in player.projectiles.iter_mut().filter(|proj| !proj.hit) {
            for enemy in enemies.iter_mut() {
                if proj.x > enemy.x - 32.0
                    && proj.x < enemy.x + 32.0
                    && (proj.y - enemy.y).abs() < 150.0
                {
                    enemy.notify_hit(10);
                    proj.notify_hit();
                }
            }
        }

        for enemy in enemies.iter_mut() {
            enemy.update();
        }
        player.update();

        for enemy in enemies.iter() {
            enemy.draw();
        }
        player.draw();

        next_frame().await
    }
}

struct Ship {
    x: f32,
    y: f32,
    angle: f32,
    is_player: bool,
    textures: Rc<Vec<Texture2D>>,
    textures_dmg: Rc<Vec<Texture2D>>,
    textures_expl: Rc<Vec<Texture2D>>,
    proj_textures: Rc<Vec<Texture2D>>,
    health: i32,
    projectiles: Vec<Projectile>,
    current_frame_idx: usize,
    frame_time_elapsed: f32,
}

impl Ship {
    fn new(
        x: f32,
        y: f32,
        angle: f32,
        is_player: bool,
        textures: Rc<Vec<Texture2D>>,
        textures_dmg: Rc<Vec<Texture2D>>,
        textures_expl: Rc<Vec<Texture2D>>,
        proj_textures: Rc<Vec<Texture2D>>,
    ) -> Self {
        Ship {
            x,
            y,
            angle,
            is_player,
            textures,
            textures_dmg,
            textures_expl,
            proj_textures,
            health: 100,
            projectiles: Vec::new(),
            current_frame_idx: 0,
            frame_time_elapsed: 0.0,
        }
    }

    fn mov(&mut self, delta: f32) {
        self.x += delta;
    }

    fn shoot(&mut self) {
        self.projectiles.push(Projectile::new(
            self.x + 24.0,
            self.y - 40.0,
            self.proj_textures.clone(),
        ));
    }

    fn can_be_removed(&self) -> bool {
        self.y > screen_height()
    }

    fn notify_hit(&mut self, damage: i32) {
        self.health -= damage;

        if self.health <= 0 {
            if self.is_player {
                self.x -= 20.4;
            } else {
                self.x -= 26.0;
                self.y += 45.0;
            }
            self.textures = self.textures_expl.clone();
            self.current_frame_idx = 0;
        } else if self.health <= 50 {
            self.textures = self.textures_dmg.clone();
            self.current_frame_idx = 0;
        }
    }

    fn update(&mut self) {
        self.frame_time_elapsed += get_frame_time();
        if self.frame_time_elapsed >= 0.1 {
            if self.health > 0 && !self.is_player {
                self.y += if self.health > 50 { 2.0 } else { 1.0 };
            }

            self.current_frame_idx += 1;
            if self.health <= 0 && self.current_frame_idx == self.textures.len() {
                self.current_frame_idx = self.textures.len() - 1;
                self.y = 1000.0;
            } else {
                self.current_frame_idx %= self.textures.len();
            }

            self.frame_time_elapsed = 0.0;
        }

        self.projectiles.retain(|proj| !proj.can_be_removed());
        for proj in self.projectiles.iter_mut() {
            proj.update();
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            &self.textures[self.current_frame_idx],
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                rotation: self.angle,
                ..Default::default()
            },
        );

        for proj in self.projectiles.iter() {
            proj.draw();
        }
    }
}

struct Projectile {
    x: f32,
    y: f32,
    textures: Rc<Vec<Texture2D>>,
    hit: bool,
    current_frame_idx: usize,
    frame_time_elapsed: f32,
}

impl Projectile {
    fn new(x: f32, y: f32, textures: Rc<Vec<Texture2D>>) -> Self {
        Projectile {
            x,
            y,
            textures,
            hit: false,
            current_frame_idx: 1,
            frame_time_elapsed: 0.0,
        }
    }

    fn can_be_removed(&self) -> bool {
        self.y < 0.0
    }

    fn notify_hit(&mut self) {
        self.hit = true;
    }

    fn update(&mut self) {
        self.frame_time_elapsed += get_frame_time();
        if self.frame_time_elapsed >= 0.01 {
            if !self.hit {
                self.y -= 5.0;
            }

            if self.hit {
                self.current_frame_idx += 1;
                if self.current_frame_idx == self.textures.len() {
                    self.current_frame_idx = self.textures.len() - 1;
                    self.y = -1000.0;
                }
            }

            self.frame_time_elapsed = 0.0;
        }
    }

    fn draw(&self) {
        draw_texture(
            &self.textures[self.current_frame_idx],
            self.x,
            self.y,
            WHITE,
        );
    }
}

async fn load_textures(paths: &[&str]) -> Vec<Texture2D> {
    future::join_all(paths.iter().map(|path| load_single_texture(path))).await
}

async fn load_single_texture(path: &str) -> Texture2D {
    load_texture(path).await.unwrap()
}
