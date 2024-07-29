use crate::{makepad_derive_widget::*, makepad_draw::*, widget::*};
live_design! {
    BareStep = {{BareStep}} {}
}

#[derive(Live, LiveHook, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct BareStep {
    #[rust] draw_state: DrawStateWrap<()>
}

impl WidgetNode for BareStep{
    fn walk(&mut self, _cx:&mut Cx) -> Walk{
        Walk::default()
    }
        
    fn redraw(&mut self, _cx: &mut Cx){}
        
    fn find_widgets(&mut self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
    }
}   

impl Widget for BareStep {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, _walk: Walk) -> DrawStep {
        if self.draw_state.begin(cx, ()) {
            return DrawStep::make_step()
        }
        DrawStep::done()
    }
}