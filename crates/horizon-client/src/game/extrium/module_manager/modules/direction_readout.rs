use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Direction Readout",
    category: Category::Movement,
    description: "Preview a compass direction label in the HUD.",
    settings: &[
        SettingDefinition { name: "Format", kind: SettingKind::Choice { values: &["Cardinal", "Degrees"], default: 0 } },
        SettingDefinition { name: "Show axis", kind: SettingKind::Toggle { default: false } },
    ],
};