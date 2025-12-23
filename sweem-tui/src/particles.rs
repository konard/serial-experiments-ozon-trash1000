//! Particle system for background animations.
//!
//! This module implements a lightweight particle system that creates
//! a "Digital Rain" or "Starfield" effect in the background of the TUI.

use rand::Rng;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

/// Types of background animations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParticleMode {
    /// Matrix-style digital rain effect
    #[default]
    DigitalRain,
    /// Space starfield effect
    Starfield,
    /// No particles (static background)
    None,
}

impl ParticleMode {
    /// Cycle to the next mode
    pub fn next(&self) -> Self {
        match self {
            ParticleMode::DigitalRain => ParticleMode::Starfield,
            ParticleMode::Starfield => ParticleMode::None,
            ParticleMode::None => ParticleMode::DigitalRain,
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            ParticleMode::DigitalRain => "Digital Rain",
            ParticleMode::Starfield => "Starfield",
            ParticleMode::None => "None",
        }
    }
}

/// A single particle in the system
#[derive(Debug, Clone)]
pub struct Particle {
    /// X position (column)
    pub x: f32,
    /// Y position (row)
    pub y: f32,
    /// Velocity in Y direction
    pub vy: f32,
    /// Velocity in X direction (for starfield)
    pub vx: f32,
    /// Character to display
    pub char: char,
    /// Brightness (0.0 - 1.0)
    pub brightness: f32,
    /// Fade rate
    pub fade_rate: f32,
}

impl Particle {
    /// Create a new digital rain particle
    pub fn new_rain(x: u16, _max_y: u16) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: x as f32,
            y: 0.0,
            vy: rng.gen_range(0.3..1.5),
            vx: 0.0,
            char: Self::random_rain_char(),
            brightness: 1.0,
            fade_rate: rng.gen_range(0.01..0.05),
        }
    }

    /// Create a new starfield particle
    pub fn new_star(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0.0..width as f32),
            y: rng.gen_range(0.0..height as f32),
            vy: 0.0,
            vx: rng.gen_range(0.1..0.8),
            char: Self::random_star_char(),
            brightness: rng.gen_range(0.3..1.0),
            fade_rate: rng.gen_range(0.005..0.02),
        }
    }

    /// Get a random character for digital rain
    fn random_rain_char() -> char {
        let mut rng = rand::thread_rng();
        let chars: Vec<char> = "01アイウエオカキクケコサシスセソタチツテト".chars().collect();
        chars[rng.gen_range(0..chars.len())]
    }

    /// Get a random character for starfield
    fn random_star_char() -> char {
        let mut rng = rand::thread_rng();
        let chars = ['·', '•', '∙', '○', '◦', '*', '+', '×'];
        chars[rng.gen_range(0..chars.len())]
    }

    /// Update particle position and state
    pub fn update(&mut self) {
        self.y += self.vy;
        self.x += self.vx;
        self.brightness -= self.fade_rate;

        // Occasionally change the character (for rain effect)
        if rand::thread_rng().gen_ratio(1, 10) {
            self.char = Self::random_rain_char();
        }
    }

    /// Check if particle is still visible
    pub fn is_alive(&self, max_y: u16, max_x: u16) -> bool {
        self.brightness > 0.0 && self.y < max_y as f32 && self.x < max_x as f32
    }

    /// Get the color based on brightness
    pub fn get_color(&self, mode: ParticleMode) -> Color {
        match mode {
            ParticleMode::DigitalRain => {
                let intensity = (self.brightness * 255.0) as u8;
                Color::Rgb(0, intensity, intensity / 3)
            }
            ParticleMode::Starfield => {
                let intensity = (self.brightness * 255.0) as u8;
                Color::Rgb(intensity, intensity, intensity)
            }
            ParticleMode::None => Color::Reset,
        }
    }
}

/// The particle system managing all particles
#[derive(Debug, Clone)]
pub struct ParticleSystem {
    /// All active particles
    particles: Vec<Particle>,
    /// Current animation mode
    mode: ParticleMode,
    /// Maximum number of particles
    max_particles: usize,
    /// Frame counter for spawn timing
    frame_count: u64,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new(ParticleMode::DigitalRain, 100)
    }
}

impl ParticleSystem {
    /// Create a new particle system
    pub fn new(mode: ParticleMode, max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            mode,
            max_particles,
            frame_count: 0,
        }
    }

    /// Set the animation mode
    pub fn set_mode(&mut self, mode: ParticleMode) {
        if self.mode != mode {
            self.mode = mode;
            self.particles.clear();
        }
    }

    /// Get current mode
    pub fn mode(&self) -> ParticleMode {
        self.mode
    }

    /// Toggle to the next animation mode
    pub fn toggle_mode(&mut self) {
        self.set_mode(self.mode.next());
    }

    /// Update all particles and spawn new ones
    pub fn update(&mut self, width: u16, height: u16) {
        self.frame_count = self.frame_count.wrapping_add(1);

        if self.mode == ParticleMode::None {
            return;
        }

        // Update existing particles
        for particle in &mut self.particles {
            particle.update();
        }

        // Remove dead particles
        self.particles
            .retain(|p| p.is_alive(height, width));

        // Spawn new particles
        self.spawn_particles(width, height);
    }

    /// Spawn new particles based on mode
    fn spawn_particles(&mut self, width: u16, height: u16) {
        let mut rng = rand::thread_rng();

        match self.mode {
            ParticleMode::DigitalRain => {
                // Spawn a few new rain drops each frame
                if self.frame_count % 3 == 0 && self.particles.len() < self.max_particles {
                    let num_new = rng.gen_range(1..=3).min(self.max_particles - self.particles.len());
                    for _ in 0..num_new {
                        let x = rng.gen_range(0..width);
                        self.particles.push(Particle::new_rain(x, height));
                    }
                }
            }
            ParticleMode::Starfield => {
                // Maintain a steady number of stars
                while self.particles.len() < self.max_particles / 2 {
                    self.particles.push(Particle::new_star(width, height));
                }
            }
            ParticleMode::None => {}
        }
    }

    /// Render the particle system
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if self.mode == ParticleMode::None {
            return;
        }

        for particle in &self.particles {
            let x = particle.x as u16;
            let y = particle.y as u16;

            if x < area.width && y < area.height {
                let pos = (area.x + x, area.y + y);
                let color = particle.get_color(self.mode);
                buf[pos].set_char(particle.char);
                buf[pos].set_style(Style::default().fg(color));
            }
        }
    }
}

/// Widget wrapper for the particle system
pub struct ParticleWidget<'a> {
    system: &'a ParticleSystem,
}

impl<'a> ParticleWidget<'a> {
    pub fn new(system: &'a ParticleSystem) -> Self {
        Self { system }
    }
}

impl Widget for ParticleWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.system.render(area, buf);
    }
}
