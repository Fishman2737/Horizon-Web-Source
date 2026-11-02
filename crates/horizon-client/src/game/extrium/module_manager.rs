mod modules;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Category {
    Combat,
    Movement,
    Render,
    Misc,
    Player,
    World,
}

impl Category {
    pub(super) const ALL: [Self; 6] = [
        Self::Combat,
        Self::Movement,
        Self::Render,
        Self::Misc,
        Self::Player,
        Self::World,
    ];

    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Combat => "Combat",
            Self::Movement => "Movement",
            Self::Render => "Render",
            Self::Misc => "Misc",
            Self::Player => "Player",
            Self::World => "World",
        }
    }

    pub(super) fn description(self) -> &'static str {
        match self {
            Self::Combat => "Combat interface previews",
            Self::Movement => "Movement HUD previews",
            Self::Render => "Visual overlay previews",
            Self::Misc => "Client interface extras",
            Self::Player => "Player status displays",
            Self::World => "World information displays",
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum SettingKind {
    Toggle { default: bool },
    Choice {
        values: &'static [&'static str],
        default: usize,
    },
    Slider { min: u8, max: u8, default: u8 },
}

#[derive(Clone, Copy)]
pub(super) struct SettingDefinition {
    pub(super) name: &'static str,
    pub(super) kind: SettingKind,
}

#[derive(Clone, Copy)]
pub(super) struct ModuleDefinition {
    pub(super) name: &'static str,
    pub(super) category: Category,
    pub(super) description: &'static str,
    pub(super) settings: &'static [SettingDefinition],
}

#[derive(Clone, Copy)]
pub(super) enum SettingValue {
    Toggle(bool),
    Choice(usize),
    Slider(u8),
}

pub(super) struct SettingState {
    pub(super) definition: SettingDefinition,
    pub(super) value: SettingValue,
}

pub(super) struct ModuleState {
    pub(super) definition: ModuleDefinition,
    pub(super) enabled: bool,
    pub(super) settings: Vec<SettingState>,
}

impl ModuleState {
    fn new(definition: ModuleDefinition) -> Self {
        let settings = definition
            .settings
            .iter()
            .copied()
            .map(|setting| SettingState {
                definition: setting,
                value: match setting.kind {
                    SettingKind::Toggle { default } => SettingValue::Toggle(default),
                    SettingKind::Choice { default, .. } => SettingValue::Choice(default),
                    SettingKind::Slider { default, .. } => SettingValue::Slider(default),
                },
            })
            .collect();
        Self {
            definition,
            enabled: false,
            settings,
        }
    }
}

#[derive(Default)]
pub(super) struct ModuleManager {
    modules: Vec<ModuleState>,
}

impl ModuleManager {
    pub(super) fn new() -> Self {
        Self {
            modules: modules::definitions()
                .into_iter()
                .map(ModuleState::new)
                .collect(),
        }
    }

    pub(super) fn module(&self, index: usize) -> Option<&ModuleState> {
        self.modules.get(index)
    }

    pub(super) fn module_mut(&mut self, index: usize) -> Option<&mut ModuleState> {
        self.modules.get_mut(index)
    }

    pub(super) fn category_indices(&self, category: Category) -> impl Iterator<Item = usize> + '_ {
        self.modules
            .iter()
            .enumerate()
            .filter(move |(_, module)| module.definition.category == category)
            .map(|(index, _)| index)
    }

    pub(super) fn category_module(&self, category: Category, offset: usize) -> Option<usize> {
        self.category_indices(category).nth(offset)
    }

    pub(super) fn toggle_module(&mut self, index: usize) {
        if let Some(module) = self.module_mut(index) {
            module.enabled = !module.enabled;
        }
    }

    pub(super) fn toggle_setting(&mut self, module_index: usize, setting_index: usize) {
        let Some(setting) = self
            .module_mut(module_index)
            .and_then(|module| module.settings.get_mut(setting_index))
        else {
            return;
        };
        match setting.value {
            SettingValue::Toggle(value) => setting.value = SettingValue::Toggle(!value),
            SettingValue::Choice(value) => {
                if let SettingKind::Choice { values, .. } = setting.definition.kind
                    && !values.is_empty()
                {
                    setting.value = SettingValue::Choice((value + 1) % values.len());
                }
            }
            SettingValue::Slider(_) => {}
        }
    }

    pub(super) fn set_slider(&mut self, module_index: usize, setting_index: usize, value: f32) {
        let Some(setting) = self
            .module_mut(module_index)
            .and_then(|module| module.settings.get_mut(setting_index))
        else {
            return;
        };
        if let SettingKind::Slider { min, max, .. } = setting.definition.kind {
            let value = (value.clamp(0.0, 1.0) * f32::from(max.saturating_sub(min))).round()
                as u8
                + min;
            setting.value = SettingValue::Slider(value);
        }
    }
}