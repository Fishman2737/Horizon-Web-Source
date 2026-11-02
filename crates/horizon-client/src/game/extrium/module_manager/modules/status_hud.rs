use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Status HUD",
    category: Category::Player,
    description: "Preview player status labels in a compact HUD.",
    settings: &[
        SettingDefinition { name: "Show health", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Show food", kind: SettingKind::Toggle { default: true } },
    ],
};