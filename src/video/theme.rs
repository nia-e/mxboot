// Themes from https://gitlab.com/cherrypicker/unl0kr/-/blob/master/themes.c
// (c) 2021 Johannes Marbach for the original theme

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
    pub bg_color: u32,
}

pub struct MxHeader {
    pub bg_color: u32,
    pub border_width: u16,
    pub border_color: u32,
    pub pad: u16,
    pub gap: u16,
}

pub struct MxKeyboard {
    pub bg_color: u32,
    pub border_width: u16,
    pub border_color: u32,
    pub pad: u16,
    pub gap: u16,
    pub keys: MxKeys,
}

pub struct MxKeys {
    pub border_width: u16,
    pub corner_radius: u16,
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
    pub fg_color: u32,
    pub bg_color: u32,
    pub border_color: u32,
}

pub struct MxButton {
    pub border_width: u16,
    pub corner_radius: u16,
    pub pad: u16,
    pub normal: MxKeyState,
    pub pressed: MxKeyState,
}

pub struct MxTextArea {
    pub fg_color: u32,
    pub bg_color: u32,
    pub border_width: u16,
    pub border_color: u32,
    pub corner_radius: u16,
    pub pad: u16,
    pub placeholder_color: u32,
    pub cursor: MxCursor,
}

pub struct MxCursor {
    pub width: u16,
    pub color: u32,
    pub period: u16,
}

pub struct MxDropdown {
    pub button: MxButton,
    pub list: MxList,
}

pub struct MxList {
    pub fg_color: u32,
    pub bg_color: u32,
    pub selection_fg_color: u32,
    pub selection_bg_color: u32,
    pub border_width: u16,
    pub border_color: u32,
    pub corner_radius: u16,
    pub pad: u16,
}

pub struct MxLabel {
    pub fg_color: u32,
}

pub struct MxMsgbox {
    pub fg_color: u32,
    pub bg_color: u32,
    pub border_width: u16,
    pub border_color: u32,
    pub corner_radius: u16,
    pub pad: u16,
    pub gap: u16,
    pub buttons: MxMsgButtons,
    pub dimming: MxDimming,
}

pub struct MxMsgButtons {
    pub gap: u16,
}

pub struct MxDimming {
    pub color: u32,
    pub opacity: u16,
}

pub struct MxBar {
    pub border_width: u16,
    pub border_color: u32,
    pub corner_radius: u16,
    pub indicator: MxIndicator,
}

pub struct MxIndicator {
    pub bg_color: u32,
}

impl MxTheme {
    pub fn pmos_dark() -> Self {
        Self {
            window: MxWindow { bg_color: 0x070c0d },
            header: MxHeader {
                bg_color: 0x070c0d,
                border_width: 0,
                border_color: 0x070c0d,
                pad: 20,
                gap: 10,
            },
            keyboard: MxKeyboard {
                bg_color: 0x162427,
                border_width: 2,
                border_color: 0x395e66,
                pad: 20,
                gap: 10,
                keys: MxKeys {
                    border_width: 1,
                    corner_radius: 3,
                    key_char: MxKeyType {
                        normal: MxKeyState {
                            fg_color: 0xf2f7f8,
                            bg_color: 0x162427,
                            border_color: 0x395e66,
                        },
                        pressed: MxKeyState {
                            fg_color: 0xf2f7f8,
                            bg_color: 0x009900,
                            border_color: 0x009900,
                        },
                    },
                    key_non_char: MxKeyType {
                        normal: MxKeyState {
                            fg_color: 0xf2f7f8,
                            bg_color: 0x253c41,
                            border_color: 0x2c484e,
                        },
                        pressed: MxKeyState {
                            fg_color: 0xf2f7f8,
                            bg_color: 0x009900,
                            border_color: 0x009900,
                        },
                    },
                    key_mod_act: MxKeyType {
                        normal: MxKeyState {
                            fg_color: 0x009900,
                            bg_color: 0x253c41,
                            border_color: 0x009900,
                        },
                        pressed: MxKeyState {
                            fg_color: 0xf2f7f8,
                            bg_color: 0x009900,
                            border_color: 0x009900,
                        },
                    },
                    key_mod_inact: MxKeyType {
                        normal: MxKeyState {
                            fg_color: 0xf2f7f8,
                            bg_color: 0x253c41,
                            border_color: 0x2c484e,
                        },
                        pressed: MxKeyState {
                            fg_color: 0xf2f7f8,
                            bg_color: 0x009900,
                            border_color: 0x009900,
                        },
                    },
                },
            },
            button: MxButton {
                border_width: 1,
                corner_radius: 3,
                pad: 8,
                normal: MxKeyState {
                    fg_color: 0xf2f7f8,
                    bg_color: 0x253c41,
                    border_color: 0x2c484e
                },
                pressed: MxKeyState {
                    fg_color: 0xf2f7f8,
                    bg_color: 0x009900,
                    border_color: 0x009900
                }
            },
            text_area: MxTextArea {
                fg_color: 0xf2f7f8,
                bg_color: 0x002900,
                border_width: 1,
                border_color: 0x009900,
                corner_radius: 3,
                pad: 8,
                placeholder_color: 0x009900,
                cursor: MxCursor {
                    width: 2,
                    color: 0x009900,
                    period: 700
                }
            },
            dropdown: MxDropdown {
                button: MxButton {
                    border_width: 1,
                    corner_radius: 3,
                    pad: 8,
                    normal: MxKeyState {
                        fg_color: 0xf2f7f8,
                        bg_color: 0x253c41,
                        border_color: 0x2c484e
                    },
                    pressed: MxKeyState {
                        fg_color: 0xf2f7f8,
                        bg_color: 0x009900,
                        border_color: 0x009900
                    }
                },
                list: MxList {
                    fg_color: 0xf2f7f8,
                    bg_color: 0x162427,
                    selection_fg_color: 0xf2f7f8,
                    selection_bg_color: 0x009900,
                    border_width: 1,
                    border_color: 0x395e66,
                    corner_radius: 0,
                    pad: 8
                }
            },
            label: MxLabel {
                fg_color: 0xf2f7f8
            },
            msgbox: MxMsgbox {
                fg_color: 0xf2f7f8,
                bg_color: 0x162427,
                border_width: 1,
                border_color: 0x395e66,
                corner_radius: 3,
                pad: 20,
                gap: 20,
                buttons: MxMsgButtons {
                    gap: 10
                },
                dimming: MxDimming {
                    color: 0x070c0d,
                    opacity: 225
                }
            },
            bar: MxBar {
                border_width: 1,
                border_color: 0x009900,
                corner_radius: 3,
                indicator: MxIndicator {
                    bg_color: 0x009900
                }
            }
        }
    }
}
