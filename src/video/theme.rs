// Themes from https://gitlab.com/cherrypicker/unl0kr/-/blob/master/themes.c
// (c) 2021 Johannes Marbach for the original theme

#![allow(dead_code)]

use lvgl::{
    style::{Opacity, Style},
    Color,
};

#[repr(u8)]
enum BorderSide {
    None = 0x00,
    Bottom = 0x01,
    Top = 0x02,
    Left = 0x04,
    Right = 0x08,
    Full = 0x0f,
    Internal = 0x10,
}

/// An mxboot theme. Initialized from presets, currently only `pmos_dark()`.
/// Methods on this struct return LVGL `Style` objects which can be added to
/// widgets.
pub struct MxTheme {
    window: MxWindow,
    header: MxHeader,
    keyboard: MxKeyboard,
    button: MxButton,
    text_area: MxTextArea,
    dropdown: MxDropdown,
    label: MxLabel,
    msgbox: MxMsgbox,
    bar: MxBar,
}

struct MxWindow {
    bg_color: Color,
}

struct MxHeader {
    bg_color: Color,
    border_width: i16,
    border_color: Color,
    pad: i16,
    gap: i16,
}

struct MxKeyboard {
    bg_color: Color,
    border_width: i16,
    border_color: Color,
    pad: i16,
    gap: i16,
    keys: MxKeys,
}

struct MxKeys {
    border_width: i16,
    corner_radius: i16,
    key_char: MxKeyType,
    key_non_char: MxKeyType,
    key_mod_act: MxKeyType,
    key_mod_inact: MxKeyType,
}

struct MxKeyType {
    normal: MxKeyState,
    pressed: MxKeyState,
}

struct MxKeyState {
    fg_color: Color,
    bg_color: Color,
    border_color: Color,
}

struct MxButton {
    border_width: i16,
    corner_radius: i16,
    pad: i16,
    normal: MxKeyState,
    pressed: MxKeyState,
}

struct MxTextArea {
    fg_color: Color,
    bg_color: Color,
    border_width: i16,
    border_color: Color,
    corner_radius: i16,
    pad: i16,
    placeholder_color: Color,
    cursor: MxCursor,
}

struct MxCursor {
    width: i16,
    color: Color,
    period: i16,
}

struct MxDropdown {
    button: MxButton,
    list: MxList,
}

struct MxList {
    fg_color: Color,
    bg_color: Color,
    selection_fg_color: Color,
    selection_bg_color: Color,
    border_width: i16,
    border_color: Color,
    corner_radius: i16,
    pad: i16,
}

struct MxLabel {
    fg_color: Color,
}

struct MxMsgbox {
    fg_color: Color,
    bg_color: Color,
    border_width: i16,
    border_color: Color,
    corner_radius: i16,
    pad: i16,
    gap: i16,
    //buttons: MxMsgButtons,
    dimming: MxDimming,
}

//struct MxMsgButtons {
//    gap: i16,
//}

struct MxDimming {
    color: Color,
    opacity: Opacity,
}

struct MxBar {
    border_width: i16,
    border_color: Color,
    corner_radius: i16,
    indicator: MxIndicator,
}

struct MxIndicator {
    bg_color: Color,
}

impl MxTheme {
    /// Gets the style to be used on windows, given the current theme.
    pub fn style_window(&self) -> Style {
        let mut style = Style::default();
        style.set_bg_opa(Opacity::OPA_COVER);
        style.set_bg_color(self.window.bg_color.clone());
        style
    }

    /// Gets the style to be used on the header, given the current theme.
    pub fn style_header(&self) -> Style {
        let mut style = Style::default();
        style.set_bg_opa(Opacity::OPA_COVER);
        style.set_bg_color(self.header.bg_color.clone());
        style.set_border_side(BorderSide::Bottom as u8);
        style.set_border_width(self.header.border_width);
        style.set_border_color(self.header.border_color.clone());
        style.set_pad_bottom(self.header.pad);
        style.set_pad_left(self.header.pad);
        style.set_pad_top(self.header.pad);
        style.set_pad_right(self.header.pad);
        style
    }

    /// Gets the style to be used on the keyboard, given the current theme.
    pub fn style_keyboard(&self) -> Style {
        let mut style = Style::default();
        style.set_bg_opa(Opacity::OPA_COVER);
        style.set_bg_color(self.keyboard.bg_color.clone());
        style.set_border_side(BorderSide::Top as u8);
        style.set_border_width(self.keyboard.border_width);
        style.set_border_color(self.keyboard.border_color.clone());
        style.set_pad_bottom(self.keyboard.pad);
        style.set_pad_left(self.keyboard.pad);
        style.set_pad_top(self.keyboard.pad);
        style.set_pad_right(self.keyboard.pad);
        style
    }

