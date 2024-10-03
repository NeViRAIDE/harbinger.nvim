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
        footer::create_footer,
        header::{create_header, create_subheader},
        Content,
    },
    error::{handle_error, PluginError},
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

        self.content.add_element(create_header(&self.config.header));
        self.content
            .add_element(create_subheader(&self.config.sub_header));
        for button in create_buttons() {
            self.content.add_element(button);
        }
        self.content.add_element(create_footer(&self.config.footer));

        handle_error(
            set_keymap(
                Mode::Normal,
                &self.config.keymap,
                "<cmd>Harbinger<cr>",
                &Default::default(),
            ),
            "Failed to set keymap",
        )?;

        Ok(())
    }

    pub fn toggle_dashboard(&mut self) -> Result<(), PluginError> {
        let current_buf = get_current_buf();
        let buf_opts = OptionOpts::builder().scope(OptionScope::Local).build();
        let filetype: String = handle_error(
            get_option_value("filetype", &buf_opts),
            "Failed to get filetype",
        )?;

        if filetype == "harbinger" {
            let alternate_buf = list_bufs().find(|b| *b != current_buf && b.is_valid());

            if let Some(alternate_buf) = alternate_buf {
                handle_error(set_current_buf(&alternate_buf), "Failed to switch buffer")?;
            } else {
                let temp_buf =
                    handle_error(create_buf(false, true), "Failed to create temporary buffer")?;
                handle_error(set_current_buf(&temp_buf), "Failed to set temporary buffer")?;
            }

            if current_buf.is_valid() {
                handle_error(
                    BufferManager::delete_buffer(&current_buf),
                    "Failed to delete buffer",
                )?;
            }
        } else {
            match create_buf(false, true) {
                Ok(mut buf) => {
                    handle_error(set_current_buf(&buf), "Failed to set current buffer")?;

                    let (dashboard_content, button_count, button_index) = self.content.render();
                    handle_error(
                        BufferManager::set_buffer_content(&mut buf, &dashboard_content.join("\n")),
                        "Failed to set buffer content",
                    )?;

                    handle_error(
                        BufferManager::configure_buffer(&mut buf),
                        "Failed to configure buffer",
                    )?;

                    handle_error(
                        get_current_win().set_cursor(button_index - 1, 0),
                        "Failed to set cursor position",
                    )?;

                    self.create_autocmd_for_buffer_deletion(buf)?;
                }
                Err(e) => {
                    return Err(PluginError::Custom(format!(
                        "Failed to create buffer: {}",
                        e
                    )));
                }
            }
        }

        Ok(())
    }

    fn create_autocmd_for_buffer_deletion(&self, buf: Buffer) -> Result<(), PluginError> {
        let autocmd_opts = CreateAutocmdOpts::builder()
            .buffer(buf.clone())
            .callback(move |_| {
                if let Err(e) = BufferManager::delete_buffer(&buf) {
                    err_writeln(&format!("Failed to delete buffer: {}", e));
                }
                true
            })
            .build();

        handle_error(
            create_autocmd(["BufLeave"], &autocmd_opts),
            "Failed to create autocmd",
        )?;

        Ok(())
    }
}
