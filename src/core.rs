use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{
    api::{
        create_autocmd, create_buf, err_writeln, opts::CreateAutocmdOpts, set_current_buf, Buffer,
    },
    Dictionary, Result as OxiResult,
};

use crate::buffer::BufferManager;
use crate::setup::Config;

#[derive(Debug)]
pub struct Dashboard {
    pub config: Config,
    dashboard_buf_id: Rc<RefCell<Option<Buffer>>>,
}

impl Dashboard {
    pub fn new(config: Config) -> Self {
        Dashboard {
            config,
            dashboard_buf_id: Rc::new(RefCell::new(None)),
        }
    }

    pub fn setup(&mut self, dict: Dictionary) -> OxiResult<()> {
        self.config = Config::from_dict(dict);
        Ok(())
    }

    pub fn toggle_dashboard(&self) -> OxiResult<()> {
        let mut dashboard_buf_id = self.dashboard_buf_id.borrow_mut();

        if let Some(buf) = dashboard_buf_id.as_ref() {
            if buf.is_valid() {
                // Удаляем буфер с опцией force
                BufferManager::delete_buffer(buf)?;
            }
            *dashboard_buf_id = None;
        } else {
            // Создаём новый буфер
            match create_buf(false, true) {
                Ok(mut buf) => {
                    set_current_buf(&buf)?;

                    let dashboard_text =
                        "Welcome to Neovim!\n===================\n\nUse :q to quit, or :e to open a file.";

                    BufferManager::set_buffer_content(&mut buf, dashboard_text)?;
                    BufferManager::configure_buffer()?;

                    *dashboard_buf_id = Some(buf.clone());

                    let buf_clone = buf.clone();

                    // Создаем автокоманду для удаления буфера при его выгрузке
                    let autocmd_opts = CreateAutocmdOpts::builder()
                        .buffer(buf_clone.clone())
                        .callback(move |_| {
                            if let Err(e) = BufferManager::delete_buffer(&buf_clone) {
                                err_writeln(&format!("Failed to delete buffer: {}", e));
                            }
                            true
                        })
                        .build();

                    create_autocmd(["BufLeave"], &autocmd_opts)?;
                }
                Err(e) => {
                    err_writeln(&format!("Failed to create buffer: {}", e));
                }
            }
        }

        Ok(())
    }
}
