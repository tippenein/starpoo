// use specs::World;
// use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::image::{LoadTexture};

use specs::prelude::*;
use specs_derive::Component;

use std::time::Duration;
use std::vec::Vec;

const PLAYER_MOVEMENT_SPEED: i32 = 8;
const BULLET_SPEED: i32 = 10;
const DEBUG: bool = true;
const FRAME_RATE: u32 = 20;

/// The current position of a given entity
// #[derive(Component, Debug)]
// #[storage(VecStorage)]
// pub struct Position(pub Point);

/// The current speed and direction of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Wasd {
    W,
    A,
    S,
    D,
}

struct Bullet {
    directions: Vec<Direction>,
    position: Point,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Player {
    position: Point,
    sprite: Rect,
    directions: Vec<Direction>,
    speed: i32,
}

// fn update_bullets(bullets: &mut Vec<Bullet>) -> &mut Vec<Bullet> {
//     bullets.into_iter().map(|b| update_bullet(b)).collect::<Vec<Bullet>>();
//     return bullets;
// }

fn update_bullet(bullet: &mut Bullet) -> &mut Bullet {
    let (x,y) = mk_move(&bullet.directions, &BULLET_SPEED);
    bullet.position = bullet.position.offset(x,y);
    return bullet;
}



fn update_player(player: &mut Player) {
    let (x,y) = mk_move(&player.directions, &player.speed);
    player.position = player.position.offset(x, y);
}

fn mk_move(directions: &Vec<Direction>, speed: &i32) -> (i32, i32) {
    use self::Direction::*;
    let mut movement_x = 0;
    let mut movement_y = 0;
    let mut dirs = directions.clone();
    while let Some(top) = dirs.pop() {
        match top {
            Left => {
                movement_x = movement_x - speed;
            },
            Right => {
                movement_x = movement_x + speed;
            },
            Up => {
                movement_y = movement_y - speed;
            },
            Down => {
                movement_y = movement_y + speed;
            },
        }
    }
    return (movement_x, movement_y);
}

fn render(canvas: &mut WindowCanvas, texture: &Texture, player: &Player) -> Result<(), String> {
    canvas.clear();
    let (width, height) = canvas.output_size()?;
    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());
    canvas.copy(&texture, player.sprite, screen_rect)?;

    canvas.present();

    Ok(())

}

fn render_bullets(canvas: &mut WindowCanvas, texture: &Texture, bullets: &Vec<Bullet>) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    bullets.into_iter().for_each(|b| {
        let screen_position = b.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, 24, 24);
        canvas.copy(&texture, Rect::new(0,0,24,24), screen_rect);
        canvas.present();
    });

    Ok(())

}
fn debug(s: String) {
    if DEBUG {
        println!("{}", &s);
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("starpoo", 800, 800)
        .position_centered()
        .build()
        .expect("could not window");

    let mut canvas = window.into_canvas().build()
        .expect("couldnt make canvas");

    debug(format!("Using SDL_Renderer \"{}\"", canvas.info().name));

    // canvas.set_draw_color(Color::RGB(0,0,0));
    // canvas.clear();
    // canvas.present();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/tweety_bird.png")?;
    let bullet_texture = texture_creator.load_texture("assets/bomb.png")?;

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 32, 32),
        directions: Vec::with_capacity(4),
        speed: 0
    };
    let mut bullets = Vec::new();
    // let mut world = World::new();
    // world.create_entity()
    //     .with(Position(Point::new(0, 0)))
    //     // .with(Velocity {speed: 0, directions: Vec::new()})
    //     .build();

    let mut event_pump = sdl_context.event_pump()
        .expect("couldnt make event pump");
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.directions.push(Direction::Left);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.directions.push(Direction::Right);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.directions.push(Direction::Up);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.directions.push(Direction::Down);
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.directions.retain(|s| *s != Direction::Left);
                },
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.directions.retain(|s| *s != Direction::Right);
                },
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.directions.retain(|s| *s != Direction::Up);
                },
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.directions.retain(|s| *s != Direction::Down);
                },
                Event::TextInput { text: _t, .. } => {
                    let bullet = Bullet {
                        position: player.position,
                        directions: vec![Direction::Up]
                    };
                    bullets.push(bullet);
                    debug(format!("bullet"));
                },
                Event::MouseMotion { .. } => {},
                e => {
                    debug(format!("{:?}", e));
                }
            }
        }

        update_player(&mut player);
        // update_bullets(&mut bullets);
        render_bullets(&mut canvas, &bullet_texture, &bullets).expect("hmm");
        render(&mut canvas, &texture, &player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAME_RATE));
    }
    Ok(())
}
