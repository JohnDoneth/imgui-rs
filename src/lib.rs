pub extern crate imgui_sys as sys;

use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_float, c_int, c_uchar, c_void};
use std::ptr;
use std::slice;
use std::str;
use sys::ImGuiStyleVar;

pub use sys::{ImDrawIdx, ImDrawVert, ImGuiColorEditFlags, ImGuiHoveredFlags, ImGuiInputTextFlags,
              ImGuiKey, ImGuiSelectableFlags, ImGuiCond, ImGuiCol, ImGuiStyle, ImGuiTreeNodeFlags,
              ImGuiWindowFlags, ImVec2, ImVec4};
pub use child_frame::ChildFrame;
pub use color_editors::{ColorButton, ColorEdit, ColorEditMode, ColorFormat, ColorPicker,
                        ColorPickerMode, ColorPreview, EditableColor};
pub use input::{InputFloat, InputFloat2, InputFloat3, InputFloat4, InputInt, InputInt2, InputInt3,
                InputInt4, InputText};
pub use menus::{Menu, MenuItem};
pub use plothistogram::PlotHistogram;
pub use plotlines::PlotLines;
pub use progressbar::ProgressBar;
pub use sliders::{SliderFloat, SliderFloat2, SliderFloat3, SliderFloat4, SliderInt, SliderInt2,
                  SliderInt3, SliderInt4};
pub use string::{ImStr, ImString};
pub use style::StyleVar;
pub use trees::{CollapsingHeader, TreeNode};
pub use window::Window;

mod child_frame;
mod color_editors;
mod input;
mod menus;
mod plothistogram;
mod plotlines;
mod progressbar;
mod sliders;
mod string;
mod style;
mod trees;
mod window;

pub struct ImGui {
    // We need to keep ownership of the ImStr values to ensure the *const char pointer
    // lives long enough in case the ImStr contains a Cow::Owned
    ini_filename: Option<ImString>,
    log_filename: Option<ImString>,
}

#[macro_export]
macro_rules! im_str {
    ($e:tt) => ({
        unsafe {
          $crate::ImStr::from_utf8_with_nul_unchecked(concat!($e, "\0").as_bytes())
        }
    });
    ($e:tt, $($arg:tt)*) => ({
        unsafe {
          &$crate::ImString::from_utf8_with_nul_unchecked(
            format!(concat!($e, "\0"), $($arg)*).into_bytes())
        }
    })
}

pub struct TextureHandle<'a> {
    pub width: u32,
    pub height: u32,
    pub pixels: &'a [c_uchar],
}

pub fn get_style_color_name(color: ImGuiCol) -> &'static ImStr {
    unsafe {
        let bytes = CStr::from_ptr(sys::igGetStyleColorName(color)).to_bytes_with_nul();
        ImStr::from_utf8_with_nul_unchecked(bytes)
    }
}

