use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
use nalgebra::{Vector2, Vector3, Vector4};


fn calc_det(p: Vector2<f32>, p1: Vector2<f32>, p2: Vector2<f32>) -> f32
{
    let v1: Vector2<f32> = p2 - p1;
    let v2: Vector2<f32> = p - p1;
    v1.x * v2.y - v1.y * v2.x
}

fn inside_triangle(p: Vector2<f32>, p0: Vector2<f32>, p1: Vector2<f32>, p2: Vector2<f32>) -> bool
{
    let sum = calc_det(p, p0, p1).signum() + calc_det(p, p1, p2).signum() + calc_det(p, p2, p0).signum();
    let sumi = sum as i32;
    if sumi.abs() == 3 {return true;}
    false
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("3d renderer", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;

    
    let mut last_frame_time = Instant::now();
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Calculate delta time
        let current_frame_time = Instant::now();
        let delta_time = current_frame_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_frame_time;

    
        

        // Update
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let (w, h) = canvas.output_size().unwrap();
        let (w, h): (i32, i32) = (
            w.try_into().expect("Width is out of range for i32"),
            h.try_into().expect("Height is out of range for i32"),
        );
        

        canvas.present();

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}