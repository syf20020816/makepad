use crate::draw_game::DrawGame;
use crate::model::SimpleModel;
use crate::render::SimpleRender;
use makepad_widgets::*;
use rust_pixel::game::Game;
use rust_pixel::render::adapter::RenderCell;
use rust_pixel::util::get_project_path;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    GameView = {{GameView}}{
        height: 400,
        width: 500,
    }
}

#[derive(Live, Widget)]
pub struct GameView {
    #[live]
    #[redraw]
    pub draw_view: DrawGame,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    instance: Option<Game<SimpleModel, SimpleRender>>,
    #[rust]
    render_pool: Vec<RenderCell>,
}

impl Widget for GameView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 渲染前传入渲染池
        self.draw_view.render_pool = self.render_pool.clone();
        // 绘制game view
        self.draw_view.draw_walk(cx, walk);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if let Event::Signal = event {
            // 这里表示接收到了rust pixel的draw的信号, 进行重绘
            dbg!("signal get and redraw");
            self.render_and_redraw(cx);
        }
    }
}

impl LiveHook for GameView {
    fn after_apply_from_doc(&mut self, _cx: &mut Cx) {
        self.init_game();
    }
}

impl GameView {
    /// init game
    pub fn init_game(&mut self) {
        let m = SimpleModel::new();
        let r = SimpleRender::new();
        let pp = get_project_path();
        self.instance = Some(Game::new(m, r, "Simple Game", &pp));
        // g.init();
        self.instance.as_mut().unwrap().init();
    }
    pub fn run_game(&mut self, cx: &mut Cx) {
        self.instance.as_mut().unwrap().run().unwrap();
        self.render_and_redraw(cx);
    }

    pub fn render_and_redraw(&mut self, cx: &mut Cx) {
        // 获取渲染池
        self.render_pool = self
            .instance
            .as_mut()
            .unwrap()
            .context
            .adapter
            .get_base()
            .rbuf
            .clone();
        // 进行重绘
        self.redraw(cx);
    }
}

pub struct SimpleGame {
    g: Game<SimpleModel, SimpleRender>,
}

pub fn init_game() -> SimpleGame {
    let m = SimpleModel::new();
    let r = SimpleRender::new();
    let pp = get_project_path();
    let mut g = Game::new(m, r, "Simple Game", &pp);
    g.init();
    SimpleGame { g }
}

pub fn run() {
    let mut g = init_game().g;
    g.run().unwrap();
    g.render.panel.reset(&mut g.context);
}
