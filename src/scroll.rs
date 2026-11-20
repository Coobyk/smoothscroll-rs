//! Smooth Scroll's game-side state: the wheel delta a scrollable surface
//! still owes, eased out over the next frames.
//!
//! Each smoothed scroll function is overwritten (`horizon-mod.toml`): a raw
//! wheel event queues its delta here and returns; `frame` replays the
//! pending delta in eased steps through the same functions, which run their
//! vanilla body while `flushing` is set.

use super::{Game, Screen};

/// The screen a queued wheel delta belongs to. One is open at a time, so a
/// single pending channel covers them all.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    Chat,
    Options,
    Social,
    Stats,
    DebugOptions,
    ServerLinks,
    CodeOfConduct,
    Creative,
    Advancements,
    Station,
    WorldOptions,
    Title,
}

std::thread_local! {
    /// The surface the pending delta belongs to.
    static SURFACE: std::cell::Cell<Option<Surface>> = const { std::cell::Cell::new(None) };
    /// Wheel delta still owed, in that surface's own units (chat's is whole
    /// lines; the rest take the raw wheel `delta` their function receives).
    static PENDING: std::cell::Cell<f32> = const { std::cell::Cell::new(0.0) };
    /// The advancements screen's horizontal wheel delta (`PENDING` is y).
    static PENDING_X: std::cell::Cell<f32> = const { std::cell::Cell::new(0.0) };
    /// `frame` is replaying a queued delta: scroll functions apply their
    /// step instead of queueing it again.
    static FLUSHING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// Settings, refreshed each frame so toggles apply live.
    static ENABLED: std::cell::Cell<bool> = const { std::cell::Cell::new(true) };
    static SPEED: std::cell::Cell<f32> = const { std::cell::Cell::new(1.0) };
}

/// `frame` is replaying a queued wheel delta right now.
pub fn flushing() -> bool {
    FLUSHING.with(|f| f.get())
}

/// The mod is on and this is a raw wheel event, not a flush.
pub fn smoothing() -> bool {
    ENABLED.with(|e| e.get()) && !FLUSHING.with(|f| f.get())
}

/// Switch the pending channel to `surface`, dropping a delta another
/// surface left over (its units would not fit anyway).
fn queue(surface: Surface) {
    SURFACE.with(|s| {
        if s.get() != Some(surface) {
            PENDING.with(|p| p.set(0.0));
            PENDING_X.with(|p| p.set(0.0));
        }
        s.set(Some(surface));
    });
}

/// Queue `delta` for `surface` outright (for helpers that already know the
/// event scrolls).
pub fn push(surface: Surface, delta: f32) {
    queue(surface);
    PENDING.with(|p| p.set(p.get() + delta));
}

/// A scroll function takes a raw wheel event: when smoothing is on, queue
/// the delta and skip the scroll this call would do.
pub fn accept(surface: Surface, delta: f32) -> bool {
    if !smoothing() {
        return false;
    }
    push(surface, delta);
    true
}

/// The same, for the advancements screen's two wheel axes.
pub fn accept_xy(surface: Surface, x: f32, y: f32) -> bool {
    if !accept(surface, y) {
        return false;
    }
    PENDING_X.with(|p| p.set(p.get() + x));
    true
}

fn clear() {
    SURFACE.with(|s| s.set(None));
    PENDING.with(|p| p.set(0.0));
    PENDING_X.with(|p| p.set(0.0));
}

/// The surface's screen is still up; a queued delta dies with it.
fn open(game: &Game<'_>, surface: Surface) -> bool {
    match surface {
        Surface::Chat => game.screen == Screen::Chat,
        Surface::Options => game.screen == Screen::Options,
        Surface::Social => game.screen == Screen::Social,
        Surface::Stats => game.screen == Screen::Stats,
        Surface::DebugOptions => game.screen == Screen::DebugOptions,
        Surface::ServerLinks => game.screen == Screen::ServerLinks,
        Surface::Creative => game.screen == Screen::Creative,
        Surface::Advancements => game.screen == Screen::Advancements,
        Surface::Station => game.screen == Screen::Inventory,
        Surface::WorldOptions => game.screen == Screen::WorldOptions,
        Surface::CodeOfConduct => game.code_of_conduct.is_some(),
        Surface::Title => game.title_input_active(),
    }
}

