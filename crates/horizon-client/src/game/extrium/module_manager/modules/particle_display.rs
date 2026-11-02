use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Particle Display",
    category: Category::Render,
    description: "Preview particle status and display options.",
    settings: &[
        SettingDefinition { name: "Show status", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Opacity", kind: SettingKind::Slider { min: 0, max: 100, default: 75 } },
    ],
};