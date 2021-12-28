mod events;
mod x11;

use anyhow::Result;
use events::EventHandler;
use x11::X11Handler;
use x11rb::rust_connection::RustConnection;

fn main() -> Result<()> {
    println!("Extra Cosmic XKill Applet v0.1.0");
    println!("Clique na janela que deseja fechar...");

    let (conn, screen_num) = RustConnection::connect(None)?;
    let handler = X11Handler::new()?;
    let mut event_handler = EventHandler::new(conn, screen_num);

    if let Some(window) = event_handler.wait_for_window_click()? {
        if let Ok(name) = handler.get_window_name(window) {
            println!("Fechando janela: {}", name);
        }
        handler.kill_window(window)?;
        println!("Janela fechada com sucesso");
    } else {
        println!("Operação cancelada");
    }

    Ok(())
}
