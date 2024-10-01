use std::{cell::RefCell, rc::Rc};
mod ui;

use fltk::{
    button::Button,
    draw,
    enums::{Color,FrameType},
    frame::Frame,
    group::{self, Flex, FlexType, Pack, PackType, Group},
    prelude::*,
    window::Window,
    *,
};
use rand::{prelude::*, seq::index};

fn draw_table(widget: &mut Group, array: &[i32], widget_position_x: i32, widget_position_y: i32, ceil_width: i32, ceil_height: i32) {
    widget.clear(); // Очищаем группу перед добавлением новых ячеек

    for (index, &num) in array.iter().enumerate() {
        let index_i32: i32 = index as i32;
        let position_ceil_x = (index_i32 / 10) * ceil_width;
        let position_ceil_y = (index_i32 % 10) * ceil_height;

        let mut _frame = Frame::new(
            position_ceil_x + widget_position_x + (ceil_width / 3) * 2,
            position_ceil_y + widget_position_y,
            ceil_width / 3,
            ceil_height,
            None,
        );

        let mut frame = Frame::new(
            position_ceil_x + widget_position_x,
            position_ceil_y + widget_position_y,
            (ceil_width / 3) * 2,
            ceil_height,
            &*num.to_string(),
        );

        // _frame.set_color(Color::White);
        // _frame.set_frame(FrameType::BorderBox);

        frame.set_color(Color::from_rgb(236, 236, 236));
        frame.set_frame(FrameType::BorderBox);

        widget.add(&_frame);
        widget.add(&frame);
    }

    widget.redraw(); // Перерисовываем таблицу
}

fn main() {
    let app = app::App::default();
    let ui_window = ui::UIwindow::new(
        100, 
        100,
        600,
        400,
        "Random Array Generator",
        ui::UIgeneral::hex_to_rgb("#FFFFFF").unwrap(),
        true,
        24,
        20,
    );

    let size = ui_window.get_grid_sizes();
    let column = size.0;
    let row = size.1;

    let ui_header = ui::UIheader::new(
        0,
        0,
        column * 24,
        row * 2,
        "Тестовое приложение",
        ui::UIgeneral::hex_to_rgb("#d1d9e0").unwrap()
    );

    ui_header.paint();
    ui_header.draw();
    ui_window.paint();
    ui_window.draw();

    // let table = UI::Table::new(
    //     position_x + column_size,
    //     position_y + row_size * 6,
    //     window_width - column_size * 2,
    //     window_height - row_size * 10,
    // );

    // let mut button_new = Button::new(
    //     position_x + column_size * 1,
    //     position_y + row_size * 3,
    //     column_size * 2,
    //     row_size * 2,
    //     "New",
    // );

    // let mut table_clone = table.clone();
    // button_new.set_callback(move |_| {
    //     let empty_vector: Vec<i32> = Vec::new();
    //     // Вызываем функцию для построения таблицы, используя borrow_mut для доступа к Flex
    //     draw_table(
    //         &mut table_clone, 
    //         &empty_vector, 
    //         position_x + column_size,  
    //         position_y + row_size * 6,
    //         column_size * 3, 
    //         row_size * 1
    //     );
    // });

    // let mut button_fill = Button::new(
    //     position_x + column_size * 4,
    //     position_y + row_size * 3,
    //     column_size * 2,
    //     row_size * 2,
    //     "Fill",
    // );

    // let mut table_clone = table.clone();
    // button_fill.set_callback(move |_| {
    //     let mut rng = rand::thread_rng();
    //     // Ограничение 70
    //     let random_numbers: Vec<i32> = (0..70).map(|_| rng.gen_range(0..100)).collect();
    //     // Вызываем функцию для построения таблицы, используя borrow_mut для доступа к Flex
    //     draw_table(
    //         &mut table_clone, 
    //         &random_numbers, 
    //         position_x + column_size,  
    //         position_y + row_size * 6,
    //         column_size * 3, 
    //         row_size * 1
    //     );
    // });

    // let mut button_ins = Button::new(
    //     position_x + column_size * 7,
    //     position_y + row_size * 3,
    //     column_size * 2,
    //     row_size * 2,
    //     "Ins",
    // );

    // let mut button_find = Button::new(
    //     position_x + column_size * 10,
    //     position_y + row_size * 3,
    //     column_size * 2,
    //     row_size * 2,
    //     "Find",
    // );

    // window.add(&button_new);
    // window.add(&button_fill);
    // window.add(&button_ins);
    // window.add(&button_find);

    // let mut button_test = Button::new(
    //     position_x + column_size * 1,
    //     position_y + row_size * 17,
    //     column_size * 2,
    //     row_size * 2,
    //     "Test",
    // );

    // window.add(&button_test);

    // window.set_color(Color::White);
    // window.end();
    // window.show();
    app.run().unwrap();
}
