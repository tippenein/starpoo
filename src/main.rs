use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::image::{LoadTexture, InitFlag};
use std::time::Duration;

const PLAYER_MOVEMENT_SPEED: i32 = 10;
const FRAME_RATE: u32 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Player {
    position: Point,
    sprite: Rect,
    direction: Direction,
    speed: i32,
}
fn update_player(player: &mut Player) {
    use self::Direction::*;
    match player.direction {
        Left => {
            player.position = player.position.offset(-player.speed, 0);
        },
        Right => {
            player.position = player.position.offset(player.speed, 0);
        },
        Up => {
            player.position = player.position.offset(0, -player.speed);
        },
        Down => {
            player.position = player.position.offset(0, player.speed);
        },
    }
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

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("starpoo", 800, 600)
        .position_centered()
        .build()
        .expect("could not window");

    let mut canvas = window.into_canvas().build()
        .expect("couldnt make canvas");

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);

    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/tweety.png")?;

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        direction: Direction::Up,
        speed: 0
    };

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
                    player.direction = Direction::Left;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Right;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Up;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Down;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.speed = 0;
                },
                Event::MouseMotion { .. } => {},
                e => {
                    println!("{:?}", e);
                }
            }
        }

        update_player(&mut player);

        render(&mut canvas, &texture, &player)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAME_RATE));
    }
    Ok(())
}
