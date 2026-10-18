use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use std::{env, fs, path::PathBuf};

use crate::app::App;
use crate::ui::{View, ViewId};

const BG: Color = Color::Black;
const AMBER: Color = Color::Rgb(255, 180, 65);
const GREEN: Color = Color::Rgb(90, 220, 145);
const RED: Color = Color::Rgb(255, 85, 85);
const CYAN: Color = Color::Rgb(90, 195, 255);
const MUTED: Color = Color::Gray;
const WHITE: Color = Color::White;

#[derive(Default)]
struct Summary {
    transactions: String,
    relationships: String,
    entities: String,
    alerts: String,
    high: String,
    medium: String,
    low: String,
    circular: String,
    hub: String,
    dense: String,
    frequent: String,
    synthetic: bool,
}

struct Alert {
    id: String,
    kind: String,
    severity: String,
    score: String,
    entities: String,
}

struct Entity {
    id: String,
    score: String,
    severity: String,
    counterparties: String,
}

struct Relationship {
    sender: String,
    receiver: String,
    count: String,
    amount: String,
    alerts: String,
}

struct EngineData {
    summary: Summary,
    alerts: Vec<Alert>,
    entities: Vec<Entity>,
    relationships: Vec<Relationship>,
    error: Option<String>,
}

pub struct OmegaView;

impl View for OmegaView {
    fn id(&self) -> ViewId {
        ViewId::Omega
    }

    fn title(&self) -> &'static str {
        "OMEGA"
    }

    fn render(&self, f: &mut Frame, area: Rect, _app: &mut App) {
        let data = load_engine_data();

        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(5),
                Constraint::Min(8),
                Constraint::Length(1),
            ])
            .split(area);

        let header = Paragraph::new(Line::from(vec![
            Span::styled(
                " MULENETX OMEGA ",
                Style::default()
                    .fg(BG)
                    .bg(AMBER)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  FINANCIAL CRIME INTELLIGENCE  ",
                Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
            ),
            Span::styled("HEURISTIC ANALYSIS", Style::default().fg(CYAN)),
        ]))
        .style(Style::default().bg(BG));

        f.render_widget(header, sections[0]);

        let s = &data.summary;
        let metrics = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(sections[1]);

        metric(f, metrics[0], "TRANSACTIONS", &s.transactions, CYAN);
        metric(f, metrics[1], "ENTITIES", &s.entities, AMBER);
        metric(f, metrics[2], "RELATIONSHIPS", &s.relationships, GREEN);
        metric(f, metrics[3], "TOTAL ALERTS", &s.alerts, RED);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(67), Constraint::Percentage(33)])
            .split(sections[2]);

        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(56), Constraint::Percentage(44)])
            .split(columns[1]);

        graph_panel(f, columns[0], &data);
        alert_panel(f, right[0], &data);
        entity_panel(f, right[1], &data);

        let footer = if let Some(error) = &data.error {
            Line::from(vec![
                Span::styled(" ENGINE ERROR ", Style::default().fg(BG).bg(RED)),
                Span::styled(format!(" {error}"), Style::default().fg(RED)),
            ])
        } else {
            Line::from(vec![
                Span::styled(" SYNTHETIC DATA ", Style::default().fg(AMBER)),
                Span::styled(
                    format!("  HIGH {}  MEDIUM {}  LOW {}", s.high, s.medium, s.low),
                    Style::default().fg(WHITE),
                ),
                Span::styled(
                    "  |  Risk signals only — not proof of wrongdoing",
                    Style::default().fg(MUTED),
                ),
            ])
        };

        f.render_widget(Paragraph::new(footer), sections[3]);
    }
}

fn metric(f: &mut Frame, area: Rect, label: &str, value: &str, color: Color) {
    let lines = vec![
        Line::styled(
            label,
            Style::default().fg(MUTED).add_modifier(Modifier::BOLD),
        ),
        Line::styled(
            if value.is_empty() { "--" } else { value },
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
    ];

    f.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color)),
        ),
        area,
    );
}