pub fn get_version() -> &'static str {
    unsafe {
        let bytes = CStr::from_ptr(sys::igGetVersion()).to_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

impl ImGui {
    pub fn init() -> ImGui {
        ImGui {
            ini_filename: None,
            log_filename: None,
        }
    }
    fn io(&self) -> &sys::ImGuiIO { unsafe { &*sys::igGetIO() } }
    fn io_mut(&mut self) -> &mut sys::ImGuiIO { unsafe { &mut *sys::igGetIO() } }
    pub fn style(&self) -> &ImGuiStyle { unsafe { &*sys::igGetStyle() } }
    pub fn style_mut(&mut self) -> &mut ImGuiStyle { unsafe { &mut *sys::igGetStyle() } }
    pub fn prepare_texture<'a, F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(TextureHandle<'a>) -> T,
    {
        let io = self.io();
        let mut pixels: *mut c_uchar = ptr::null_mut();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut bytes_per_pixel: c_int = 0;
        unsafe {
            sys::ImFontAtlas_GetTexDataAsRGBA32(
                io.fonts,
                &mut pixels,
                &mut width,
                &mut height,
                &mut bytes_per_pixel,
            );
            f(TextureHandle {
                width: width as u32,
                height: height as u32,
                pixels: slice::from_raw_parts(pixels, (width * height * bytes_per_pixel) as usize),
            })
        }
    }
    pub fn set_texture_id(&mut self, value: usize) {
        unsafe {
            (*self.io_mut().fonts).tex_id = value as *mut c_void;
        }
    }
    pub fn set_ini_filename(&mut self, value: Option<ImString>) {
        {
            let io = self.io_mut();
            io.ini_filename = match value {
                Some(ref x) => x.as_ptr(),
                None => ptr::null(),
            }
        }
        self.ini_filename = value;
    }
    pub fn set_log_filename(&mut self, value: Option<ImString>) {
        {
            let io = self.io_mut();
            io.log_filename = match value {
                Some(ref x) => x.as_ptr(),
                None => ptr::null(),
            }
        }
        self.log_filename = value;
    }
    pub fn set_ini_saving_rate(&mut self, value: f32) {
        let io = self.io_mut();
        io.ini_saving_rate = value;
    }
    pub fn set_font_global_scale(&mut self, value: f32) {
        let io = self.io_mut();
        io.font_global_scale = value;
    }
    pub fn set_mouse_double_click_time(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_double_click_time = value;
    }
    pub fn set_mouse_double_click_max_dist(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_double_click_max_dist = value;
    }
    pub fn set_mouse_drag_threshold(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_drag_threshold = value;
    }
    pub fn set_key_repeat_delay(&mut self, value: f32) {
        let io = self.io_mut();
        io.key_repeat_delay = value;
    }
    pub fn set_key_repeat_rate(&mut self, value: f32) {
        let io = self.io_mut();
        io.key_repeat_rate = value;
    }
    pub fn display_size(&self) -> (f32, f32) {
        let io = self.io();
        (io.display_size.x, io.display_size.y)
    }
    pub fn display_framebuffer_scale(&self) -> (f32, f32) {
        let io = self.io();
        (
            io.display_framebuffer_scale.x,
            io.display_framebuffer_scale.y,
        )
    }
    pub fn mouse_pos(&self) -> (f32, f32) {
        let io = self.io();
        (io.mouse_pos.x, io.mouse_pos.y)
    }
    pub fn set_mouse_pos(&mut self, x: f32, y: f32) {
        let io = self.io_mut();
        io.mouse_pos.x = x;
        io.mouse_pos.y = y;
    }
    pub fn set_mouse_down(&mut self, states: &[bool; 5]) {
        let io = self.io_mut();
        io.mouse_down = *states;
    }
    pub fn set_mouse_wheel(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_wheel = value;
    }
    pub fn set_mouse_draw_cursor(&mut self, value: bool) {
        let io = self.io_mut();
        io.mouse_draw_cursor = value;
    }
    pub fn key_ctrl(&self) -> bool {
        let io = self.io();
        io.key_ctrl
    }
    pub fn set_key_ctrl(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_ctrl = value;
    }
    pub fn key_shift(&self) -> bool {
        let io = self.io();
        io.key_shift
    }
    pub fn set_key_shift(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_shift = value;
    }
    pub fn key_alt(&self) -> bool {
        let io = self.io();
        io.key_alt
    }
    pub fn set_key_alt(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_alt = value;
    }
    pub fn set_key_super(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_super = value;
    }
    pub fn set_key(&mut self, key: u8, pressed: bool) {
        let io = self.io_mut();
        io.keys_down[key as usize] = pressed;
    }
    pub fn set_imgui_key(&mut self, key: ImGuiKey, mapping: u8) {
        let io = self.io_mut();
        io.key_map[key as usize] = mapping as i32;
    }
    pub fn add_input_character(&mut self, character: char) {
        let mut buf = [0; 5];
        character.encode_utf8(&mut buf);
        unsafe {
            sys::ImGuiIO_AddInputCharactersUTF8(buf.as_ptr() as *const _);
        }
    }
    pub fn get_time(&self) -> f32 { unsafe { sys::igGetTime() } }
    pub fn get_frame_count(&self) -> i32 { unsafe { sys::igGetFrameCount() } }
    pub fn get_frame_rate(&self) -> f32 { self.io().framerate }
    pub fn frame<'ui, 'a: 'ui>(
        &'a mut self,
        size_points: (u32, u32),
        size_pixels: (u32, u32),
        delta_time: f32,
    ) -> Ui<'ui> {
        {
            let io = self.io_mut();
            io.display_size.x = size_points.0 as c_float;
            io.display_size.y = size_points.1 as c_float;
            io.display_framebuffer_scale.x = if size_points.0 > 0 {
                size_pixels.0 as c_float / size_points.0 as c_float
            } else {
                0.0
            };
            io.display_framebuffer_scale.y = if size_points.1 > 0 {
                size_pixels.1 as c_float / size_points.1 as c_float
            } else {
                0.0
            };
            io.delta_time = delta_time;
        }
        unsafe {
            sys::igNewFrame();
            CURRENT_UI = Some(Ui { imgui: mem::transmute(self as &'a ImGui) });
        }
        Ui { imgui: self }
    }
}

impl Drop for ImGui {
    fn drop(&mut self) {
        unsafe {
            CURRENT_UI = None;
            sys::igShutdown();
        }
    }
}

static mut CURRENT_UI: Option<Ui<'static>> = None;

pub struct DrawList<'a> {
    pub cmd_buffer: &'a [sys::ImDrawCmd],
    pub idx_buffer: &'a [sys::ImDrawIdx],
    pub vtx_buffer: &'a [sys::ImDrawVert],
}

