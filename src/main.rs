use candelabre_core::{CandlRenderer, CandlUpdate};
use candelabre_windowing::{
    CandlDimension, CandlManager, CandlOptions, CandlSurface,
    CandlSurfaceBuilder, CandlWindow as _
};
use gl;
use glutin::dpi::{PhysicalPosition, PhysicalSize};
use glutin::event::{
    ElementState, Event, KeyboardInput, StartCause,
    VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};
use nvg::{Context as NvgContext};
use nvg_gl::Renderer;
use std::cmp;

// ##### UPDATE ###############################################################

struct PocUpdate;

impl CandlUpdate<()> for PocUpdate {
    fn update(&mut self, _: ()) {}
}

impl PocUpdate {
    fn new() -> Self { PocUpdate }
}

// ##### RENDERER #############################################################

struct PocRenderer {
    nvg_contexts: Vec<NvgContext<Renderer>>,
    size: (u32, u32)
}

impl CandlRenderer<PocRenderer, PocUpdate, ()> for PocRenderer {
    fn init() -> Self {
        Self {nvg_contexts: vec!(), size: (0, 0)}
    }

    fn finalize(&mut self) {
        for _ in 0..4 {
            let renderer = Renderer::create().unwrap();
            let context = NvgContext::create(renderer).unwrap();
            self.nvg_contexts.push(context);
        }
    }

    fn set_scale_factor(&mut self, _: f64) {}

    fn set_size(&mut self, (w, h): (u32, u32)) {
        self.size = (w, h);
        //unsafe { gl::Viewport(0, 0, w as i32, h as i32); }
    }

    fn draw_frame(&mut self, _: &PocUpdate) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(
                gl::COLOR_BUFFER_BIT |
                gl::DEPTH_BUFFER_BIT |
                gl::STENCIL_BUFFER_BIT
            );
        }
        let (w, h) = self.size;
        let views = [
            [0, h/2],
            [w/2, h/2],
            [0, 0],
            [w/2, 0]
        ];
        for i in 0..cmp::min(4, self.nvg_contexts.len()) {
            unsafe { gl::Viewport(
                views[i][0] as i32,
                views[i][1] as i32,
                (w/2) as i32,
                (h/2) as i32
            ); }
            //
            let ctxt = &mut self.nvg_contexts[i];
            ctxt.begin_frame( nvg::Extent::new((w/2) as f32, (h/2) as f32), 1.0).unwrap();
            ctxt.save();
            ctxt.reset_transform();
            //
            if i == 0 {
                ctxt.begin_path();
                ctxt.rect(nvg::Rect::new(
                    nvg::Point::new(0.0, 0.0),
                    nvg::Extent::new(10.0, 10.0)
                ));
                ctxt.close_path();
                ctxt.fill_paint(nvg::Color::rgb_i(0, 255, 0));
                ctxt.fill().unwrap();
            }
            //
            ctxt.begin_path();
            //
            ctxt.rect(nvg::Rect::new(
                nvg::Point::new(10.0, 10.0),
                nvg::Extent::new(50.0, 50.0)
            ));
            //
            ctxt.close_path();
            //
            let color = if i == 0 {
                nvg::Color::rgb_i(0, 100, 255)
            } else if i == 1 {
                nvg::Color::rgb_i(0, 255, 100)
            } else if i == 2 {
                nvg::Color::rgb_i(100, 100, 0)
            } else {
                nvg::Color::rgb_i(255, 100, 0)
            };
            //
            ctxt.fill_paint(color);
            ctxt.fill().unwrap();
            //
            ctxt.restore();
            //
            ctxt.end_frame().unwrap();
            //
        }
    }
}

impl PocRenderer {
    //
    //
}

// ##### SURFACE ##############################################################

type PocSurface = CandlSurface<PocRenderer, PocUpdate, ()>;

// ##### MAIN #################################################################

fn main() {
    let el = EventLoop::new();
    let mut win_manager: CandlManager<PocSurface, ()> = CandlManager::new();
    let video_mode = el.primary_monitor().video_modes().next().unwrap();
    let (w, h) = {
        let size = el.primary_monitor().size();
        (size.width / 2 -20, size.height / 2 -50)
    };
    let dim = CandlDimension::Classic(w, h);
    let pos = [(2, 2), (w+10, 2), (2, h+50)];
    for wid in 0..3 {
        let title = &format!("PoC multiview - #{}", wid+1);
        let builder = CandlSurfaceBuilder::new()
            .dim(dim.clone())
            .video_mode(video_mode.clone())
            .title(title)
            .options(CandlOptions::default())
            .render(PocRenderer::init())
            .state(PocUpdate::new());
        let win_id = win_manager.create_window_from_builder(builder, &el).unwrap();
        let window = win_manager.get_current(win_id).unwrap().get_window().unwrap();
        window.set_outer_position(PhysicalPosition::new(pos[wid].0, pos[wid].1));
        window.set_inner_size(PhysicalSize::new(w, h));
    }
    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::NewEvents(StartCause::Init) =>
                *ctrl_flow = ControlFlow::Wait,
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, window_id} => match event {
                WindowEvent::CloseRequested => {
                    win_manager.remove_window(window_id);
                    if win_manager.is_empty() { *ctrl_flow = ControlFlow::Exit; }
                }
                WindowEvent::Resized(psize) =>
                    win_manager.get_current(window_id).unwrap().resize(psize),
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    }, ..
                } => *ctrl_flow = ControlFlow::Exit,
                _ => ()
            }
            Event::MainEventsCleared => {
                for wid in win_manager.list_window_ids() {
                    let surface = win_manager.get_current(wid).unwrap();
                    if surface.check_redraw() { surface.request_redraw(); }
                }
            }
            Event::RedrawRequested(win_id) =>
                win_manager.get_current(win_id).unwrap().draw(),
            _ => ()
        }
    });
}
