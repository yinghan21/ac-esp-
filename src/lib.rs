mod config;
mod data;
mod esp;
mod offset;
mod memory;
mod aimbot;

use std::ffi::c_void;
use std::sync::{Arc, OnceLock, RwLock};

use hudhook::hooks::opengl3::ImguiOpenGl3Hooks;
use hudhook::*;
use imgui::*;
use windows_sys::Win32::System::SystemServices::*;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_INSERT};
use windows_sys::Win32::UI::WindowsAndMessaging::ClipCursor;

use crate::config::Config;
use crate::data::Snapshot;

struct EspLoop {
    config: Config,
    snapshot: Arc<RwLock<Snapshot>>,
    prew_insert_down: bool,
}

impl EspLoop {
    fn poll_menu_key(&mut self) {
        let down = unsafe { (GetAsyncKeyState(VK_INSERT as i32) as u16 & 0x8000) != 0};
        if down && !self.prew_insert_down {
            self.config.menu_open = !self.config.menu_open;
        }
        self.prew_insert_down = down;
    }
}

fn draw_menu(ui: &Ui, config: &mut Config) {
    if !config.menu_open {
        return;
    }
    ui.window("AC ESP --dong")
        .size([260.0,300.0], Condition::FirstUseEver)
        .position([20.0, 20.0], Condition::FirstUseEver)
        .build(|| {
            ui.checkbox("Enable ESP", &mut config.esp_enabled);
            ui.checkbox("Enable BOX", &mut config.draw_box);
            ui.checkbox("Enable LINE", &mut config.draw_snapline);
            ui.checkbox("Enable HEALTH", &mut config.draw_health_bar);

            ui.separator();

            ui.checkbox("Enable auto-aim", &mut config.aim_enabled);
            if config.aim_enabled {
                ui.slider("FOV", 20.0, 600.0, &mut config.aim_fov);
                ui.slider("Smooth", 1.0, 10.0, &mut config.aim_smooth);
                ui.slider("speed", 0.2, 3.0,  &mut config.aim_speed);
            }

            ui.separator();
            ui.text_disabled("Hold right-click for auto-aim, Insert to toggle menu");
        });
}

impl ImguiRenderLoop for EspLoop {
    fn before_render<'a>(&'a mut self, ctx: &mut Context, _rc: &'a mut dyn RenderContext) {
        let io = ctx.io_mut();
        
        io.mouse_draw_cursor = self.config.menu_open;
        if let Some((w,h)) = memory::game_client_size() {
            io.display_size = [w, h];
        }

        if self.config.menu_open {
            let _ = unsafe{ ClipCursor(std::ptr::null())};
        }
    }

    fn render(&mut self, ui: &mut Ui) {
        self.poll_menu_key();
        
        let snapshot = match self.snapshot.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        esp::draw(ui, &self.config, &snapshot);
        aimbot::run(&self.config, &snapshot);
        draw_menu(ui, &mut self.config);
    }

    fn message_filter(&self, io: &Io) ->  MessageFilter {
        if !self.config.menu_open {
            return MessageFilter::empty();
        }
        let mut filter = MessageFilter::empty();

        if io.want_capture_mouse {
            filter |= MessageFilter::InputMouse | MessageFilter::InputRaw;
        }
        if io.want_capture_keyboard {
            filter |= MessageFilter::InputKeyboard;
        }
        filter
    }
}

fn run(hmodule: *mut c_void) -> anyhow::Result<()> {
    let module_base = memory::module_base().ok_or_else(|| anyhow::anyhow!("GetModuleHandleA 失败"))?;
    let snapshot = Arc::new(RwLock::new(Snapshot::default()));
    data::spawn(module_base, Arc::clone(&snapshot));

    let render_loop = EspLoop {
        config: Config::default(),
        snapshot,
        prew_insert_down: false,
    };
    Hudhook::builder()
        .with::<ImguiOpenGl3Hooks>(render_loop)
        .with_hmodule(hudhook::windows::Win32::Foundation::HINSTANCE(hmodule))
        .build()
        .apply()
        .map_err(|e| anyhow::anyhow!("hudhook 挂钩失败: {e:?}"))?;

    Ok(())
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn DllMain(module: *mut c_void, call_reason: u32, _: *mut c_void) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        let module_raw = module as usize;
        std::thread::spawn(move || {
            if let Err(err) = run(module_raw as *mut c_void) {
                eprintln!("[ac-esp] 初始化失败: {err:?}");
            }
        });
    }
    true
}