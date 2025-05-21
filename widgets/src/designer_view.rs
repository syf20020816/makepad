use crate::{
    makepad_derive_widget::*,
    makepad_draw::*,
    makepad_platform::studio::*,
    designer_data::*,
    turtle_step::*,
    view::*,
    widget::*,
};
use std::collections::BTreeMap;

live_design!{
    DesignerViewBase = {{DesignerView}}{
    }
    
    DesignerContainerBase = {{DesignerContainer}}{
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum DesignerViewAction {
    None,
    Selected{
        id:LiveId, 
        km:KeyModifiers,
        tap_count: u32,
    }
}


struct ContainerData{
    ptr: LivePtr,
    component: WidgetRef,
    container: WidgetRef,
    rect: Rect
}

enum Edge{
    Left,
    Right,
    Bottom,
    Top,
    Body
}

impl ContainerData{
    fn get_edge(&self, rel:DVec2, zoom:f64, pan: DVec2)->Option<Edge>{
        let cp = rel * zoom + pan;
        let edge_outer:f64 = 5.0 * zoom ;
        let edge_inner:f64  = 5.0 * zoom ;
                
        if cp.x >= self.rect.pos.x - edge_outer && 
        cp.x <= self.rect.pos.x + edge_inner && 
        cp.y >= self.rect.pos.y && 
        cp.y <= self.rect.pos.y + self.rect.size.y{
            // left edge
            return Some(Edge::Left);
        }
        if cp.x >= self.rect.pos.x + self.rect.size.x- edge_outer && 
        cp.x <= self.rect.pos.x + self.rect.size.x+ edge_inner && 
        cp.y >= self.rect.pos.y && 
        cp.y <= self.rect.pos.y + self.rect.size.y{
            return Some(Edge::Right);
        }
        else if cp.y >= self.rect.pos.y - edge_outer && 
        cp.y <= self.rect.pos.y + edge_inner &&
        cp.x >= self.rect.pos.x && 
        cp.x <= self.rect.pos.x + self.rect.size.x{
            // top edge
            return Some(Edge::Top);
        }
        else if cp.y >= self.rect.pos.y + self.rect.size.y- edge_outer && 
        cp.y <= self.rect.pos.y + self.rect.size.y + edge_inner &&
        cp.x >= self.rect.pos.x && 
        cp.x <= self.rect.pos.x + self.rect.size.x{
            // bottom edge
            return Some(Edge::Bottom);
        }
        else if self.rect.contains(cp){
            // bottom edge
            return Some(Edge::Body);
        }
        None
    }
}

#[derive(Live, Widget, LiveHook)]
pub struct DesignerContainer {
    #[deref] view: View
}

impl Widget for DesignerContainer {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope){
        self.view.handle_event(cx, event, scope);
    }
                
    fn draw_walk(&mut self, cx: &mut Cx2d, scope:&mut Scope, _walk: Walk) -> DrawStep {
        let data = scope.props.get::<ContainerData>().unwrap();
        // alright lets draw the container, then the child
        let _turtle_step = self.view.turtle_step(id!(inner));
        self.walk = Walk{
            abs_pos: Some(data.rect.pos),
            width: Size::Fixed(data.rect.size.x),
            height: Size::Fixed(data.rect.size.y),
            margin: Default::default()
        };
        while let Some(_next) = self.view.draw(cx, &mut Scope::empty()).step() {
            data.component.draw_all(cx, &mut Scope::empty());
        }
        
        DrawStep::done()
    }
}

#[allow(unused)]
enum FingerMove{
    Pan{start_pan: DVec2},
    DragBody{ptr: LivePtr},
    DragEdge{edge: Edge, rect:Rect, id: LiveId},
    DragAll{rects:BTreeMap<LiveId,Rect>},
    DragSubComponentOrder(SelectedSubcomponent)
}

#[derive(Live, Widget)]
pub struct DesignerView {
    #[walk] walk:Walk,
    #[area]
    #[rust] area:Area,
    #[rust] reapply: bool,
    #[rust(1.5)] zoom: f64,
    #[rust] undo_group: u64,
    #[rust] pan: DVec2,
    #[live] clear_color: Vec4,
    #[rust] finger_move: Option<FingerMove>,
    #[live] container: Option<LivePtr>,
    #[live] draw_bg: DrawColor,
    #[live] draw_outline: DrawQuad,
    #[rust] view_file: Option<LiveId>,
    #[rust] selected_component: Option<LiveId>,
    #[rust] selected_subcomponent: Option<SelectedSubcomponent>,
    #[rust] containers: ComponentMap<LiveId, ContainerData>,
    #[redraw] #[rust(DrawList2d::new(cx))] draw_list: DrawList2d,
    #[rust(Pass::new(cx))] pass: Pass,
    #[rust] color_texture: Option<Texture>,
}

#[derive(Clone, Debug)]
struct SelectedSubcomponent{
    parent:Option<WidgetRef>,
    component:WidgetRef
}

impl LiveHook for DesignerView {
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]){
        
        // find a bit cleaner way to do this
        self.reapply = true;
    }
}

