use makepad_widgets::SignalToUI;
use rust_pixel::{
    asset2sprite, context::Context, event::event_check, game::Render, render::{panel::Panel, sprite::Sprite}
};
use rust_pixel::asset::AssetType;
use crate::model::{SimpleModel, MYAPPH, MYAPPW};

#[derive(Default)]
pub struct SimpleRender {
    pub panel: Panel,
}

impl SimpleRender {
    pub fn new() -> Self {
        let mut panel = Panel::new();

        let bg = Sprite::new(0, 0, 200, 200);
        panel.add_sprite(bg, "back");
        Self { panel }
    }
    pub fn draw_tile(&mut self, ctx: &mut rust_pixel::context::Context, d: &mut SimpleModel) {
        let l = self.panel.get_sprite("t0");

        // make asset identifier...
        // in text mode, poker card asset file named n.txt
        let ext = "txt";
        let cn = if d.card == 0 {
            format!("poker/back.{}", ext)
        } else {
            format!("poker/{}.{}", d.card, ext)
        };
        // set sprite content by asset identifier...
        asset2sprite!(l, ctx, &cn);

        // set sprite position...
        l.set_pos(1, 7);
    }
}

impl Render for SimpleRender {
    type Model = SimpleModel;

    fn init(&mut self, context: &mut Context, data: &mut Self::Model) {
        context
            .adapter
            .init(MYAPPW + 2, MYAPPH, 0.5, 0.5, "myapp".to_string());
        self.panel.init(context);

        let gb = self.panel.get_sprite("back");
        asset2sprite!(gb, context, "back.txt");
        context.adapter.only_render_buffer();
    }

    fn handle_event(&mut self, context: &mut Context, data: &mut Self::Model, _dt: f32) {
        // if a Block.RedrawTile event checked, call draw_tile function...
        if event_check("Myapp.RedrawTile", "draw_tile") {
            self.draw_tile(context, data);
        }
    }

    fn handle_timer(&mut self, context: &mut Context, d: &mut Self::Model, _dt: f32) {
        // if event_check("Myapp.TestTimer", "test_timer") {
        //     let ml = self.panel.get_sprite("TIMER-MSG");
        //     ml.set_color_str(
        //         (context.stage / 6) as u16 % MYAPPW as u16,
        //         0,
        //         "Myapp",
        //         Color::Yellow,
        //         Color::Reset,
        //     );
        //     timer_fire("Myapp.TestTimer", 0);
        // }
    }

    fn draw(&mut self, ctx: &mut Context, d: &mut Self::Model, dt: f32) {
        // draw all compents in panel...
        self.panel.draw(ctx).unwrap();
        // set Signal
        SignalToUI::set_ui_signal();
    }
}