pub struct Ui<'ui> {
    imgui: &'ui ImGui,
}

static FMT: &'static [u8] = b"%s\0";

fn fmt_ptr() -> *const c_char { FMT.as_ptr() as *const c_char }

impl<'ui> Ui<'ui> {
    pub fn imgui(&self) -> &ImGui { self.imgui }
    pub fn want_capture_mouse(&self) -> bool {
        let io = self.imgui.io();
        io.want_capture_mouse
    }
    pub fn want_capture_keyboard(&self) -> bool {
        let io = self.imgui.io();
        io.want_capture_keyboard
    }
    pub fn framerate(&self) -> f32 {
        let io = self.imgui.io();
        io.framerate
    }
    pub fn metrics_allocs(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_allocs
    }
    pub fn metrics_render_vertices(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_render_vertices
    }
    pub fn metrics_render_indices(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_render_indices
    }
    pub fn metrics_active_windows(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_active_windows
    }
    pub fn render<F, E>(self, mut f: F) -> Result<(), E>
    where
        F: FnMut(&Ui, DrawList) -> Result<(), E>,
    {
        unsafe {
            sys::igRender();

            let draw_data = sys::igGetDrawData();
            for &cmd_list in (*draw_data).cmd_lists() {
                let draw_list = DrawList {
                    cmd_buffer: (*cmd_list).cmd_buffer.as_slice(),
                    idx_buffer: (*cmd_list).idx_buffer.as_slice(),
                    vtx_buffer: (*cmd_list).vtx_buffer.as_slice(),
                };
                try!(f(&self, draw_list));
            }
            CURRENT_UI = None;
        }
        Ok(())
    }
    pub fn show_user_guide(&self) { unsafe { sys::igShowUserGuide() }; }
    pub fn show_default_style_editor(&self) { unsafe { sys::igShowStyleEditor(ptr::null_mut()) }; }
    pub fn show_style_editor<'p>(&self, style: &'p mut ImGuiStyle) {
        unsafe {
            sys::igShowStyleEditor(style as *mut ImGuiStyle);
        }
    }
    pub fn show_test_window(&self, opened: &mut bool) {
        unsafe {
            sys::igShowTestWindow(opened);
        }
    }
    pub fn show_metrics_window(&self, opened: &mut bool) {
        unsafe {
            sys::igShowMetricsWindow(opened);
        }
    }
}

impl<'a> Ui<'a> {
    pub unsafe fn current_ui() -> Option<&'a Ui<'a>> { CURRENT_UI.as_ref() }
}

// Window
impl<'ui> Ui<'ui> {
    pub fn window<'p>(&self, name: &'p ImStr) -> Window<'ui, 'p> { Window::new(self, name) }
}

// Layout
impl<'ui> Ui<'ui> {
    /// Pushes a value to the item width stack.
    pub fn push_item_width(&self, width: f32) { unsafe { sys::igPushItemWidth(width) } }

    /// Pops a value from the item width stack.
    ///
    /// # Aborts
    /// The current process is aborted if the item width stack is empty.
    pub fn pop_item_width(&self) { unsafe { sys::igPopItemWidth() } }

    /// Runs a function after temporarily pushing a value to the item width stack.
    pub fn with_item_width<F>(&self, width: f32, f: F)
    where
        F: FnOnce(),
    {
        self.push_item_width(width);
        f();
        self.pop_item_width();
    }

