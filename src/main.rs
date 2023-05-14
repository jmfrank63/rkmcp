extern crate enigo;
extern crate screenshots;

use device_query::{DeviceState, MouseState, DeviceQuery};
use enigo::*;
use screenshots::DisplayInfo;
use std::{f64::consts::PI, println, thread, time};

fn main() {
    let mut enigo = Enigo::new();

    let display_info = DisplayInfo::all().unwrap();
    // println!("Display Info: {:#?}", display_info);
    for display in display_info {
        println!("Display: {:#?}", display.id);

        let center_x = display.x as f64 + (display.width as f64) / 2.0;
        let center_y = display.y as f64 + (display.height as f64) / 2.0;
        let radius = if display.width < display.height {
            (display.width as f64) / 2.0
        } else {
            (display.height as f64) / 2.0
        };

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
    thread::sleep(time::Duration::from_secs(2));
    // Record mouse positions for a certain duration
    for _ in 0..1000 {
        let mouse: MouseState = device_state.get_mouse();
        let (x, y) = mouse.coords;
        positions.push((x, y));
        thread::sleep(time::Duration::from_millis(10));
    }

    println!("Mouse moves are now replayed");
    thread::sleep(time::Duration::from_secs(2));
    // Replay the recorded mouse positions
    for (x, y) in positions {
        enigo.mouse_move_to(x, y);
        thread::sleep(time::Duration::from_millis(10));
    }
}
