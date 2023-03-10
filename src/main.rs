use smithay_client_toolkit::default_connect;
use smithay_client_toolkit::window::{Window, WindowType};

fn main() {
    // Initialize the Wayland connection
    let (display, mut event_queue) = default_connect().expect("Failed to connect to Wayland server");

    // Create a window
    let window = Window::init(&display, WindowType::Basic).expect("Failed to create window");
}
