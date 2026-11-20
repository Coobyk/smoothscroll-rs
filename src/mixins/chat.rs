fn smoothscroll_chat_scroll(game: &mut Game<'_>, amount: f32) {
    let cursor = game.cursor;
    let mouse = (cursor.0.floor() as i32, cursor.1.floor() as i32);
    if let Some(screen) = game.ui.chat_screen.as_mut()
        && screen.scroll(mouse, f64::from(amount))
    {
        return;
    }
    // ChatScreen.mouseScrolled: `hasShiftDown`, either Shift key.
    let shift = game.has_shift_down();
    let amount = amount.clamp(-1.0, 1.0) * if shift { 1.0 } else { 7.0 };
    if crate::game::umbx_smoothscroll::accept(
        crate::game::umbx_smoothscroll::Surface::Chat,
        amount,
    ) {
        return;
    }
    game.ui.chat.scroll_by(amount as i32, true);
}
