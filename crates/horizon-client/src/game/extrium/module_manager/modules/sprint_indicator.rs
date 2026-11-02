use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Sprint Indicator",
    category: Category::Movement,
    description: "Preview sprint state as a compact interface indicator.",
    settings: &[
        SettingDefinition { name: "Show indicator", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Position", kind: SettingKind::Choice { values: &["Left", "Right"], default: 0 } },
    ],
};