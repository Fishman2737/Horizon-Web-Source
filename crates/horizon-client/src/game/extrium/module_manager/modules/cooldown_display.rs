use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Cooldown Display",
    category: Category::Combat,
    description: "Preview cooldown information in a small HUD label.",
    settings: &[
        SettingDefinition { name: "Show item cooldowns", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Scale", kind: SettingKind::Slider { min: 50, max: 150, default: 100 } },
    ],
};