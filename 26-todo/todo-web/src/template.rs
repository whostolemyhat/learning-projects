use rustc_serialize::json::ToJson;
use nickel::{ Response, MiddlewareResult };
use std::path::Path;
use std::collections::{ VecDeque };
use handlebars::{ Handlebars, Renderable, RenderError, RenderContext, Helper, Context, JsonRender };

pub fn render<'mw, T:ToJson>(res: Response<'mw>, path: &str, data: &T) -> MiddlewareResult<'mw> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("template", &Path::new(path)).ok().unwrap();
    handlebars.register_helper("filter_todo", Box::new(filter_todo));
    let result = handlebars.render("template", data).ok().unwrap();

    res.send(result)
}

fn filter_todo(c: &Context, h: &Helper, ha: &Handlebars, rc: RenderContext) -> Result<(), RenderError> {
    let active_filter = c.navigate(".", &VeqDeque::new(), "visibility_filter").as_string().unwrap();
    // let active_filter = h.param(0).and_then(|v| v.value().as_string()).unwrap_or("");
    let is_completed = c.navigate(rc.get_path(), &VecDeque::new(), "completed").as_boolean().unwrap();
    // let is_completed = h.param(0).and_then(|v| v.value().as_string()).unwrap_or("");
    let show_todo: bool = match active_filter {
        "ShowAll" => true,
        "ShowCompleted" => is_completed,
        "ShowActive" => !is_completed,
        _ => false
    };

    if show_todo {
        h.template().unwrap().render(c, ha, rc)
    } else {
        Ok(())
    }
}