impl DesignerView{
    fn draw_container(&mut self, cx:&mut Cx2d, id:LiveId, ptr: LivePtr, name:&str, pos: &mut Vec<DesignerComponentPosition>){
        
        let rect = if let Some(v) = pos.iter().find(|v| v.id == id){
            rect(v.left, v.top, v.width, v.height)
        }
        else{
            pos.push(DesignerComponentPosition{
                id,
                left: 50.0,
                top: 50.0,
                width: 200.0,
                height: 200.00
            });
            return self.draw_container(cx, id, ptr, name, pos);
        };
                                    
        let container_ptr = self.container.unwrap();
        let mut is_new = false;
        let cd = self.containers.get_or_insert(cx, id, | cx | {
            is_new = true;
            ContainerData{
                ptr,
                component :WidgetRef::new_from_ptr(cx, Some(ptr)),
                container: WidgetRef::new_from_ptr(cx, Some(container_ptr)),
                rect
            }
        });
        cd.rect = rect;
        cd.ptr = ptr;
        // fix up the livecoding of the
        if self.reapply{
            cd.container.apply_from_ptr(cx, Some(self.container.unwrap()));
            cd.component.apply_from_ptr(cx, Some(ptr));
        }
        if self.reapply || is_new{
            cd.container.apply_over(cx, live!{
                widget_label = { label = {text:(name)}}
            });
        }
        // ok so we're going to draw the container with the widget inside
        cd.container.draw_all(cx, &mut Scope::with_props(cd))
    }
    
    fn select_component(&mut self, cx:&mut Cx, what_id:Option<LiveId>){
        /*for (id, comp) in self.containers.iter_mut(){
            if what_id == Some(*id){
                comp.container.as_designer_container().borrow_mut().unwrap()
                    .animator_cut(cx, id!(select.on));
            }
            else{
                comp.container.as_designer_container().borrow_mut().unwrap()
                    .animator_cut(cx, id!(select.off));
            }
        }
        */
        self.redraw(cx);
        self.selected_component = what_id;
    }
    
    fn sync_zoom_pan(&self, _cx:&mut Cx){
        Cx::send_studio_message(AppToStudio::DesignerZoomPan(
            DesignerZoomPan{
                zoom: self.zoom,
                pan_x: self.pan.x,
                pan_y: self.pan.y
            }
        ));
    }
    
    fn get_component_rect(&self, cx:&mut Cx, comp:&WidgetRef)->Rect{
        let mut rect = self.area().rect(cx);
        let cr = comp.area().rect(cx);
        rect.pos += (cr.pos - self.pan)/self.zoom;
        rect.size = cr.size;
        rect.size /= self.zoom;
        rect
    }
    
