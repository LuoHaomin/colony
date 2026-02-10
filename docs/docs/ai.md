# AI System

## 决策模型 (Decision Model)
项目的 AI 采用一种基于动机的层次化决策结构：
1. **感官层 (Sensing)**: 实时监测 `PhysicalBody` 中的需求数值。
2. **动机层 (Motivation)**: 当数值低于临界点，产生 `Hunger`, `Tired` 或 `Bored` 动机。
3. **计划层 (Planning)**: 将动机转化为具体的 `Task`。
4. **行动层 (Action)**: 调用各任务系统（如 `task_system_eat`）进行具体的环境交互和寻路。

## 寻路算法 (Pathfinding)
- **A* 寻路**: 实现在 `src/simulation/movetoward_system.rs` 中。
- **障碍物感知**: 寻路算法会实时查询 `TileHash` 和 `Object` 位置，避开墙壁和实体。
- **三维扩展**: 目前寻路锁定在单层 Z 轴，未来将支持多层垂直导向。
