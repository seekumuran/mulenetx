
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};

use crate::app::{App, FeedKind};
use crate::keymap::Action;
use crate::ui::{Hint, View, ViewId};
use crate::ui::{chart, table};

pub struct SplitView;

impl View for SplitView {
    fn id(&self) -> ViewId {
        ViewId::Split
    }

    fn title(&self) -> &'static str {
        "MARKET"
    }

    fn hints(&self) -> &'static [Hint] {
        const HINTS: &[Hint] = &[
            Hint::act(&[Action::Quit], "quit"),
            Hint::fixed("tab", "view"),
            Hint::act(&[Action::Up, Action::Down], "select"),
            Hint::act(&[Action::CycleScope], "scope"),
            Hint::act(
                &[
                    Action::ChartStyle,
                    Action::ToggleSma,
                    Action::ToggleRsi,
                    Action::ToggleVolume,
                    Action::MaType,
                    Action::NewsMarkers,
                ],
                "chart",
            ),
            Hint::act(&[Action::NextPreset], "interval"),
            Hint::act(&[Action::FindTicker], "find"),
            Hint::act(&[Action::Help], "help"),
        ];
        HINTS
    }

    fn feed_shown(&self) -> Option<FeedKind> {
        None
    }

    fn has_chart_panel(&self) -> bool {
        true
    }

    fn render(&self, f: &mut Frame, area: Rect, app: &mut App) {
        if area.height < 18 || area.width < 70 {
            let [left, right] =
                Layout::horizontal([
                    Constraint::Percentage(40),
                    Constraint::Percentage(60),
                ])
                .areas(area);

            table::render_table(f, left, app);
            chart::render_price_chart(f, right, app);
            return;
        }

        let [charts_area, watchlist_area] =
            Layout::vertical([
                Constraint::Percentage(90),
                Constraint::Percentage(10),
            ])
            .areas(area);

        let [top_row, bottom_row] =
            Layout::vertical([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .areas(charts_area);

        let [chart_1, chart_2] =
            Layout::horizontal([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .areas(top_row);

        let [chart_3, chart_4] =
            Layout::horizontal([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .areas(bottom_row);

        chart::render_price_chart_for_symbol(f, chart_1, app, "AAPL");
        chart::render_price_chart_for_symbol(f, chart_2, app, "MSFT");
        chart::render_price_chart_for_symbol(f, chart_3, app, "NVDA");
        chart::render_price_chart_for_symbol(f, chart_4, app, "BTC-USD");

        table::render_table(f, watchlist_area, app);
    }
}
