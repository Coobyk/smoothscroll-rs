fn smoothscroll_code_of_conduct_scroll(game: &mut Game<'_>, delta: f32) {
    if crate::game::umbx_smoothscroll::accept(
        crate::game::umbx_smoothscroll::Surface::CodeOfConduct,
        delta,
    ) {
        return;
    }
    let size = game.gui_size();
    let Some(prompt) = game.code_of_conduct.as_mut() else {
        return;
    };
    let layout = prompt.layout(&game.gui.atlas.font, size);
    let [_, ay, _, ah] = layout.area;
    prompt.scroll.wheel(delta, 10.0, ay, ay + ah, layout.content);
}
