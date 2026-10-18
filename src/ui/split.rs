use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::app::{App, FeedKind};
use crate::keymap::Action;
use crate::ui::omega_data::{self, Account, Transaction};
use crate::ui::{Hint, View, ViewId};
use crate::ui::{chart, table as watchlist};

pub struct SplitView;

const COMPANIES: [&str; 10] = [
    "AAPL", "MSFT", "NVDA", "AMZN", "GOOGL", "META", "TSLA", "AVGO", "ORCL", "AMD",
];

fn money(value: u64) -> String {
    if value >= 1_000_000 {
        format!("${:.2}M", value as f64 / 1_000_000.0)
    } else if value >= 1_000 {
        format!("${:.1}K", value as f64 / 1_000.0)
    } else {
        format!("${value}")
    }
}

fn account_id(id: usize) -> String {
    format!("AC-{:05}", id + 1)
}

fn risk_color(risk: u8) -> Color {
    if risk >= 80 {
        Color::Red
    } else if risk >= 60 {
        Color::Yellow
    } else {
        Color::Green
    }
}

fn bold<'a>(text: &'a str, color: Color) -> Span<'a> {
    Span::styled(
        text,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )
}

fn terminal_block<'a>(title: &'a str, color: Color) -> Block<'a> {
    Block::default()
        .title(bold(title, color))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
}

