fn push_render_buffer(
    rbuf: &mut Vec<RenderCell>,
    fc: &(u8, u8, u8, u8),
    bgc: &Option<(u8, u8, u8, u8)>,
    texidx: usize,
    symidx: usize,
    s: ARect,
    angle: f64,
    ccp: &PointI32,
) {
    let mut wc = RenderCell {
        fcolor: (
            fc.0 as f32 / 255.0,
            fc.1 as f32 / 255.0,
            fc.2 as f32 / 255.0,
            fc.3 as f32 / 255.0,
        ),
        ..Default::default()
    };
    if let Some(bc) = bgc {
        wc.bcolor = Some((
            bc.0 as f32 / 255.0,
            bc.1 as f32 / 255.0,
            bc.2 as f32 / 255.0,
            bc.3 as f32 / 255.0,
        ));
    } else {
        wc.bcolor = None;
    }
    let x = symidx as u32 % 16u32 + (texidx as u32 % 8u32) * 16u32;
    let y = symidx as u32 / 16u32 + (texidx as u32 / 8u32) * 16u32;
    wc.texsym = (y * 16u32 * 8u32 + x) as usize;
    wc.x = s.x as f32 + PIXEL_SYM_WIDTH.get().expect("lazylock init");
    wc.y = s.y as f32 + PIXEL_SYM_HEIGHT.get().expect("lazylock init");
    wc.w = s.w;
    wc.h = s.h;
    if angle == 0.0 {
        wc.angle = angle as f32;
    } else {
        let mut aa = (1.0 - angle / 180.0) * std::f64::consts::PI;
        let pi2 = std::f64::consts::PI * 2.0;
        while aa < 0.0 {
            aa += pi2;
        }
        while aa > pi2 {
            aa -= pi2;
        }
        wc.angle = aa as f32;
    }
    wc.cx = ccp.x as f32;
    wc.cy = ccp.y as f32;
    rbuf.push(wc);
}

pub fn render_border<F>(cell_w: u16, cell_h: u16, rx: f32, ry: f32, mut f: F)
where
    F: FnMut(&(u8, u8, u8, u8), &Option<(u8, u8, u8, u8)>, ARect, ARect, ARect, usize, usize),
{
    let sh_top = (102u8, 1u8, Color::Indexed(7), Color::Reset);
    let sh_other = (24u8, 2u8, Color::Indexed(7), Color::Reset);
    let sh_close = (214u8, 1u8, Color::Indexed(7), Color::Reset);

    for n in 0..cell_h as usize + 2 {
        for m in 0..cell_w as usize + 2 {
            if n != 0 && n != cell_h as usize + 1 && m != 0 && m != cell_w as usize + 1 {
                continue;
            }
            let rsh;
            if n == 0 {
                if m as u16 <= cell_w {
                    rsh = &sh_top;
                } else {
                    rsh = &sh_close;
                }
            } else {
                rsh = &sh_other;
            }
            let (s0, s1, s2, texidx, symidx) = render_helper(
                cell_w + 2,
                PointF32 { x: rx, y: ry },
                n * (cell_w as usize + 2) + m,
                rsh,
                PointU16 { x: 0, y: 0 },
                true,
            );
            let fc = rsh.2.get_rgba();
            let bc = None;
            f(&fc, &bc, s0, s1, s2, texidx, symidx);
        }
    }
}
fn render_helper(
    cell_w: u16,
    r: PointF32,
    i: usize,
    sh: &(u8, u8, Color, Color),
    p: PointU16,
    is_border: bool,
) -> (ARect, ARect, ARect, usize, usize) {
    let w = *PIXEL_SYM_WIDTH.get().expect("lazylock init") as i32;
    let h = *PIXEL_SYM_HEIGHT.get().expect("lazylock init") as i32;
    let dstx = i as u16 % cell_w;
    let dsty = i as u16 / cell_w;
    let tex_count = 64;
    let tx = if sh.1 < tex_count { sh.1 as usize } else { 1 };
    let srcy = sh.0 as u32 / w as u32 + (tx as u32 / 2u32) * w as u32;
    let srcx = sh.0 as u32 % w as u32 + (tx as u32 % 2u32) * w as u32;
    let bsrcy = 160u32 / w as u32;
    let bsrcx = 160u32 % w as u32 + w as u32;

    (
        // background sym rect in texture(sym=160 tex=1)
        ARect {
            x: w * bsrcx as i32,
            y: h * bsrcy as i32,
            w: w as u32,
            h: h as u32,
        },
        // sym rect in texture
        ARect {
            x: w * srcx as i32,
            y: h * srcy as i32,
            w: w as u32,
            h: h as u32,
        },
        // dst rect in render texture
        ARect {
            x: (dstx + if is_border { 0 } else { 1 }) as i32 * (w as f32 / r.x) as i32 + p.x as i32,
            y: (dsty + if is_border { 0 } else { 1 }) as i32 * (h as f32 / r.y) as i32 + p.y as i32,
            w: (w as f32 / r.x) as u32,
            h: (h as f32 / r.y) as u32,
        },
        // texture id
        tx,
        // sym id
        sh.0 as usize,
    )
}