fn alert_panel(f: &mut Frame, area: Rect, data: &EngineData) {
    let rows = data.alerts.iter().take(12).map(|a| {
        let color = severity_color(&a.severity);

        Row::new(vec![
            Cell::from(a.id.clone()),
            Cell::from(a.kind.clone()),
            Cell::from(a.severity.clone()).style(Style::default().fg(color)),
            Cell::from(a.score.clone()),
            Cell::from(a.entities.clone()),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Percentage(30),
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Min(10),
        ],
    )
    .header(
        Row::new(["ALERT", "PATTERN", "LEVEL", "SCORE", "ENTITIES"])
            .style(Style::default().fg(AMBER).add_modifier(Modifier::BOLD)),
    )
    .block(
        Block::default()
            .title(" DETECTION FEED ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(AMBER)),
    )
    .row_highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(table, area);
}

fn entity_panel(f: &mut Frame, area: Rect, data: &EngineData) {
    let rows = data.entities.iter().take(7).map(|e| {
        Row::new(vec![
            Cell::from(e.id.clone()),
            Cell::from(e.score.clone()),
            Cell::from(e.severity.clone()).style(Style::default().fg(severity_color(&e.severity))),
            Cell::from(e.counterparties.clone()),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(6),
            Constraint::Length(9),
            Constraint::Min(8),
        ],
    )
    .header(
        Row::new(["ENTITY", "RISK", "LEVEL", "COUNTERPARTIES"])
            .style(Style::default().fg(CYAN).add_modifier(Modifier::BOLD)),
    )
    .block(
        Block::default()
            .title(" ENTITY RISK ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(CYAN)),
    );

    f.render_widget(table, area);
}

fn graph_panel(f: &mut Frame, area: Rect, data: &EngineData) {
    let block = Block::default()
        .title(" ENTITY NETWORK / DIRECTED FLOWS ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(CYAN));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let width = inner.width as usize;
    let height = inner.height as usize;
    if width < 20 || height < 8 {
        f.render_widget(
            Paragraph::new("Enlarge terminal to view the network graph")
                .style(Style::default().fg(MUTED)),
            inner,
        );
        return;
    }

    let mut ids: Vec<String> = Vec::new();
    for entity in data.entities.iter().take(14) {
        if !entity.id.is_empty() && !ids.contains(&entity.id) {
            ids.push(entity.id.clone());
        }
    }
    for rel in data.relationships.iter().take(40) {
        for id in [&rel.sender, &rel.receiver] {
            if !id.is_empty() && ids.len() < 18 && !ids.contains(id) {
                ids.push(id.clone());
            }
        }
    }
    if ids.is_empty() {
        f.render_widget(
            Paragraph::new("No entities loaded. Check the engine output directory.")
                .style(Style::default().fg(RED)),
            inner,
        );
        return;
    }

    let mut grid = vec![vec![(' ', MUTED); width]; height];
    let mut positions: std::collections::HashMap<String, (usize, usize)> =
        std::collections::HashMap::new();
    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0;
    let rx = (width as f64 * 0.40).max(5.0);
    let ry = (height as f64 * 0.34).max(2.0);

    for (i, id) in ids.iter().enumerate() {
        let angle = -std::f64::consts::FRAC_PI_2
            + std::f64::consts::TAU * i as f64 / ids.len().max(1) as f64;
        let x = (cx + rx * angle.cos()).round().clamp(1.0, (width - 2) as f64) as usize;
        let y = (cy + ry * angle.sin()).round().clamp(1.0, (height - 2) as f64) as usize;
        positions.insert(id.clone(), (x, y));
    }

    // Draw directed relationships first so the entity labels remain legible.
    for rel in data.relationships.iter().take(42) {
        let (Some(&(x0, y0)), Some(&(x1, y1))) =
            (positions.get(&rel.sender), positions.get(&rel.receiver))
        else {
            continue;
        };
        if (x0, y0) == (x1, y1) {
            continue;
        }
        let flagged = !rel.alerts.trim().is_empty() && rel.alerts != "-";
        let color = if flagged { RED } else { Color::Rgb(55, 105, 125) };
        draw_edge(&mut grid, x0, y0, x1, y1, color);
    }

    // Draw nodes on top of the edges. Use actual engine entity IDs and risk.
    for id in &ids {
        let Some(&(x, y)) = positions.get(id) else { continue };
        let entity = data.entities.iter().find(|e| e.id == *id);
        let color = entity
            .map(|e| severity_color(&e.severity))
            .unwrap_or(CYAN);
        let label = format!("{}", id.chars().take(7).collect::<String>());
        let label = format!("[{}]", label);
        let start = x.saturating_sub(label.chars().count() / 2);
        for (offset, ch) in label.chars().enumerate() {
            if y < height && start + offset < width {
                grid[y][start + offset] = (ch, color);
            }
        }
    }

    let lines: Vec<Line> = grid
        .into_iter()
        .map(|row| {
            let mut spans: Vec<Span> = Vec::new();
            for (ch, color) in row {
                if let Some(last) = spans.last_mut() {
                    if last.style.fg == Some(color) {
                        let mut content = last.content.to_string();
                        content.push(ch);
                        *last = Span::styled(content, Style::default().fg(color));
                        continue;
                    }
                }
                spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
            }
            Line::from(spans)
        })
        .collect();

    f.render_widget(Paragraph::new(lines).style(Style::default().bg(BG)), inner);
}

fn draw_edge(grid: &mut [Vec<(char, Color)>], x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
    let width = grid.first().map_or(0, |row| row.len()) as isize;
    let height = grid.len() as isize;
    if width == 0 || height == 0 { return; }

    let (mut x, mut y) = (x0 as isize, y0 as isize);
    let (tx, ty) = (x1 as isize, y1 as isize);
    let dx = (tx - x).abs();
    let sx = if x < tx { 1 } else { -1 };
    let dy = -(ty - y).abs();
    let sy = if y < ty { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x >= 0 && x < width && y >= 0 && y < height {
            let ch = if dx.abs() > (-dy).abs() * 2 { '─' }
                else if (-dy).abs() > dx.abs() * 2 { '│' }
                else if (tx - x0 as isize) * (ty - y0 as isize) >= 0 { '╲' }
                else { '╱' };
            grid[y as usize][x as usize] = (ch, color);
        }
        if x == tx && y == ty { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x += sx; }
        if e2 <= dx { err += dx; y += sy; }
    }
}

fn severity_color(severity: &str) -> Color {
    match severity.to_ascii_uppercase().as_str() {
        "HIGH" => RED,
        "MEDIUM" => AMBER,
        "LOW" => GREEN,
        _ => WHITE,
    }
}

fn load_engine_data() -> EngineData {
    let Some(output) = find_output_dir() else {
        return EngineData {
            error: Some("Cannot find engine output directory".into()),
            ..EngineData::default()
        };
    };

    let summary_text = fs::read_to_string(output.join("summary.json")).ok();
    let alerts_text = fs::read_to_string(output.join("alerts.csv")).ok();
    let entities_text = fs::read_to_string(output.join("entities.csv")).ok();
    let relationships_text = fs::read_to_string(output.join("relationships.csv")).ok();

    let mut data = EngineData::default();

    if let Some(text) = summary_text.as_deref() {
        data.summary = parse_summary(text);
    }

    if let Some(text) = alerts_text.as_deref() {
        data.alerts = parse_alerts(text);
    }

    if let Some(text) = entities_text.as_deref() {
        data.entities = parse_entities(text);
    }

    if let Some(text) = relationships_text.as_deref() {
        data.relationships = parse_relationships(text);
    }

    if summary_text.is_none()
        || alerts_text.is_none()
        || entities_text.is_none()
        || relationships_text.is_none()
    {
        data.error = Some(format!(
            "Some engine files are missing in {}",
            output.display()
        ));
    }

    data
}

impl Default for EngineData {
    fn default() -> Self {
        Self {
            summary: Summary::default(),
            alerts: Vec::new(),
            entities: Vec::new(),
            relationships: Vec::new(),
            error: None,
        }
    }
}

fn find_output_dir() -> Option<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(current) = env::current_dir() {
        candidates.push(
            current
                .join("..")
                .join("omega_engine")
                .join("omega_engine")
                .join("output"),
        );

        candidates.push(
            current
                .join("omega_engine")
                .join("omega_engine")
                .join("output"),
        );
    }

    if let Ok(profile) = env::var("USERPROFILE") {
        candidates.push(
            PathBuf::from(profile)
                .join("Downloads")
                .join("omega_engine")
                .join("omega_engine")
                .join("output"),
        );
    }

    candidates
        .into_iter()
        .find(|p| p.join("summary.json").is_file() && p.join("alerts.csv").is_file())
}

fn json_number(text: &str, key: &str) -> String {
    let needle = format!("\"{key}\"");
    let Some(start) = text.find(&needle) else {
        return String::new();
    };

    let rest = &text[start + needle.len()..];
    let Some(colon) = rest.find(':') else {
        return String::new();
    };

    rest[colon + 1..]
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect()
}

fn json_object_number(text: &str, object: &str, key: &str) -> String {
    let needle = format!("\"{object}\"");
    let Some(start) = text.find(&needle) else {
        return String::new();
    };

    let rest = &text[start + needle.len()..];
    let Some(open) = rest.find('{') else {
        return String::new();
    };

    let object_text = &rest[open + 1..];
    let Some(close) = object_text.find('}') else {
        return String::new();
    };

    json_number(&object_text[..close], key)
}

fn parse_summary(text: &str) -> Summary {
    Summary {
        transactions: json_number(text, "transactions"),
        relationships: json_number(text, "directed_relationships"),
        entities: json_number(text, "entities"),
        alerts: json_number(text, "alerts"),
        high: json_object_number(text, "by_severity", "HIGH"),
        medium: json_object_number(text, "by_severity", "MEDIUM"),
        low: json_object_number(text, "by_severity", "LOW"),
        circular: json_object_number(text, "by_type", "CIRCULAR_FLOW"),
        hub: json_object_number(text, "by_type", "HUB_PASS_THROUGH"),
        dense: json_object_number(text, "by_type", "DENSE_CONNECTION"),
        frequent: json_object_number(text, "by_type", "FREQUENT_TRANSFERS"),
        synthetic: text.contains("Synthetic data"),
    }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut chars = line.chars().peekable();
    let mut quoted = false;

    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                fields.push(std::mem::take(&mut field));
            }
            _ => field.push(ch),
        }
    }

    fields.push(field);
    fields
}

fn column(headers: &[String], name: &str) -> Option<usize> {
    headers.iter().position(|h| h.trim() == name)
}

fn field(fields: &[String], headers: &[String], name: &str) -> String {
    column(headers, name)
        .and_then(|i| fields.get(i))
        .cloned()
        .unwrap_or_default()
}

fn parse_alerts(text: &str) -> Vec<Alert> {
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };

    let headers = parse_csv_line(header);

    lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let fields = parse_csv_line(line);
            let entities = field(&fields, &headers, "entities")
                .split(';')
                .take(4)
                .collect::<Vec<_>>()
                .join(";");

            Alert {
                id: field(&fields, &headers, "alert_id"),
                kind: field(&fields, &headers, "type"),
                severity: field(&fields, &headers, "severity"),
                score: field(&fields, &headers, "score"),
                entities,
            }
        })
        .collect()
}

