use anyhow::{Context, Result};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConnectionExt, EventMask, GrabMode, ModMask, Window};
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;

pub struct EventHandler {
    conn: RustConnection,
    screen_num: usize,
    active: bool,
}

impl EventHandler {
    pub fn new(conn: RustConnection, screen_num: usize) -> Self {
        Self {
            conn,
            screen_num,
            active: false,
        }
    }

    pub fn grab_pointer(&mut self) -> Result<()> {
        let root = self.conn.setup().roots[self.screen_num].root;

        self.conn
            .grab_pointer(
                false,
                root,
                u32::from(EventMask::BUTTON_PRESS | EventMask::BUTTON_RELEASE) as u16,
                GrabMode::ASYNC,
                GrabMode::ASYNC,
                x11rb::NONE,
                x11rb::NONE,
                x11rb::CURRENT_TIME,
            )
            .context("Falha ao capturar ponteiro")?
            .reply()
            .context("Falha ao obter resposta do grab")?;

        self.active = true;
        Ok(())
    }

    pub fn ungrab_pointer(&mut self) -> Result<()> {
        self.conn
            .ungrab_pointer(x11rb::CURRENT_TIME)
            .context("Falha ao liberar ponteiro")?;
        self.conn.flush().context("Falha ao flush")?;
        self.active = false;
        Ok(())
    }

    pub fn wait_for_window_click(&mut self) -> Result<Option<Window>> {
        if !self.active {
            self.grab_pointer()?;
        }

        loop {
            let event = self
                .conn
                .wait_for_event()
                .context("Falha ao aguardar evento")?;

            match event {
                Event::ButtonPress(btn_event) => {
                    self.ungrab_pointer()?;
                    return Ok(Some(self.get_top_level_window(btn_event.child)?));
                }
                Event::KeyPress(_) => {
                    self.ungrab_pointer()?;
                    return Ok(None);
                }
                _ => continue,
            }
        }
    }

    fn get_top_level_window(&self, mut window: Window) -> Result<Window> {
        let root = self.conn.setup().roots[self.screen_num].root;

        loop {
            let tree = self
                .conn
                .query_tree(window)
                .context("Falha ao consultar árvore")?
                .reply()
                .context("Falha ao obter resposta")?;

            if tree.parent == root || tree.parent == 0 {
                return Ok(window);
            }

            window = tree.parent;
        }
    }

    pub fn register_hotkey(&self, modifiers: ModMask, keycode: u8) -> Result<()> {
        let root = self.conn.setup().roots[self.screen_num].root;

        self.conn
            .grab_key(
                false,
                root,
                modifiers,
                keycode,
                GrabMode::ASYNC,
                GrabMode::ASYNC,
            )
            .context("Falha ao registrar hotkey")?;

        self.conn.flush().context("Falha ao flush")?;
        Ok(())
    }
}
