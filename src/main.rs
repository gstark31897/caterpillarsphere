extern crate octengine;

use octengine::*;

fn main() {
    let app = App::new();
    let mut scene = Scene::empty();
    
    let teapot = Teapot::new(app.get_display());
    let teapot_object = GameObject::new(Box::new(teapot));
    scene.register_gameobject(teapot_object);

    app.run(&scene);
}
