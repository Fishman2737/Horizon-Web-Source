use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Speedometer",
    category: Category::Movement,
    description: "Preview a movement-speed readout in the HUD.",
    settings: &[
        SettingDefinition { name: "Units", kind: SettingKind::Choice { values: &["Blocks/sec", "Meters/sec"], default: 0 } },
        SettingDefinition { name: "Precision", kind: SettingKind::Slider { min: 0, max: 3, default: 1 } },
    ],
};