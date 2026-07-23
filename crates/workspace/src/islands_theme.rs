//! Islands theme — visual restyle of the workspace.
//!
//! Mirrors the JetBrains "Islands" theme released alongside the
//! 2025.2 IDEs:
//! https://blog.jetbrains.com/platform/2025/09/islands-theme-the-new-look-coming-to-jetbrains-ides/
//!
//! Each tool window (left/right/bottom dock) and the center editor area
//! are rendered as rounded "cards" with a subtle background fill and a
//! gap between them, so the workspace background bleeds through. There
//! are two variants:
//!
//!   * `Islands`   — separate card per dock + center.
//!   * `OneIsland` — a single card around the whole dock+center
//!                   composition.
//!
//! No layout change: the existing `bottom_dock_layout` branches still
//! compose the same way. The Islands theme is purely chrome.
//!
//! The `Islands` variant is the baked-in default. Override via
//! `ZED_ISLANDS_THEME=one` for the single-card variant, or
//! `ZED_ISLANDS_THEME=off` to disable.

use gpui::{App, Div, Pixels, Styled, px};
use ui::ActiveTheme;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IslandsTheme {
    Off,
    Islands,
    OneIsland,
}

impl IslandsTheme {
    /// Read the current setting. The `Islands` variant is the baked-in
    /// default; the env var only needs to be set to override it.
    pub fn current() -> IslandsTheme {
        match std::env::var("ZED_ISLANDS_THEME").as_deref() {
            Ok("off") | Ok("0") => IslandsTheme::Off,
            Ok("one") | Ok("one_island") => IslandsTheme::OneIsland,
            _ => IslandsTheme::Islands,
        }
    }

    pub fn is_on(self) -> bool {
        !matches!(self, IslandsTheme::Off)
    }
}

/// Gap between the workspace's outer edge and the card region. Also
/// the gap between adjacent cards in the `Islands` variant.
pub const GAP: Pixels = px(6.);

/// Card corner radius — matches the JetBrains visual where each panel
/// has soft rounded corners.
pub const CARD_RADIUS: Pixels = px(8.);

/// Wrap a per-panel element (dock or center) in card chrome: rounded
/// corners, surface background, soft border. Caller is responsible for
/// the outer margin/gap that separates adjacent cards.
///
/// In `OneIsland` mode this is a no-op — the outer wrapper handles
/// rounding for the whole composition.
pub fn card<E: Styled>(element: E, theme: IslandsTheme, cx: &App) -> E {
    if !matches!(theme, IslandsTheme::Islands) {
        return element;
    }
    let colors = cx.theme().colors();
    // 3px margin on all sides means adjacent cards have a 6px gap
    // between them — half the gap belongs to each side.
    element
        .m(px(3.))
        .rounded(CARD_RADIUS)
        .bg(colors.surface_background)
        .border_1()
        .border_color(colors.border_variant)
        .overflow_hidden()
}

/// Apply outer styling directly to the workspace's dock+center composition
/// div. Returning a *modified* div instead of a *wrapping* div is critical:
/// wrapping breaks the inner flex layout (bottom dock collapses to zero
/// height) because the wrapper's default flex direction conflicts with the
/// composition's `flex_row`/`flex_col` choices.
///
/// In `Islands` mode this adds padding so the workspace background shows
/// through around the cards.
///
/// In `OneIsland` mode this adds margin + rounded card chrome around the
/// whole composition.
pub fn outer(div: Div, theme: IslandsTheme, cx: &App) -> Div {
    match theme {
        IslandsTheme::Off => div,
        // The cards each contribute their own 3px margin, so we only
        // add 3px of outer padding to make the workspace edges match
        // the inter-card spacing.
        IslandsTheme::Islands => div.p(px(3.)),
        IslandsTheme::OneIsland => {
            let colors = cx.theme().colors();
            div.m(GAP)
                .rounded(CARD_RADIUS)
                .bg(colors.surface_background)
                .border_1()
                .border_color(colors.border_variant)
                .overflow_hidden()
        }
    }
}
