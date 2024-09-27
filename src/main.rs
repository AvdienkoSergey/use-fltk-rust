use std::{cell::RefCell, rc::Rc};

use fltk::{
    button::Button,
    draw,
    enums::Color,
    frame::Frame,
    group::{self, Flex, FlexType, Pack, PackType},
    prelude::*,
    window::Window,
    *,
};
use rand::prelude::*;

const POSITION_X: i32 = 200;
const POSITION_Y: i32 = 200;
const WINDOW_WIDTH: i32 = 600;
const WINDOW_HEIGHT: i32 = 400;
const SPACE: i32 = 5;
const SPACE_BIG: i32 = 5 * 15;

#[derive(Clone)]
struct DataCell {
    value: i32,
}

fn draw_grid(
    widget: &mut window::DoubleWindow,
    columns_count: i32,
    column_size: i32,
    rows_count: i32,
    row_size: i32,
    window_height: i32,
    window_width: i32,
    color: Color,
) {
    widget.draw(move |_| {
        draw::set_draw_color(color);

        // Рисуем вертикальные линии (сетку)
        for col in 0..columns_count {
            let x = col * column_size;
            draw::draw_line(x, 0, x, window_height);
        }

        // Рисуем горизонтальные линии (сетку)
        for row in 0..rows_count {
            let y = row * row_size;
            draw::draw_line(0, y, window_width, y);
        }
    });
}

fn draw_header(w: i32, h: i32, title: &str, bg: Color) {
    let x = 0;
    let y = 0;
    let mut header = group::Flex::new(x, y, w, h, None);
    header.set_type(FlexType::Row);
    // background
    let mut header_background = frame::Frame::new(x, y, w, h, title);
    header_background.set_color(bg);
    header_background.set_frame(enums::FrameType::FlatBox);
    // render
    header.add(&header_background);
    header.end();
}

fn draw_table(widget: &mut Flex, array: &[i32]) {
    // Создаем виджеты для каждого элемента массива
    for &num in array {
        let frame = Frame::new(0, 0, 50, 30, &*num.to_string());
        widget.add(&frame);
    }
    widget.redraw(); // Перерисовываем таблицу
}

fn main() {
    let app = app::App::default();

    let window_width = 600;
    let window_height = 400;
    let columns_count = 24;
    let rows_count = 20;
    let column_size = window_width / columns_count; // 25
    let row_size = window_height / rows_count; // 20
    let mut position_x = 0;
    let mut position_y = 0;

    let mut window = Window::new(
        100,
        100,
        window_width,
        window_height,
        "Random Array Generator",
    );

    // draw_grid(
    //     &mut window,
    //     columns_count,
    //     column_size,
    //     rows_count,
    //     row_size,
    //     window_height,
    //     window_width,
    //     Color::from_rgb(236, 236, 236),
    // );

    draw_header(
        column_size * 24,
        row_size * 2,
        "Тестовое приложение",
        Color::from_rgb(236, 236, 236),
    );

    let mut table_layout = Flex::new(
        position_x + column_size, 
        position_y + row_size * 6, 
        window_width - column_size * 2,
        window_height - row_size * 10, 
        None
    );
    table_layout.set_type(FlexType::Row);
    table_layout.set_frame(enums::FrameType::BorderBox);

    let mut button_new = Button::new(
        position_x + column_size * 1,
        position_y + row_size * 3,
        column_size * 2,
        row_size * 2,
        "New",
    );

    let mut table_layout_clone = table_layout.clone();
    button_new.set_callback(move |_| {
        let empty_vector: Vec<i32> = Vec::new();
        // Вызываем функцию для построения таблицы, используя borrow_mut для доступа к Flex
        table_layout_clone.clear();
        draw_table(&mut table_layout_clone, &empty_vector);
    });

    let mut button_fill = Button::new(
        position_x + column_size * 4,
        position_y + row_size * 3,
        column_size * 2,
        row_size * 2,
        "Fill",
    );

    let mut table_layout_clone = table_layout.clone();
    button_fill.set_callback(move |_| {
        let mut rng = rand::thread_rng();
        let random_numbers: Vec<i32> = (0..10).map(|_| rng.gen_range(0..100)).collect();
        // Вызываем функцию для построения таблицы, используя borrow_mut для доступа к Flex
        draw_table(&mut table_layout_clone, &random_numbers);
    });

    let mut button_3 = Button::new(
        position_x + column_size * 7,
        position_y + row_size * 3,
        column_size * 2,
        row_size * 2,
        "3",
    );

    let mut button_4 = Button::new(
        position_x + column_size * 10,
        position_y + row_size * 3,
        column_size * 2,
        row_size * 2,
        "4",
    );

    window.add(&button_new);
    window.add(&button_fill);
    window.add(&button_3);
    window.add(&button_4);

    let mut button_test = Button::new(
        position_x + column_size * 1,
        position_y + row_size * 17,
        column_size * 2,
        row_size * 2,
        "Test",
    );

    window.add(&button_test);

    window.set_color(Color::White);
    window.end();
    window.show();

    app.run().unwrap();
}

