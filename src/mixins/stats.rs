fn smoothscroll_stats_scroll(game: &mut Game<'_>, amount: f32) {
    if crate::game::umbx_smoothscroll::accept(
        crate::game::umbx_smoothscroll::Surface::Stats,
        amount,
    ) {
        return;
    }
    let (row_h, _) = game.stats_rows();
    let h = game.gui_size().1;
    let content = game.stats_content();
    game.stats_ui
        .scroll
        .wheel(amount, row_h / 2.0, TAB_H, h - FOOTER, content);
}
