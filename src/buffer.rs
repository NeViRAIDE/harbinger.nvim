use std::cell::RefCell; // Импортируем RefCell
use std::rc::Rc;

use nvim_oxi::{
    api::{create_buf, opts::OptionOpts, set_option_value, Buffer},
    Result as OxiResult,
};

use crate::error::handle_error;

#[derive(Debug)]
pub struct BufferManager {
    buffer: Rc<RefCell<Buffer>>, // Используем RefCell для возможности мутации
}

#[derive(Debug)]
pub struct BufferConfig;

#[derive(Debug)]
pub struct BufferContent;

impl BufferManager {
    pub fn new() -> OxiResult<Self> {
        let buffer = handle_error(create_buf(false, true), "Failed to create new buffer")?;

        let buffer_options = OptionOpts::builder().buffer(buffer.clone()).build();

        set_option_value("buftype", "nofile", &buffer_options)?;
        set_option_value("bufhidden", "wipe", &buffer_options)?;
        set_option_value("swapfile", false, &buffer_options)?;

        Ok(BufferManager {
            buffer: Rc::new(RefCell::new(buffer)), // Оборачиваем Buffer в RefCell
        })
    }

    pub fn get_buffer(&self) -> Rc<RefCell<Buffer>> {
        Rc::clone(&self.buffer)
    }

    pub fn set_content(&self, content: &str) -> OxiResult<()> {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        // Получаем доступ к Buffer через borrow_mut()
        self.buffer.borrow_mut().set_lines(0.., true, lines)?; // Теперь можем изменять Buffer
        Ok(())
    }
}
