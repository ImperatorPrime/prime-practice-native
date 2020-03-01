pub mod c_interface {
    extern "C" {
        #[no_mangle]
        pub fn draw_text(str: *const u8, len: u32, x: f32, y: f32);
        #[no_mangle]
        pub fn text_color(r: f32, g: f32, b: f32, a: f32);
        #[no_mangle]
        pub fn get_fps() -> f32;
        #[no_mangle]
        pub fn is_pause_screen() -> bool;

        #[no_mangle]
        pub fn warp(world: u32, area: u32);

        #[no_mangle]
        pub fn pad_start(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_r(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_l(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_z(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_y(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_x(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_b(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_a(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_d_right(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_d_left(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_d_down(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_d_up(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_l_trigger(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_r_trigger(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_l_a_right(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_l_a_left(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_l_a_down(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_l_a_up(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_r_a_right(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_r_a_left(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_r_a_down(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_p_r_a_up(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_start(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_r(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_l(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_z(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_y(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_x(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_b(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_a(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_d_right(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_d_left(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_d_down(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_d_up(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_l_trigger(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_r_trigger(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_l_a_right(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_l_a_left(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_l_a_down(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_l_a_up(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_r_a_right(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_r_a_left(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_r_a_down(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_d_r_a_up(pad_index: u32) -> bool;
        #[no_mangle]
        pub fn pad_a_start(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_r(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_l(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_z(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_y(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_x(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_b(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_a(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_d_right(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_d_left(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_d_down(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_d_up(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_r_trigger(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_l_trigger(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_r_a_right(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_r_a_left(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_r_a_down(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_r_a_up(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_l_a_right(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_l_a_left(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_l_a_down(pad_index: u32) -> f32;
        #[no_mangle]
        pub fn pad_a_l_a_up(pad_index: u32) -> f32;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModConfig {
    pub show_speed: bool,
    pub show_pos: bool,
    pub show_high_p_pos: bool,
    pub show_room_timers: bool,
    pub show_igt: bool,
    pub show_input: bool,
    pub show_fps: bool,
    pub show_unknown_triggers: bool,
    pub show_load_triggers: bool,
    pub show_door_triggers: bool,
    pub show_force_triggers: bool,
    pub show_camera_hint_triggers: bool,
}

pub static CONFIG: ModConfig = ModConfig {
    show_speed: false,
    show_pos: false,
    show_high_p_pos: false,
    show_room_timers: false,
    show_igt: true,
    show_input: true,
    show_fps: true,
    show_unknown_triggers: false,
    show_load_triggers: false,
    show_door_triggers: false,
    show_force_triggers: false,
    show_camera_hint_triggers: false,
};

pub mod text_renderer {
    use super::c_interface;

    pub fn get_fps() -> f32 {
        unsafe { c_interface::get_fps() }
    }

    pub fn set_text_color(r: f32, g: f32, b: f32, a: f32) {
        unsafe { c_interface::text_color(r, g, b, a) }
    }

    pub fn draw_text(str: &str, x: f32, y: f32) {
        unsafe { c_interface::draw_text(str.as_ptr(), str.len() as u32, x, y) }
    }

    pub fn is_pause_screen() -> bool {
        unsafe { c_interface::is_pause_screen() }
    }

    pub fn warp(world: u32, area: u32) {
        unsafe { c_interface::warp(world, area) }
    }

    pub fn input_up() -> bool {
        unsafe { c_interface::pad_d_d_up(0) || c_interface::pad_d_l_a_up(0) }
    }

    pub fn input_up_fast() -> bool {
        unsafe { c_interface::pad_d_l(0) || c_interface::pad_d_l_a_left(0) }
    }

    pub fn input_down() -> bool {
        unsafe { c_interface::pad_d_d_down(0) || c_interface::pad_d_l_a_down(0) }
    }

    pub fn input_down_fast() -> bool {
        unsafe { c_interface::pad_d_r(0) || c_interface::pad_d_l_a_right(0) }
    }

    pub fn input_pressed_start() -> bool {
        unsafe { c_interface::pad_start(0) }
    }

    pub fn input_pressed_ok() -> bool {
        unsafe { c_interface::pad_p_a(0) }
    }

    pub fn input_pressed_back() -> bool {
        unsafe { c_interface::pad_p_b(0) }
    }
}