// fn the_header(w: i32, h: i32, color: Color, title: &str) {
//     // Создаём контейнер Flex
//     let mut container = group::Flex::new(0, 0, w, h, "").column();
//     // Вставляем Frame в Flex для фона
//     let mut background = frame::Frame::new(0, 0, w, h, title);
//     background.set_color(color); // Устанавливаем цвет фона
//     background.set_frame(enums::FrameType::FlatBox); // Убираем границы
//     container.end();
//     // Добавляем Frame в Flex
//     container.add(&background);
// }

// fn radio_btn_group() -> Pack {
//     let mut vert_pack = Pack::new(0, 0, 60, 40, "");
//     vert_pack.set_type(PackType::Vertical);
//     let spacer_top = Frame::new(0, 0, 0, 5, ""); // Отступ сверху
//     let radio1 = button::RadioRoundButton::new(0, 0, 0, 10, "Dups OK");
//     let spacer_center = Frame::new(0, 0, 0, 10, ""); // Отступ сверху
//     let radio2 = button::RadioRoundButton::new(0, 0, 0, 10, "No dups");
//     let spacer_bottom = Frame::new(0, 0, 0, 5, ""); // Отступ сверху
//     vert_pack.add(&spacer_top);
//     vert_pack.add(&radio1);
//     vert_pack.add(&spacer_center);
//     vert_pack.add(&radio2);
//     vert_pack.add(&spacer_bottom);
//     vert_pack.end();
//     vert_pack
// }

// fn the_input(w: i32, h: i32, padding: i32, title: &str) -> Pack {
//     // Создаем контейнер
//     let mut vert_pack = Pack::new(600, 0, w, h + padding * 2, None);
//     // Определяем направление размещения дочерних элементов
//     vert_pack.set_type(PackType::Vertical);
//     // Отступ сверху
//     let spacer_top = Frame::new(0, 0, w, padding, None);
//     // Инпут
//     let mut input = input::Input::new(0, 0, 30, 30, title);
//     input.set_callback(|inp| {
//         // Получаем значение из поля ввода
//         if let Ok(value) = inp.value().parse::<usize>() {
//             println!("{value}")
//         }
//     });
//     // Отступ снизу
//     let spacer_bottom = Frame::new(0, 0, w, padding, None);
//     // Наполняем контейнер
//     vert_pack.add(&spacer_top);
//     vert_pack.add(&input);
//     vert_pack.add(&spacer_bottom);
//     // Завершаем контейнер
//     vert_pack.end();
//     // Возвращаем контейнер
//     vert_pack
// }

