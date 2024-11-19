use makepad_widgets::*;
use rust_pixel::render::adapter::AdapterBase;

use crate::game_view::GameViewWidgetRefExt;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import crate::game_view::*;

    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                body = <ScrollXYView>{
                    flow: Down,
                    spacing:10,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    gview = <GameView>{}
                    button1 = <Button> {
                        text: "Play",
                        padding: {left: 12.0, right: 12.0, top: 8.0, bottom: 8.0},
                    }
                    // input1 = <TextInput> {
                    //     width: 200.0,
                    //     text: "Click to count"
                    // }
                    // label1 = <Label> {
                    //     draw_text: {
                    //         color: #f
                    //     },
                    //     text: r#"Lorem ipsum dolor sit amet"#,
                    //     width: 200.0,
                    // }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::game_view::live_design(cx);
        crate::draw_game::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(button1)).clicked(&actions) {
            self.ui.game_view(id!(gview)).borrow_mut().map(|mut x|{
                x.run_game(cx);
            });
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
