use smithay::reexports::calloop::EventLoop;
use smithay::reexports::wayland_server::Display;
use std::time::Instant;

mod state;
use state::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {

	let mut event_loop: EventLoop<App> = EventLoop::try_new()?;
    let loop_handle = event_loop.handle();

    let mut display: Display<App> = Display::new()?;
    let display_handle = display.handle();

    let mut state = App {
        start_time: Instant::now(),
        display_handle,
        loop_handle,
    };

    let listening_socket = display.add_socket_auto()?;
    println!("flow_wm listening on: {:?}", listening_socket.into_string());

    event_loop.run(None, &mut state, move |_data| {
    })?;
    Ok(())
}