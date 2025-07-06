use raylib::prelude::*;
use rfd::FileDialog;
use std::{env, process::Command};

#[derive(PartialEq)]
enum ButtonType {
    File,
    Input,
    Script,
}

struct Button<'a> {
    name: &'a mut String,
    label: ButtonType,
    x: i32,
    y: i32,
    file: Option<String>,
}

impl Button<'_> {
    fn new<'a>(
        name: &'a mut String,
        label: ButtonType,
        x: i32,
        y: i32,
        f: Option<String>,
    ) -> Button<'a> {
        Button {
            name,
            label,
            x,
            y,
            file: f,
        }
    }

    fn update(&mut self, scr_w: i32, scr_y: i32) {
        self.x += scr_w;
        self.y += scr_y;
    }
}

fn render_button(d: &mut RaylibDrawHandle, button: &mut Button) {
    let button_width: i32;
    let button_height: i32;
    if button.label == ButtonType::Input && button.file.is_some() {
        button_width = button.file.as_slice().len() as i32;
        button_height = 30;
    } else {
        button_width = (button.name.len() * 22) as i32;
        button_height = 50;
    }

    d.draw_rectangle(button.x, button.y, button_width, button_height, Color::GRAY);
    if button.label == ButtonType::Input && button.file.is_some() {
        d.draw_text(
            button.file.clone().unwrap_or("Empty".to_string()).as_str(),
            button.x + 5,
            button.y + 5,
            10,
            Color::WHITE,
        );
    } else {
        d.draw_text(button.name, button.x + 5, button.y + 5, 40, Color::WHITE);
    }

    if button.label == ButtonType::Input {
        return;
    }

    // Check for mouse click
    if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        let mouse_pos = d.get_mouse_position();
        if mouse_pos.x >= button.x as f32
            && mouse_pos.x <= (button.x + button_width) as f32
            && mouse_pos.y >= button.y as f32
            && mouse_pos.y <= (button.y + button_height) as f32
        {
            match button.label {
                ButtonType::File => {
                    let file: Option<std::path::PathBuf> = FileDialog::new()
                        .set_directory("Videos")
                        .add_filter("Video files", &["mp4", "mov", "mkv"])
                        .pick_file();

                    button.file = file.and_then(|path| path.to_str().map(|s| s.to_string()));
                }
                ButtonType::Input => {}
                ButtonType::Script => {
                    if button.file.is_none() {
                        return;
                    }

                    //LLM generated code
                    let file_path = button.file.as_ref().unwrap();

                    let script_path = std::env::var("CARGO_MANIFEST_DIR").unwrap().to_string()
                        + "/src/convert.sh";

                    println!("--------------");
                    println!("{}", file_path);

                    //Partially LLM generated code
                    println!(
                        "{:#?}",
                        Command::new("sh")
                            .arg(&script_path)
                            .arg(file_path)
                            .output()
                            .expect("Failed to convert File!")
                    );

                    env::set_current_dir("../").unwrap();

                    //End of LLM generated code
                }
            }
        }
    }
}

fn main() {
    let mut width: i32 = 800;
    let mut widthd = width;
    let mut height: i32 = 600;
    let mut heightd = height;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .resizable()
        .title("Resolve Convert")
        .build();

    rl.set_window_min_size(width, height);

    let mut f_binding = "Open File".to_string();
    let mut file_button = Button::new(&mut f_binding, ButtonType::File, 30, 30, None);
    let mut t_binding = "     ".to_string();
    let mut text_button = Button::new(
        &mut t_binding,
        ButtonType::Input,
        width / 2 - 60,
        height / 2,
        None,
    );

    let mut s_binding = "Convert".to_string();
    let mut script_button = Button::new(
        &mut s_binding,
        ButtonType::Script,
        width / 2 - 60,
        height / 4 * 3,
        None,
    );

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            width = rl.get_render_width();
            height = rl.get_render_height();

            Button::update(&mut file_button, width - widthd, height - heightd);
            widthd = width;
            heightd = height;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        render_button(&mut d, &mut file_button);

        if file_button.file.is_some() {
            text_button.file = file_button.file.clone();
            script_button.file = file_button.file.clone();
        }

        render_button(&mut d, &mut text_button);
        render_button(&mut d, &mut script_button);
    }
}
