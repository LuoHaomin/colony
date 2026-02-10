# Architecture

## 技术栈 (Tech Stack)
- **引擎**: [Bevy 0.18](https://bevyengine.org/) (当前最前沿版本)
- **语言**: Rust 2021 Edition
- **渲染**: 2D Sprite + 3D Transform (Z-Level Slicing)

## 核心架构组件 (Core Components)

### 1. 坐标系统 (`Position`)
- 使用自定义的 `Position { x, y, z }` 组件表示物理位置。
- `x, y` 是磁贴网格坐标，`z` 是层高度。
- 逻辑处理（如寻路、碰撞）使用 `Position`。
- 表现层通过 `position.to_transform()` 映射到 Bevy 的 `Transform`。

### 2. Z-Level 切片显示 (`CurrentDisplayZ`)
- 一个全局资源 `CurrentDisplayZ { z: i32 }`。
- 控制当前相机焦点和可见性算法。
- **Visibility System**: 
    - `pos.z > current_z`: 隐藏 (Visibility::Hidden)。
    - `pos.z == current_z`: 正常显示 (Visibility::Visible)。
    - `pos.z < current_z`: 按深度衰减颜色 (Dimming)，模拟深度感。

### 3. 地图存储 (`TileHash`)
- 使用 `HashMap<Position, TileType>` 存储地形数据。
- 支持快速的空间查询（例如：判断某个位置是否是墙）。

### 4. 插件化系统 (Plugins)
- 逻辑按功能拆分为不同模块：
    - `initializations`: 处理加载、启动、地图生成。
    - `simulation`: 处理 AI、需求、任务、生物行为。
    - `rendering`: 处理摄像机、界面、可见性逻辑。
