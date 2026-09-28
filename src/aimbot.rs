use std::sync::atomic::{AtomicI32, Ordering};

use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;

use crate::config::Config;
use crate::data::Snapshot;
use crate::memory::{self, Vec2};

pub fn run (config: &Config, snapshot: &Snapshot) {
    if !config.aim_enabled || !snapshot.valid {
        return;
    }

    if !right_mouse_down() {
        return;
    }
    let Some((screen_w, screen_h)) = memory::game_client_size() else {
        return;
    };
    let center = Vec2::new(screen_w * 0.5, screen_h * 0.5);
    let fov_sq = config.aim_fov * config.aim_fov;
    
    let mut best: Option<(f32, Vec2)> = None;

    for p in &snapshot.players {
        if !p.alive {
            continue;
        }

        let Some(sq) = memory::world_to_screen(p.head, &snapshot.view_matrix, screen_w, screen_h) else {
            continue;
        };

        let dx = sq.x - center.x;
        let dy = sq.y - center.y;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq > fov_sq {
            continue;
        }
        if best.map_or(true, |(d, _)| dist_sq < d) {
            best = Some((dist_sq, Vec2::new(dx, dy)));
        }
    }
    let Some((_, delte)) = best else {
        return;
    };

    let dx = delte.x * config.aim_speed / config.aim_smooth.max(1.0);
    let dy = delte.y * config.aim_speed / config.aim_smooth.max(1.0);

    if  dx.abs() < 0.5 && dy.abs() < 0.5 {
        return;
    }

    send_move(dx, dy);
}

fn right_mouse_down() -> bool {
    unsafe{(GetAsyncKeyState(VK_RBUTTON as i32) as u16 & 0x8000) != 0}
}

static RESIDUAL_X: AtomicI32 = AtomicI32::new(0);
static RESIDUAL_Y: AtomicI32 = AtomicI32::new(0);

fn send_move(dx: f32, dy: f32) {
    let want_x = (dx * 100.0) as i32 + RESIDUAL_X.swap(0, Ordering::Relaxed);
    let want_y = (dy * 100.0) as i32 + RESIDUAL_Y.swap(0, Ordering::Relaxed);

    let move_x = want_x / 100;
    let move_y = want_y / 100;
    RESIDUAL_X.store(want_x - move_x * 100, Ordering::Relaxed);
    RESIDUAL_Y.store(want_y - move_y * 100, Ordering::Relaxed);

    if move_x == 0 && move_y == 0 {
        return;
    }

    let mut input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: move_x,
                dy: move_y,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_MOVE,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    let _ = unsafe {SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32)};
}