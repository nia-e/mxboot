// Themes from https://gitlab.com/cherrypicker/unl0kr/-/blob/master/themes.c
// (c) 2021 Johannes Marbach for the original theme

use lvgl::{style::Style, Color, State};

#[repr(i16)]
enum BorderSide {
    None = 0x00,
    Bottom = 0x01,
    Top = 0x02,
    Left = 0x04,
    Right = 0x08,
    Full = 0x0f,
    Internal = 0x10
}

pub struct MxTheme {
    pub window: MxWindow,
    pub header: MxHeader,
    pub keyboard: MxKeyboard,
    pub button: MxButton,
    pub text_area: MxTextArea,
    pub dropdown: MxDropdown,
    pub label: MxLabel,
    pub msgbox: MxMsgbox,
    pub bar: MxBar,
}

pub struct MxWindow {
    pub bg_color: Color,
}

pub struct MxHeader {
    pub bg_color: Color,
    pub border_width: i16,
    pub border_color: Color,
    pub pad: i16,
    pub gap: i16,
}

pub struct MxKeyboard {
    pub bg_color: Color,
    pub border_width: i16,
    pub border_color: Color,
    pub pad: i16,
    pub gap: i16,
    pub keys: MxKeys,
}

pub struct MxKeys {
    pub border_width: i16,
    pub corner_radius: i16,
    pub key_char: MxKeyType,
    pub key_non_char: MxKeyType,
    pub key_mod_act: MxKeyType,
    pub key_mod_inact: MxKeyType,
}

pub struct MxKeyType {
    pub normal: MxKeyState,
    pub pressed: MxKeyState,
}

pub struct MxKeyState {
    pub fg_color: Color,
    pub bg_color: Color,
    pub border_color: Color,
}

pub struct MxButton {
    pub border_width: i16,
    pub corner_radius: i16,
    pub pad: i16,
    pub normal: MxKeyState,
    pub pressed: MxKeyState,
}

pub struct MxTextArea {
    pub fg_color: Color,
    pub bg_color: Color,
    pub border_width: i16,
    pub border_color: Color,
    pub corner_radius: i16,
    pub pad: i16,
    pub placeholder_color: Color,
    pub cursor: MxCursor,
}

pub struct MxCursor {
    pub width: i16,
    pub color: Color,
    pub period: i16,
}

pub struct MxDropdown {
    pub button: MxButton,
    pub list: MxList,
}

pub struct MxList {
    pub fg_color: Color,
    pub bg_color: Color,
    pub selection_fg_color: Color,
    pub selection_bg_color: Color,
    pub border_width: i16,
    pub border_color: Color,
    pub corner_radius: i16,
    pub pad: i16,
}

pub struct MxLabel {
    pub fg_color: Color,
}

pub struct MxMsgbox {
    pub fg_color: Color,
    pub bg_color: Color,
    pub border_width: i16,
    pub border_color: Color,
    pub corner_radius: i16,
    pub pad: i16,
    pub gap: i16,
    pub buttons: MxMsgButtons,
    pub dimming: MxDimming,
}

pub struct MxMsgButtons {
    pub gap: i16,
}

pub struct MxDimming {
    pub color: Color,
    pub opacity: i16,
}

pub struct MxBar {
    pub border_width: i16,
    pub border_color: Color,
    pub corner_radius: i16,
    pub indicator: MxIndicator,
}

pub struct MxIndicator {
    pub bg_color: Color,
}

impl MxTheme {
    /// Gets the style to be used on windows, given the current theme.
    pub fn style_window(&self) -> Style {
        let mut style = Style::default();
        style.set_bg_color(State::DEFAULT, self.window.bg_color.clone());
        style
    }

