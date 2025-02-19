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

    let mut player = Ship::new(
        screen_width() / 2.0,
        screen_height() - 180.0,
        player_textures.clone(),
        proj_textures.clone(),
    );

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

        player.update();

        player.draw();

        next_frame().await
    }
}

struct Ship {
    x: f32,
    y: f32,
    textures: Rc<Vec<Texture2D>>,
    proj_textures: Rc<Vec<Texture2D>>,
    projectiles: Vec<Projectile>,
    current_frame: usize,
    frame_time_elapsed: f32,
}

impl Ship {
    fn new(
        x: f32,
        y: f32,
        textures: Rc<Vec<Texture2D>>,
        proj_textures: Rc<Vec<Texture2D>>,
    ) -> Self {
        Ship {
            x,
            y,
            textures,
            proj_textures,
            projectiles: Vec::new(),
            current_frame: 0,
            frame_time_elapsed: 0.0,
        }
    }

    fn mov(&mut self, delta: f32) {
        self.x += delta;
    }

    fn shoot(&mut self) {
        self.projectiles.push(Projectile::new(
            self.x - 3.0,
            self.y + 20.0,
            self.proj_textures.clone(),
        ));
    }

    fn update(&mut self) {
        self.frame_time_elapsed += get_frame_time();
        if self.frame_time_elapsed >= 0.1 {
            self.current_frame = (self.current_frame + 1) % self.textures.len();

            self.frame_time_elapsed = 0.0;
        }

        self.projectiles.retain(|proj| !proj.can_be_removed());
        for proj in self.projectiles.iter_mut() {
            proj.update();
        }
    }

    fn draw(&self) {
        draw_texture(&self.textures[self.current_frame], self.x, self.y, WHITE);

        for proj in self.projectiles.iter() {
            proj.draw();
        }
    }
}

struct Projectile {
    x: f32,
    y: f32,
    textures: Rc<Vec<Texture2D>>,
    frame_time_elapsed: f32,
}

impl Projectile {
    fn new(x: f32, y: f32, textures: Rc<Vec<Texture2D>>) -> Self {
        Projectile {
            x,
            y,
            textures,
            frame_time_elapsed: 0.0,
        }
    }

    fn can_be_removed(&self) -> bool {
        self.y < 0.0
    }

    fn update(&mut self) {
        self.frame_time_elapsed += get_frame_time();
        if self.frame_time_elapsed >= 0.01 {
            self.y -= 5.0;

            self.frame_time_elapsed = 0.0;
        }
    }

    fn draw(&self) {
        draw_texture(&self.textures[1], self.x, self.y, WHITE);
        draw_texture(&self.textures[1], self.x + 54.0, self.y, WHITE);
    }
}

async fn load_textures(paths: &[&str]) -> Vec<Texture2D> {
    future::join_all(paths.iter().map(|path| load_single_texture(path))).await
}

async fn load_single_texture(path: &str) -> Texture2D {
    load_texture(path).await.unwrap()
}
