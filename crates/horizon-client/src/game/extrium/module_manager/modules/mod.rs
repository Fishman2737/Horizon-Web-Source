pub(super) mod ambient_light;
pub(super) mod attack_timing;
pub(super) mod armor_status;
pub(super) mod biome_label;
pub(super) mod clean_screenshots;
pub(super) mod cooldown_display;
pub(super) mod dimension_label;
pub(super) mod interface_sounds;
pub(super) mod key_hints;
pub(super) mod particle_display;
pub(super) mod session_timer;
pub(super) mod speedometer;
pub(super) mod sprint_indicator;
pub(super) mod status_hud;
pub(super) mod hit_counter;
pub(super) mod direction_readout;
pub(super) mod time_display;
pub(super) mod weather_overlay;

use super::ModuleDefinition;

pub(super) fn definitions() -> Vec<ModuleDefinition> {
    vec![
        attack_timing::DEFINITION,
        cooldown_display::DEFINITION,
        hit_counter::DEFINITION,
        speedometer::DEFINITION,
        sprint_indicator::DEFINITION,
        direction_readout::DEFINITION,
        ambient_light::DEFINITION,
        weather_overlay::DEFINITION,
        particle_display::DEFINITION,
        clean_screenshots::DEFINITION,
        interface_sounds::DEFINITION,
        session_timer::DEFINITION,
        status_hud::DEFINITION,
        key_hints::DEFINITION,
        armor_status::DEFINITION,
        time_display::DEFINITION,
        biome_label::DEFINITION,
        dimension_label::DEFINITION,
    ]
}