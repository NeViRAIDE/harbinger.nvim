use nvim_oxi::{
    api::{
        create_autocmd, create_buf, err_writeln, get_current_buf, get_option_value, list_bufs,
        opts::{CreateAutocmdOpts, OptionOpts, OptionScope},
        set_current_buf, set_keymap,
        types::Mode,
    },
    Dictionary, Result as OxiResult,
};

use crate::{
    buffer::BufferManager,
    content::{
        button::create_buttons,
        footer::create_footer,
        header::{create_header, create_subheader},
        Content,
    },
    setup::Config,
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

        set_keymap(
            Mode::Normal,
            &self.config.keymap,
            "<cmd>Harbinger<cr>",
            &Default::default(),
        )?;

        Ok(())
    }

    pub fn toggle_dashboard(&self) -> OxiResult<()> {
        let current_buf = get_current_buf();

        let buf_opts = OptionOpts::builder().scope(OptionScope::Local).build();
        let filetype: String = get_option_value("filetype", &buf_opts)?;

        if filetype == "harbinger" {
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
        } else {
            match create_buf(false, true) {
                Ok(mut buf) => {
                    set_current_buf(&buf)?;

                    let dashboard_content = self.content.render();

                    BufferManager::set_buffer_content(&mut buf, &dashboard_content.join("\n"))?;
                    BufferManager::configure_buffer()?;

                    let buf_clone = buf.clone();
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
