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

pub fn create_widget_table(x: i32, y: i32, w: i32, h: i32) -> Group {
  let mut w = Group::new(x, y, w, h, None);
  w.set_color(Color::TransparentBg);
  w.set_label_color(Color::TransparentBg);
  w.set_frame(FrameType::FlatBox);
  w
}

pub fn create_widget_window(x: i32, y: i32, w: i32, h: i32, title: &str) -> Window {
  let mut w = Window::new(x, y, w, h, title);
  w.set_color(Color::TransparentBg);
  w.set_label_color(Color::TransparentBg);
  w.set_frame(FrameType::FlatBox);
  w
}

pub fn create_widget_header(x: i32, y: i32, w: i32, h: i32, title: &str) -> Flex {
  let mut w = Flex::new(x, y, w, h, title);
  w.set_color(Color::TransparentBg);
  w.set_label_color(Color::TransparentBg);
  w.set_frame(FrameType::FlatBox);
  w
}
