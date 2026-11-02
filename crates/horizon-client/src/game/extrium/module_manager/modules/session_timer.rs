use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Session Timer",
    category: Category::Misc,
    description: "Preview elapsed play time in a compact readout.",
    settings: &[
        SettingDefinition { name: "Format", kind: SettingKind::Choice { values: &["Compact", "Full"], default: 0 } },
        SettingDefinition { name: "Reset on world change", kind: SettingKind::Toggle { default: false } },
    ],
};