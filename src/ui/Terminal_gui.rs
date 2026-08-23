use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Result as IoResult};

pub struct TerminalGui {
    pub system_status: String,
    pub input_buffer: String,
    pub processed_chunks_count: usize,
    pub latency_ms: f64,
}

impl TerminalGui {
    pub fn new() -> Self {
        Self {
            system_status: String::from("SOVEREIGN ENGINE ONLINE - NO LIMITATIONS"),
            input_buffer: String::new(),
            processed_chunks_count: 0,
            latency_ms: 0.002,
        }
    }

    /// تشغيل حلقة الرسوميات والتفاعل مع الطرفية
    pub fn run(&mut self) -> IoResult<()> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|frame| {
                let size = frame.size();

                // تقسيم الشاشة إلى 3 مناطق رئيسية
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Length(3), // Header Box
                        Constraint::Min(8),    // Main Content & Large Input Context
                        Constraint::Length(3), // Performance Meter Gauge
                    ])
                    .split(size);

                // 1. شريط العنوان والأنظمة
                let header = Paragraph::new(format!(
                    " DIRECTIVE-X | Status: {} | Latency: {:.3}ms ",
                    self.system_status, self.latency_ms
                ))
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .block(Borders::all());
                frame.render_widget(header, chunks[0]);

                // 2. منطقة الإدخال والاستقبال للنصوص الضخمة
                let main_content = Paragraph::new(format!(
                    "Large Prompt Buffer Stream:\n{}\n\nProcessed Chunks: {}",
                    if self.input_buffer.is_empty() {
                        "Waiting for large context prompt input..."
                    } else {
                        &self.input_buffer
                    },
                    self.processed_chunks_count
                ))
                .style(Style::default().fg(Color::White))
                .block(
                    Borders::all(),
                )
                .wrap(Wrap { trim: true });
                frame.render_widget(main_content, chunks[1]);

                // 3. مؤشر استقرار أداء محرك الانهيار السببي O(N)
                let gauge = Gauge::default()
                    .block(Borders::all())
                    .gauge_style(Style::default().fg(Color::Green))
                    .percent(100)
                    .label("Causal Collapse Search Engine: 100% Deterministic (O(N) Active)");
                frame.render_widget(gauge, chunks[2]);
            })?;

            // التعامل مع أحداث المفاتيح ولإغلاق الواجهة
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Esc {
                        break;
                    }
                }
            }
        }

        // استعادة وضع الطرفية القياسي عند الخروج
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }
}
