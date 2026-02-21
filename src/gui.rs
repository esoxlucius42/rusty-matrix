use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::window::Window;
use winit::event::{Event, WindowEvent, KeyEvent, ElementState};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::event_loop::EventLoopWindowTarget;

use crate::renderer::Renderer;
use crate::rain::RainSimulation;

const TARGET_FPS: f32 = 75.0;
const TARGET_FRAME_TIME: Duration = Duration::from_micros((1_000_000.0 / TARGET_FPS) as u64);

pub struct App {
    renderer: Option<Renderer>,
    rain: RainSimulation,
    window: Arc<Window>,
    last_frame_time: Instant,
    frame_count: u32,
}

impl App {
    pub async fn new(window: Arc<Window>) -> Self {
        let renderer = Renderer::new(window.clone()).await;
        let rain = RainSimulation::new(1280, 720);

        Self {
            renderer: Some(renderer),
            rain,
            window,
            last_frame_time: Instant::now(),
            frame_count: 0,
        }
    }

    pub fn handle_event(
        &mut self,
        event: &Event<()>,
        target: &EventLoopWindowTarget<()>,
    ) {
        match event {
            Event::WindowEvent {
                window_id: _,
                event: window_event,
            } => {
                self.handle_window_event(window_event, target);
            }
            Event::AboutToWait => {
                self.window.request_redraw();
            }
            _ => {}
        }
    }

    fn handle_window_event(
        &mut self,
        event: &WindowEvent,
        target: &EventLoopWindowTarget<()>,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                target.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                if self.window.fullscreen().is_some() {
                    self.window.set_fullscreen(None);
                } else {
                    target.exit();
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.on_window_resized(*size);
                    self.rain.resize(size.width as usize, size.height as usize);
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::F11),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                let fullscreen = match self.window.fullscreen() {
                    None => Some(winit::window::Fullscreen::Borderless(None)),
                    Some(_) => None,
                };
                self.window.set_fullscreen(fullscreen);
            }
            WindowEvent::RedrawRequested => {
                // Implement 75 FPS hard limiter
                let now = Instant::now();
                let elapsed = now.duration_since(self.last_frame_time);

                if elapsed < TARGET_FRAME_TIME {
                    let sleep_time = TARGET_FRAME_TIME - elapsed;
                    std::thread::sleep(sleep_time);
                }

                let frame_start = Instant::now();
                self.frame_count = self.frame_count.wrapping_add(1);

                // Log FPS every 60 frames (approximately every 0.8 seconds at 75 FPS)
                if self.frame_count % 60 == 0 {
                    eprintln!("Frame: {}", self.frame_count);
                }

                self.rain.update();
                if let Some(renderer) = &mut self.renderer {
                    match renderer.render_frame(&self.rain) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            renderer.resize_framebuffers();
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            target.exit();
                        }
                        Err(e) => {
                            eprintln!("Render error: {}", e);
                        }
                    }
                }

                self.last_frame_time = frame_start;
            }
            _ => {}
        }
    }
}