    pub fn separator(&self) { unsafe { sys::igSeparator() }; }
    pub fn new_line(&self) { unsafe { sys::igNewLine() } }
    pub fn same_line(&self, pos_x: f32) { unsafe { sys::igSameLine(pos_x, -1.0f32) } }
    pub fn same_line_spacing(&self, pos_x: f32, spacing_w: f32) {
        unsafe { sys::igSameLine(pos_x, spacing_w) }
    }
    pub fn spacing(&self) { unsafe { sys::igSpacing() }; }

    pub fn columns<'p>(&self, count: i32, id: &'p ImStr, border: bool) {
        unsafe { sys::igColumns(count, id.as_ptr(), border) }
    }

    pub fn next_column(&self) { unsafe { sys::igNextColumn() } }

    pub fn get_column_index(&self) -> i32 { unsafe { sys::igGetColumnIndex() } }

    pub fn get_column_offset(&self, column_index: i32) -> f32 {
        unsafe { sys::igGetColumnOffset(column_index) }
    }

    pub fn set_column_offset(&self, column_index: i32, offset_x: f32) {
        unsafe { sys::igSetColumnOffset(column_index, offset_x) }
    }

    pub fn get_column_width(&self, column_index: i32) -> f32 {
        unsafe { sys::igGetColumnWidth(column_index) }
    }

    pub fn get_columns_count(&self) -> i32 { unsafe { sys::igGetColumnsCount() } }
}

// ID scopes
impl<'ui> Ui<'ui> {
    /// Pushes an identifier to the ID stack.
    pub fn push_id(&self, id: i32) { unsafe { sys::igPushIDInt(id) }; }

    /// Pops an identifier from the ID stack.
    ///
    /// # Aborts
    /// The current process is aborted if the ID stack is empty.
    pub fn pop_id(&self) { unsafe { sys::igPopID() }; }

    /// Runs a function after temporarily pushing a value to the ID stack.
    pub fn with_id<F>(&self, id: i32, f: F)
    where
        F: FnOnce(),
    {
        self.push_id(id);
        f();
        self.pop_id();
    }
}

// Widgets
impl<'ui> Ui<'ui> {
    pub fn text<T: AsRef<str>>(&self, text: T) {
        let s = text.as_ref();
        unsafe {
            let start = s.as_ptr();
            let end = start.offset(s.len() as isize);
            sys::igTextUnformatted(start as *const c_char, end as *const c_char);
        }
    }
    pub fn text_colored<'p, A>(&self, col: A, text: &'p ImStr)
    where
        A: Into<ImVec4>,
    {
        unsafe {
            sys::igTextColored(col.into(), fmt_ptr(), text.as_ptr());
        }
    }
    pub fn text_disabled<'p>(&self, text: &'p ImStr) {
        unsafe {
            sys::igTextDisabled(fmt_ptr(), text.as_ptr());
        }
    }
    pub fn text_wrapped<'p>(&self, text: &'p ImStr) {
        unsafe {
            sys::igTextWrapped(fmt_ptr(), text.as_ptr());
        }
    }
    pub fn label_text<'p>(&self, label: &'p ImStr, text: &'p ImStr) {
        unsafe {
            sys::igLabelText(label.as_ptr(), fmt_ptr(), text.as_ptr());
        }
    }
    pub fn bullet(&self) {
        unsafe {
            sys::igBullet();
        }
    }
    pub fn bullet_text<'p>(&self, text: &'p ImStr) {
        unsafe {
            sys::igBulletText(fmt_ptr(), text.as_ptr());
        }
    }
    pub fn button<'p, S: Into<ImVec2>>(&self, label: &'p ImStr, size: S) -> bool {
        unsafe { sys::igButton(label.as_ptr(), size.into()) }
    }
    pub fn small_button<'p>(&self, label: &'p ImStr) -> bool {
        unsafe { sys::igSmallButton(label.as_ptr()) }
    }
    pub fn checkbox<'p>(&self, label: &'p ImStr, value: &'p mut bool) -> bool {
        unsafe { sys::igCheckbox(label.as_ptr(), value) }
    }
}

