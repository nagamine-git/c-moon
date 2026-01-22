//! TUI (Terminal User Interface) モジュール
//!
//! ratatuiを使用したリアルタイム進捗表示

use std::io::{self, Stdout};
use std::sync::{Arc, Mutex};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, Paragraph},
    Frame, Terminal,
};

use crate::layout::Layout as KeyboardLayout;

/// TUI状態
pub struct TuiState {
    pub generation: usize,
    pub max_generations: usize,
    pub best_fitness: f64,
    pub fitness_history: Vec<f64>,
    pub best_layout: Option<KeyboardLayout>,
    pub running: bool,
}

impl TuiState {
    pub fn new(max_generations: usize) -> Self {
        Self {
            generation: 0,
            max_generations,
            best_fitness: 0.0,
            fitness_history: Vec::with_capacity(max_generations),
            best_layout: None,
            running: true,
        }
    }

    pub fn update(&mut self, generation: usize, fitness: f64, layout: &KeyboardLayout) {
        self.generation = generation;
        if fitness > self.best_fitness {
            self.best_fitness = fitness;
            self.best_layout = Some(layout.clone());
        }
        self.fitness_history.push(fitness);
    }
}

/// TUIアプリケーション
pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TuiApp {
    /// TUIを初期化
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    /// TUIを描画
    pub fn draw(&mut self, state: &TuiState) -> io::Result<()> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),  // Progress bar
                    Constraint::Length(12), // Graph
                    Constraint::Min(10),    // Layout display
                ])
                .split(f.area());

            render_progress(f, chunks[0], state);
            render_graph(f, chunks[1], state);
            render_keyboard(f, chunks[2], state);
        })?;
        Ok(())
    }

    /// イベントをポーリング（ノンブロッキング）
    pub fn poll_event(&self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// TUIを終了
    pub fn cleanup(mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

/// プログレスバーを描画
fn render_progress(f: &mut Frame, area: Rect, state: &TuiState) {
    let progress = if state.max_generations > 0 {
        state.generation as f64 / state.max_generations as f64
    } else {
        0.0
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .percent((progress * 100.0) as u16)
        .label(format!(
            "Gen {}/{} | Best: {:.4}",
            state.generation, state.max_generations, state.best_fitness
        ));

    f.render_widget(gauge, area);
}

/// フィットネスグラフを描画
fn render_graph(f: &mut Frame, area: Rect, state: &TuiState) {
    let data: Vec<(f64, f64)> = state
        .fitness_history
        .iter()
        .enumerate()
        .map(|(i, &f)| (i as f64, f))
        .collect();

    if data.is_empty() {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Fitness History");
        f.render_widget(block, area);
        return;
    }

    let min_fitness = state
        .fitness_history
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min)
        .max(0.0);
    let max_fitness = state
        .fitness_history
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(min_fitness + 1.0);

    let datasets = vec![Dataset::default()
        .name("Fitness")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .data(&data)];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Fitness History (Gen 0 to Max)"),
        )
        .x_axis(
            Axis::default()
                .title("Generation")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, state.max_generations as f64])
                .labels(vec![
                    Span::raw("0"),
                    Span::raw(format!("{}", state.max_generations / 2)),
                    Span::raw(format!("{}", state.max_generations)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Fitness")
                .style(Style::default().fg(Color::Gray))
                .bounds([min_fitness - 5.0, max_fitness + 5.0])
                .labels(vec![
                    Span::raw(format!("{:.0}", min_fitness)),
                    Span::raw(format!("{:.0}", (min_fitness + max_fitness) / 2.0)),
                    Span::raw(format!("{:.0}", max_fitness)),
                ]),
        );

    f.render_widget(chart, area);
}

/// キーボード配列を描画
fn render_keyboard(f: &mut Frame, area: Rect, state: &TuiState) {
    let layout = match &state.best_layout {
        Some(l) => l,
        None => {
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Best Layout");
            f.render_widget(block, area);
            return;
        }
    };

    let mut lines: Vec<Line> = vec![Line::from(Span::styled(
        format!("Fitness: {:.4}", state.best_fitness),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ))];

    lines.push(Line::from(""));

    for (layer_idx, layer_name) in ["Layer 0 (無シフト)", "Layer 1 (☆)", "Layer 2 (★)"]
        .iter()
        .enumerate()
    {
        lines.push(Line::from(Span::styled(
            format!("{}:", layer_name),
            Style::default().fg(Color::Cyan),
        )));

        for row in 0..3 {
            let row_str: String = layout.layers[layer_idx][row]
                .iter()
                .map(|&c| if c == '　' { '□' } else { c })
                .collect::<Vec<_>>()
                .iter()
                .map(|c| format!("{} ", c))
                .collect();
            lines.push(Line::from(format!("  {}", row_str)));
        }
        lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Current Best Layout"),
    );

    f.render_widget(paragraph, area);
}

/// TUIスレッドを実行
pub fn run_tui_thread(state: Arc<Mutex<TuiState>>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut app = match TuiApp::new() {
            Ok(app) => app,
            Err(e) => {
                eprintln!("TUI error: {}", e);
                return;
            }
        };

        loop {
            {
                let state = state.lock().unwrap();
                if !state.running {
                    break;
                }
                if let Err(e) = app.draw(&state) {
                    eprintln!("TUI draw error: {}", e);
                    break;
                }
            }

            match app.poll_event() {
                Ok(true) => {
                    let mut state = state.lock().unwrap();
                    state.running = false;
                    break;
                }
                Err(e) => {
                    eprintln!("TUI event error: {}", e);
                    break;
                }
                _ => {}
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        if let Err(e) = app.cleanup() {
            eprintln!("TUI cleanup error: {}", e);
        }
    })
}
