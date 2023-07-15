extern crate enigo;
extern crate screenshots;

use device_query::{DeviceQuery, DeviceState, MouseState};
use enigo::*;
use screenshots::DisplayInfo;
use std::{f64::consts::PI, println, thread, time};

const MILLIS: u64 = 1;
const SLEEP: u64 = 2;

// Use srt for transfer of data to other devices
// Use enigo for mouse and keyboard control
// Use screenshots for display info

fn main() {
    let mut enigo = Enigo::new();

    let display_info = DisplayInfo::all().unwrap();
    for display in display_info {
        let center_x = display.x as f64 + (display.width as f64) / 2.0;
        let center_y = display.y as f64 + (display.height as f64) / 2.0;
        let radius = if display.width < display.height {
            (display.width as f64) / 2.0
        } else {
            (display.height as f64) / 2.0
        };
        println!("Display: {:#?}", display.id);
        println!("Center: ({}, {})", center_x, center_y);
        println!("Width: {}", display.width);
        println!("Height: {}", display.height);

        for i in 0..360 {
            let radian = (i as f64) * PI / 180.0;
            let x = center_x + radian.cos() * radius;
            let y = center_y + radian.sin() * radius;
            enigo.mouse_move_to(x as i32, y as i32);
            thread::sleep(time::Duration::from_millis(5));
        }
    }

    // Initialize the device state
    let device_state = DeviceState::new();

    // Initialize Enigo for later use
    let mut enigo = Enigo::new();

    // Create a vector to store mouse positions
    let mut positions = Vec::new();

    println!("Please do use the mouse for a while");
    thread::sleep(time::Duration::from_secs(SLEEP));
    // Record mouse positions for a certain duration
    for _ in 0..4000 {
        let mouse: MouseState = device_state.get_mouse();
        let (x, y) = mouse.coords;
        positions.push((x, y));

        thread::sleep(time::Duration::from_millis(MILLIS));
    }

    println!("Mouse moves are now replayed");
    thread::sleep(time::Duration::from_secs(SLEEP));
    // Replay the recorded mouse positions
    for (x, y) in positions {
        enigo.mouse_move_to(x, y);
        thread::sleep(time::Duration::from_millis(MILLIS));
    }
}