// Widgets: Input
impl<'ui> Ui<'ui> {
    pub fn input_text<'p>(&self, label: &'p ImStr, buf: &'p mut ImString) -> InputText<'ui, 'p> {
        InputText::new(self, label, buf)
    }
    pub fn input_float<'p>(&self, label: &'p ImStr, value: &'p mut f32) -> InputFloat<'ui, 'p> {
        InputFloat::new(self, label, value)
    }
    pub fn input_float2<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 2],
    ) -> InputFloat2<'ui, 'p> {
        InputFloat2::new(self, label, value)
    }
    pub fn input_float3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 3],
    ) -> InputFloat3<'ui, 'p> {
        InputFloat3::new(self, label, value)
    }
    pub fn input_float4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 4],
    ) -> InputFloat4<'ui, 'p> {
        InputFloat4::new(self, label, value)
    }
    pub fn input_int<'p>(&self, label: &'p ImStr, value: &'p mut i32) -> InputInt<'ui, 'p> {
        InputInt::new(self, label, value)
    }
    pub fn input_int2<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 2]) -> InputInt2<'ui, 'p> {
        InputInt2::new(self, label, value)
    }
    pub fn input_int3<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 3]) -> InputInt3<'ui, 'p> {
        InputInt3::new(self, label, value)
    }
    pub fn input_int4<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 4]) -> InputInt4<'ui, 'p> {
        InputInt4::new(self, label, value)
    }
}

// Widgets: Sliders
impl<'ui> Ui<'ui> {
    pub fn slider_float<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut f32,
        min: f32,
        max: f32,
    ) -> SliderFloat<'ui, 'p> {
        SliderFloat::new(self, label, value, min, max)
    }
    pub fn slider_float2<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 2],
        min: f32,
        max: f32,
    ) -> SliderFloat2<'ui, 'p> {
        SliderFloat2::new(self, label, value, min, max)
    }
    pub fn slider_float3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 3],
        min: f32,
        max: f32,
    ) -> SliderFloat3<'ui, 'p> {
        SliderFloat3::new(self, label, value, min, max)
    }
    pub fn slider_float4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 4],
        min: f32,
        max: f32,
    ) -> SliderFloat4<'ui, 'p> {
        SliderFloat4::new(self, label, value, min, max)
    }
    pub fn slider_int<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut i32,
        min: i32,
        max: i32,
    ) -> SliderInt<'ui, 'p> {
        SliderInt::new(self, label, value, min, max)
    }
    pub fn slider_int2<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [i32; 2],
        min: i32,
        max: i32,
    ) -> SliderInt2<'ui, 'p> {
        SliderInt2::new(self, label, value, min, max)
    }
    pub fn slider_int3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [i32; 3],
        min: i32,
        max: i32,
    ) -> SliderInt3<'ui, 'p> {
        SliderInt3::new(self, label, value, min, max)
    }
    pub fn slider_int4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [i32; 4],
        min: i32,
        max: i32,
    ) -> SliderInt4<'ui, 'p> {
        SliderInt4::new(self, label, value, min, max)
    }
}

// Widgets: Color Editor/Picker
impl<'ui> Ui<'ui> {
    /// Constructs a new color editor builder.
    pub fn color_edit<'p, V: Into<EditableColor<'p>>>(
        &self,
        label: &'p ImStr,
        value: V,
    ) -> ColorEdit<'ui, 'p> {
        ColorEdit::new(self, label, value.into())
    }
    /// Constructs a new color picker builder.
    pub fn color_picker<'p, V: Into<EditableColor<'p>>>(
        &self,
        label: &'p ImStr,
        value: V,
    ) -> ColorPicker<'ui, 'p> {
        ColorPicker::new(self, label, value.into())
    }
    /// Constructs a new color button builder.
    pub fn color_button<'p, C: Into<ImVec4>>(
        &self,
        desc_id: &'p ImStr,
        color: C,
    ) -> ColorButton<'ui, 'p> {
        ColorButton::new(self, desc_id, color.into())
    }
    /// Initialize current options (generally on application startup) if you want to select a
    /// default format, picker type, etc. Users will be able to change many settings, unless you
    /// use .options(false) in your widget builders.
    pub fn set_color_edit_options(&self, flags: ImGuiColorEditFlags) {
        unsafe {
            sys::igSetColorEditOptions(flags);
        }
    }
}

// Widgets: Trees
impl<'ui> Ui<'ui> {
    pub fn tree_node<'p>(&self, id: &'p ImStr) -> TreeNode<'ui, 'p> { TreeNode::new(self, id) }
    pub fn collapsing_header<'p>(&self, label: &'p ImStr) -> CollapsingHeader<'ui, 'p> {
        CollapsingHeader::new(self, label)
    }
}