    fn update_rect(&mut self, cx:&mut Cx, id: LiveId, rect:Rect, pos:&mut Vec<DesignerComponentPosition>){
        if let Some(container) = self.containers.get_mut(&id){
            container.container.redraw(cx);
            container.rect = rect;
            // lets send the info over
            if let Some(v) = pos.iter_mut().find(|v| v.id == id){
                v.left = rect.pos.x;
                v.top = rect.pos.y;
                v.width = rect.size.x;
                v.height = rect.size.y;
                // alright lets send it over
                Cx::send_studio_message(AppToStudio::DesignerComponentMoved(v.clone()));
            }
            // ok first we have to build a
            
            /*
            // alright lets send over the rect to the editor
            // we need to find out the text position
            let registry = cx.live_registry.clone();
            let mut registry = registry.borrow_mut();
            //let replace = format!("dx:{:.1} dy:{:.1} dw:{:.1} dh:{:.1}", r.pos.x, r.pos.y, r.size.x, r.size.y);
            let design_info = LiveDesignInfo{
                span: Default::default(),
                dx: rect.pos.x,
                dy: rect.pos.y,
                dw: rect.size.x,
                dh: rect.size.y,
            };
            // lets find the ptr
                                        
            if let Some((replace, file_name, range)) =  registry.patch_design_info(container.ptr, design_info){
                                                
                Cx::send_studio_message(AppToStudio::PatchFile(PatchFile{
                    file_name: file_name.into(),
                    line: range.line,
                    undo_group: self.undo_group,
                    column_start: range.start_column,
                    column_end: range.end_column,
                    replace
                }));
                //self.finger_move = Some(FingerMove::Pan{start_pan:dvec2(0.0,0.0)});
            }*/
        }
    }
}

