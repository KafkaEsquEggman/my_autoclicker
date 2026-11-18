use evdev::{uinput::VirtualDevice, AttributeSet, KeyCode, KeyEvent, RelativeAxisCode};
use std::{thread, time::Duration};

fn main() -> std::io::Result<()> {

    let mut buttons = AttributeSet::<KeyCode>::new();
    buttons.insert(KeyCode::BTN_LEFT);

    let mut axes = AttributeSet::<RelativeAxisCode>::new();
    axes.insert(RelativeAxisCode::REL_X);
    axes.insert(RelativeAxisCode::REL_Y);

    let mut mouse = VirtualDevice::builder()?
        .name("autoclicker")
        .with_keys(&buttons)?
        .with_relative_axes(&axes)?
        .build()?;

    println!("Virtual Mouse created. . Clicking in 3s...");
    thread::sleep(Duration::from_secs(3));

    mouse.emit(&[*KeyEvent::new(KeyCode::BTN_LEFT, 1)])?;
    thread::sleep(Duration::from_millis(20));
    mouse.emit(&[*KeyEvent::new(KeyCode::BTN_LEFT, 0)])?;

    println!("Click");
    Ok(())
}