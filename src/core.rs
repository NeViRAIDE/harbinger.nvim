use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use nvim_oxi::{
    api::{
        create_buf, get_current_buf, get_current_win, get_option_value, list_bufs,
        opts::{OptionOpts, OptionScope, SetKeymapOpts},
        set_current_buf, set_keymap, set_option_value,
        types::Mode,
        Buffer,
    },
    Dictionary, Function, Result as OxiResult,
};

use crate::{
    autocmd::buffer_delete,
    buffer::BufferManager,
    config::Config,
    content::{
        button::create_buttons,
        empty_line::EmptyLineElement,
        footer::create_footer,
        header::{create_header, create_subheader},
        Content,
    },
    error::PluginError,
    utils::count_file_buffers,
};

pub struct Dashboard {
    pub config: Config,
    pub content: Content,
    pub previous_buf: Option<Buffer>,
}

impl Dashboard {
    pub fn new(config: Config) -> Self {
        Dashboard {
            config,
            content: Content::new(),
            previous_buf: None,
        }
    }

    pub fn setup(&mut self, dict: Dictionary, app_handle: Rc<RefCell<Dashboard>>) -> OxiResult<()> {
        self.config = Config::from_dict(dict);

        self.content = Content::new();

        self.content.add_element(create_header(
            &self.config.header.text,
            &self.config.header.position,
        ));
        // Add empty line after header
        self.content.add_element(Box::new(EmptyLineElement));

        self.content.add_element(create_subheader(
            &self.config.sub_header.text,
            &self.config.sub_header.position,
        ));
        // Add empty line after subheader
        self.content.add_element(Box::new(EmptyLineElement));

        for button in create_buttons(&self.config.buttons.items, &self.config.buttons.position) {
            self.content.add_element(button);
        }
        // Add empty line before footer
        self.content.add_element(Box::new(EmptyLineElement));

        self.content.add_element(create_footer(
            &self.config.footer.text,
            &self.config.footer.position,
        ));

        let app_handle_clone = Rc::clone(&app_handle);

        set_keymap(
            Mode::Normal,
            &self.config.keymap,
            "",
            &SetKeymapOpts::builder()
                .callback(Function::from_fn(move |_| -> OxiResult<()> {
                    app_handle_clone.borrow_mut().toggle_dashboard()
                }))
                .build(),
        )?;

        Ok(())
    }

    pub fn toggle_dashboard(&mut self) -> OxiResult<()> {
        let current_buf = get_current_buf();
        let buf_opts = OptionOpts::builder().scope(OptionScope::Local).build();
        let filetype: String = get_option_value("filetype", &buf_opts)?;

        if filetype == "harbinger" {
            self.close_dashboard(current_buf)
        } else {
            self.open_dashboard()
        }
    }

    fn close_dashboard(&mut self, current_buf: Buffer) -> OxiResult<()> {
        let file_buf_count = count_file_buffers();

        if file_buf_count <= 1 {
            // Do not close the dashboard if there are no other file buffers
            return Ok(());
        }

        if let Some(prev_buf) = self.previous_buf.take() {
            if prev_buf.is_valid() {
                set_current_buf(&prev_buf)?;
            } else {
                // Switch to another valid buffer if available
                for buf in list_bufs() {
                    if buf.is_valid() && buf != current_buf {
                        set_current_buf(&buf)?;
                        break;
                    }
                }
            }
        } else {
            // No previous buffer; switch to another valid buffer if available
            for buf in list_bufs() {
                if buf.is_valid() && buf != current_buf {
                    set_current_buf(&buf)?;
                    break;
                }
            }
        }

        if current_buf.is_valid() {
            BufferManager::delete_buffer(&current_buf)?;
        }

        Ok(())
    }

    fn open_dashboard(&mut self) -> OxiResult<()> {
        let file_buf_count = count_file_buffers();

        if file_buf_count == 0 {
            // There are no file buffers open
            self.previous_buf = None;
        } else {
            self.previous_buf = Some(get_current_buf());
        }
        let current_buf = get_current_buf();

        // Get the buffer name
        let buf_name = current_buf.get_name()?;
        let buf_name_str = buf_name.to_str().unwrap_or("");

        // Create OptionOpts for the current buffer
        let buf_opts = OptionOpts::builder()
            // .scope(OptionScope::Buffer)
            .buffer(current_buf.clone())
            .build();

        // Check if the buffer is modified
        let is_modified: bool = get_option_value("modified", &buf_opts)?;

        if buf_name_str.is_empty() && !is_modified {
            // Set 'bufhidden' to 'wipe' for the initial empty buffer
            set_option_value("bufhidden", "wipe", &buf_opts)?;
        }

        let mut buf = create_buf(false, true)?;
        set_current_buf(&buf)?;

        let (dashboard_content, button_count, first_button_line, command_mapping, highlights) =
            self.content.render();

        let top_padding =
            BufferManager::set_buffer_content(&mut buf, &dashboard_content.join("\n"))?;

        if button_count == 0 {
            return Err(PluginError::Custom("No buttons found for keybinds".into()).into());
        }

        // Adjust first and last button lines to account for top padding
        let adjusted_first_button_line = first_button_line + top_padding;
        let last_button_line = adjusted_first_button_line + button_count - 1;

        // Adjust the command_mapping line numbers
        let adjusted_command_mapping: HashMap<usize, String> = command_mapping
            .into_iter()
            .map(|(line, command)| (line + top_padding, command))
            .collect();
        let command_mapping = Arc::new(adjusted_command_mapping);

        // Set the cursor line (line numbers are 1-based)
        get_current_win().set_cursor(adjusted_first_button_line, 0)?;

        // Adjust the cursor column
        let line_iter = buf.get_lines(
            adjusted_first_button_line - 1..adjusted_first_button_line,
            false,
        )?;
        let line_content: Vec<String> = line_iter.map(|s| s.to_string()).collect();

        if let Some(line) = line_content.first() {
            let col = line.chars().take_while(|c| c.is_whitespace()).count();
            get_current_win().set_cursor(adjusted_first_button_line, col)?;
        }

        BufferManager::configure_buffer(
            &mut buf,
            adjusted_first_button_line,
            last_button_line,
            Arc::clone(&command_mapping),
        )?;

        let ns_id = nvim_oxi::api::create_namespace("harbinger");

        for (line_number, (highlight_group, text_start_pos, text_len)) in highlights.iter() {
            if *text_len > 0 {
                nvim_oxi::print!(
                    "Applying highlight for group: {} on line: {} from position: {}",
                    highlight_group,
                    line_number + top_padding,
                    text_start_pos
                );
                if let Err(err) = buf.add_highlight(
                    ns_id,
                    highlight_group,
                    line_number + top_padding - 1,
                    *text_start_pos..(*text_start_pos + text_len),
                ) {
                    nvim_oxi::api::err_writeln(&format!("Failed to apply highlight: {}", err));
                }
            }
        }

        // Set up autocommand to delete dashboard buffer when appropriate
        buffer_delete(buf.clone())?;

        Ok(())
    }
}
