fn smoothscroll_advancements_scroll(game: &mut Game<'_>, x: f32, y: f32) {
    if crate::game::umbx_smoothscroll::accept_xy(
        crate::game::umbx_smoothscroll::Surface::Advancements,
        x,
        y,
    ) {
        return;
    }
    let view = game.adv_view();
    game.adv_scroll_by(&view, f64::from(x) * 16.0, f64::from(y) * 16.0);
}
