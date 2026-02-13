# 开发计划 Phase 1.2.0: 原子进化——属性矩阵与原子动作

## 1. 目标 (Goals)
将游戏逻辑从“硬编码类型”转向“基于属性的仿真”。通过去枚举化实现万物属性化，并引入原子化动作作为生物行为的基础。

- **属性化**: 弱化 `ItemType`/`ActorType`，强化 `MaterialProperties`。
- **原子动作**: 引入 `AtomicAction` 替代复杂的 `Task`。
- **交互公理**: 实现基于硬度、质量和能量的交互判定逻辑。

## 2. 关键任务 (Key Tasks)

### T1: 材质属性系统升级 (Material Overhaul)
- [x] **扩展组件**: 在 `MaterialProperties` 中增加 `Toughness` (韧性) 和 `Conductivity` (传导性)。
- [x] **映射逻辑**: 在 `ItemType` 中实现了 `material_properties()` 映射。
- [x] **环境感应**: 在 `PhysicalBody` (实际在 `Genome`) 中引入了 `thermal_tolerance` 基因位。

### T2: 原子动作基础设施 (Atomic Action Infrastructure)
- [x] **定义枚举**: 定义了 `AtomicAction`。
- [x] **实现动作处理器**: 创建了 `ActionProcessorSystem`。

### T3: 需求驱使的效用评估 (Utility AI Preview)
- [x] **欲望向量化**: 通过 `Genome` 权重实现了动机打分。
- [x] **效用评分公式**: 实现了 $Score = f(Energy, Distance, Weight)$。
- [ ] **记忆增强**: 扩展 `Memory` 使其能存储更多物理细节。

### T4: Z-Level 渲染与移动 (3D Layering)
- [x] **相机与层级**: 实现了 `CurrentDisplayZ` 驱动的可见性剔除。
- [x] **垂直移动**: `Move` 原子动作支持 Z 轴移动。

## 3. 技术调整
- **废弃**: `Task` 枚举（逐步替换为 `AtomicAction` 序列）。
- **新增**: `ActionQueue` 组件，用于存放原子动作序列。
- **修改**: `src/simulation/thinking_system.rs` 核心逻辑。

## 4. 预期产物
- 生物不再仅仅是因为“肚子饿了”去执行 `Task::Eat`，而是因为它的“能量密度感知权重”极高，导致它选择 `Move` 到一个具有 `High Energy` 属性的实体旁并执行 `Consume`。
