use std::time::Duration;
use tracing::info;
use smithay::reexports::calloop::EventLoop;
use smithay::reexports::wayland_server::Display;

mod state;
mod backend;
mod config;

use state::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }

    info!("initializing flow_wm...");

    let mut event_loop: EventLoop<App> = EventLoop::try_new()?;
    let loop_handle = event_loop.handle();

    let mut display: Display<App> = Display::new()?;
    let display_handle = display.handle();

    let mut state = App {
        start_time: std::time::Instant::now(),
        display_handle,
        loop_handle,
    };

    event_loop.run(
        Some(Duration::from_millis(16)),
        &mut state, 
        |_data| {
			// screen refresh logic later
        }
    )?;

    Ok(())
}