    /// Gets the style to be used on the header, given the current theme.
    pub fn style_header(&self) -> Style {
        let mut style = Style::default();
        style.set_bg_color(State::DEFAULT, self.header.bg_color.clone());
        style.set_border_side(State::DEFAULT, BorderSide::Bottom as i16);
        style.set_border_width(State::DEFAULT, self.header.border_width);
        style.set_border_color(State::DEFAULT, self.header.border_color.clone());
        style.set_pad_bottom(State::DEFAULT, self.header.pad);
        style.set_pad_left(State::DEFAULT, self.header.pad);
        style.set_pad_top(State::DEFAULT, self.header.pad);
        style.set_pad_right(State::DEFAULT, self.header.pad);
        style.set_margin_bottom(State::DEFAULT, self.header.gap);
        style.set_margin_left(State::DEFAULT, self.header.gap);
        style.set_margin_top(State::DEFAULT, self.header.gap);
        style.set_margin_right(State::DEFAULT, self.header.gap);
        style
    }

    /// Gets the style to be used on the keyboard, given the current theme.
    pub fn style_keyboard(&self) -> Style {
        let mut style = Style::default();
        style.set_bg_color(State::DEFAULT, self.keyboard.bg_color.clone());
        style.set_border_side(State::DEFAULT, BorderSide::Top as i16);
        style.set_border_width(State::DEFAULT, self.keyboard.border_width);
        style.set_border_color(State::DEFAULT, self.keyboard.border_color.clone());
        style.set_pad_bottom(State::DEFAULT, self.keyboard.pad);
        style.set_pad_left(State::DEFAULT, self.keyboard.pad);
        style.set_pad_top(State::DEFAULT, self.keyboard.pad);
        style.set_pad_right(State::DEFAULT, self.keyboard.pad);
        style.set_margin_bottom(State::DEFAULT, self.keyboard.gap);
        style.set_margin_left(State::DEFAULT, self.keyboard.gap);
        style.set_margin_top(State::DEFAULT, self.keyboard.gap);
        style.set_margin_right(State::DEFAULT, self.keyboard.gap);
        style
    }

    pub fn style_key(&self) -> Style {
        let mut style = Style::default();
        // ...
        style
    }

