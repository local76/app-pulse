//! Mock/Stub fallback backend for non-Windows platforms.
//!
//! **Taxonomy Classification**: Abstraction (Platform Abstraction Layer).

#[allow(unused_imports)]
pub use library::toolkit::clipboard::copy_text_to_clipboard;
#[allow(unused_imports)]
pub use library::apps::identity::{hostname, os_str, user_host, username};
#[allow(unused_imports)]
pub use library::apps::window::{
    center_console_window, get_window_rect, query_cursor_pos, set_window_pos,
    BorderlessConsole, ConsoleTitleGuard, SingleInstanceGuard,
};
#[allow(unused_imports)]
pub use library::toolkit::monitors::get_all_monitors;
#[allow(unused_imports)]
pub use library::toolkit::sys_info::{
    query_bios_info, query_dark_mode as is_dark_mode, query_disk_drives, query_gpu_names,
    query_network_adapters, query_os_version, query_power_status, query_shell_and_terminal,
};

pub fn get_win_accent_color() -> String {
    "#00F5FF".to_string()
}