// Widgets: Selectable / Lists
impl<'ui> Ui<'ui> {
    pub fn selectable<'p, S: Into<ImVec2>>(
        &self,
        label: &'p ImStr,
        selected: bool,
        flags: ImGuiSelectableFlags,
        size: S,
    ) -> bool {
        unsafe { sys::igSelectable(label.as_ptr(), selected, flags, size.into()) }
    }
}

/// # Tooltips
impl<'ui> Ui<'ui> {
    /// Construct a tooltip window that can have any kind of content.
    ///
    /// Typically used with `Ui::is_item_hovered()` or some other conditional check.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate imgui;
    /// # use imgui::*;
    /// fn user_interface(ui: &Ui) {
    ///     ui.text("Hover over me");
    ///     if ui.is_item_hovered() {
    ///         ui.tooltip(|| {
    ///             ui.text_colored((1.0, 0.0, 0.0, 1.0), im_str!("I'm red!"));
    ///         });
    ///     }
    /// }
    /// # fn main() {
    /// # }
    /// ```
    pub fn tooltip<F: FnOnce()>(&self, f: F) {
        unsafe { sys::igBeginTooltip() };
        f();
        unsafe { sys::igEndTooltip() };
    }
    /// Construct a tooltip window with simple text content.
    ///
    /// Typically used with `Ui::is_item_hovered()` or some other conditional check.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate imgui;
    /// # use imgui::*;
    /// fn user_interface(ui: &Ui) {
    ///     ui.text("Hover over me");
    ///     if ui.is_item_hovered() {
    ///         ui.tooltip_text("I'm a tooltip!");
    ///     }
    /// }
    /// # fn main() {
    /// # }
    /// ```
    pub fn tooltip_text<T: AsRef<str>>(&self, text: T) { self.tooltip(|| self.text(text)); }
}

// Widgets: Menus
impl<'ui> Ui<'ui> {
    pub fn main_menu_bar<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        let render = unsafe { sys::igBeginMainMenuBar() };
        if render {
            f();
            unsafe { sys::igEndMainMenuBar() };
        }
    }
    pub fn menu_bar<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        let render = unsafe { sys::igBeginMenuBar() };
        if render {
            f();
            unsafe { sys::igEndMenuBar() };
        }
    }
    pub fn menu<'p>(&self, label: &'p ImStr) -> Menu<'ui, 'p> { Menu::new(self, label) }
    pub fn menu_item<'p>(&self, label: &'p ImStr) -> MenuItem<'ui, 'p> {
        MenuItem::new(self, label)
    }
}

// Widgets: Popups
impl<'ui> Ui<'ui> {
    pub fn open_popup<'p>(&self, str_id: &'p ImStr) {
        unsafe { sys::igOpenPopup(str_id.as_ptr()) };
    }
    pub fn popup<'p, F>(&self, str_id: &'p ImStr, f: F)
    where
        F: FnOnce(),
    {
        let render = unsafe { sys::igBeginPopup(str_id.as_ptr()) };
        if render {
            f();
            unsafe { sys::igEndPopup() };
        }
    }
    pub fn close_current_popup(&self) { unsafe { sys::igCloseCurrentPopup() }; }
}

// Widgets: Combos
impl<'ui> Ui<'ui> {
    pub fn combo<'p>(
        &self,
        label: &'p ImStr,
        current_item: &mut i32,
        items: &'p [&'p ImStr],
        height_in_items: i32,
    ) -> bool {
        let items_inner: Vec<*const c_char> = items.into_iter().map(|item| item.as_ptr()).collect();
        unsafe {
            sys::igCombo(
                label.as_ptr(),
                current_item,
                items_inner.as_ptr() as *mut *const c_char,
                items_inner.len() as i32,
                height_in_items,
            )
        }
    }
}

// Widgets: ListBox
impl<'ui> Ui<'ui> {
    pub fn list_box<'p>(
        &self,
        label: &'p ImStr,
        current_item: &mut i32,
        items: &'p [&'p ImStr],
        height_in_items: i32,
    ) -> bool {
        let items_inner: Vec<*const c_char> = items.into_iter().map(|item| item.as_ptr()).collect();
        unsafe {
            sys::igListBox(
                label.as_ptr(),
                current_item,
                items_inner.as_ptr() as *mut *const c_char,
                items_inner.len() as i32,
                height_in_items,
            )
        }
    }
}

