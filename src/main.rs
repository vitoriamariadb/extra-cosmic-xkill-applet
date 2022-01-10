mod events;
mod wayland;
mod x11;

use anyhow::Result;
use events::EventHandler;
use wayland::{detect_session_type, SessionType};
use x11::X11Handler;
use x11rb::rust_connection::RustConnection;

fn main() -> Result<()> {
    println!("Extra Cosmic XKill Applet v0.1.0");

    let session_type = detect_session_type();
    println!("Tipo de sessão detectada: {}", session_type.as_str());

    match session_type {
        SessionType::X11 => run_x11_mode()?,
        SessionType::Wayland => {
            println!("Modo Wayland ainda não suportado para kill de janelas");
            println!("Usando fallback X11 se disponível...");
            run_x11_mode()?;
        }
        SessionType::Unknown => {
            println!("Tipo de sessão desconhecida. Tentando X11...");
            run_x11_mode()?;
        }
    }

    Ok(())
}

fn run_x11_mode() -> Result<()> {
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
