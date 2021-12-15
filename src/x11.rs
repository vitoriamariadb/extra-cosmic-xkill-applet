use anyhow::{Context, Result};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConnectionExt, Window};
use x11rb::rust_connection::RustConnection;

pub struct X11Handler {
    conn: RustConnection,
    screen_num: usize,
}

impl X11Handler {
    pub fn new() -> Result<Self> {
        let (conn, screen_num) = RustConnection::connect(None)
            .context("Falha ao conectar ao servidor X11")?;

        Ok(Self { conn, screen_num })
    }

    pub fn get_root_window(&self) -> Window {
        self.conn.setup().roots[self.screen_num].root
    }

    pub fn list_windows(&self) -> Result<Vec<Window>> {
        let root = self.get_root_window();
        let tree = self
            .conn
            .query_tree(root)
            .context("Falha ao consultar árvore de janelas")?
            .reply()
            .context("Falha ao obter resposta da árvore")?;

        Ok(tree.children)
    }

    pub fn kill_window(&self, window: Window) -> Result<()> {
        self.conn
            .kill_client(window)
            .context("Falha ao matar janela")?;
        self.conn.flush().context("Falha ao flush da conexão")?;
        Ok(())
    }

    pub fn get_window_name(&self, window: Window) -> Result<String> {
        let reply = self
            .conn
            .get_property(
                false,
                window,
                x11rb::protocol::xproto::AtomEnum::WM_NAME,
                x11rb::protocol::xproto::AtomEnum::STRING,
                0,
                1024,
            )
            .context("Falha ao obter propriedade da janela")?
            .reply()
            .context("Falha ao obter resposta da propriedade")?;

        let name = String::from_utf8_lossy(&reply.value).to_string();
        Ok(name)
    }
}
