use super::{term_ui, theme::MxTheme};
use cstr_core::CString;
use lvgl::{widgets, Align, Color, Display, Event, LvError, Part, Widget};
use std::{sync::mpsc::channel, time::{Duration, Instant}, thread::sleep};

/// Possible screens to which the UI can navigate. `Exit` represents quitting
/// mxboot.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavLocation {
    Home,
    Terminal,
    Exit,
}

/// A GUI event to be processed in the main event loop.
#[derive(Debug, Clone, Copy)]
pub enum GuiEvent {
    Navigate(NavLocation),
}

/// Loads the actual GUI on the display.
pub fn load_gui(
    mut display: Display,
) -> Result<(), LvError>
{
    let theme = MxTheme::pmos_dark();
    let mut screen = display.get_scr_act()?;
    let mut scr_style = theme.style_window();
    screen.add_style(Part::Main, &mut scr_style)?;

    let mut button = widgets::Btn::create(&mut screen)?;
    button.set_align(Align::TopMid, 0, 0)?;
    button.set_size(200, 100)?;
    let mut btn_style = theme.style_button();
    button.add_style(Part::Main, &mut btn_style)?;

    let mut label = widgets::Label::create(&mut button)?;
    label.set_text(CString::new("Terminal").unwrap().as_c_str())?;
    let mut label_style = theme.style_label();
    label.add_style(Part::Main, &mut label_style)?;

    let (tx, rx) = channel::<GuiEvent>();

    button.on_event(|_, event| {
        if let lvgl::Event::Clicked = event {
            println!("Clicked!");
            match tx.send(GuiEvent::Navigate(NavLocation::Terminal)) {
                Ok(_) => (),
                Err(e) => eprintln!("{e}"),
            }
        }
    })?;

    'running: loop {
        let start = Instant::now();
        lvgl::task_handler();
        while let Ok(event) = rx.try_recv() {
            match event {
                GuiEvent::Navigate(route) => {
                    println!("Navigating to {:?}", route);
                    match route {
                        NavLocation::Terminal => {
                            let mut term_screen = term_ui::term_ui(&theme, &tx)?;
                            display.set_scr_act(&mut term_screen)?;
                        }
                        NavLocation::Home => display.set_scr_act(&mut screen)?,

                        NavLocation::Exit => break 'running,
                    }
                }
            }
        }
        sleep(Duration::from_millis(15));
        lvgl::tick_inc(Instant::now().duration_since(start));
    }
    Ok(())
}
