use smithay_client_toolkit::window::{ButtonState, Event as WEvent, Theme};
use smithay_client_toolkit::window::{Window, WindowState};

fn main() {
    let (display, mut event_queue) = wayland_client::default_connect().unwrap();
    let mut theme = Theme::load_default();

    let mut window = Window::init(
        &mut event_queue,
        theme.clone(),
        (800, 600),
        None,
        WindowState::Fullscreen,
    )
    .unwrap();

    loop {
        let event = event_queue
            .dispatch(&mut window, |_, _, _| {})
            .unwrap_or(WEvent::Refresh);

        match event {
            WEvent::Refresh => {}
            WEvent::Configure { new_size, .. } => window.resize(new_size),
            WEvent::Close => break,
            WEvent::Button {
                state: ButtonState::Pressed,
                ..
            } => window.close(),
            _ => {}
        }
    }
}