/// Once a frame (`render_frame_pre`): read the settings live and ease the
/// queued wheel delta a step closer to done.
pub fn frame(game: &mut Game<'_>, dt: f32) {
    let on = game.setting("smoothscroll", "enabled").as_deref() != Some("false");
    ENABLED.with(|e| e.set(on));
    let speed = game
        .setting("smoothscroll", "speed")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(1.0)
        .clamp(0.25, 4.0);
    SPEED.with(|s| s.set(speed));
    if !on {
        clear();
        return;
    }
    let Some(surface) = SURFACE.with(|s| s.get()) else {
        return;
    };
    if !open(game, surface) {
        clear();
        return;
    }
    let mut pending = PENDING.with(|p| p.get());
    let mut pending_x = PENDING_X.with(|p| p.get());
    if pending == 0.0 && pending_x == 0.0 {
        SURFACE.with(|s| s.set(None));
        return;
    }
    // Exponential ease-out: a fixed share of what is owed each frame, so the
    // end slows on its own. The Fabric mod divides the owed offset by a
    // smoothness factor every render; a rate keeps it frame-rate independent.
    let f = 1.0 - (-dt.clamp(0.0, 0.1) * 14.0 * speed).exp();
    FLUSHING.with(|fl| fl.set(true));
    if surface == Surface::Chat {
        // Chat scrolls whole lines: hand over at least one while a line is
        // owed, and drop a sub-line remainder the way `scroll_by`'s `as i32`
        // truncation would.
        let step = pending * f;
        let mut applied = if step.abs() >= 1.0 {
            step.trunc()
        } else {
            step.signum()
        };
        if applied.abs() > pending.abs() {
            applied = pending.signum();
        }
        pending -= applied;
        if pending.abs() < 0.5 {
            pending = 0.0;
        }
        if applied != 0.0 {
            game.ui.chat.scroll_by(applied as i32, true);
        }
    } else {
        let step = pending * f;
        let step_x = pending_x * f;
        pending -= step;
        pending_x -= step_x;
        // A residue this small is past what the eye resolves.
        if pending.abs() < 0.02 {
            pending = 0.0;
        }
        if pending_x.abs() < 0.02 {
            pending_x = 0.0;
        }
        match surface {
            // `scroll_at` is the pure-scroll leaf: `options.wheel` would let
            // a hovered control take an eased step too.
            Surface::Options => {
                let size = game.gui_size();
                game.ui
                    .options
                    .scroll_at(&game.settings, size, game.cursor, step);
            }
            Surface::Social => game.social_scroll(step),
            Surface::Stats => game.stats_scroll(step),
            Surface::DebugOptions => game.debug_options_scroll(step),
            Surface::ServerLinks => game.server_links_scroll(step),
            Surface::CodeOfConduct => game.code_of_conduct_scroll(step),
            Surface::Creative => game
                .creative_screen
                .wheel(f64::from(step), &game.pack.names),
            Surface::Advancements => game.advancements_scroll(step_x, step),
            Surface::Station => game.station_scroll(step),
            Surface::WorldOptions => game.world_options_scroll(step),
            Surface::Title => {
                let (size, cursor) = (game.gui_size(), game.cursor);
                if let Some(title) = game.title.as_mut() {
                    title.scroll_worlds(step, cursor, size);
                }
            }
            Surface::Chat => {}
        }
    }
    FLUSHING.with(|fl| fl.set(false));
    PENDING.with(|p| p.set(pending));
    PENDING_X.with(|p| p.set(pending_x));
    if pending == 0.0 && pending_x == 0.0 {
        SURFACE.with(|s| s.set(None));
    }
}
