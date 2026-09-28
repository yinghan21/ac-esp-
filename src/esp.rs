use imgui::Ui;

use crate::config::Config;
use crate::data::Snapshot;
use crate::memory;

const BOX_COLOR: [f32; 4] = [1.0,1.0,1.0,1.0];
const LINE_COLOR: [f32; 4] = [1.0,1.0,1.0,1.0];

pub fn draw(ui: &Ui, config: & Config, snapshot: &Snapshot) {
    if !config.esp_enabled || !snapshot.valid {
        return;
    }

    let Some((screen_w, screen_h)) = memory::game_client_size() else {
        return;
    };

    let line_origin = [screen_w * 0.5, -0.1];

    for p in &snapshot.players {
        if !p.alive {
            continue;
        }

        let Some(head) = memory::world_to_screen(p.head, &snapshot.view_matrix, screen_w, screen_h) else {
            continue;
        };
        let Some(feet) = memory::world_to_screen(p.feet, &snapshot.view_matrix, screen_w, screen_h) else {
            continue;
        };

        let height = (feet.y - head.y).abs();
        if height < 2.0 {
            continue;
        }
        let width = height * 0.45;
        //矩形左上角
        let box_min = [head.x - width * 0.5, head.y.min(feet.y)];
        //矩形右下角
        let box_max = [head.x + width * 0.5, head.y.max(feet.y)];

        /// 引导线
        if config.draw_snapline {
            ui.get_foreground_draw_list()
                .add_line(line_origin, head.to_array(), LINE_COLOR)
                .thickness(1.0)
                .build()
        }
        /// 方框/矩形
        if config.draw_box {
            ui.get_foreground_draw_list()
                .add_rect(box_min, box_max, BOX_COLOR)
                .thickness(1.0)
                .build()
        }
        /// 血量
        if config.draw_health_bar {
            if let Some(hp) = p.health {
                if hp > 0 {
                    let bar_x = box_min[0] - 4.0 -3.0;
                    let bar_top = box_min[1];
                    let bar_h = box_max[1] - bar_top;
                    let ratio = (hp as f32 / 100.0).clamp(0.0, 1.0);
                    let filled = bar_h * ratio;

                    ui.get_foreground_draw_list()
                        .add_rect(
                            [bar_x, bar_top],
                            [bar_x + 4.0, box_max[1]],
                            [0.0,0.0,0.0,0.6],
                        )
                        .filled(true)
                        .build();

                    ui.get_foreground_draw_list()
                        .add_rect(
                            [bar_x, box_max[1] - filled],
                            [bar_x + 4.0, box_max[1]],
                            [1.0 - ratio, ratio, 0.0 , 1.0],
                        )
                        .filled(true)
                        .build();
                }
            }
        }
    }
}
