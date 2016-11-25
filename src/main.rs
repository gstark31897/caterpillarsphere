extern crate octengine;

use octengine::*;

fn main() {
    let app = App::new();
    let mut scene = Scene::empty();
    
    let teapot_function = | x: &mut f32, y: &mut f32, z: &mut f32, x_rot: &mut f32, y_rot: &mut f32, z_rot: &mut f32 | {
        *y_rot = 5.0;
    };
    let teapot = Teapot::new(app.get_display());
    let mut teapot_object = GameObject::new(Box::new(teapot));
    teapot_object.set_update_function(Box::new(teapot_function));
    scene.register_gameobject(teapot_object);

    app.run(&mut scene);
}
