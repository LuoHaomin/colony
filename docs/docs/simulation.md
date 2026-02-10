# Simulation

## 自主决策循环 (Autonomous Decision Loop)

### 1. 需求感官 (`needs.rs`)
- 单位拥有 `PhysicalBody` 组件。
- 包含 `needs_food`, `needs_sleep`, `needs_entertainment`。
- 每秒钟需求会按 `rate` 扣减。

### 2. 性格与动机 (`thinking_system.rs`)
- 当某项需求低于 `low` 阈值时，`Motivation` 被触发。
- `Motivation` 被转化为 `Task`。

### 3. 任务执行 (`task_system/`)
- 每个 `Task` 对应一个独立的系统。
- **Eat**: 寻找食物并移动过去。
- **Sleep**: 寻找床或空地休息。
- **Meander**: 随机闲逛（休闲）。
- **Chop/Forage**: 执行玩家下达的资源采集指令。

## 空间行为
- **高度适应性**: 单位在移动和生成时，会通过 `TileHash` 探测当前坐标下最高的非墙砖体（Surface）。
- **Z轴移动限制**: 当前寻路主要在单层 Z 轴内进行，未来可扩展爬楼梯/斜坡逻辑。
