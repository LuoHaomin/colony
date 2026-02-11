# 开发计划 Phase 1.1.2: 演化基石——去耦合与代谢基础

## 1. 目标 (Goals)
本阶段是迈向“涌现式演变”的关键转折点。我们将数据结构从静态枚举（Enumerations）转向动态属性（Attributes），并建立驱动进化的“能量熵减”闭环。

- **去枚举化重构**: 将硬编码的生物/资源模板拆解为物理属性组件。
- **基因组 (Genome) 初步实现**: 实现基础的遗传参数序列。
- **代谢循环 (Metabolism)**: 建立基于基因的能量消耗与死亡判定系统。
- **环境场 (Environment Fields) 框架**: 实现首个动态场——“温度与矿物养分场”。

## 2. 关键任务 (Key Tasks)

### T1: 核心数据结构的“属性化” (Attribute-Driven Refactor)
- [ ] **重构 `Position` 与 `Grid`**: 
    - 磁贴不仅是类型，而是 `EnvironmentalData` 的载体（温度、湿度、肥沃度）。
- [ ] **引入 `MaterialProperties`**: 
    - 物品（木石肉）不再只靠 `ItemType` 区分，而是具备 `Hardness`, `Mass`, `EnergyValue` 属性。
- [ ] **重构 `PhysicalBody`**:
    - 将固定的 `needs_food` 等需求，转化为统一的 `EnergyStorage` (能量池)。

### T2: 基因组 (Genome) 系统搭建
- [ ] **定义 `Genome` 组件**:
    - 物理基因：`Size`, `Mobility`, `SensoryRange`。
    - 代谢基因：`MetabolicEfficiency`, `DietType` (0=光合, 1=肉食)。
- [ ] **视觉映射系统**:
    - 根据基因参数（如 `Size`）自动调整 `Sprite` 的 `Scale` 和 `Color` 混色。

### T3: 代谢引擎实现 (Metabolic Engine)
- [ ] **能量熵减系统 (`MetabolicDrainSystem`)**:
    - `Cost = f(Size, Mobility, BrainComplexity) * TimeTick`。
- [ ] **能量获取逻辑**:
    - **光合路径**: 处于 `Fertility > 0` 的磁贴且有光照时，自动转化能量。
    - **进食路径**: 消费具备 `EnergyValue` 的实体，效率受 `MetabolicEfficiency` 基因调节。
- [ ] **生死判定**: 能量归零时触发 `Death` 逻辑，实体转化为具备“高肥沃养分”的尸体资源。

### T4: 基础环境场模拟 (Early Environment)
- [ ] **静态场生成**: 利用噪声生成初始温度、肥沃度分布图。
- [ ] **环境压力**: 极端温度将根据基因匹配度产生额外的“能量惩罚”。

## 3. 架构调整
- **新增模块**: `src/core/genome.rs`, `src/simulation/environment_system.rs`。
- **调度更新**: 在 `FixedUpdate` 中引入 **代谢 Tick (2Hz)**。

## 4. 进度预期
- **第一阶段 (重构)**: 完成 `MaterialProperties` 与 `EnvironmentalData` 的定义与接入。
- **第二阶段 (基因)**: 生物实体能根据 `Genome` 初始化其外观与基础消耗。
- **第三阶段 (闭环)**: 实现在不干预的情况下，生物因为代谢导致能量波动，最终产生“饿死”或“生长”的反馈。
