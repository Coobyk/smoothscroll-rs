fn smoothscroll_options_scroll(game: &mut Game<'_>, amount: f32) -> Result<()> {
    // A pad-stick scroll is already per-frame, and Ctrl + the wheel on
    // video settings steps the GUI scale: both stay instant.
    if game.pad_stick_scroll() {
        let size = game.gui_size();
        game.ui
            .options
            .scroll_at(&game.settings, size, game.cursor, amount);
        return Ok(());
    }
    if game.ui.options.video_open() && game.has_control_down() {
        let framebuffer = game.renderer.as_ref().map_or([1280, 720], |r| {
            [r.config.width as i32, r.config.height as i32]
        });
        let mut ui = std::mem::take(&mut game.ui.options);
        let events = ui.video_wheel_gui_scale(&mut game.settings, framebuffer, amount);
        game.ui.options = ui;
        return game.handle_option_events(events);
    }
    // With the `wheel_cycles_settings` extra a hovered control takes the
    // wheel; keep it instant so an eased step never re-triggers it.
    if !game.settings.extras.wheel_cycles_settings
        && crate::game::umbx_smoothscroll::accept(
            crate::game::umbx_smoothscroll::Surface::Options,
            amount,
        )
    {
        return Ok(());
    }
    let framebuffer = game.renderer.as_ref().map_or([1280, 720], |r| {
        [r.config.width as i32, r.config.height as i32]
    });
    let size = game.gui_size();
    let events = game
        .ui
        .options
        .wheel(&mut game.settings, size, game.cursor, framebuffer, amount);
    game.handle_option_events(events)
}
