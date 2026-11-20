fn smoothscroll_frame(game: &mut Game<'_>, elapsed: std::time::Duration) {
    crate::game::umbx_smoothscroll::frame(game, elapsed.as_secs_f32());
}
