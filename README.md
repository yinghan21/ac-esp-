# AC ESP

一个使用 Rust 编写的 AssaultCube 图形叠加层实验项目。项目通过 DLL 形式加载到游戏进程中，使用 Hudhook 和 Dear ImGui 绘制调试菜单与屏幕信息，并从游戏内存中读取实体数据进行可视化。

> **免责声明**
>
> 本项目仅用于 Rust、Windows API、OpenGL Hook、进程内存读取和 ImGui Overlay 等技术的学习与研究。请只在你拥有并明确获准测试的本地环境中使用，并遵守游戏软件的许可协议、服务器规则及相关法律法规。项目作者不对违规使用、账号处罚或其他损失负责。

## 功能

- ImGui 可视化配置菜单
- ESP：
  - 实体方框
  - 目标引导线
  - 生命值条
- 基于屏幕距离和 FOV 的辅助瞄准实验功能
- 右键按住时启用辅助瞄准
- Insert 键切换配置菜单
- 对内存区域、指针和坐标进行基础有效性检查
- 后台线程持续读取实体快照，渲染线程只消费快照数据

## 技术栈

- [Rust](https://www.rust-lang.org/)
- [hudhook](https://github.com/veeenu/hudhook)
- [imgui-rs](https://github.com/imgui-rs/imgui-rs)
- [windows-sys](https://crates.io/crates/windows-sys)
- Windows x86 / MSVC

## 项目结构

```text
src/
├── aimbot.rs  # 辅助瞄准逻辑与鼠标输入
├── config.rs  # 运行时配置
├── data.rs    # 实体数据读取与快照
├── esp.rs     # ESP 绘制
├── lib.rs     # DLL 入口、Hook 和 ImGui 菜单
├── memory.rs  # 内存读取、窗口尺寸和坐标转换
└── offset.rs  # 游戏版本相关偏移
```

## 环境要求

- Windows 10/11
- Rust toolchain（建议使用 `stable`）
- Visual Studio Build Tools 或 Visual Studio
- **x86 MSVC** 编译工具链
- 与代码中偏移匹配的 AssaultCube 版本

检查 Rust 工具链：

```powershell
rustup show
rustup target list --installed
```

如果尚未安装 x86 MSVC 目标：

```powershell
rustup target add i686-pc-windows-msvc
```

## 构建

在项目根目录执行：

```powershell
cargo build --release --target i686-pc-windows-msvc
```

构建完成后，DLL 位于：

```text
target/i686-pc-windows-msvc/release/ac_esp.dll
```

调试构建：

```powershell
cargo build --target i686-pc-windows-msvc
```

## 使用说明

1. 启动与项目版本匹配的本地测试环境。
2. 按照你所在环境允许的方式加载构建得到的 DLL。
3. 使用 **Insert** 键打开或关闭菜单。
4. 在菜单中按需开启 ESP 功能并调整辅助瞄准参数。
5. 退出测试后卸载 DLL 或关闭游戏进程。

默认配置：

| 配置项 | 默认值 |
| --- | --- |
| ESP | 开启 |
| 方框 | 开启 |
| 引导线 | 开启 |
| 生命值条 | 开启 |
| 辅助瞄准 | 关闭 |
| FOV | 150 |
| 平滑 | 2 |
| 速度倍率 | 1 |

## 兼容性与偏移

`src/offset.rs` 中的地址是针对特定游戏版本的硬编码偏移。游戏更新、编译版本变化或模块布局改变后，偏移可能失效，表现为：

- Overlay 无法显示；
- 实体数据为空或异常；
- 坐标投影错误；
- 进程崩溃。

如需适配其他版本，应在隔离的本地测试环境中重新确认视觉矩阵、实体列表、坐标和生命值字段的布局，并同步更新相关数据校验逻辑。

## 开发与检查

格式化代码：

```powershell
cargo fmt
```

运行 Clippy：

```powershell
cargo clippy --target i686-pc-windows-msvc --all-targets --all-features
```

## 已知限制

- 当前配置仅在运行时生效，尚未提供配置文件持久化。
- 偏移地址与目标游戏版本强绑定。
- 项目依赖 Windows 和 OpenGL Hook，不支持 Linux/macOS。
- 目前没有自动化的游戏进程集成测试。

## 贡献

欢迎提交 Issue 或 Pull Request 来改进代码质量、内存安全检查、版本适配和文档。提交前请确保：

```powershell
cargo fmt -- --check
cargo check --target i686-pc-windows-msvc
```

请不要提交游戏文件、账号信息、私有偏移数据或任何敏感信息。

## 许可证

当前仓库尚未声明正式开源许可证。除非仓库后续补充许可证，否则请勿将本项目代码用于再分发或商业用途。
