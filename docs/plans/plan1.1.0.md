# 🧭 Colony 改造计划（Idle Colony Simulator）

目标：
将开源项目 Colony 改造为 **三维空间 + 自动行为 + 观察驱动乐趣** 的挂机模拟游戏
（RimWorld × Dwarf Fortress × Idle Game）

核心体验：

> 玩家不直接操作单位
> 玩家制定规则 → 世界自动运转 → 观察 emergent gameplay

---

# 一、总体阶段划分（重要）

## Phase 0 — 可编译与稳定运行、架构重构（最关键阶段）


👉 目标：能跑、能调试、结构清晰
**不加新玩法**

### 任务

1. 修复编译问题（Bevy 0.18）
2. 移除弃用 API
3. 拆分模块结构
4. 建立日志 & Debug UI

!!!!!!=最新的 Bevy 版本API调用可参考 /bevy_example=!!!!!!

### 成功标准

* `cargo run` 无 warning（尽量）
* 热重载可用
* 能稳定运行 10 分钟

👉 目标：把“Demo 项目”变成“可维护引擎”

当前 Colony 的问题：

* 系统职责混乱
* Game logic 与 rendering 耦合
* 没有清晰 simulation tick
* AI 是行为触发而不是决策模型

### 新架构（目标）

```
App
 ├── Engine Layer
 │    ├── Time & Tick
 │    ├── Event Bus
 │    ├── Save/Load
 │
 ├── Simulation Layer   ← 游戏核心
 │    ├── World Grid (3D)
 │    ├── Entities (colonists, animals)
 │    ├── Needs System
 │    ├── Job System
 │    ├── Economy System
 │
 ├── Presentation Layer
 │    ├── Rendering
 │    ├── Animation
 │    ├── Camera
 │
 └── Player Layer
      ├── Policies
      ├── Orders
      ├── Priorities
```

### 要做的重构

#### 1. 引入固定 Simulation Tick

游戏不再跟帧率绑定：

```
Render FPS: 60+
Simulation: 4 ticks/sec
```

这一步极其关键
👉 否则挂机玩法必炸

---

#### 2. 事件驱动替代直接调用

旧：

```
miner.mine(tile)
```

新：

```
Event::JobCompleted(Job::Mine(tile))
```

所有系统通过事件通信
→ 解耦
→ 可扩展
→ 可录制 replay

---

#### 3. AI 从“动作触发”改为“决策循环”

每个小人运行：

```
Perception → Needs → Goal → Plan → Execute
```

不是：

```
看见矿 → 挖矿
```

---

### 成功标准

* 单位行为不依赖 UI
* 可以关掉渲染仍运行模拟
* 可加速到 20x 仍稳定

---

## Phase 1 — 三维世界（DF风格Z轴）

👉 只扩展空间，不改玩法

### 设计原则

不要做体素渲染
先做 **3D逻辑 + 2.5D显示**

```
(x, y, z) grid
但只渲染当前层
```

### 新世界结构

```
Chunk
 ├── Tile[x][y][z]
 │    ├── Terrain
 │    ├── Material
 │    ├── Walkable
 │    ├── Light
```

### 要解决的关键问题

1. 跨层寻路（A* 3D）
2. 楼梯连接
3. 掉落物理（重力）
4. 气体/水流（后期）

---

### 成功标准

* 单位能上下楼
* 可挖地下矿洞
* 不崩性能

---

## Phase 2 — 挂机/模拟玩法（核心玩法）

👉 从“建造游戏”变为“观察游戏”

玩家输入 ↓
系统运转 ↑

---

## 新玩法核心：Policy Game（政策游戏）

玩家不控制单位
玩家设置规则

例子：

| 玩家做的事   | 游戏发生的事 |
| ------- | ------ |
| 设置食物优先级 | 自动种田   |
| 设置工作时间  | 小人作息改变 |
| 提高安全等级  | 建造防御   |
| 设立贸易政策  | 经济变化   |

---

### 核心系统

## 1️⃣ Needs System（需求驱动世界）

每个单位有：

```
Hunger
Rest
Safety
Social
Temperature
Purpose
```

行为来自需求
不是任务列表

---

## 2️⃣ Job Market（任务市场）

世界生成工作
单位选择工作

不是：

> 玩家指派任务

而是：

> 世界提供机会

---

## 3️⃣ Economy（经济系统）

资源自动流动：

```
需求 → 生产 → 消耗 → 短缺 → 行为改变
```

玩家只调控：

* 税率
* 配给
* 工作制度

---

## 4️⃣ Emergent Events（涌现事件）

不写剧本
写规则

例：

```
寒冷 + 饥饿 + 黑暗 → 恐慌
恐慌 → 斗殴
斗殴 → 受伤 → 医疗压力
```

---

### 成功标准

玩家 5 分钟无操作仍有趣
（这是挂机游戏的生死线）

---

## Phase 3 — 长期系统（后期）

* 个性人格
* 文化
* 信仰
* 历史记录
* 自动故事生成

---

# 技术补充建议

## 保存系统（必须尽早做）

挂机游戏必须支持：

```
离线模拟
时间跳跃
Replay
```

否则后期无法补救

---

## 调试工具（极其重要）

必须实现：

* 查看AI决策
* 查看需求数值
* 查看路径
* 查看事件

