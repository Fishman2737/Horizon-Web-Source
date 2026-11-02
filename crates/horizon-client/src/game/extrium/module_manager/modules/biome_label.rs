use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Biome Label",
    category: Category::World,
    description: "Preview the current biome name in a HUD label.",
    settings: &[
        SettingDefinition { name: "Fade duration", kind: SettingKind::Slider { min: 0, max: 10, default: 3 } },
        SettingDefinition { name: "Show coordinates", kind: SettingKind::Toggle { default: false } },
    ],
};