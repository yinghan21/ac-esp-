pub struct Config {
    /// 菜单
    pub menu_open: bool,
    /// esp 开关
    pub esp_enabled: bool,
    /// 画框
    pub draw_box: bool,
    /// 画引导线
    pub draw_snapline: bool,
    /// 画血条
    pub draw_health_bar: bool,
    /// 自瞄开关
    pub aim_enabled: bool,
    /// 自瞄 FOV 半径
    pub aim_fov: f32,
    /// 平滑
    pub aim_smooth: f32,
    /// 速度倍率 (对应游戏灵敏度)
    pub aim_speed: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self{
            /// 菜单
            menu_open: true,
            /// esp 开关
            esp_enabled: true,
            /// 画框
            draw_box: true,
            /// 画引导线
            draw_snapline: true,
            /// 画血条
            draw_health_bar: true,
            /// 自瞄开关
            aim_enabled: false,
            /// 自瞄 FOV 半径
            aim_fov: 150.0,
            /// 平滑
            aim_smooth: 2.0,
            /// 速度倍率 (对应游戏灵敏度)
            aim_speed: 1.0,
        }
    }
}