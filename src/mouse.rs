use evdev::{uinput::VirtualDevice, AttributeSet, KeyCode, KeyEvent, RelativeAxisCode};
use std::{io, thread, time::Duration};

const DEFAULT_HOLD: Duration = Duration::from_millis(20);

pub struct VirtualMouse {
    device: VirtualDevice,
    buttons: AttributeSet<KeyCode>,
    hold: Duration,
}

impl VirtualMouse {
    pub fn new(name: &str, buttons: &[KeyCode], hold: Duration) -> io::Result<Self> {

        let button_set: AttributeSet<KeyCode> = buttons.iter().collect();

        let mut axes = AttributeSet::<RelativeAxisCode>::new();
        axes.insert(RelativeAxisCode::REL_X);
        axes.insert(RelativeAxisCode::REL_Y);

        let device = VirtualDevice::builder()?
            .name(name)
            .with_keys(&button_set)?
            .with_relative_axes(&axes)?
            .build()?;

        Ok(Self { device, buttons: button_set, hold: DEFAULT_HOLD })

    }

    pub fn click(&mut self, button: KeyCode) -> io::Result<()> {
        self.press(button)?;
        thread::sleep(self.hold);
        self.release(button)
    }

    fn press(&mut self, button: KeyCode) -> io::Result<()> {
        self.device.emit(&[*KeyEvent::new(button, 1)])
    }

    fn release(&mut self, button: KeyCode) -> io::Result<()> {
        self.device.emit(&[*KeyEvent::new(button, 0)])
    }

    pub fn with_hold(mut self, hold: Duration) -> Self {
        self.hold = hold;
        self
    }

    pub fn set_hold(&mut self, hold: Duration){
        self.hold = hold;
    }
}