impl Widget for DesignerView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope){
        let uid = self.widget_uid();
        // alright so. our widgets dont have any 'event' flow here
        // so what can we do.
        // 
        let data = scope.data.get_mut::<DesignerData>().unwrap();
        match event.hits(cx, self.area) {
            Hit::FingerHoverOver(fh) =>{
                // lets poll our widget structure with a special event
                // alright so we hover over. lets determine the mouse cursor
                //let corner_inner:f64  = 10.0 * self.zoom;
                //let corner_outer:f64  = 10.0 * self.zoom;
                // lets send a designer pick into all our widgets
                self.selected_subcomponent = None;
                for (_id, cd) in self.containers.iter(){
                    // ok BUT our mouse pick is not dependent on the container
                    // ok so we are in a pass. meaning 0,0 origin
                    let abs = (fh.abs - fh.rect.pos) * self.zoom + self.pan;
                    // lets capture the actions
                    let actions = cx.capture_actions(|cx|{
                        cd.component.handle_event(cx, &Event::DesignerPick(DesignerPickEvent{
                            abs: abs
                        }), &mut Scope::empty())
                    });
                    for action in actions{
                        if let Some(action) = action.as_widget_action(){
                            match action.cast(){
                                WidgetDesignAction::PickedBody=>{
                                    // alright so lets draw a quad on top
                                    // alright our widget got clicked.
                                    let mut cs = action.widgets.iter();
                                    // scan for the parent with more than one child node
                                    // and then make our subcomponent/parent those widgets
                                    let mut component = cs.next();
                                    let mut parent = cs.next();
                                    while parent.is_some(){
                                        let p = parent.unwrap();
                                        if let Some(p) = p.as_view().borrow(){
                                            if p.child_count()>1{
                                                break;
                                            }
                                        }
                                        component = parent;
                                        parent = cs.next();
                                    }
                                    
                                    self.selected_subcomponent = Some(
                                        SelectedSubcomponent{
                                            component: component.unwrap().clone(),
                                            parent: parent.cloned(),
                                        }
                                    );
                                    //println!("{:?}", self.selected_subcomponent);
                                    break
                                }
                                _=>()
                            }
                        }
                    }
                    self.draw_list.redraw(cx);
                }
                
                let mut cursor = None;
                for cd in self.containers.values(){
                    match cd.get_edge(fh.abs -fh.rect.pos, self.zoom, self.pan){
                        Some(edge)=> {
                            cursor = Some(match edge{
                                Edge::Left|Edge::Right=>MouseCursor::EwResize,
                                Edge::Top|Edge::Bottom=>MouseCursor::NsResize,
                                Edge::Body=>MouseCursor::Move
                            });
                            break;
                        }
                        None=>{}
                    }
                }
                if let Some(cursor) = cursor{
                    cx.set_cursor(cursor);
                }
                else{
                    cx.set_cursor(MouseCursor::Default);
                }
            }
            Hit::FingerHoverOut(_fh)=>{
            }
            Hit::FingerDown(fe) => {
                self.undo_group += 1;
                if !fe.modifiers.shift{
                    if fe.modifiers.control && fe.modifiers.alt{
                        let mut rects = BTreeMap::new();
                        for (id, cd) in self.containers.iter(){
                            rects.insert(*id, cd.rect);
                        }
                        self.finger_move = Some(FingerMove::DragAll{rects})
                    }
                    else{
                        if fe.modifiers.logo || self.selected_subcomponent.is_none(){
                            for (id, cd) in self.containers.iter(){
                                match cd.get_edge(fe.abs -fe.rect.pos, self.zoom, self.pan){
                                    Some(edge)=>{
                                        self.finger_move = Some(FingerMove::DragEdge{
                                            rect: cd.rect,
                                            id: *id,
                                            edge
                                        });
                                        // lets send out a click on this containter
                                        cx.widget_action(uid, &scope.path, DesignerViewAction::Selected{
                                            id:*id, 
                                            tap_count: fe.tap_count , 
                                            km:fe.modifiers
                                        });
                                        // set selected component
                                        // unselect all other components
                                        self.select_component(cx, Some(*id));
                                        break;
                                    }
                                    None=>()
                                }
                            }
                        }
                        else{
                            if let Some(sc) = &self.selected_subcomponent{
                                self.finger_move = Some(FingerMove::DragSubComponentOrder(sc.clone()));
                            }
                        }
                    }
                }
                
                if self.finger_move.is_none(){
                    self.finger_move = Some(FingerMove::Pan{
                        start_pan: self.pan
                    });
                }
            },
            Hit::KeyDown(_k)=>{
            }
            Hit::FingerScroll(fs)=>{
                let last_zoom = self.zoom;
                
                if fs.scroll.y < 0.0{
                    let step = (-fs.scroll.y).min(200.0) / 500.0;
                    self.zoom *= 1.0 - step; 
                }
                else{
                    let step = (fs.scroll.y).min(200.0) / 500.0;
                    self.zoom *= 1.0 + step;
                }
                self.zoom = self.zoom.max(0.01).min(10.0);
                
                // we should shift the pan to stay in the same place
                let pan1 = (fs.abs - fs.rect.pos) * last_zoom;
                let pan2 = (fs.abs - fs.rect.pos) * self.zoom;
                // we should keep it in the same place
                
                self.pan += pan1 - pan2;
                self.sync_zoom_pan(cx);
                self.redraw(cx);
            }
            Hit::FingerMove(fe) => {
                match self.finger_move.as_ref().unwrap(){
                    FingerMove::DragSubComponentOrder(cs)=>{
                        // ok how do we do this.
                        // lets compare our rect to the mouse
                        // and if we are below ask the parent component view to move down
                        if let Some(parent) = &cs.parent{
                            let vw = parent.as_view();
                            if let Some(mut vw) = vw.borrow_mut(){
                                
                                // we need to be below the 'next item' in the list
                                let index = vw.child_index(&cs.component).unwrap();
                                if let Some(next_child) = vw.child_at_index(index+1){
                                    let rect = self.get_component_rect(cx, &next_child);
                                    if let Flow::Down = vw.layout.flow{
                                        if fe.abs.y > rect.pos.y + 0.6* rect.size.y{
                                            vw.swap_child(index, index + 1);
                                        }     
                                    }
                                    else{
                                        if fe.abs.x > rect.pos.x + 0.6* rect.size.x{
                                            vw.swap_child(index, index + 1);
                                        }   
                                    } 
                                }
                                if index > 0{
                                    if let Some(prev_child) = vw.child_at_index(index-1){
                                        let rect = self.get_component_rect(cx, &prev_child);
                                        if let Flow::Down = vw.layout.flow{
                                            if fe.abs.y < rect.pos.y + 0.6 *rect.size.y{
                                                vw.swap_child(index, index - 1);
                                            }
                                        }else{
                                            if fe.abs.x < rect.pos.x + 0.6 *rect.size.x{
                                                vw.swap_child(index, index - 1);
                                            }
                                        }
                                    }
                                }                                   
                            }
                            vw.redraw(cx);
                        }
                    }
                    FingerMove::Pan{start_pan} =>{
                        self.pan= *start_pan - (fe.abs - fe.abs_start) * self.zoom;
                        self.sync_zoom_pan(cx);
                        self.redraw(cx);
                    }
                    FingerMove::DragAll{rects}=>{
                        let delta = (fe.abs - fe.abs_start)* self.zoom;
                        let rects = rects.clone();
                        for (id, rect) in rects{
                            let r = Rect{
                                pos: rect.pos + delta,
                                size: rect.size
                            };
                            self.update_rect(cx, id, r, &mut data.positions);
                        }
                    }
                    FingerMove::DragEdge{edge, rect, id}=>{
                        let delta = (fe.abs - fe.abs_start)* self.zoom;
                        let rect = match edge{
                             Edge::Left=>Rect{
                                pos:dvec2(rect.pos.x + delta.x, rect.pos.y),
                                size:dvec2(rect.size.x - delta.x, rect.size.y)
                             },
                             Edge::Right=>Rect{
                                 pos: rect.pos,
                                 size: dvec2(rect.size.x + delta.x, rect.size.y)
                             },
                             Edge::Top=>Rect{
                                 pos:dvec2(rect.pos.x, rect.pos.y + delta.y),
                                 size:dvec2(rect.size.x, rect.size.y - delta.y)
                             },
                             Edge::Bottom=>Rect{
                                 pos: rect.pos,
                                 size: dvec2(rect.size.x, rect.size.y + delta.y)
                             },
                             Edge::Body=>Rect{
                                 pos: rect.pos + delta,
                                 size: rect.size
                             }
                        };
                        self.update_rect(cx, *id, rect, &mut data.positions);
                    }
                    FingerMove::DragBody{ptr:_}=>{
                        
                    }
                }
            }
            Hit::FingerUp(_) => {
                self.finger_move = None;
            }
            _ => ()
        }
    }
        
    fn draw_walk(&mut self, cx: &mut Cx2d, scope:&mut Scope, walk: Walk) -> DrawStep {
       
        if self.color_texture.is_none(){
            self.color_texture = Some(Texture::new_with_format(
                cx,
                TextureFormat::RenderBGRAu8 {
                    size: TextureSize::Auto,
                    initial: true,
                },
            ));
        }
        
        if cx.will_redraw(&mut self.draw_list, walk) {
            
            cx.make_child_pass(&self.pass);
            cx.begin_pass(&self.pass, None);
            self.pass.clear_color_textures(cx);
            self.pass.add_color_texture(
                cx,
                self.color_texture.as_ref().unwrap(),
                PassClearColor::ClearWith(self.clear_color),
            );
                        
            self.draw_list.begin_always(cx);
    
            cx.begin_pass_sized_turtle_no_clip(Layout::flow_down());
            
            let data = scope.data.get_mut::<DesignerData>().unwrap();
            
            // lets draw the component container windows and components
            
            if let Some(view_file) = &self.view_file{
                // so either we have a file, or a single component.
                match data.node_map.get(view_file){
                    Some(OutlineNode::File{children,..})=>{
                        for child in children{
                            // alright so. we need to use a path entry in our
                            // datastructure
                            if let Some(OutlineNode::Component{ptr,name,..}) = data.node_map.get(child){
                                if name == "App=<App>"{ // we need to skip inwards to 
                                    if let Some(child) = data.get_node_by_path(*child, "ui:/main_window=/body="){
                                        if let Some(OutlineNode::Component{ptr,name,..}) = data.node_map.get(&child){
                                            // lets fetch the position of this thing
                                            
                                            self.draw_container(cx, child, *ptr, name, &mut data.positions);
                                        }
                                    }
                                }
                                else{
                                    // lets fetch the position of this thing
                                    
                                    self.draw_container(cx, *child, *ptr, name, &mut data.positions);
                                }
                            }
                        }
                    }
                    _=>()
                }
            }
            self.reapply = false;
                        
            cx.end_pass_sized_turtle_no_clip();
            self.draw_list.end(cx);
            cx.end_pass(&self.pass);
            self.containers.retain_visible()
        }
        
        self.draw_bg.draw_vars.set_texture(0, self.color_texture.as_ref().unwrap());
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        self.draw_bg.draw_abs(cx, rect);
        // lets draw all the outlines on top
        
        // alright and now we need to highlight a component
        if let Some(cs) = &self.selected_subcomponent{
            let rect = self.get_component_rect(cx, &cs.component);
            self.draw_outline.draw_abs(cx, rect);
        } 
        else if let Some(component) = self.selected_component{
            if let Some(container) = self.containers.get(&component){
                let mut rect = rect;
                rect.pos += (container.rect.pos - self.pan)/self.zoom;
                rect.size = container.rect.size;
                rect.size /= self.zoom;
                self.draw_outline.draw_abs(cx, rect);
            } 
        }
        cx.set_pass_area_with_origin(
            &self.pass,
            self.area,
            dvec2(0.0,0.0)
        );
        cx.set_pass_shift_scale(&self.pass, self.pan, dvec2(self.zoom,self.zoom));
        
        DrawStep::done()
    }
}

