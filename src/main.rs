mod x11;

use anyhow::Result;
use x11::X11Handler;

fn main() -> Result<()> {
    println!("Extra Cosmic XKill Applet v0.1.0");

    let handler = X11Handler::new()?;
    let windows = handler.list_windows()?;

    println!("Janelas detectadas: {}", windows.len());

    for window in windows.iter().take(5) {
        if let Ok(name) = handler.get_window_name(*window) {
            if !name.is_empty() {
                println!("  - {} (ID: {})", name, window);
            }
        }
    }

    Ok(())
}
