use makepad_widgets::*;
use rust_pixel::render::adapter::RenderCell;

live_design!{
    DrawGame = {{DrawGame}}{        
        // precision mediump float;
        // uniform sampler2D source;

        // layout(location=0) in vertex: vec2;
        // layout(location=1) in a1: vec4;
        // layout(location=2) in a2: vec4;
        // layout(location=3) in a3: vec4;
        // layout(location=4) in color: vec4;

        // fn fragment() -> vec4 {
        //     return texture(source, uv) * colorj;
        // }

        // fn vertex(self) -> vec4 {
        //     let uv = a1.zw + vertex * a2.xy;
        //     let transformed = (((vertex - a1.xy) * mat2(a2.zw, a3.xy) + a3.zw) * mat2(tw.xy, th.xy) + vec2(tw.z, th.z)) / vec2(tw.w, th.w) * 2.0;
        //     self.gl_Position = vec4(transformed - vec2(1.0, 1.0), 0.0, 1.0);
        //     return color * colorFilter;
        // }

        fn pixel(self) -> vec4{
            return #f00;
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawGame{
    #[deref]
    #[live]
    super_shader: DrawQuad,
    #[rust]
    pub render_pool: Vec<RenderCell>,
}