impl DesignerViewRef{
    pub fn select_component_and_redraw(&self, cx:&mut Cx, comp:Option<LiveId>) {
        if let Some(mut inner) = self.borrow_mut(){
            inner.select_component(cx, comp);
            inner.redraw(cx);
        }
    }
    
    pub fn view_file_and_redraw(&self, cx:&mut Cx, file_id:LiveId){
        if let Some(mut inner) = self.borrow_mut(){
            if inner.view_file != Some(file_id){
                inner.containers.clear();
                inner.view_file = Some(file_id);
                inner.redraw(cx);
            }
        }
    }
    
    pub fn set_zoom_pan (&self, cx:&mut Cx, zp:&DesignerZoomPan){
        if let Some(mut inner) = self.borrow_mut(){
            inner.zoom = zp.zoom;
            inner.pan.x = zp.pan_x;
            inner.pan.y = zp.pan_y;
            inner.redraw(cx);
        }
    }
    
    pub fn reload_view(&self, cx:&mut Cx) {
        if let Some(mut inner) = self.borrow_mut(){
            inner.containers.clear();
            inner.redraw(cx);
        }
    }
    
    pub fn selected(&self, actions: &Actions) -> Option<(LiveId,KeyModifiers,u32)> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let DesignerViewAction::Selected{id, km, tap_count} = item.cast() {
                return Some((id, km, tap_count))
            }
        }
        None
    }
}