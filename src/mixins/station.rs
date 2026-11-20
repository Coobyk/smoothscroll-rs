fn smoothscroll_station_scroll(game: &mut Game<'_>, d: f32) {
    // The merchant's trade list moves whole rows (`scrolled` truncates), so
    // an eased fraction would still jump a row: it stays instant. The loom's
    // and stonecutter's normalized scrolls ease.
    if !crate::game::umbx_smoothscroll::flushing() {
        game.merchant_scroll(d);
    }
    if crate::game::umbx_smoothscroll::accept(
        crate::game::umbx_smoothscroll::Surface::Station,
        d,
    ) {
        return;
    }
    game.loom_scroll(d);
    let Some(s) = game.station.as_mut() else {
        return;
    };
    if s.kind != Kind::Stonecutter || !s.stonecutter_scrolls() {
        return;
    }
    let rows = stonecutter_offscreen_rows(s.stone.recipes.len());
    s.stonecutter_scroll_to(s.scroll - d / rows as f32);
}