fn render_risk_table(f: &mut Frame, area: Rect, selected: &str, scroll: &mut usize) {
    let network = omega_data::network();

    let mut accounts: Vec<&Account> = network
        .accounts
        .iter()
        .filter(|account| account.company == selected)
        .collect();

    accounts.sort_by(|a, b| b.risk.cmp(&a.risk).then(a.id.cmp(&b.id)));

    let visible = area.height.saturating_sub(3) as usize;
    let max_scroll = accounts.len().saturating_sub(visible);
    *scroll = (*scroll).min(max_scroll);
    let end = (*scroll + visible).min(accounts.len());

    let header = Row::new(vec![
        "ACCOUNT", "ENTITY", "RISK", "SIGNAL", "CLUSTER", "EXPOSURE",
    ])
    .style(
        Style::default()
            .fg(Color::White)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = accounts[*scroll..end]
        .iter()
        .map(|account| {
            let color = risk_color(account.risk);

            Row::new(vec![
                Cell::from(account_id(account.id)),
                Cell::from(account.company),
                Cell::from(format!("{:>3}", account.risk))
                    .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Cell::from(account.signal).style(Style::default().fg(color)),
                Cell::from(format!("N-{:03}", account.cluster + 1))
                    .style(Style::default().fg(Color::Cyan)),
                Cell::from(money(account.exposure)),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(10),
        Constraint::Length(7),
        Constraint::Length(5),
        Constraint::Min(10),
        Constraint::Length(8),
        Constraint::Length(10),
    ];

    let title = format!(
        " RISK WATCH // {} // {}/{} ",
        selected,
        accounts.len(),
        omega_data::ACCOUNT_COUNT
    );

    f.render_widget(
        Table::new(rows, widths)
            .header(header)
            .block(terminal_block(&title, Color::Yellow))
            .column_spacing(1),
        area,
    );
}

fn render_flow_table(f: &mut Frame, area: Rect, selected: &str, scroll: &mut usize) {
    let network = omega_data::network();

    let mut flows: Vec<&Transaction> = network
        .transactions
        .iter()
        .filter(|transaction| {
            network.accounts[transaction.source].company == selected
                || network.accounts[transaction.target].company == selected
        })
        .collect();

    flows.reverse();

    let visible = area.height.saturating_sub(3) as usize;
    let max_scroll = flows.len().saturating_sub(visible);
    *scroll = (*scroll).min(max_scroll);
    let end = (*scroll + visible).min(flows.len());

    let header = Row::new(vec!["TIME", "SOURCE", "->", "TARGET", "SIGNAL", "AMOUNT"]).style(
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = flows[*scroll..end]
        .iter()
        .map(|transaction| {
            let time = format!(
                "{:02}:{:02}",
                (transaction.minute / 60) % 24,
                transaction.minute % 60
            );

            let color = match transaction.signal {
                "CIRCULAR" => Color::Red,
                "VELOCITY" => Color::Yellow,
                "CROSS-CLUSTER" => Color::Magenta,
                _ => Color::Cyan,
            };

            Row::new(vec![
                Cell::from(time),
                Cell::from(account_id(transaction.source)),
                Cell::from("->"),
                Cell::from(account_id(transaction.target)),
                Cell::from(transaction.signal).style(Style::default().fg(color)),
                Cell::from(money(transaction.amount)),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(5),
        Constraint::Length(10),
        Constraint::Length(3),
        Constraint::Length(10),
        Constraint::Min(12),
        Constraint::Length(10),
    ];

    f.render_widget(
        Table::new(rows, widths)
            .header(header)
            .column_spacing(1)
            .block(terminal_block(
                &format!(" TRANSACTION TRACE // {} LINKED FLOWS ", selected),
                Color::Cyan,
            )),
        area,
    );
}

fn render_engine_bar(f: &mut Frame, area: Rect, selected: &str) {
    let network = omega_data::network();

    let linked_accounts = network
        .accounts
        .iter()
        .filter(|account| account.company == selected)
        .count();

    let line = Line::from(vec![
        bold("ENTITY ", Color::Cyan),
        Span::raw(selected.to_string()),
        Span::raw("  |  "),
        bold("ACCOUNTS ", Color::Cyan),
        Span::raw(linked_accounts.to_string()),
        Span::raw("  |  "),
        bold("GRAPH ", Color::Cyan),
        Span::raw("CLUSTER / FLOW SIGNALS"),
        Span::raw("  |  "),
        bold("DATA ", Color::Yellow),
        Span::raw("SYNTHETIC"),
    ]);

    f.render_widget(
        Paragraph::new(line).block(
            Block::default()
                .borders(Borders::TOP | Borders::BOTTOM)
                .border_style(Style::default().fg(Color::DarkGray)),
        ),
        area,
    );
}

fn render_intelligence(f: &mut Frame, area: Rect, selected: &str, app: &mut App) {
    let [table_area, flow_area, engine_area] = Layout::vertical([
        Constraint::Percentage(62),
        Constraint::Percentage(34),
        Constraint::Min(1),
    ])
    .areas(area);

    render_risk_table(f, table_area, selected, &mut app.omega_risk_scroll);

    render_flow_table(f, flow_area, selected, &mut app.omega_flow_scroll);

    render_engine_bar(f, engine_area, selected);
}

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
            Hint::fixed("tab/1-9", "view"),
            Hint::act(&[Action::Up, Action::Down], "select ticker"),
            Hint::act(&[Action::FindTicker], "find"),
            Hint::act(&[Action::NextPreset], "interval"),
            Hint::act(&[Action::Refresh], "refresh"),
            Hint::act(&[Action::Settings], "settings"),
            Hint::act(&[Action::Help], "help"),
            Hint::act(&[Action::PageUp, Action::PageDown], "scroll data"),
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
        let [workspace, watchlist_area] =
            Layout::vertical([Constraint::Min(10), Constraint::Length(5)]).areas(area);

        let [left, right] =
            Layout::horizontal([Constraint::Percentage(62), Constraint::Percentage(38)])
                .areas(workspace);

        let [top_chart, bottom_chart] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(left);

        let selected = app.selected_symbol().to_string();

        let next_symbol = if app.symbols.is_empty() {
            selected.clone()
        } else {
            app.symbols[(app.selected + 1) % app.symbols.len()].clone()
        };

        chart::render_price_chart_for_symbol(f, top_chart, app, &selected);

        chart::render_price_chart_for_symbol(f, bottom_chart, app, &next_symbol);

        render_intelligence(f, right, &selected, app);

        watchlist::render_table(f, watchlist_area, app);
    }
}
