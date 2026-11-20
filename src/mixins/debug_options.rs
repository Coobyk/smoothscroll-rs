fn smoothscroll_debug_options_scroll(game: &mut Game<'_>, delta: f32) {
    if crate::game::umbx_smoothscroll::accept(
        crate::game::umbx_smoothscroll::Surface::DebugOptions,
        delta,
    ) {
        return;
    }
    let (_, h) = game.gui_size();
    if let Some(s) = &mut game.debug_options {
        let content = content_height(rows(&s.search.value).len());
        s.scroll.wheel(delta, ITEM / 2.0, HEADER, h - FOOTER, content);
    }
}
