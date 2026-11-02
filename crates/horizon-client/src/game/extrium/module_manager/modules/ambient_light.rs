use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Ambient Light",
    category: Category::Render,
    description: "Preview a subtle ambient-light status overlay.",
    settings: &[
        SettingDefinition { name: "Intensity", kind: SettingKind::Slider { min: 0, max: 100, default: 50 } },
        SettingDefinition { name: "Color", kind: SettingKind::Choice { values: &["Violet", "Rose", "White"], default: 0 } },
    ],
};