// Widgets: Radio
impl<'ui> Ui<'ui> {
    /// Creates a radio button for selecting an integer value.
    /// Returns true if pressed.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// # let mut selected_radio_value = 2;
    /// ui.radio_button(im_str!("Item 1"), &mut selected_radio_value, 1);
    /// ui.radio_button(im_str!("Item 2"), &mut selected_radio_value, 2);
    /// ui.radio_button(im_str!("Item 3"), &mut selected_radio_value, 3);
    /// ```
    pub fn radio_button<'p>(&self, label: &'p ImStr, value: &'p mut i32, wanted: i32) -> bool {
        unsafe { sys::igRadioButton(label.as_ptr(), value, wanted) }
    }

    /// Creates a radio button that shows as selected if the given value is true.
    /// Returns true if pressed.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// # let mut radio_button_test = "cats".to_string();
    /// if ui.radio_button_bool(im_str!("Cats"), radio_button_test == "cats") {
    ///     radio_button_test = "cats".to_string();
    /// }
    /// if ui.radio_button_bool(im_str!("Dogs"), radio_button_test == "dogs") {
    ///     radio_button_test = "dogs".to_string();
    /// }
    /// ```
    pub fn radio_button_bool<'p>(&self, label: &'p ImStr, value: bool) -> bool {
        unsafe { sys::igRadioButtonBool(label.as_ptr(), value) }
    }
}

impl<'ui> Ui<'ui> {
    pub fn plot_lines<'p>(&self, label: &'p ImStr, values: &'p [f32]) -> PlotLines<'ui, 'p> {
        PlotLines::new(self, label, values)
    }
}

impl<'ui> Ui<'ui> {
    pub fn plot_histogram<'p>(
        &self,
        label: &'p ImStr,
        values: &'p [f32],
    ) -> PlotHistogram<'ui, 'p> {
        PlotHistogram::new(self, label, values)
    }
}

impl<'ui> Ui<'ui> {
    /// Calculate the size required for a given text string.
    ///
    /// hide_text_after_double_hash allows the user to insert comments into their text, using a double hash-tag prefix.
    /// This is a feature of imgui.
    ///
    /// wrap_width allows you to request a width at which to wrap the text to a newline for the calculation.
    pub fn calc_text_size(
        &self,
        text: &ImStr,
        hide_text_after_double_hash: bool,
        wrap_width: f32,
    ) -> ImVec2 {
        let mut buffer = ImVec2::new(0.0, 0.0);
        unsafe {
            sys::igCalcTextSize(
                &mut buffer as *mut ImVec2,
                text.as_ptr(),
                std::ptr::null(),
                hide_text_after_double_hash,
                wrap_width,
            );
        }
        buffer
    }
}

impl<'ui> Ui<'ui> {
    /// Creates a progress bar. Fraction is the progress level with 0.0 = 0% and 1.0 = 100%.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// ui.progress_bar(0.6)
    ///     .size((100.0, 12.0))
    ///     .overlay_text(im_str!("Progress!"))
    ///     .build();
    /// ```
    pub fn progress_bar<'p>(&self, fraction: f32) -> ProgressBar<'ui, 'p> {
        ProgressBar::new(self, fraction)
    }
}

impl<'ui> Ui<'ui> {
    /// Creates a child frame. Size is size of child_frame within parent window.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// ui.window(im_str!("ChatWindow"))
    ///     .title_bar(true)
    ///     .scrollable(false)
    ///     .build(|| {
    ///         ui.separator();
    ///
    ///         ui.child_frame(im_str!("child frame"), (400.0, 100.0))
    ///             .show_borders(true)
    ///             .always_show_vertical_scroll_bar(true)
    ///             .build(|| {
    ///                 ui.text_colored((1.0, 0.0, 0.0, 1.0), im_str!("hello mate!"));
    ///             });
    /// });
    pub fn child_frame<'p, S: Into<ImVec2>>(
        &self,
        name: &'p ImStr,
        size: S,
    ) -> ChildFrame<'ui, 'p> {
        ChildFrame::new(self, name, size.into())
    }
}

impl<'ui> Ui<'ui> {
    /// Runs a function after temporarily pushing a value to the style stack.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// ui.with_style_var(StyleVar::Alpha(0.2), || {
    ///     ui.text(im_str!("AB"));
    /// });
    /// ```
    pub fn with_style_var<F: FnOnce()>(&self, style_var: StyleVar, f: F) {
        self.push_style_var(style_var);
        f();
        unsafe { sys::igPopStyleVar(1) }
    }

