fn smoothscroll_server_links_scroll(game: &mut Game<'_>, delta: f32) {
    if crate::game::umbx_smoothscroll::accept(
        crate::game::umbx_smoothscroll::Surface::ServerLinks,
        delta,
    ) {
        return;
    }
    let (_, h) = game.gui_size();
    game.links_ui.scroll = (game.links_ui.scroll - delta * 10.0)
        .clamp(0.0, scroll_limit(h, game.server_links().len()));
}
