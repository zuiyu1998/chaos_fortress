# SkillDefinitionLoader

`SkillDefinitionLoader` 是一个资产加载器，实现了 Bevy 的 [`AssetLoader`] trait，用于从 TOML 文件加载 [`SkillDefinition`] 资产。

## 用途

- 从 `.skill.toml` 文件中读取技能模板定义，解析为 [`SkillDefinition`] 资产。
- 支持通过 Bevy 的资产系统加载技能数据，无需在代码中硬编码技能参数。
- 参考 [`AttributeTemplateLoader`] 的设计模式。

## TOML 格式

每个技能定义文件包含一个 `[skill]` 表，以及可选的多个 `[[features]]` 条目。

```toml
[skill]
id = "archer_shot"
name = "Archer Shot"

[[features]]
id = "cooldown"
cooldown_duration = 1.0

[[features]]
id = "damage"
base_damage = 50.0
ratio = 1.5
crit_bonus = 0.2

[[features]]
id = "range"
radius = 3.0
min_range = 1.0
max_range = 8.0
```

### 格式说明

| 部分 | 说明 |
|------|------|
| `[skill]` | 技能模板的 `id` 和 `name` 字段。 |
| `[[features]]` | 技能特征定义列表，每个条目对应一个 [`SkillFeatureDefinition`]。条目内的键值对将填入 `features` 字典。 |
| 文件扩展名 | `.skill.toml` |

## 定义

```rust
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    ecs::error::BevyError,
    prelude::*,
    tasks::ConditionalSendFuture,
};
use serde::Deserialize;
use std::collections::HashMap;

/// Loads [`SkillDefinition`] from `.skill.toml` files.
#[derive(TypePath)]
pub struct SkillDefinitionLoader;

/// No-op settings for [`SkillDefinitionLoader`].
#[derive(Default, Clone, Serialize, Deserialize, TypePath)]
pub struct SkillDefinitionLoaderSettings;

impl AssetLoader for SkillDefinitionLoader {
    type Asset = SkillDefinition;
    type Settings = SkillDefinitionLoaderSettings;
    type Error = BevyError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let content = String::from_utf8(bytes)?;
            let skill = parse_toml(&content).map_err(BevyError::from)?;
            Ok(skill)
        }
    }

    fn extensions(&self) -> &[&str] {
        &["skill.toml"]
    }
}
```

### TOML 解析

```rust
/// Intermediate deserialization structure matching the TOML format.
#[derive(Deserialize)]
struct SkillDefinitionToml {
    skill: SkillHeader,
    #[serde(default)]
    features: Vec<HashMap<String, toml::Value>>,
}

#[derive(Deserialize)]
struct SkillHeader {
    id: String,
    name: String,
}

fn parse_toml(content: &str) -> Result<SkillDefinition, Box<dyn std::error::Error + Send + Sync>> {
    let raw: SkillDefinitionToml = toml::from_str(content)?;

    let mut definition = SkillDefinition::new(raw.skill.id, raw.skill.name);

    for feature_map in raw.features {
        let feature_id = feature_map
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or("each [[features]] entry must have an `id` field")?
            .to_string();

        let mut feature = SkillFeatureDefinition::new(feature_id);

        for (key, value) in &feature_map {
            if key == "id" {
                continue;
            }
            if let Some(num) = value.as_float().or_else(|| value.as_integer().map(|i| i as f64)) {
                feature.set(key, num as f32);
            }
        }

        definition.add_feature(feature);
    }

    Ok(definition)
}
```

## 字段说明

`SkillDefinitionLoader` 本身没有字段，是一个空结构体。

| 关联类型 | 说明 |
|----------|------|
| `Asset` | [`SkillDefinition`] |
| `Settings` | `SkillDefinitionLoaderSettings`（无配置项） |
| `Error` | `BevyError` |

## 注册方式

```rust
app.init_asset::<SkillDefinition>();
app.register_asset_loader(SkillDefinitionLoader);
```

## 与现有模块的关系

- **[`SkillDefinition`]**：加载器的输出目标，解析 TOML 后生成 [`SkillDefinition`] 资产并存放在 `Assets<SkillDefinition>` 中。
- **[`SkillFeatureDefinition`]**：TOML 中每个 `[[features]]` 条目转换为一个 [`SkillFeatureDefinition`] 实例。
- **[`SkillInstance`]**：加载后的 [`SkillDefinition`] 可通过 `skill()` 工厂函数创建运行时 [`SkillInstance`] 组件。
- **[`AttributeTemplateLoader`]**：参考了相同的 [`AssetLoader`] 实现模式。
- **`toml` crate**：加载器依赖 `toml` crate 进行 TOML 解析（需要添加到 `Cargo.toml`）。

[`SkillDefinition`]: ./SkillDefinition.md
[`SkillFeatureDefinition`]: ./SkillFeatureDefinition.md
[`SkillInstance`]: ./SkillInstance.md
[`AttributeTemplateLoader`]: ../attribute/AttributeTemplateLoader.md
