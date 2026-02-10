# World

## 地形生成 (Terrain Generation)
### 算法：正弦波噪声 (Sine Wave Noise)
- 地理高度通过以下公式生成：
  ```rust
  let hill_height = ((x as f32 * 0.1).sin() * (y as f32 * 0.1).cos() * 2.0).round() as i32;
  ```
- 该算法创造了起伏平缓的山丘和低谷。

## 层分布 (Z-Level Distribution)
- **Z < hill_height**: 实心土层 (`TileType::Dirt`)。
- **Z == hill_height**: 草原或表层 (`TileType::Grass`)。
- **Z > hill_height**: 虚空 (`TileType::Void`)。
- **地图边界**: 自动生成深度 3 层的围墙 (`TileType::WallGame`) 防止单位走失。

## 生物群落 (Biome)
- `Biome` 资源控制了地表地块的种类权重、植物密度及初始产物。
