use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Time Display",
    category: Category::World,
    description: "Preview the world clock in an interface label.",
    settings: &[
        SettingDefinition { name: "Format", kind: SettingKind::Choice { values: &["24 hour", "12 hour"], default: 0 } },
        SettingDefinition { name: "Show day", kind: SettingKind::Toggle { default: true } },
    ],
};