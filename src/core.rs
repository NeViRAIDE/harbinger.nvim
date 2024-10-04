use std::{collections::HashMap, sync::Arc};

use nvim_oxi::{
    api::{
        create_autocmd, create_buf, err_writeln, get_current_buf, get_current_win,
        get_option_value, list_bufs,
        opts::{CreateAutocmdOpts, OptionOpts, OptionScope},
        set_current_buf, set_keymap,
        types::Mode,
        Buffer,
    },
    Dictionary, Result as OxiResult,
};

use crate::{
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
};

pub struct Dashboard {
    pub config: Config,
    pub content: Content,
}

impl Dashboard {
    pub fn new(config: Config) -> Self {
        Dashboard {
            config,
            content: Content::new(),
        }
    }

    pub fn setup(&mut self, dict: Dictionary) -> OxiResult<()> {
        self.config = Config::from_dict(dict);

        self.content = Content::new();

        self.content
            .add_element(create_header(&self.config.header, &self.config.header_pos));
        // Add empty line after header
        self.content.add_element(Box::new(EmptyLineElement));

        self.content.add_element(create_subheader(
            &self.config.sub_header,
            &self.config.sub_header_pos,
        ));
        // Add empty line after subheader
        self.content.add_element(Box::new(EmptyLineElement));

        for button in create_buttons(&self.config.buttons, &self.config.buttons_pos) {
            self.content.add_element(button);
        }
        // Add empty line before footer
        self.content.add_element(Box::new(EmptyLineElement));

        self.content
            .add_element(create_footer(&self.config.footer, &self.config.footer_pos));

        set_keymap(
            Mode::Normal,
            &self.config.keymap,
            "<cmd>Harbinger<cr>",
            &Default::default(),
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

    fn close_dashboard(&self, current_buf: Buffer) -> OxiResult<()> {
        let alternate_buf = list_bufs().find(|b| *b != current_buf && b.is_valid());

        if let Some(alternate_buf) = alternate_buf {
            set_current_buf(&alternate_buf)?;
        } else {
            let temp_buf = create_buf(false, true)?;
            set_current_buf(&temp_buf)?;
        }

        if current_buf.is_valid() {
            BufferManager::delete_buffer(&current_buf)?;
        }

        Ok(())
    }

    fn open_dashboard(&mut self) -> OxiResult<()> {
        let mut buf = create_buf(false, true)?;
        set_current_buf(&buf)?;

        let (dashboard_content, button_count, first_button_line, command_mapping) =
            self.content.render();

        let top_padding =
            BufferManager::set_buffer_content(&mut buf, &dashboard_content.join("\n"))?;

        if button_count == 0 {
            return Err(PluginError::Custom("No buttons found for keybinds".into()).into());
        }

        // Adjust first and last button lines to account for top padding
        let adjusted_first_button_line = first_button_line + top_padding;
        let last_button_line = adjusted_first_button_line + button_count - 1;

        // **Adjust the command_mapping line numbers**
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

        self.create_autocmd_for_buffer_deletion(buf)
    }

    fn create_autocmd_for_buffer_deletion(&self, buf: Buffer) -> OxiResult<()> {
        let autocmd_opts = CreateAutocmdOpts::builder()
            .buffer(buf.clone()) // Clone buf here
            .callback({
                move |_| {
                    if let Err(e) = BufferManager::delete_buffer(&buf) {
                        err_writeln(&format!("Failed to delete buffer: {}", e));
                    }
                    true
                }
            })
            .build();

        create_autocmd(["BufLeave"], &autocmd_opts)?;

        Ok(())
    }
}
