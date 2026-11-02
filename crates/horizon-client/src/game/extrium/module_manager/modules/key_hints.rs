use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Key Hints",
    category: Category::Player,
    description: "Preview the current key hints in a small overlay.",
    settings: &[
        SettingDefinition { name: "Show hints", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Layout", kind: SettingKind::Choice { values: &["Horizontal", "Vertical"], default: 0 } },
    ],
};