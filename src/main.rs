mod events;
mod ui;
mod wayland;
mod x11;

use anyhow::Result;
use events::EventHandler;
use ui::{create_notification, AppUI, UICommand};
use wayland::{detect_session_type, SessionType};
use x11::X11Handler;
use x11rb::rust_connection::RustConnection;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--cli" {
        run_cli_mode()?;
    } else {
        run_gui_mode()?;
    }

    Ok(())
}

fn run_cli_mode() -> Result<()> {
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

fn run_gui_mode() -> Result<()> {
    let ui = AppUI::new()?;
    let (_tx_status, rx_cmd) = ui.run();

    std::thread::spawn(move || {
        while let Ok(cmd) = rx_cmd.recv() {
            match cmd {
                UICommand::Kill => {
                    if let Err(e) = run_x11_mode() {
                        eprintln!("Erro ao fechar janela: {}", e);
                        create_notification("Erro", &format!("Falha: {}", e)).ok();
                    }
                }
                UICommand::Quit => {
                    std::process::exit(0);
                }
            }
        }
    });

    ui.execute();
    Ok(())
}

fn run_x11_mode() -> Result<()> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let handler = X11Handler::new()?;
    let mut event_handler = EventHandler::new(conn, screen_num);

    if let Some(window) = event_handler.wait_for_window_click()? {
        if let Ok(name) = handler.get_window_name(window) {
            create_notification("XKill", &format!("Fechando: {}", name)).ok();
        }
        handler.kill_window(window)?;
        create_notification("XKill", "Janela fechada com sucesso").ok();
    }

    Ok(())
}
