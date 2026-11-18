use evdev::{uinput::VirtualDevice, AttributeSet, KeyCode, KeyEvent, RelativeAxisCode};
use std::{io, thread, time::{Duration, Instant}};

const INTERVAL: Duration = Duration::from_millis(500);
const HOLD_MILISECONDS: Duration = Duration::from_millis(10);
const TOTAL_SECONDS: Duration = Duration::from_secs(3);

fn main() -> std::io::Result<()> {

    let mut mouse = create_mouse()?;

    println!("Virtual Mouse created. . Clicking in 3s...");
    thread::sleep(Duration::from_secs(3));

    let start = Instant::now();
    let mut count = 0u32;

    while start.elapsed() < TOTAL_SECONDS {
        click(&mut mouse)?;
        thread::sleep(INTERVAL);
        count += 1;
        println!("Click!");
    }

    println!("Done: {count} clicks in {:.2?}", start.elapsed());
    Ok(())

}

fn create_mouse() -> io::Result<VirtualDevice> {
    let mut buttons = AttributeSet::<KeyCode>::new();
    buttons.insert(KeyCode::BTN_LEFT);

    let mut axes = AttributeSet::<RelativeAxisCode>::new();
    axes.insert(RelativeAxisCode::REL_X);
    axes.insert(RelativeAxisCode::REL_Y);

    VirtualDevice::builder()?
        .name("my_autoclicker")
        .with_keys(&buttons)?
        .with_relative_axes(&axes)?
        .build()
}

fn click(mouse: &mut VirtualDevice) -> io::Result<()> {
    mouse.emit(&[*KeyEvent::new(KeyCode::BTN_LEFT, 1)])?;
    thread::sleep(HOLD_MILISECONDS);
    mouse.emit(&[*KeyEvent::new(KeyCode::BTN_LEFT, 0)])
}