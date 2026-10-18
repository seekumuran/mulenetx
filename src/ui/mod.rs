pub mod article;
pub mod calendar;
pub mod categories;
pub mod chart;
pub mod earnings;
pub mod help;
pub mod insider;
pub mod insider_chart;
pub mod news;
pub mod news_marks;
pub mod omega;
pub mod omega_data;
pub mod portfolio;
pub mod prompt;
pub mod rail;
pub mod settings;
pub mod split;
pub mod summary;
pub mod table;
pub mod ticker_search;
mod time_axis;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::app::{App, FeedKind, PromptKind};
use crate::keymap::Action;

/// Stable identity of a display mode, decoupled from its position in the
/// tab order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewId {
    Omega,
    Split,
    News,
    Table,
    Chart,
    Insider,
    Earnings,
    Summary,
    Portfolio,
    Calendar,
}

/// One footer hint: the keys of `actions` (looked up in the live keymap so
/// the footer always shows what is actually bound) plus a short label.
/// `fixed` names keys outside the keymap (the positional 1-9 digits).
pub struct Hint {
    pub actions: &'static [Action],
    pub fixed: &'static str,
    pub label: &'static str,
}

impl Hint {
    pub const fn act(actions: &'static [Action], label: &'static str) -> Self {
        Self {
            actions,
            fixed: "",
            label,
        }
    }

    pub const fn fixed(fixed: &'static str, label: &'static str) -> Self {
        Self {
            actions: &[],
            fixed,
            label,
        }
    }
}

/// Footer hints of views that only navigate the watchlist (the trait
/// default; Table uses it as is).
pub static DEFAULT_HINTS: &[Hint] = &[
    Hint::act(&[Action::Quit], "quit"),
    Hint::fixed("tab/1-2", "view"),
    Hint::act(&[Action::Up, Action::Down], "select"),
    Hint::act(&[Action::AddTicker], "add"),
    Hint::act(&[Action::FindTicker], "find"),
    Hint::act(&[Action::NextPreset], "interval"),
    Hint::act(&[Action::Refresh], "refresh"),
    Hint::act(&[Action::Settings], "settings"),
    Hint::act(&[Action::Help], "help"),
];

/// A display mode. Views are stateless renderers: all mutable state
/// (selection, scroll) lives in `App`, so adding a view is a unit struct,
/// a ViewId variant and one entry in VIEWS.
pub trait View: Sync {
    fn id(&self) -> ViewId;
    fn title(&self) -> &'static str;

    fn hints(&self) -> &'static [Hint] {
        DEFAULT_HINTS
    }

    fn feed_shown(&self) -> Option<FeedKind> {
        None
    }

    fn navigates_articles(&self) -> bool {
        false
    }

    fn has_chart_panel(&self) -> bool {
        false
    }

    fn shows_earnings(&self) -> bool {
        false
    }

    fn shows_calendar(&self) -> bool {
        false
    }

    fn render(&self, f: &mut Frame, area: Rect, app: &mut App);
}

/// Internal registry: keep all original views registered.
pub static VIEWS: [&dyn View; 10] = [
    &split::SplitView,
    &news::NewsView,
    &table::TableView,
    &chart::ChartView,
    &insider::InsiderView,
    &earnings::EarningsView,
    &summary::SummaryView,
    &portfolio::PortfolioView,
    &calendar::CalendarView,
    &omega::OmegaView,
];

/// Index of a view in VIEWS.
pub fn view_index(id: ViewId) -> usize {
    VIEWS
        .iter()
        .position(|v| v.id() == id)
        .unwrap_or_else(|| panic!("view {id:?} is not registered in VIEWS"))
}

pub fn draw(f: &mut Frame, app: &mut App) {
    app.mark_selected_seen();

    let rail_h = u16::from(rail::visible(app, f.area().height));
    let chrome_h = u16::from(!app.bare);

    let [header, rail_area, body, footer] = Layout::vertical([
        Constraint::Length(chrome_h),
        Constraint::Length(rail_h),
        Constraint::Min(0),
        Constraint::Length(chrome_h),
    ])
    .areas(f.area());

    if chrome_h > 0 {
        f.render_widget(header_line(app, header.width), header);
    }
    if rail_h > 0 {
        rail::render(f, rail_area, app);
    }

    app.article_overlay.inline = false;
    VIEWS[app.view_idx].render(f, body, app);

    if chrome_h > 0 {
        f.render_widget(footer_line(app), footer);
    }
    if app.article_overlay.open && !app.article_overlay.inline {
        article::render(f, app);
    }
    if app.help.open {
        help::render(f, app);
    }
    if app.category_picker.open {
        categories::render(f, app);
    }
    if app.settings.open {
        settings::render(f, app);
    }
    if app.prompt.open {
        prompt::render(f, app);
    }
    if app.ticker_search.open {
        ticker_search::render(f, app);
    }
}

