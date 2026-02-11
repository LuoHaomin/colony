# 开发计划 Phase 1.1.1: UI 现代化与 AI 逻辑重构

## 1. 目标 (Goals)
- **视觉去杂**: 规范化 UI 渲染，移除散落在各处的硬编码文本。
- **UI 响应化**: 建立标准的 HUD 框架（顶部资源栏、左侧状态栏、右侧详情页）。
- **AI 框架升级**: 实现基于权重的动机评估系统 (Motivation Weighting System) 与 行动队列 (Action Queue)。
- **交互增强**: 优化 Z 轴视图下的选中逻辑，支持点击物体弹出 Inspector。

## 2. 关键任务 (Key Tasks)

### T1: UI 框架搭建 (The HUD Foundation)
- [ ] **全屏布局**: 使用 `bevy_ui` 建立基础布局容器。
    - **Top Bar**: 显示周期、日期、核心资源 (Energy/Materials)。
    - **Bottom Bar**: 政策/指令快捷键。
    - **Side Panel (Inspector)**: 延迟加载选中单位的详细数据。
- [ ] **单位头顶 UI**: 移除 `Text2dBundle` 手动更新，改用统一的 `WorldUI` 系统（支持按键切换开启/关闭）。

### T2: AI 决策模型重构 (Decision Engine)
- [ ] **动机计算**: 
    - 废弃 `ThinkingSystem` 中的简单 `if-else`。
    - 引入 `Motivation` 结构体，根据 `Need` 的盈亏比计算动态权重。
- [ ] **行动序列 (Action Queue)**:
    - 支持复合动作。例如：`Eat` 任务由 `[FindPathToFood, MoveTo, PlayAnim(Eat), RemoveFood]` 组成。
- [ ] **性格增益 (Traits)**: 
    - 为单位增加 `SpeedFactor`, `HungerRate` 等性格影响因子。

### T3: 交互逻辑优化 (Interaction)
- [ ] **射线检测修正**: 确保在 2D 相机 + 3D 坐标下，鼠标点击能准确映射到当前 $Z$ 层及以下的 2D 实体。
- [ ] **层级切换平滑化**: 增加层级切换时的视觉反馈（如边缘闪烁或数值显示）。

## 3. 技术栈建议
- **UI**: 优先使用 Bevy 原生 `Node` 系统。
- **AI**: 状态机 + 权重池。

## 4. 进度表
- **Day 1-2**: 完成 HUD 框架与响应式布局。
- **Day 3-4**: 重构 `Brain` 组件与动机评估逻辑。
- **Day 5**: 联调单位详情页与交互系统。
