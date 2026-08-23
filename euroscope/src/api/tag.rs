//! Tag-item rendering — the writer handed to
//! [`Plugin::on_get_tag_item`](crate::Plugin::on_get_tag_item).

use std::ffi::{c_char, c_int};

/// Fills in a custom tag item's text, colour and font during
/// [`Plugin::on_get_tag_item`](crate::Plugin::on_get_tag_item).
///
/// The text buffer EuroScope provides is 16 bytes, so [`set_text`](Self::set_text)
/// truncates to 15 characters. If you set nothing, the item renders empty with
/// default styling.
pub struct TagItem {
    text: *mut c_char,
    color_code: *mut c_int,
    rgb: *mut u32,
    font_size: *mut f64,
}

impl TagItem {
    /// # Safety
    /// All pointers must be the valid out-params EuroScope passed to
    /// `OnGetTagItem` (text points to a 16-byte buffer).
    pub(crate) unsafe fn new(
        text: *mut c_char,
        color_code: *mut c_int,
        rgb: *mut u32,
        font_size: *mut f64,
    ) -> Self {
        Self {
            text,
            color_code,
            rgb,
            font_size,
        }
    }

    /// Set the displayed text (truncated to 15 bytes to fit EuroScope's buffer).
    pub fn set_text(&mut self, text: &str) {
        let bytes = text.as_bytes();
        let n = bytes.len().min(15);
        // SAFETY: `self.text` is EuroScope's 16-byte buffer; we write n<16 bytes
        // plus a NUL terminator.
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), self.text.cast::<u8>(), n);
            *self.text.add(n) = 0;
        }
    }

    /// Set the colour to a built-in [`TagColor`](crate::TagColor).
    pub fn set_color(&mut self, color: crate::TagColor) {
        self.set_color_code(color.to_raw());
    }

    /// Set the colour to a raw `TAG_COLOR_*` code. Prefer [`set_color`](Self::set_color).
    pub fn set_color_code(&mut self, code: i32) {
        // SAFETY: valid out-param.
        unsafe { *self.color_code = code }
    }

    /// Set an explicit RGB colour (used when the color code selects a custom
    /// colour).
    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        // Windows COLORREF is 0x00BBGGRR.
        let colorref = u32::from(r) | (u32::from(g) << 8_i32) | (u32::from(b) << 16_i32);
        // SAFETY: valid out-param.
        unsafe { *self.rgb = colorref }
    }

    /// Override the font size (points).
    pub fn set_font_size(&mut self, size: f64) {
        // SAFETY: valid out-param.
        unsafe { *self.font_size = size }
    }
}
