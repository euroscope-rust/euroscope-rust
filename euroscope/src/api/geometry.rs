//! Screen geometry mirroring Win32 `POINT` / `RECT`, used by
//! [`Plugin::on_function_call`](crate::Plugin::on_function_call) and the popup
//! APIs on [`Context`](crate::Context).

/// A screen point in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// A screen rectangle in pixels (Win32 `RECT` layout).
///
/// The `area` passed to [`Plugin::on_function_call`](crate::Plugin::on_function_call)
/// is the natural argument to the `open_popup_*` methods, so a menu/edit box
/// appears where the user clicked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}
