mod config;
mod display;
mod render;
mod animation;

use animation::{sunny::SunnyAnimation, AnimationController};
use config::Config;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use display::AsciiDisplay;
use render::TerminalRenderer;
use std::io;
use std::time::{Duration, Instant};

const REFRESH_INTERVAL: Duration = Duration::from_secs(60);
const FRAME_DELAY: Duration = Duration::from_millis(500);

fn main() -> io::Result<()> {
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            eprintln!("\nContinuing with default location (Berlin: 52.52°N, 13.41°E)");
            eprintln!("\nTo customize, create a config file at:");
            eprintln!("  $XDG_CONFIG_HOME/weathr/config.toml");
            eprintln!("  or ~/.config/weathr/config.toml");
            eprintln!("\nExample config.toml:");
            eprintln!("  [location]");
            eprintln!("  latitude = 52.52");
            eprintln!("  longitude = 13.41");
            eprintln!();
            Config::default()
        }
    };

    let mut renderer = TerminalRenderer::new()?;
    renderer.init()?;

    let result = run_app(&config, &mut renderer);

    renderer.cleanup()?;

    result
}

fn run_app(config: &Config, renderer: &mut TerminalRenderer) -> io::Result<()> {
    let house = AsciiDisplay::render_house();
    let sunny_animation = SunnyAnimation::new();
    let mut animation_controller = AnimationController::new();
    
    let mut last_update = Instant::now();
    let mut last_frame_time = Instant::now();

    loop {
        renderer.update_size()?;
        let (_term_width, term_height) = renderer.get_size();
        
        renderer.clear()?;

        let weather_info = format!(
            "Weather: Sunny | Location: {:.2}°N, {:.2}°E | Press 'q' to quit",
            config.location.latitude, config.location.longitude
        );
        
        renderer.render_line_colored(
            2,
            1,
            &weather_info,
            crossterm::style::Color::Cyan,
        )?;

        let animation_y = if term_height > 20 { 3 } else { 2 };
        animation_controller.render_frame(renderer, &sunny_animation, animation_y)?;

        let house_y = animation_y + 7;
        let house_strings: Vec<String> = house.iter().map(|s| s.to_string()).collect();
        renderer.render_centered(&house_strings, house_y)?;

        renderer.flush()?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        break
                    }
                    _ => {}
                }
            }
        }

        if last_frame_time.elapsed() >= FRAME_DELAY {
            animation_controller.next_frame(&sunny_animation);
            last_frame_time = Instant::now();
        }

        if last_update.elapsed() >= REFRESH_INTERVAL {
            last_update = Instant::now();
        }
    }

    Ok(())
}
