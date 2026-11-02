use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Attack Timing",
    category: Category::Combat,
    description: "Preview an attack-timing indicator in the interface.",
    settings: &[
        SettingDefinition { name: "Show indicator", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Style", kind: SettingKind::Choice { values: &["Compact", "Detailed"], default: 0 } },
    ],
};