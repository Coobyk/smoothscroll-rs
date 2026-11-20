fn smoothscroll_social_scroll(game: &mut Game<'_>, delta: f32) {
    if crate::game::umbx_smoothscroll::accept(
        crate::game::umbx_smoothscroll::Surface::Social,
        delta,
    ) {
        return;
    }
    game.social.scroll -= delta * 18.0;
    // `social_view` clamps it to the list.
    let _ = game.social_view();
}
