# Smooth Scroll

A Horizon native mod port of [Minecraft-Smooth-Scrolling](https://github.com/SmajloSlovakian/Minecraft-Smooth-Scrolling)
(`smoothscroll`): scrollable UI animates its offset instead of snapping to
the new position.

## How it works

The Fabric mod intercepts changes to a screen's scroll offset, adds them to
an owed amount, and divides that amount every frame. This port does the
same against Horizon's scroll functions: each smoothed scroll function is
overwritten by a native mixin that queues its wheel delta instead of
applying it, and a `render_frame_pre` mixin replays the pending delta in
eased steps through the same code path (the overwritten bodies apply their
step while the runtime flushes). The easing is exponential ease-out —
a fixed share of what is owed per second, so it is frame-rate independent —
with a `speed` setting (Options → Mods → Smooth Scroll) scaling the rate.

Chat scrolls whole lines in vanilla, so its pending delta is delivered in
whole-line steps, never stalling or overshooting.

## Surfaces smoothed

- Chat screen (whole-line animation; the command-suggestion list under the
  cursor keeps its instant scroll)
- Options / settings lists (`scroll_at`, including pack screens, the mods
  screen and the studio page inside options)
- Social interactions list
- Statistics list
- Debug options list (F3+F6)
- Server links dialog
- Code-of-conduct prompt text
- Creative inventory
- Advancements screen (both wheel axes)
- Station screens: loom pattern list and stonecutter recipe list
- World options / LAN screen: the options scroll and the game-rules list
- Title screens: classic world list, multiplayer server list, realms list,
  flat and buffet preset lists, the game-rules overlay, and the missing-mods
  message list

Whole-notch consumers stay instant and are never fed an eased step:
cycle buttons on the create-world and LAN screens, Ctrl+wheel GUI-scale
steps on video settings, the `wheel_cycles_settings` extra, and the
merchant trade list (its offset is a whole number of rows).

## Left out

- The hotbar's selection animation the original mod draws.
- `command_block` output, the HUD editor, and other scroll surfaces not
  driven by the screen scroll functions above.
- Scrollbar dragging: vanilla moves the offset with the pointer directly,
  so it stays 1:1.
- On the options screens, a wheel turn over a non-list cycle button scrolls
  the list instead of cycling the button (the mod cannot split that decision
  inside `OptionsUi::wheel`); with the `wheel_cycles_settings` extra on,
  options scrolling is left instant entirely.
