fn smoothscroll_world_options_scroll(game: &mut Game<'_>, delta: f32) {
    let (w, h) = game.gui_size();
    let stick_scroll = game.pad_stick_scroll();
    match game.world_options.as_mut() {
        Some(o) if o.view == View::Options => {
            // A cycle button takes a whole notch at once, never an eased step.
            if !crate::game::umbx_smoothscroll::flushing() {
                let hit_btn = o.hit(game.cursor, (w, h)).or(o.focused_button);
                if let Some(btn) = hit_btn.filter(|_| !stick_scroll) {
                    let step = if delta > 0.0 { -1 } else { 1 };
                    match btn {
                        Button::DefaultMode => {
                            o.default_mode = (o.default_mode + step).rem_euclid(4);
                            game.audio.play_button_click();
                            return;
                        }
                        Button::PersonalMode => {
                            o.personal_mode = (o.personal_mode + step).rem_euclid(4);
                            game.audio.play_button_click();
                            return;
                        }
                        Button::Difficulty => {
                            o.difficulty = ((o.difficulty as i32 + step).rem_euclid(4)) as u8;
                            game.audio.play_button_click();
                            return;
                        }
                        Button::Cheats => {
                            o.cheats = !o.cheats;
                            if !o.cheats {
                                o.default_mode = o.initial.default_mode;
                                o.personal_mode = o.default_mode;
                            }
                            game.audio.play_button_click();
                            return;
                        }
                        Button::Lan => {
                            o.lan = !o.lan;
                            o.port.focused = false;
                            game.audio.play_button_click();
                            return;
                        }
                        Button::CommandAccess => {
                            o.guest_commands = !o.guest_commands;
                            game.audio.play_button_click();
                            return;
                        }
                        Button::Force => {
                            o.force = !o.force;
                            game.audio.play_button_click();
                            return;
                        }
                        _ => {}
                    }
                }
            }
            if crate::game::umbx_smoothscroll::accept(
                crate::game::umbx_smoothscroll::Surface::WorldOptions,
                delta,
            ) {
                return;
            }
            o.scroll = (o.scroll - delta * 10.0).clamp(0.0, o.scroll_limit(h));
        }
        Some(o) => {
            if crate::game::umbx_smoothscroll::accept(
                crate::game::umbx_smoothscroll::Surface::WorldOptions,
                delta,
            ) {
                return;
            }
            if let Some(rules) = o.rules.as_mut() {
                rules.wheel(delta, h);
            }
        }
        None => {}
    }
}
