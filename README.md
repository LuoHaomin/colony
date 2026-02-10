# Colony - 3D Autonomous Idle Simulator

Colony 是一款基于 Bevy 0.18 开发的“矮人要塞”风格 2D/3D 混合架构挂机模拟游戏。

## 核心特性

- **Z-Level 分层世界**: 真正的三维坐标系，支持多层地形显示与切换。
- **深度视觉效果**: 观察视角上方的层自动隐藏，下方的层产生阴影暗淡效果。
- **完全自主的 AI**: 单位根据饥饿、睡眠、娱乐等动机自主决策，不再需要频繁手动干预。
- **现代化 Bevy 引擎**: 迁移至 Bevy 0.18，利用高性能 ECS 驱动大量实体的自主行为。

## 快速开始

### 运行环境
- Rust (最新稳定版)
- 支持 Vulkan/Metal/DX12 的显卡驱动

### 启动命令
```bash
cargo run --release
```

## 操作指南

- **Z轴切换 (高度观察)**:
    - `Q` 或 `,` (逗号): 升高观察面
    - `E` 或 `.` (句号): 降低观察面
- **游戏控制**:
    - `空格 (Space)`: 暂停/恢复模拟
    - `Esc`: 退出游戏
- **观察**:
    - 你可以观察单位头顶的文字，实时了解其当前状态（如 `[Eat]`, `[Sleep]`, `[Thinking...]`）。

## 项目结构 (开发者指南)

### 文档
更详细的系统设计请参考 `docs/docs/` 目录：
- `vision.md`: 核心设计愿景
- `architecture.md`: 技术细节与坐标映射逻辑
- `rendering.md`: 2D-in-3D 渲染与可见性系统
- `simulation.md`: 需求、动机与任务决策循环
- `world.md`: 地形生成算法

### 核心模块
- `src/main.rs`: 插件注册与系统调度中心。
- `src/core/components.rs`: `Position`, `Brain`, `PhysicalBody` 等核心数据定义。
- `src/rendering/visibility_system.rs`: Z-Level 切片效果的实现核心。
- `src/simulation/thinking_system.rs`: 决策逻辑的“大脑”。

## 未来规划 (TODO)
1. **多层寻路**: 允许单位通过斜坡或梯子在 Z 轴间移动。
2. **建筑系统**: 支持玩家在不同高度建设多层建筑。
3. **更丰富的 UI**: 增强侧边栏信息显示，点击单位查看详细属性。
4. **资源保存与加载**: 序列化 `TileHash` 与实体状态。

## 许可
本项目采用 MIT / Apache 2.0 双重许可。
贴图资源来自 [Dungeon Crawl](https://opengameart.org/content/dungeon-crawl-32x32-tiles) (CC-0)。

