use egui_glow::glow::HasContext;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};
use libmpv::{
    render::{OpenGLInitParams, RenderContext, RenderParam, RenderParamApiType},
    FileState, Mpv,
};
use std::rc::Rc;
use std::{env, ffi::c_void};

fn get_proc_address(ctx: &&ContextWrapper<PossiblyCurrent, Window>, name: &str) -> *mut c_void {
    ctx.get_proc_address(name) as *mut c_void
}

#[derive(Debug)]
enum UserEvent {
    MpvEventAvailable,
    RedrawRequested,
}

fn main() {
    unsafe {
        let (gl, _, window, event_loop) = {
            let event_loop = glutin::event_loop::EventLoop::<UserEvent>::with_user_event();
            let window_builder = glutin::window::WindowBuilder::new()
                .with_title("Hello triangle!")
                .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .with_hardware_acceleration(Some(true))
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();

            let gl = egui_glow::painter::Context::from_loader_function(|s| {
                window.get_proc_address(s) as *const _
            });
            (gl, "#version 410", window, event_loop)
        };

        let mut mpv = Mpv::new().expect("Error while creating MPV");
        let mut render_context = RenderContext::new(
            mpv.ctx.as_mut(),
            vec![
                RenderParam::ApiType(RenderParamApiType::OpenGl),
                RenderParam::InitParams(OpenGLInitParams {
                    get_proc_address,
                    ctx: &window,
                }),
            ],
        )
        .expect("Failed creating render context");
        mpv.event_context_mut().disable_deprecated_events().unwrap();
        let event_proxy = event_loop.create_proxy();
        render_context.set_update_callback(move || {
            event_proxy.send_event(UserEvent::RedrawRequested).unwrap();
        });
        let event_proxy = event_loop.create_proxy();
        mpv.event_context_mut().set_wakeup_callback(move || {
            event_proxy
                .send_event(UserEvent::MpvEventAvailable)
                .unwrap();
        });
        let path = env::args()
            .nth(1)
            .expect("Provide a filename as first argument");
        let render_context = Some(render_context);
        mpv.playlist_load_files(&[(&path, FileState::AppendPlay, None)])
            .unwrap();
        mpv.set_property("video-timing-offset", 0).unwrap();

        let gl = Rc::new(gl);
        let mut egui_glow = egui_glow::winit::EguiGlow::new(window.window(), gl.clone());

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => (),
                Event::MainEventsCleared => {
                    window.window().request_redraw();
                }
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => {
                    match event {
                        WindowEvent::Resized(physical_size) => {
                            window.resize(physical_size);
                        }
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => {}
                    }
                    egui_glow.on_event(&event);
                    window.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let egui_repaint = egui_glow.run(window.window(), |egui_ctx| {
                        egui::Area::new("my_area")
                            .fixed_pos(egui::pos2(100.0, 100.0))
                            .show(egui_ctx, |ui| {
                                egui::Frame::none()
                                    .fill(egui::Color32::BLACK)
                                    .inner_margin(10.0)
                                    .outer_margin(10.0)
                                    .show(ui, |ui| {
                                        ui.heading("MPV Overlay");
                                        if ui.button("Quit").clicked() {
                                            println!("clicked quit");
                                            *control_flow = ControlFlow::Exit;
                                        }
                                    })
                            });
                    });

                    if let Some(render_context) = &render_context {
                        let size = window.window().inner_size();
                        render_context
                            .render::<Window>(0, size.width as _, size.height as _, true)
                            .expect("Failed to draw on glutin window");
                        egui_glow.paint(window.window());
                        gl.disable(glow::FRAMEBUFFER_SRGB);
                        gl.disable(glow::BLEND);
                        window.swap_buffers().unwrap();
                    } else if egui_repaint {
                        egui_glow.paint(window.window());
                        window.swap_buffers().unwrap();
                    }
                }

                _ => (),
            }
        });
    }
}
