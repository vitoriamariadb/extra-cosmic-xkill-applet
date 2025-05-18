mod config;
mod events;
mod logging;
mod ui;
mod wayland;
mod x11;

use anyhow::Result;
use config::Config;
use events::EventHandler;
use ui::{create_notification, AppUI, UICommand};
use wayland::{detect_session_type, SessionType};
use x11::X11Handler;
use x11rb::rust_connection::RustConnection;

fn main() -> Result<()> {
    logging::info("main", "Iniciando Extra Cosmic XKill Applet");
    let config = Config::load()?;
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--cli" {
        run_cli_mode(&config)?;
    } else {
        run_gui_mode(&config)?;
    }

    Ok(())
}

fn run_cli_mode(config: &Config) -> Result<()> {
    println!("Extra Cosmic XKill Applet v0.1.0");

    let session_type = detect_session_type();
    println!("Tipo de sessão detectada: {}", session_type.as_str());

    let use_wayland = config.general.prefer_wayland && session_type == SessionType::Wayland;

    if use_wayland {
        println!("Modo Wayland preferido, mas ainda não implementado");
        println!("Usando fallback X11...");
    }

    run_x11_mode(config)?;
    Ok(())
}

fn run_gui_mode(config: &Config) -> Result<()> {
    let ui = AppUI::new()?;
    let (_tx_status, rx_cmd) = ui.run();

    let config_clone = config.clone();
    std::thread::spawn(move || {
        while let Ok(cmd) = rx_cmd.recv() {
            match cmd {
                UICommand::Kill => {
                    if let Err(e) = run_x11_mode(&config_clone) {
                        eprintln!("Erro ao fechar janela: {}", e);
                        if config_clone.general.show_notifications {
                            create_notification("Erro", &format!("Falha: {}", e)).ok();
                        }
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

fn run_x11_mode(config: &Config) -> Result<()> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let handler = X11Handler::new()?;
    let mut event_handler = EventHandler::new(conn, screen_num);

    if let Some(window) = event_handler.wait_for_window_click()? {
        if config.ui.show_window_names {
            if let Ok(name) = handler.get_window_name(window) {
                if config.general.show_notifications {
                    create_notification("XKill", &format!("Fechando: {}", name)).ok();
                }
            }
        }
        handler.kill_window(window)?;
        if config.general.show_notifications {
            create_notification("XKill", "Janela fechada com sucesso").ok();
        }
    }

    Ok(())
}

// "O impedimento para a acao impulsiona a acao. O que esta no caminho se torna o caminho." - Marco Aurelio
