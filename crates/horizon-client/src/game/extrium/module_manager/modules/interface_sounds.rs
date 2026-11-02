use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Interface Sounds",
    category: Category::Misc,
    description: "Preview configurable sounds for menu interactions.",
    settings: &[
        SettingDefinition { name: "Enabled", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Volume", kind: SettingKind::Slider { min: 0, max: 100, default: 70 } },
    ],
};