    /// Gets the style to be used on keys, given the current theme.
    pub fn style_key(&self) -> Style {
        let mut style = Style::default();
        style.set_bg_opa(Opacity::OPA_COVER);
        style.set_border_side(BorderSide::Full as u8);
        style.set_border_width(self.keyboard.keys.border_width);
        style.set_radius(self.keyboard.keys.corner_radius);
        style
    }

    /// Gets the style to be used on buttons, given the current theme.
    pub fn style_button(&self) -> Style {
        let mut style = Style::default();
        style.set_text_color(self.button.normal.fg_color.clone());
        style.set_bg_opa(Opacity::OPA_COVER);
        style.set_bg_color(self.button.normal.bg_color.clone());
        style.set_border_side(BorderSide::Full as u8);
        style.set_border_width(self.button.border_width);
        style.set_border_color(self.button.normal.border_color.clone());
        style.set_radius(self.button.corner_radius);
        style.set_pad_bottom(self.button.pad);
        style.set_pad_left(self.button.pad);
        style.set_pad_top(self.button.pad);
        style.set_pad_right(self.button.pad);
        style
    }

    /// Gets the style to be used on pressed buttons, given the current theme.
    pub fn style_button_pressed(&self) -> Style {
        let mut style = Style::default();
        style.set_text_color(self.button.pressed.fg_color.clone());
        style.set_bg_color(self.button.pressed.bg_color.clone());
        style.set_border_color(self.button.pressed.border_color.clone());
        style
    }

    /// Gets the style to be used on text areas, given the current theme.
    pub fn style_text_area(&self) -> Style {
        let mut style = Style::default();
        style.set_text_color(self.text_area.fg_color.clone());
        style.set_bg_opa(Opacity::OPA_COVER);
        style.set_bg_color(self.text_area.bg_color.clone());
        style.set_border_side(BorderSide::Full as u8);
        style.set_border_width(self.text_area.border_width);
        style.set_border_color(self.text_area.border_color.clone());
        style.set_radius(self.text_area.corner_radius);
        style.set_pad_bottom(self.text_area.pad);
        style.set_pad_left(self.text_area.pad);
        style.set_pad_top(self.text_area.pad);
        style.set_pad_right(self.text_area.pad);
        style
    }

    /// Gets the style to be used on text area placeholders, given the current
    /// theme.
    pub fn style_text_area_placeholder(&self) -> Style {
        let mut style = Style::default();
        style.set_text_color(self.text_area.placeholder_color.clone());
        style
    }

    /// Gets the style to be used on cursor text areas, given the current
    /// theme.
    pub fn style_text_area_cursor(&self) -> Style {
        let mut style = Style::default();
        style.set_border_side(BorderSide::Left as u8);
        style.set_border_width(self.text_area.cursor.width);
        style.set_border_color(self.text_area.cursor.color.clone());
        style
    }

    /// Gets the style to be used on dropdown menys, given the current theme.
    pub fn style_dropdown(&self) -> Style {
        let mut style = Style::default();
        style.set_text_color(self.dropdown.button.normal.fg_color.clone());
        style.set_bg_opa(Opacity::OPA_COVER);
        style.set_bg_color(self.dropdown.button.normal.bg_color.clone());
        style.set_border_side(BorderSide::Full as u8);
        style.set_border_width(self.dropdown.button.border_width);
        style.set_border_color(
            
            self.dropdown.button.normal.border_color.clone(),
        );
        style.set_radius(self.dropdown.button.corner_radius);
        style.set_pad_bottom(self.dropdown.button.pad);
        style.set_pad_left(self.dropdown.button.pad);
        style.set_pad_top(self.dropdown.button.pad);
        style.set_pad_right(self.dropdown.button.pad);
        style
    }

    /// Gets the style to be used on pressed dropdown menus, given the current
    /// theme.
    pub fn style_dropdown_pressed(&self) -> Style {
        let mut style = Style::default();
        style.set_text_color(
            
            self.dropdown.button.pressed.fg_color.clone(),
        );
        style.set_bg_color(
            
            self.dropdown.button.pressed.bg_color.clone(),
        );
        style.set_border_color(
            
            self.dropdown.button.pressed.border_color.clone(),
        );
        style
    }

    /// Gets the style to be used on dropdown menu lists, given the current
    /// theme.
    pub fn style_dropdown_list(&self) -> Style {
        let mut style = Style::default();
        style.set_text_color(self.dropdown.list.fg_color.clone());
        style.set_bg_opa(Opacity::OPA_COVER);
        style.set_bg_color(self.dropdown.list.bg_color.clone());
        style.set_border_side(BorderSide::Full as u8);
        style.set_border_width(self.dropdown.list.border_width);
        style.set_border_color(self.dropdown.list.border_color.clone());
        style.set_radius(self.dropdown.list.corner_radius);
        style.set_pad_bottom(self.dropdown.list.pad);
        style.set_pad_left(self.dropdown.list.pad);
        style.set_pad_top(self.dropdown.list.pad);
        style.set_pad_right(self.dropdown.list.pad);
        style
    }