fn parse_entities(text: &str) -> Vec<Entity> {
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };

    let headers = parse_csv_line(header);

    let mut entities: Vec<Entity> = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let fields = parse_csv_line(line);

            Entity {
                id: field(&fields, &headers, "entity_id"),
                score: field(&fields, &headers, "risk_score"),
                severity: field(&fields, &headers, "severity"),
                counterparties: field(&fields, &headers, "counterparties"),
            }
        })
        .collect();

    entities.sort_by(|a, b| {
        b.score
            .parse::<i32>()
            .unwrap_or(0)
            .cmp(&a.score.parse::<i32>().unwrap_or(0))
    });

    entities
}

fn parse_relationships(text: &str) -> Vec<Relationship> {
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };

    let headers = parse_csv_line(header);

    let mut relationships: Vec<Relationship> = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let fields = parse_csv_line(line);

            Relationship {
                sender: field(&fields, &headers, "sender"),
                receiver: field(&fields, &headers, "receiver"),
                count: field(&fields, &headers, "txn_count"),
                amount: field(&fields, &headers, "total_amount"),
                alerts: field(&fields, &headers, "alert_ids"),
            }
        })
        .collect();

    relationships.sort_by(|a, b| b.alerts.is_empty().cmp(&a.alerts.is_empty()));

    relationships
}
