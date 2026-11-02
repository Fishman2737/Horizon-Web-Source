use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Dimension Label",
    category: Category::World,
    description: "Preview the current dimension in a HUD label.",
    settings: &[
        SettingDefinition { name: "Show icon", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Fade duration", kind: SettingKind::Slider { min: 0, max: 10, default: 4 } },
    ],
};