    /// Runs a function after temporarily pushing an array of values into the stack. Supporting
    /// multiple is also easy since you can freely mix and match them in a safe manner.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// # let styles = [StyleVar::Alpha(0.2), StyleVar::WindowPadding(ImVec2::new(1.0, 1.0))];
    /// ui.with_style_vars(&styles, || {
    ///     ui.text(im_str!("A"));
    ///     ui.text(im_str!("B"));
    ///     ui.text(im_str!("C"));
    ///     ui.text(im_str!("D"));
    /// });
    /// ```
    pub fn with_style_vars<F: FnOnce()>(&self, style_vars: &[StyleVar], f: F) {
        for &style_var in style_vars {
            self.push_style_var(style_var);
        }
        f();
        unsafe { sys::igPopStyleVar(style_vars.len() as i32) };
    }

    #[inline]
    fn push_style_var(&self, style_var: StyleVar) {
        use StyleVar::*;
        use sys::{igPushStyleVar, igPushStyleVarVec};
        match style_var {
            Alpha(v) => unsafe { igPushStyleVar(ImGuiStyleVar::Alpha, v) },
            WindowPadding(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::WindowPadding, v) },
            WindowRounding(v) => unsafe { igPushStyleVar(ImGuiStyleVar::WindowRounding, v) },
            WindowMinSize(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::WindowMinSize, v) },
            ChildWindowRounding(v) => unsafe {
                igPushStyleVar(ImGuiStyleVar::ChildWindowRounding, v)
            },
            FramePadding(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::FramePadding, v) },
            FrameRounding(v) => unsafe { igPushStyleVar(ImGuiStyleVar::FrameRounding, v) },
            ItemSpacing(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::ItemSpacing, v) },
            ItemInnerSpacing(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::ItemInnerSpacing, v) },
            IndentSpacing(v) => unsafe { igPushStyleVar(ImGuiStyleVar::IndentSpacing, v) },
            GrabMinSize(v) => unsafe { igPushStyleVar(ImGuiStyleVar::GrabMinSize, v) },
            ButtonTextAlign(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::ButtonTextAlign, v) },
        }
    }
}

impl<'ui> Ui<'ui> {
    /// Runs a function after temporarily pushing a value to the color stack.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// ui.with_color_var(ImGuiCol::Text, (1.0, 0.0, 0.0, 1.0), || {
    ///     ui.text_wrapped(im_str!("AB"));
    /// });
    /// ```
    pub fn with_color_var<F: FnOnce(), C: Into<ImVec4> + Copy>(
        &self,
        var: ImGuiCol,
        color: C,
        f: F,
    ) {
        unsafe {
            sys::igPushStyleColor(var, color.into());
        }
        f();
        unsafe {
            sys::igPopStyleColor(1);
        }
    }

    /// Runs a function after temporarily pushing an array of values to the color stack.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// let red = (1.0, 0.0, 0.0, 1.0);
    /// let green = (0.0, 1.0, 0.0, 1.0);
    /// # let vars = [(ImGuiCol::Text, red), (ImGuiCol::TextDisabled, green)];
    /// ui.with_color_vars(&vars, || {
    ///     ui.text_wrapped(im_str!("AB"));
    /// });
    /// ```
    pub fn with_color_vars<F: FnOnce(), C: Into<ImVec4> + Copy>(
        &self,
        color_vars: &[(ImGuiCol, C)],
        f: F,
    ) {
        for &(color_var, color) in color_vars {
            unsafe {
                sys::igPushStyleColor(color_var, color.into());
            }
        }
        f();
        unsafe { sys::igPopStyleColor(color_vars.len() as i32) };
    }
}

/// # Utilities
impl<'ui> Ui<'ui> {
    /// Returns `true` if the last item is being hovered by the mouse.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate imgui;
    /// # use imgui::*;
    /// fn user_interface(ui: &Ui) {
    ///     ui.text("Hover over me");
    ///     let is_hover_over_me_text_hovered = ui.is_item_hovered();
    /// }
    /// # fn main() {
    /// # }
    /// ```
    pub fn is_item_hovered(&self) -> bool {
        unsafe { sys::igIsItemHovered(ImGuiHoveredFlags::empty()) }
    }
}
