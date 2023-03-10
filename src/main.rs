use smithay_client_toolkit::default_environment::Environment;
use smithay_client_toolkit::window::SimpleWindow;

fn main() {
    // Initialize the Wayland connection
    let environment = Environment::init().unwrap();
    let (display, mut event_queue) = environment.wayland_display();

    // Create a window
    let window = SimpleWindow::new(&display, 400, 300, None).unwrap();

    // Display the window
    window.set_title("Hello, world!");
    window.map();

    // Process events
    event_queue.sync_roundtrip(&mut (), |_, _, _| {}).unwrap();
}

