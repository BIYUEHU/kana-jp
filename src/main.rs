use kana_jp;
use std::process::Command;
use std::rc::Rc;

use winsafe::{
    co,
    gui::{self, CheckState},
    prelude::*,
};

fn main() {
    let main_window = MainWindow::new(); // instantiate our main window
    if let Err(e) = main_window.wnd.run_main(None) {
        eprintln!("{}", e);
    }
}

#[derive(Clone)]
pub struct MainWindow {
    wnd: Rc<gui::WindowMain>,
    input: Rc<gui::Edit>,
    output: Rc<gui::Edit>,
    btn_to_hiragana: gui::Button,
    btn_to_katakana: gui::Button,
    btn_to_romaji: gui::Button,
    btn_get_kanji: gui::Button,
    checkbox_handle_kanji: gui::CheckBox,
}

static PROJECT_LINK: &str = "https://github.com/biyuehu/kana-jp";

impl MainWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new(gui::WindowMainOpts {
            title: "Japanese Kana Converter Ex!".to_owned(),
            size: (600, 500),
            style: co::WS::CAPTION
                | co::WS::SYSMENU
                | co::WS::CLIPCHILDREN
                | co::WS::BORDER
                | co::WS::VISIBLE
                | co::WS::MINIMIZEBOX,
            ..Default::default()
        });

        let _ = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Input".to_owned(),
                position: (32, 22),
                size: (82, 27),
                ..Default::default()
            },
        );

        let _ = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Output".to_owned(),
                position: (32, 207),
                size: (96, 27),
                ..Default::default()
            },
        );

        let link = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: format!("Project link: {}", PROJECT_LINK),
                position: (40, 455),
                size: (400, 16),
                ..Default::default()
            },
        );
        link.on().stn_clicked(|| {
            if let Err(e) = Command::new("cmd /c start https://github.com/biyuehu/kana-jp").spawn()
            {
                eprintln!("Failed to open browser: {}", e)
            }

            Ok(())
        });

        let input = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                position: (40, 63),
                width: 504,
                height: 128,
                ..Default::default()
            },
        );

        let output = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                position: (40, 247),
                width: 504,
                height: 128,
                edit_style: co::ES::READONLY | co::ES::AUTOHSCROLL | co::ES::NOHIDESEL,
                ..Default::default()
            },
        );

        let btn_to_hiragana = gui::Button::new(
            &wnd,
            gui::ButtonOpts {
                text: "To Hiragana".to_owned(),
                position: (40, 400),
                width: 97,
                height: 40,
                ..Default::default()
            },
        );

        let btn_to_katakana = gui::Button::new(
            &wnd,
            gui::ButtonOpts {
                text: "To Katakana".to_owned(),
                position: (144, 400),
                width: 97,
                height: 40,
                ..Default::default()
            },
        );

        let btn_to_romaji = gui::Button::new(
            &wnd,
            gui::ButtonOpts {
                text: "To Romaji".to_owned(),
                position: (248, 400),
                width: 97,
                height: 40,
                ..Default::default()
            },
        );

        let btn_get_kanji = gui::Button::new(
            &wnd,
            gui::ButtonOpts {
                text: "Get kanji".to_owned(),
                position: (352, 400),
                width: 97,
                height: 40,
                ..Default::default()
            },
        );

        let checkbox_handle_kanji = gui::CheckBox::new(
            &wnd,
            gui::CheckBoxOpts {
                text: "Handle kanji".to_owned(),
                position: (456, 421),
                size: (122, 20),
                ..Default::default()
            },
        );

        let new_self = Self {
            wnd: Rc::new(wnd),
            input: Rc::new(input),
            output: Rc::new(output),
            btn_to_hiragana,
            btn_to_katakana,
            btn_to_romaji,
            btn_get_kanji,
            checkbox_handle_kanji,
        };
        new_self.events();
        new_self
    }

    fn events(&self) {
        let get_elements = || {
            (
                self.input.clone(),
                self.output.clone(),
                self.checkbox_handle_kanji.clone(),
            )
        };
        let (input, output, checkbox_handle_kanji) = get_elements();
        self.btn_to_hiragana.on().bn_clicked(move || {
            output.set_text(
                kana_jp::to_hiragana(
                    input.text().as_str(),
                    checkbox_handle_kanji.check_state() == CheckState::Checked,
                )
                .as_str(),
            );
            Ok(())
        });

        let (input, output, checkbox_handle_kanji) = get_elements();
        self.btn_to_katakana.on().bn_clicked(move || {
            output.set_text(
                kana_jp::to_katakana(
                    input.text().as_str(),
                    checkbox_handle_kanji.check_state() == CheckState::Checked,
                )
                .as_str(),
            );
            Ok(())
        });

        let (input, output, checkbox_handle_kanji) = get_elements();
        self.btn_to_romaji.on().bn_clicked(move || {
            output.set_text(
                kana_jp::to_romaji(
                    input.text().as_str(),
                    checkbox_handle_kanji.check_state() == CheckState::Checked,
                )
                .as_str(),
            );
            Ok(())
        });

        let (input, output, _) = get_elements();

        self.btn_get_kanji.on().bn_clicked(move || {
            if let Some(kanji) = input.text().chars().next() {
                if let Some(result) = kana_jp::get_kanji(kanji) {
                    output.set_text(
                        vec![
                            ("Kun'yomi", result.0),
                            ("On'yomi", result.1),
                            ("Nanori", result.2),
                        ]
                        .iter()
                        .map(|(text, arr)| {
                            if arr.len() == 0 {
                                return "".to_string();
                            }
                            format!("{}: {} ", text, arr.join(", "))
                        })
                        .collect::<String>()
                        .as_str(),
                    )
                }
            }
            Ok(())
        })
    }
}
