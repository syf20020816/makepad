
use gen_components::GButtonWidgetRefExt;
use makepad_widgets::*;

live_design!{
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use link::gen_components::*;
    
    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                body = <View>{
                    flow: Down,
                    spacing: 10,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    show_bg: true,
                    draw_bg:{
                        fn pixel(self) -> vec4 {
                            // let center = vec2(0.5, 0.5);
                            // let uv = self.pos - center;
                            // let radius = length(uv);
                            // let angle = atan(uv.y, uv.x);
                            // let color1 = mix(#f00, #00f, 0.5 + 10.5 * cos(angle + self.time));
                            // let color2 = mix(#0f0, #ff0, 0.5 + 0.5 * sin(angle + self.time));
                            // return mix(color1, color2, radius);
                            return #ff0
                        }
                    }
                    header_view = <View> {
                        width: Fill,
                        height: Fit,
                        padding: {left: 12., right: 12., top: 12., bottom: 12.}
                        show_bg: true
                        draw_bg: {
                            color: #ff0000,
                            instance top_radius: 8.0,
                            
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                
                                // Draw the rounded rectangle path
                                // sdf.clear_path();
                                // sdf.move_to(0.0, self.rect_size.y);
                                // sdf.line_to(0.0, self.top_radius);
                                // sdf.fill(self.color);
                                sdf.arc2(self.top_radius, 9.0, self.top_radius, -PI * 0.5, .2);
                                
                                // sdf.line_to(self.rect_size.x - self.top_radius, 0.0);
                                // sdf.arc2(self.rect_size.x, self.top_radius, self.top_radius, 0.0);
                                // sdf.line_to(self.rect_size.x, self.rect_size.y);
                                // sdf.close_path();
                                
                    
                                sdf.fill(self.color);
                                // sdf.clear_path();
                                
                                return sdf.result;
                                // return #ff0000
                            }
                        }
                        
                        header_label = <Label> {
                            text: "hellow"
                            draw_text: {
                                color: #495057,
                                text_style: {
                                    font_size: 13.0,
                                }
                            }
                        }
                    }
                    
                    
                    // b0= <Button> {
                    //     text: "Click me 123"
                    //     draw_text:{color:#fff}
                    // }
                    // button1 = <Button> {
                    //     text: "Click me 123"
                    //     draw_text:{color:#fff}
                    // }
                    // button2 = <Button> {
                    //     text: "Click me 345"
                    //     draw_text:{color:#fff}
                    // }
                    // button3 = <GButton>{

                    // }
                    // <GImage>{
                    //     src: Url("https://avatars.githubusercontent.com/u/67356158?s=48&v=4")
                    // }
                }
            }
        }
    }
}  

app_main!(App); 
 
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] counter: usize,
    #[deref]
    prop: Prop,
}

#[derive(Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct Prop{
    #[live(100)]
    pub a: i32,
}

impl Default for Prop {
    fn default() -> Self {
        Self {
            a: 10,
        }
    }
}
 
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::gen_components::live_design(cx, None);
    }
}

impl MatchEvent for App{
    fn handle_actions(&mut self, _cx: &mut Cx, actions:&Actions){
        if self.ui.button(id!(button1)).clicked(&actions) {
            self.counter += 1;
            dbg!(self.a);
        }
        if let Some(param) = self.ui.gbutton(id!(button3)).clicked(&actions) {
            dbg!(param);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
