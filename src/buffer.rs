use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{
    api::{create_buf, set_option_value, Buffer},
    Result as OxiResult,
};

use crate::error::ResultExt;

#[derive(Debug)]
pub struct BufferManager;

impl BufferManager {
    pub fn new() -> OxiResult<Self> {
        Ok(BufferManager {})
    }

    /// Создаёт новый буфер для виджета
    pub fn create_buffer(&self) -> OxiResult<Rc<RefCell<Buffer>>> {
        let buffer = create_buf(false, true).with_context("Failed to create new buffer")?;

        let buffer_options = nvim_oxi::api::opts::OptionOpts::builder()
            .buffer(buffer.clone())
            .build();

        set_option_value("buftype", "nofile", &buffer_options)?;
        set_option_value("bufhidden", "wipe", &buffer_options)?;
        set_option_value("swapfile", false, &buffer_options)?;

        Ok(Rc::new(RefCell::new(buffer)))
    }

    /// Устанавливает содержимое буфера
    pub fn set_content(&self, buffer: &Rc<RefCell<Buffer>>, content: &str) -> OxiResult<()> {
        let lines: Vec<String> = content.lines().map(String::from).collect();
        buffer.borrow_mut().set_lines(0.., true, lines)?;
        Ok(())
    }
}
