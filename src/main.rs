use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Test monitor".to_owned(),
        // fullscreen: true,
        ..Default::default()
    }
}

#[derive(Debug, Default)]
struct App {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    speed: f32,
    thickness: f32,
    text_params: TextParams,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App {
        width: 1920.0,
        height: 1080.0,
        x: 0.0,
        y: 0.0,
        speed: 2.0,
        thickness: 1.0,
        text_params: TextParams {
            color: GRAY,
            ..Default::default()
        },
    };

    loop {
        app.width = screen_width();
        app.height = screen_height();

        app.x = app.x % app.width + app.speed;
        app.y = app.y % app.height + app.speed;

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        match mouse_wheel() {
            (_, y) if y != 0.0 => {
                if is_key_down(KeyCode::LeftShift) {
                    app.thickness *= 1.1f32.powf(y);
                } else {
                    app.speed *= 1.1f32.powf(y);
                }
            }
            _ => (),
        };

        clear_background(BLACK);

        draw_line(0.0, app.y, app.width, app.y, app.thickness, WHITE);
        draw_line(app.x, 0.0, app.x, app.height, app.thickness, WHITE);

        draw_multiline_text(
            &format!(
                "Screen width:                    {:?}\n\
                    Screen height:                   {:?}\n\
                    X:                               {:?}\n\
                    Y:                               {:?}\n\
                    Speed (mouse wheel):             {:?}\n\
                    Thickness (shift + mouse wheel): {:?}",
                app.width, app.height, app.x, app.y, app.speed, app.thickness,
            ),
            10.0,
            0.0,
            5.0,
            app.text_params,
        );
        next_frame().await;
    }
}

fn draw_multiline_text(text: &str, x: f32, y: f32, line_spacing: f32, params: TextParams) {
    let mut current_y = y;
    for line in text.lines() {
        let (_, h) = measure_text(line, Some(params.font), params.font_size, params.font_scale);
        draw_text_ex(line, x, current_y, params);
        current_y += h + line_spacing;
    }
}