    /// Gets the style to be used on selected dropdown menu lists, given the
    /// current theme.
    pub fn style_dropdown_list_selected(&self) -> Style {
        let mut style = Style::default();
        style.set_text_color(
            
            self.dropdown.list.selection_fg_color.clone(),
        );
        style.set_bg_opa(Opacity::OPA_COVER);
        style.set_bg_color(
            
            self.dropdown.list.selection_bg_color.clone(),
        );
        style
    }

    /// Gets the style to be used on labels, given the current theme.
    pub fn style_label(&self) -> Style {
        let mut style = Style::default();
        style.set_text_color(self.label.fg_color.clone());
        style
    }

    /// Gets the style to be used on message boxes, given the current theme.
    pub fn style_msgbox(&self) -> Style {
        let mut style = Style::default();
        style.set_text_color(self.msgbox.fg_color.clone());
        style.set_bg_opa(Opacity::OPA_COVER);
        style.set_bg_color(self.msgbox.bg_color.clone());
        style.set_border_side(BorderSide::Full as u8);
        style.set_border_width(self.msgbox.border_width);
        style.set_border_color(self.msgbox.border_color.clone());
        style.set_radius(self.msgbox.corner_radius);
        style.set_pad_bottom(self.msgbox.pad);
        style.set_pad_left(self.msgbox.pad);
        style.set_pad_top(self.msgbox.pad);
        style.set_pad_right(self.msgbox.pad);
        style
    }

    /// Gets the style to be used on message box labels, given the current
    /// theme.
    pub fn style_msgbox_label(&self) -> Style {
        let mut style = Style::default();
        style.set_pad_bottom(self.msgbox.pad);
        style
    }

    // / Gets the style to be used on message box button matrices, given the
    // / current theme.
    //pub fn style_msgbox_btnmatrix(&self) -> Style {
    //    let mut style = Style::default();
    //    style
    //}

    /// Gets the style to be used on message box backgrounds, given the
    /// current theme.
    pub fn style_msgbox_background(&self) -> Style {
        let mut style = Style::default();
        style.set_bg_color(self.msgbox.dimming.color.clone());
        style.set_bg_opa(self.msgbox.dimming.opacity);
        style
    }

    /// Gets the style to be used on bars, given the current theme.
    pub fn style_bar(&self) -> Style {
        let mut style = Style::default();
        style.set_border_side(BorderSide::Full as u8);
        style.set_border_width(self.bar.border_width);
        style.set_border_color(self.bar.border_color.clone());
        style.set_radius(self.bar.corner_radius);
        style
    }

    /// Gets the style to be used on bar indicators, given the current theme.
    pub fn style_bar_indicator(&self) -> Style {
        let mut style = Style::default();
        style.set_bg_opa(Opacity::OPA_COVER);
        style.set_bg_color(self.bar.indicator.bg_color.clone());
        style
    }

    /// postmarketOS dark theme. Based on the palette from unl0kr, itself from
    /// https://coolors.co/009900-395e66-db504a-e3b505-ebf5ee
    pub fn pmos_dark() -> Self {
        Self {
            window: MxWindow {
                bg_color: Color::from_rgb((0x07, 0x0c, 0x0d)),
            },
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
                    border_color: Color::from_rgb((0x2c, 0x48, 0x4e)),
                },
                pressed: MxKeyState {
                    fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                    bg_color: Color::from_rgb((0x00, 0x99, 0x00)),
                    border_color: Color::from_rgb((0x00, 0x99, 0x00)),
                },
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
                    period: 700,
                },
            },
            dropdown: MxDropdown {
                button: MxButton {
                    border_width: 1,
                    corner_radius: 3,
                    pad: 8,
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
                list: MxList {
                    fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                    bg_color: Color::from_rgb((0x16, 0x24, 0x27)),
                    selection_fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                    selection_bg_color: Color::from_rgb((0x00, 0x99, 0x00)),
                    border_width: 1,
                    border_color: Color::from_rgb((0x39, 0x5e, 0x66)),
                    corner_radius: 0,
                    pad: 8,
                },
            },
            label: MxLabel {
                fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
            },
            msgbox: MxMsgbox {
                fg_color: Color::from_rgb((0xf2, 0xf7, 0xf8)),
                bg_color: Color::from_rgb((0x16, 0x24, 0x27)),
                border_width: 1,
                border_color: Color::from_rgb((0x39, 0x5e, 0x66)),
                corner_radius: 3,
                pad: 20,
                gap: 20,
                //buttons: MxMsgButtons {
                //    gap: 10
                //},
                dimming: MxDimming {
                    color: Color::from_rgb((0x07, 0x0c, 0x0d)),
                    opacity: Opacity::OPA_90,
                },
            },
            bar: MxBar {
                border_width: 1,
                border_color: Color::from_rgb((0x00, 0x99, 0x00)),
                corner_radius: 3,
                indicator: MxIndicator {
                    bg_color: Color::from_rgb((0x00, 0x99, 0x00)),
                },
            },
        }
    }
}