pub fn render_main_buffer<F>(buf: &Buffer, width: u16, rx: f32, ry: f32, border: bool, mut f: F)
where
    F: FnMut(&(u8, u8, u8, u8), &Option<(u8, u8, u8, u8)>, ARect, ARect, ARect, usize, usize),
{
    for (i, cell) in buf.content.iter().enumerate() {
        // symidx, texidx, fg, bg
        let sh = cell.get_cell_info();
        let (s0, s1, s2, texidx, symidx) = render_helper(
            width,
            PointF32 { x: rx, y: ry },
            i,
            &sh,
            PointU16 { x: 0, y: 0 },
            border,
        );
        let fc = sh.2.get_rgba();
        let bc = if sh.3 != Color::Reset {
            Some(sh.3.get_rgba())
        } else {
            None
        };
        f(&fc, &bc, s0, s1, s2, texidx, symidx);
    }
}

pub fn render_pixel_sprites<F>(pixel_spt: &mut Sprites, rx: f32, ry: f32, mut f: F)
where
    // rgba, back rgba, back rect, sym rect, dst rect, tex, sym, angle, center point
    F: FnMut(
        &(u8, u8, u8, u8),
        &Option<(u8, u8, u8, u8)>,
        ARect,
        ARect,
        ARect,
        usize,
        usize,
        f64,
        PointI32,
    ),
{
    // sort by render_weight...
    pixel_spt.update_render_index();
    for si in &pixel_spt.render_index {
        let s = &pixel_spt.sprites[si.0];
        if s.is_hidden() {
            continue;
        }
        let px = s.content.area.x;
        let py = s.content.area.y;
        let pw = s.content.area.width;
        let ph = s.content.area.height;

        for (i, cell) in s.content.content.iter().enumerate() {
            let sh = &cell.get_cell_info();
            let (s0, s1, s2, texidx, symidx) = render_helper(
                pw,
                PointF32 { x: rx, y: ry },
                i,
                sh,
                PointU16 { x: px, y: py },
                false,
            );
            let x = i % pw as usize;
            let y = i / pw as usize;
            // center point ...
            let ccp = PointI32 {
                x: ((pw as f32 / 2.0 - x as f32) * PIXEL_SYM_WIDTH.get().expect("lazylock init") / rx) as i32,
                y: ((ph as f32 / 2.0 - y as f32) * PIXEL_SYM_HEIGHT.get().expect("lazylock init") / ry) as i32,
            };
            let mut fc = sh.2.get_rgba();
            fc.3 = s.alpha;
            let bc;
            if sh.3 != Color::Reset {
                let mut brgba = sh.3.get_rgba();
                brgba.3 = s.alpha;
                bc = Some(brgba);
            } else {
                bc = None;
            }
            f(&fc, &bc, s0, s1, s2, texidx, symidx, s.angle, ccp);
        }
    }
}



#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl SimpleGame {
    pub fn run() -> Self{
        init_game()
    }
    pub fn tick(&mut self, dt: f32) {
        self.g.on_tick(dt);
    }
    pub fn key_event(&mut self, t: u8, e: web_sys::Event) {
        let abase = &self
            .g
            .context
            .adapter
            .as_any()
            .downcast_ref::<WebAdapter>()
            .unwrap()
            .base;
        if let Some(pe) = input_events_from_web(t, e, abase.pixel_h, abase.ratio_x, abase.ratio_y) {
            self.g.context.input_events.push(pe);
        }
    }

    pub fn upload_imgdata(&mut self, w: i32, h: i32, d: &js_sys::Uint8ClampedArray) {
        let length = d.length() as usize;
        let mut pixels = vec![0u8; length];
        d.copy_to(&mut pixels);
        info!("RUST...pixels.len={}", pixels.len());

        let wa = &mut self
            .g
            .context
            .adapter
            .as_any()
            .downcast_mut::<WebAdapter>()
            .unwrap();

        wa.init_glpix(w, h, &pixels);
    }

    pub fn on_asset_loaded(&mut self, url: &str, data: &[u8]) {
        // info!("asset({:?}): {:?}!!!", url, data);
        self.g.context.asset_manager.set_data(url, data);
    }

    pub fn get_ratiox(&mut self) -> f32 {
        self.g.context.adapter.get_base().ratio_x
    }

    pub fn get_ratioy(&mut self) -> f32 {
        self.g.context.adapter.get_base().ratio_y
    }
}
