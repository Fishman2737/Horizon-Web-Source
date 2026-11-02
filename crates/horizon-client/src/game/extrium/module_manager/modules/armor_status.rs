use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Armor Status",
    category: Category::Player,
    description: "Preview a compact armor status display.",
    settings: &[
        SettingDefinition { name: "Show durability", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Layout", kind: SettingKind::Choice { values: &["Horizontal", "Vertical"], default: 0 } },
    ],
};