// fn main() {
//     let array = Rc::new(RefCell::new(vec![DataCell { value: 0 }]));
//     let length: usize = 10;
//     // Window
//     let a = app::App::default();
//     let title_app = "Hello from Rust!";
//     let title_header = "Array Workshop";
//     let mut win = Window::new(
//         POSITION_X,
//         POSITION_Y,
//         WINDOW_WIDTH,
//         WINDOW_HEIGHT,
//         title_app,
//     );
//     // Table
//     let t_position_x = 10;
//     let t_position_y = 90;
//     let table = Rc::new(RefCell::new(Flex::new(
//         t_position_x,
//         t_position_y,
//         WINDOW_WIDTH - 20,
//         (WINDOW_WIDTH / 2) - 50,
//         "",
//     )));
//     // Header
//     the_header(WINDOW_WIDTH, 30, Color::Light1, title_header);
//     // Control panel
//     let cp_position_x = 0;
//     let cp_position_y = 40;
//     let cp_height: i32 = 40;
//     let cp_btn_width: i32 = 60;
//     let mut control_panel = Pack::new(cp_position_x, cp_position_y, WINDOW_WIDTH, cp_height, None);
//     control_panel.set_type(PackType::Horizontal);
//     control_panel.set_spacing(SPACE);
//     // Control panel padding
//     let control_panel_padding_left = Frame::new(0, 0, SPACE, cp_height, None);
//     let control_panel_padding_right = Frame::new(0, 0, SPACE_BIG, cp_height, None);
//     // Control panel buttons
//     // Control panel button NEW
//     let mut button_new = Button::new(0, 0, cp_btn_width, cp_height - SPACE * 2, "New");
//     button_new.set_callback({
//         let array_clone = Rc::clone(&array);
//         move |button_new| {
//             button_new.set_color(Color::Yellow);
//             // Создаем генератор случайных чисел
//             let mut rng = rand::thread_rng();
//             // Генерируем случайное число в диапазоне от 1 до 100 включительно
//             // let random_number = rng.gen_range(1..=100);
//             let mut array_ref = array_clone.borrow_mut();
//             array_ref.clear();
//             array_ref.resize(
//                 length,
//                 DataCell {
//                     value: rng.gen_range(1..=100),
//                 },
//             );
//         }
//     });
//     // Control panel button FILL
//     let mut button_fill = Button::new(0, 0, cp_btn_width, cp_height - SPACE * 2, "Fill");
//     button_fill.set_callback({
//         let array_clone = Rc::clone(&array);
//         let table_clone = Rc::clone(&table);
//         move |button_fill| {
//             button_fill.set_color(Color::Green);
//             build_table(&mut table_clone.borrow_mut(), &array_clone.borrow());
//         }
//     });
//     // Control panel button INS
//     let mut button_ins = Button::new(0, 0, cp_btn_width, cp_height - SPACE * 2, "Ins");
//     button_ins.set_callback(move |button_ins| {
//         button_ins.set_color(Color::Cyan);
//     });
//     // Control panel button FIND
//     let mut button_find = Button::new(0, 0, cp_btn_width, cp_height - SPACE * 2, "Find");
//     button_find.set_callback(move |button_find| {
//         button_find.set_color(Color::Magenta);
//     });
//     // Control panel button DEL
//     let mut button_del = Button::new(0, 0, cp_btn_width, cp_height - SPACE * 2, "Del");
//     button_del.set_callback(move |button_del| {
//         button_del.set_color(Color::Selection);
//     });
//     // Control panel render
//     control_panel.add(&control_panel_padding_left);
//     control_panel.add(&build_button(&button_new));
//     control_panel.add(&build_button(&button_fill));
//     control_panel.add(&build_button(&button_ins));
//     control_panel.add(&build_button(&button_find));
//     control_panel.add(&build_button(&button_del));
//     control_panel.add(&radio_btn_group());
//     control_panel.add(&control_panel_padding_right);
//     control_panel.add(&the_input(100, 30, 5, "Number: "));
//     control_panel.end();
//     // Button build fn
//     fn build_button(button: &Button) -> Pack {
//         let btn_pack_width = 60;
//         let bth_pack_height = 40;
//         let mut button_pack = Pack::new(0, 0, btn_pack_width, bth_pack_height - SPACE * 2, None);
//         let button_pack_padding_top = Frame::new(0, 0, btn_pack_width, SPACE, None);
//         let button_pack_padding_bottom = Frame::new(0, 0, btn_pack_width, SPACE, None);
//         button_pack.set_type(PackType::Vertical);
//         button_pack.add(&button_pack_padding_top);
//         button_pack.add(button);
//         button_pack.add(&button_pack_padding_bottom);
//         button_pack.end();
//         button_pack
//     }
//     // Table build fn
//     fn build_table(table: &mut Flex, array: &[DataCell]) {
//         let ceil_width = 80;
//         let ceil_height = 25;
//         let arr_length = array.len() as i32;
//         if arr_length > 1 {
//             let mut table_column =
//                 group::Pack::new(0, 0, ceil_width, arr_length * ceil_height, None);
//             table_column.set_type(group::PackType::Vertical);
//             for (index, value) in array.iter().enumerate() {
//                 let label_index = format!("{}", index);
//                 let label_value: String = format!("{}", value.value);
//                 let mut ceil = group::Pack::new(0, 0, ceil_width, ceil_height, None);
//                 ceil.set_type(group::PackType::Horizontal);

