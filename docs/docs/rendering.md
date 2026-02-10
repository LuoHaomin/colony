# Rendering

## 2D-in-3D 表现 (2D-in-3D Presentation)
项目采用了一种“伪 3D”或“分层 2D”的渲染方式：
- **Camera2d**: 使用 2D 正交相机，避免透视畸变。
- **Layered Assets**: 地图块和单位均使用 `SpriteSheet` 中的 Tile。
- **Z-Ordering**: 通过 `Transform.translation.z` 和可见性管理，将多层世界投影到屏幕上。

## 可见性系统 (Visibility Logic)
### 系统实现：`src/rendering/visibility_system.rs`
- **当前层 (Active Layer)**: `sprite.color = Color::WHITE`。
- **下方层 (Below Layers)**: `Color::srgb(dim, dim, dim)`，其中 `dim` 随距离 Z 偏移量线性降低。
- **上方层 (Above Layers)**: 完全不可见。

## 材质与贴图 (Assets)
- **AllSprites.png**: 统一的 32x32 磁贴资源。
- **TextureAtlasLayout**: 在初始化阶段（`load_sprites`）定义，统一由 `SpriteSheet` 资源持有引用。
- **单位映射**: 每个 `ActorType` 通过 `sprite_index()` 返回其在图集中的偏移。
