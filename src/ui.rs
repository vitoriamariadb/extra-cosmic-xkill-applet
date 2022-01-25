use anyhow::{Context, Result};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct AppUI {
    app: Application,
}

#[derive(Debug, Clone)]
pub enum UICommand {
    Kill,
    Quit,
}

impl AppUI {
    pub fn new() -> Result<Self> {
        let app = Application::builder()
            .application_id("com.vitoriamaria.xkill")
            .build();

        Ok(Self { app })
    }

    pub fn run(&self) -> (Sender<String>, Receiver<UICommand>) {
        let (tx_status, rx_status) = channel();
        let (tx_cmd, rx_cmd) = channel();

        let tx_cmd_clone = tx_cmd.clone();

        self.app.connect_activate(move |app| {
            let window = ApplicationWindow::builder()
                .application(app)
                .title("Extra Cosmic XKill")
                .default_width(350)
                .default_height(200)
                .build();

            let vbox = Box::new(Orientation::Vertical, 10);
            vbox.set_margin_top(20);
            vbox.set_margin_bottom(20);
            vbox.set_margin_start(20);
            vbox.set_margin_end(20);

            let label = Label::new(Some("Gerenciador de Janelas"));
            label.set_markup("<b>Extra Cosmic XKill</b>");
            vbox.pack_start(&label, false, false, 0);

            let status_label = Label::new(Some("Pronto"));
            vbox.pack_start(&status_label, false, false, 10);

            let kill_btn = Button::with_label("Fechar Janela");
            let tx_kill = tx_cmd_clone.clone();
            kill_btn.connect_clicked(move |_| {
                tx_kill.send(UICommand::Kill).ok();
            });
            vbox.pack_start(&kill_btn, false, false, 5);

            let quit_btn = Button::with_label("Sair");
            let tx_quit = tx_cmd_clone.clone();
            quit_btn.connect_clicked(move |_| {
                tx_quit.send(UICommand::Quit).ok();
            });
            vbox.pack_start(&quit_btn, false, false, 5);

            window.add(&vbox);
            window.show_all();
        });

        (tx_status, rx_cmd)
    }

    pub fn execute(&self) {
        self.app.run();
    }
}

pub fn create_notification(title: &str, message: &str) -> Result<()> {
    use std::process::Command;

    Command::new("notify-send")
        .arg(title)
        .arg(message)
        .output()
        .context("Falha ao enviar notificação")?;

    Ok(())
}
