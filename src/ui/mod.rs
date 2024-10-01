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
// General
pub struct UIutil {}
impl UIutil {
  pub fn hex_to_rgb(hex: &str) -> Option<Color> {
    if hex.len() == 7 && hex.starts_with('#') {
        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
        Some(Color::from_rgb(r, g, b))
    } else {
        None
    }
  }
}
// Window
pub struct UIwindow {
  widget: Window,
  // x: i32,
  // y: i32,
  width: i32,
  height: i32,
  // title: String,
  background: Color,
  grid: bool,
  grid_columns_count: i32,
  grid_rows_count: i32,
  grid_columns_size: i32,
  grid_rows_size: i32,
}
impl UIwindow {
  pub fn new(
    x: i32, 
    y: i32, 
    width: i32, 
    height: i32, 
    title: &str,
    background: Color,
    grid: bool,
    grid_columns_count: i32,
    grid_rows_count: i32
  ) -> Self {

    let widget = crate::ui::fltk_impl::create_widget_window(
      x, 
      y, 
      width, 
      height,
      &title,
    );

    UIwindow {
      widget,
      // x, 
      // y, 
      width, 
      height, 
      // title: title.to_string(),
      background,
      grid,
      grid_columns_count,
      grid_rows_count,
      grid_columns_size: width / grid_columns_count,
      grid_rows_size: height / grid_rows_count
    }
  }

  pub fn paint(&self) {
    let mut widget = self.widget.clone();
    widget.set_color(self.background);

    if self.grid == false {
      return
    }

    let grid_columns_count = self.grid_columns_count;
    let grid_columns_size = self.grid_columns_size;
    let grid_rows_count = self.grid_rows_count;
    let grid_rows_size = self.grid_rows_size;
    let height = self.height;
    let width = self.width;
    widget.draw(move |_| {
      draw::set_draw_color(Color::from_rgb(236, 236, 236));
      for col in 0..grid_columns_count {
          let x = col * grid_columns_size;
          draw::draw_line(x, 0, x, height);
      }
      for row in 0..grid_rows_count {
          let y = row * grid_rows_size;
          draw::draw_line(0, y, width, y);
      }
    });
  }

  pub fn draw(&self) {
    let mut widget = self.widget.clone();
    widget.end();
    widget.show();
  }

  pub fn get_grid_sizes(&self) -> (i32, i32) {
    (self.grid_columns_size, self.grid_rows_size)
  }
}
// Header
pub struct UIheader {
  widget: Flex,
  x: i32,
  y: i32,
  width: i32, 
  height: i32, 
  title: String,
  background: Color,
}
impl UIheader {
  pub fn new(
    x: i32, 
    y: i32, 
    width: i32, 
    height: i32, 
    title: &str,
    background: Color,
  ) -> Self {

    let widget = crate::ui::fltk_impl::create_widget_header(
      x, 
      y, 
      width, 
      height,
      "",
    );

    UIheader {
      widget,
      x,
      y,
      width, 
      height, 
      title: title.to_string(),
      background,
    }
  }
  
  pub fn paint(&self) {
    let mut widget = self.widget.clone();
    let mut background = frame::Frame::new(
      self.x,
      self.y, 
      self.width, 
      self.height, 
      &*self.title
    );
    background.set_color(self.background);
    background.set_frame(enums::FrameType::FlatBox);
    widget.add(&background);
  }

  pub fn draw(&self) {
    let mut widget = self.widget.clone();
    widget.set_type(FlexType::Row);
    widget.end();
  }
}

// Control Panel
// Table
pub mod fltk_impl;