    /// postmarketOS dark theme. Based on the palette from unl0kr, itself from
    /// https://coolors.co/009900-395e66-db504a-e3b505-ebf5ee
    pub fn pmos_dark() -> Self {
        Self {
            window: MxWindow { bg_color: Color::from_rgb((0x07, 0x0c, 0x0d)) },
            header: MxHeader {
                bg_color: Color::from_rgb((0x07, 0x0c, 0x0d)),
                border_width: 0,
                border_color: Color::from_rgb((0x07, 0x0c, 0x0d)),
                pad: 20,
                gap: 10,
            },
            keyboard: MxKeyboard {
                bg_color: Color::from_rgb((0x16, 0x24, 0x27)),
                border_width: 2,
                border_color: Color::from_rgb((0x39, 0x5e, 0x66)),
                pad: 20,
                gap: 10,
                keys: MxKeys {
                    border_width: 1,
                    corner_radius: 3,
                    key_char: MxKeyType {
                        normal: MxKeyState {
                            fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                            bg_color: Color::from_rgb((0x16, 0x24, 0x27)),
                            border_color: Color::from_rgb((0x39, 0x5e, 0x66)),
                        },
                        pressed: MxKeyState {
                            fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                            bg_color: Color::from_rgb((0x00, 0x99, 0x00)),
                            border_color: Color::from_rgb((0x00, 0x99, 0x00)),
                        },
                    },
                    key_non_char: MxKeyType {
                        normal: MxKeyState {
                            fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                            bg_color: Color::from_rgb((0x25, 0x3c, 0x41)),
                            border_color: Color::from_rgb((0x2c, 0x48, 0x4e)),
                        },
                        pressed: MxKeyState {
                            fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                            bg_color: Color::from_rgb((0x00, 0x99, 0x00)),
                            border_color: Color::from_rgb((0x00, 0x99, 0x00)),
                        },
                    },
                    key_mod_act: MxKeyType {
                        normal: MxKeyState {
                            fg_color: Color::from_rgb((0x00, 0x99, 0x00)),
                            bg_color: Color::from_rgb((0x25, 0x3c, 0x41)),
                            border_color: Color::from_rgb((0x00, 0x99, 0x00)),
                        },
                        pressed: MxKeyState {
                            fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                            bg_color: Color::from_rgb((0x00, 0x99, 0x00)),
                            border_color: Color::from_rgb((0x00, 0x99, 0x00)),
                        },
                    },
                    key_mod_inact: MxKeyType {
                        normal: MxKeyState {
                            fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                            bg_color: Color::from_rgb((0x25, 0x3c, 0x41)),
                            border_color: Color::from_rgb((0x2c, 0x48, 0x4e)),
                        },
                        pressed: MxKeyState {
                            fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                            bg_color: Color::from_rgb((0x00, 0x99, 0x00)),
                            border_color: Color::from_rgb((0x00, 0x99, 0x00)),
                        },
                    },
                },
            },
            button: MxButton {
                border_width: 1,
                corner_radius: 3,
                pad: 8,
                normal: MxKeyState {
                    fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                    bg_color: Color::from_rgb((0x25, 0x3c, 0x41)),
                    border_color: Color::from_rgb((0x2c, 0x48, 0x4e))
                },
                pressed: MxKeyState {
                    fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                    bg_color: Color::from_rgb((0x00, 0x99, 0x00)),
                    border_color: Color::from_rgb((0x00, 0x99, 0x00))
                }
            },
            text_area: MxTextArea {
                fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                bg_color: Color::from_rgb((0x00, 0x29, 0x00)),
                border_width: 1,
                border_color: Color::from_rgb((0x00, 0x99, 0x00)),
                corner_radius: 3,
                pad: 8,
                placeholder_color: Color::from_rgb((0x00, 0x99, 0x00)),
                cursor: MxCursor {
                    width: 2,
                    color: Color::from_rgb((0x00, 0x99, 0x00)),
                    period: 700
                }
            },
            dropdown: MxDropdown {
                button: MxButton {
                    border_width: 1,
                    corner_radius: 3,
                    pad: 8,
                    normal: MxKeyState {
                        fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                        bg_color: Color::from_rgb((0x25, 0x3c, 0x41)),
                        border_color: Color::from_rgb((0x2c, 0x48, 0x4e))
                    },
                    pressed: MxKeyState {
                        fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                        bg_color: Color::from_rgb((0x00, 0x99, 0x00)),
                        border_color: Color::from_rgb((0x00, 0x99, 0x00))
                    }
                },
                list: MxList {
                    fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                    bg_color: Color::from_rgb((0x16, 0x24, 0x27)),
                    selection_fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                    selection_bg_color: Color::from_rgb((0x00, 0x99, 0x00)),
                    border_width: 1,
                    border_color: Color::from_rgb((0x39, 0x5e, 0x66)),
                    corner_radius: 0,
                    pad: 8
                }
            },
            label: MxLabel {
                fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8))
            },
            msgbox: MxMsgbox {
                fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                bg_color: Color::from_rgb((0x16, 0x24, 0x27)),
                border_width: 1,
                border_color: Color::from_rgb((0x39, 0x5e, 0x66)),
                corner_radius: 3,
                pad: 20,
                gap: 20,
                buttons: MxMsgButtons {
                    gap: 10
                },
                dimming: MxDimming {
                    color: Color::from_rgb((0x07, 0x0c, 0x0d)),
                    opacity: 225
                }
            },
            bar: MxBar {
                border_width: 1,
                border_color: Color::from_rgb((0x00, 0x99, 0x00)),
                corner_radius: 3,
                indicator: MxIndicator {
                    bg_color: Color::from_rgb((0x00, 0x99, 0x00))
                }
            }
        }
    }
}
