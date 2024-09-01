use kana_jp;
use std::rc::Rc;

use winsafe::{co, gui, prelude::*};

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
}

impl MainWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new(gui::WindowMainOpts {
            title: "Japanese Kana Converter Ex!".to_owned(),
            size: (600, 500),
            ..Default::default()
        });

        let input = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                position: (20, 50),
                ..Default::default()
            },
        );

        let output = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                position: (20, 100),
                edit_style: co::ES::READONLY,
                ..Default::default()
            },
        );

        let btn_to_hiragana = gui::Button::new(
            &wnd,
            gui::ButtonOpts {
                text: "To Hiragana".to_owned(),
                position: (20, 20),
                ..Default::default()
            },
        );

        let btn_to_katakana = gui::Button::new(
            &wnd,
            gui::ButtonOpts {
                text: "To Katakana".to_owned(),
                position: (120, 20),
                ..Default::default()
            },
        );

        let btn_to_romaji = gui::Button::new(
            &wnd,
            gui::ButtonOpts {
                text: "To Romaji".to_owned(),
                position: (220, 20),
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
        };
        new_self.events();
        new_self
    }

    fn events(&self) {
        let (input, output) = (Rc::clone(&self.input), Rc::clone(&self.output));
        self.btn_to_hiragana.on().bn_clicked(move || {
            output.set_text(kana_jp::to_hiragana(input.text().as_str()).as_str());
            Ok(())
        });

        let (input, output) = (Rc::clone(&self.input), Rc::clone(&self.output));
        self.btn_to_katakana.on().bn_clicked(move || {
            output.set_text(kana_jp::to_katakana(input.text().as_str()).as_str());
            Ok(())
        });

        let (input, output) = (Rc::clone(&self.input), Rc::clone(&self.output));
        self.btn_to_romaji.on().bn_clicked(move || {
            output.set_text(kana_jp::to_romaji(input.text().as_str()).as_str());
            Ok(())
        });
    }
}