/// Cut to display cells, preserving grapheme clusters.
pub(crate) fn ellipsize(text: &str, width: usize) -> String {
    if width == 0 || text.width() <= width {
        return text.to_string();
    }

    let mut out = String::new();
    let mut used = 0;

    for grapheme in text.graphemes(true) {
        let cells = grapheme.width();
        if used + cells > width - 1 {
            break;
        }
        out.push_str(grapheme);
        used += cells;
    }

    out.push('…');
    out
}

/// Reserve cells without truncating a meaningful price or label.
pub(crate) fn pad_right(text: &str, width: usize) -> String {
    format!("{text}{}", " ".repeat(width.saturating_sub(text.width())))
}

/// Centered modal rect used by the settings and article overlays.
pub(crate) fn centered(r: Rect, width: u16, height: u16) -> Rect {
    let w = width.min(r.width.saturating_sub(2));
    let h = height.min(r.height.saturating_sub(2));

    Rect {
        x: r.x + (r.width.saturating_sub(w)) / 2,
        y: r.y + (r.height.saturating_sub(h)) / 2,
        width: w,
        height: h,
    }
}

/// Display only the two requested tabs.
fn header_line(app: &App, width: u16) -> Paragraph<'static> {
    let mut spans = vec![Span::styled(
        " MULENETX OMEGA ",
        Style::new().bold().fg(app.theme.accent),
    )];

    let visible_views = [(0usize, "MARKET")];

    let source = format!(" · {} {}", app.source_name, app.refresh_marker());
    let key_state = if app.alphai_enabled {
        " · alphai ✓"
    } else {
        " · alphai: no key (s)"
    };

    for (tab, (index, title)) in visible_views.iter().enumerate() {
        let active = app.view_idx == *index;
        let style = if active {
            Style::new().fg(app.theme.accent_text).bg(app.theme.accent)
        } else {
            app.theme.subtle()
        };

        spans.push(Span::styled(format!(" {}:{} ", tab + 1, title), style));
    }

    let used = spans.iter().map(Span::width).sum::<usize>();
    let full_status = format!("{source}{key_state}");

    let status = if used + Span::raw(&full_status).width() <= width as usize {
        full_status
    } else {
        source
    };

    let used = spans.iter().map(Span::width).sum::<usize>();
    let status_width = Span::raw(&status).width();

    if used + status_width <= width as usize {
        spans.push(Span::raw(" ".repeat(width as usize - used - status_width)));
        spans.push(Span::styled(status, app.theme.subtle()));
    }

    Paragraph::new(Line::from(spans))
}

#[cfg(test)]
mod tests;

/// The current view's hints with their live keys.
fn hints_text(app: &App) -> String {
    let parts: Vec<String> = VIEWS[app.view_idx]
        .hints()
        .iter()
        .map(|h| {
            let keys = if h.actions.is_empty() {
                h.fixed.to_string()
            } else {
                app.keymap.labels(h.actions)
            };
            format!("{keys} {}", h.label)
        })
        .collect();

    format!(" {}", parts.join(" · "))
}

fn footer_line(app: &App) -> Paragraph<'static> {
    let hints = if app.prompt.open {
        match app.prompt.kind {
            PromptKind::Ticker => " type a ticker · enter add · esc cancel".to_string(),
            PromptKind::Position => {
                " qty and average price · enter save · empty clears · esc cancel".to_string()
            }
        }
    } else if app.ticker_search.open {
        " type to find · ↑↓ move · enter select · esc cancel".to_string()
    } else if app.settings.open {
        " ↑↓ move · ←→ change · enter edit/save · esc close".to_string()
    } else if app.article_overlay.open {
        " ↑↓/jk scroll · pgup/pgdn page · ⏎ open in browser · esc/v close".to_string()
    } else if app.help.open {
        " ↑↓/jk scroll · pgup/pgdn page · esc/? close".to_string()
    } else if app.category_picker.open {
        " ↑↓ move · space toggle · o only this · a all · enter apply · esc cancel".to_string()
    } else {
        hints_text(app)
    };

    let mut spans = vec![Span::styled(hints, app.theme.subtle())];

    if let Some(notice) = app.notice() {
        spans.insert(
            0,
            Span::styled(
                ellipsize(&format!(" {notice} "), 120),
                Style::new().fg(app.theme.warn).add_modifier(Modifier::BOLD),
            ),
        );
    } else if let Some((symbol, error)) = app.errors.iter().next() {
        let msg = ellipsize(&format!("  {symbol}: {error}"), 120);
        spans.push(Span::styled(
            msg,
            Style::new()
                .fg(app.theme.error)
                .add_modifier(Modifier::BOLD),
        ));
    }

    Paragraph::new(Line::from(spans))
}
