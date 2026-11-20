//! Smooth Scroll's mod crate: registers the settings on the mods screen.
//! The animation itself runs in the injected game module `src/scroll.rs`
//! (`umbx_smoothscroll` in `horizon-mod.toml`), driven by the mixin helpers
//! under `src/mixins/`.

use mod_api::{Hooks, Registrar, RegistrationContext, SettingSpec};

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, reg: &mut dyn Registrar, _: &RegistrationContext) {
        for spec in [
            SettingSpec::toggle("enabled", "smoothscroll.setting.enabled", true),
            SettingSpec::range("speed", "smoothscroll.setting.speed", 0.25, 4.0, 0.25, 1.0),
        ] {
            let _ = reg.register_setting(spec);
        }
    }
}