//                 // Индекс контейнер
//                 let mut index_container = frame::Frame::new(
//                     0,
//                     0,
//                     ceil_width / 4,
//                     ceil_height,
//                     Some(label_index.as_str()),
//                 );
//                 index_container.set_color(enums::Color::White);
//                 index_container.set_frame(enums::FrameType::BorderBox);

//                 // Значение контейнер
//                 let mut value_container = frame::Frame::new(
//                     0,
//                     0,
//                     ceil_width / 2,
//                     ceil_height,
//                     Some(label_value.as_str()),
//                 );
//                 value_container.set_color(enums::Color::White);
//                 value_container.set_frame(enums::FrameType::BorderBox);

//                 // Цветной контейнер
//                 let mut color_container =
//                     frame::Frame::new(0, 0, ceil_width / 4, ceil_height, None);
//                 color_container.set_color(enums::Color::Red);
//                 color_container.set_frame(enums::FrameType::BorderBox);

//                 // Добавляем элементы в горизонтальный контейнер
//                 ceil.add(&index_container);
//                 ceil.add(&value_container);
//                 ceil.add(&color_container);

//                 // Завершаем горизонтальный контейнер и добавляем его в вертикальный
//                 ceil.end();
//                 table_column.add(&ceil);
//             }
//             table.add(&table_column);
//             table.end();
//         } else {
//             table.end();
//         }
//     }
//     table.borrow().end();
//     win.set_color(Color::White);
//     win.end();
//     win.show();

//     a.run().unwrap();
// }

// use std::{cell::RefCell, rc::Rc};

// use fltk::{
//     button::Button, enums::Color, frame::Frame, group, group::Flex, group::Pack, group::PackType,
//     prelude::*, window::Window, *,
// };
// use rand::prelude::*;

// use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};
// use rand::Rng;
// use std::rc::Rc;
// use std::cell::RefCell;

// fn main() {
//     // Создаем приложение и окно
//     let app = app::App::default();
//     let mut wind = Window::new(100, 100, 400, 300, "Random Array Generator");

//     // Оборачиваем Flex в Rc<RefCell<T>>
//     let table = Rc::new(RefCell::new(Flex::new(10, 70, 380, 200, None)));

//     // Создаем фрейм для вывода массива
//     let mut frame = Frame::new(10, 10, 380, 50, "Array will be displayed here");

//     // Создаем кнопку
//     let mut btn = Button::new(150, 100, 100, 40, "Generate Array");

//     // Обработчик нажатия на кнопку
//     let table_clone = Rc::clone(&table);
//     let mut frame_clone = frame.clone();
//     btn.set_callback(move |_| {
//         let mut rng = rand::thread_rng();
//         let random_numbers: Vec<i32> = (0..10).map(|_| rng.gen_range(0..100)).collect();
//         // Обновляем текст в фрейме
//         frame_clone.set_label(&format!("{:?}", random_numbers));
//         // Вызываем функцию для построения таблицы, используя borrow_mut для доступа к Flex
//         build_table(&mut table_clone.borrow_mut(), &random_numbers);
//     });

//     // Устанавливаем размеры и показываем окно
//     wind.end();
//     wind.show();
//     app.run().unwrap();
// }

// // Функция для построения таблицы
// fn build_table(table: &mut Flex, array: &[i32]) {
//     table.clear(); // Очищаем предыдущие элементы

//     // Создаем виджеты для каждого элемента массива
//     for &num in array {
//         let frame = Frame::new(0, 0, 50, 30, &*num.to_string());
//         table.add(&frame);
//     }

//     table.redraw(); // Перерисовываем таблицу
// }
