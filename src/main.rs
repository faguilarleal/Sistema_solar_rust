use nalgebra_glm::{look_at, perspective, Mat4, Vec3, Vec4};
use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod model;
mod color;
mod fragment;
mod shaders;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use model::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader};

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
}



fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}


fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], id: f32) {
    
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // Apply fragment shader
            let shaded_color = fragment_shader(&fragment, &uniforms, id);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}



pub struct SceneObject {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
    pub vertex_array: Vec<Vertex>,
    pub id: f32, 
}


fn main() {

    let obj = Obj::load("assets/sphere.obj").expect("Failed to load obj");
    let obj2 = Obj::load("assets/rings.obj").expect("Failed to load obj");
    let nave = Obj::load("assets/nave.obj").expect("Failed to load obj");

    let mut objects = vec![
        // sol
        SceneObject {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 3.0,
            vertex_array: obj.get_vertex_array(),
            id: 3.0,
        },
        // luna
        SceneObject {
            translation: Vec3::new(3.5, 2.5, 0.0),
            rotation: Vec3::new(0.0, PI / 4.0, 0.0),
            scale: 0.3,
            vertex_array: obj.get_vertex_array(), // Reutilizando el mismo modelo
            id: 2.0, 
        },
        
        SceneObject {
            translation: Vec3::new(3.0, 2.0, 0.0),
            rotation: Vec3::new(0.0, PI / 4.0, 0.0),
            scale: 1.0,
            vertex_array: obj.get_vertex_array(), // Reutilizando el mismo modelo
            id: 1.0,
        },
       
        SceneObject {
            translation: Vec3::new(3.0, 1.0, 2.0),
            rotation: Vec3::new(0.0, PI / 4.0, 0.0),
            scale: 1.0,
            vertex_array: obj.get_vertex_array(), // Reutilizando el mismo modelo
            id: 4.0,
        },

        SceneObject {
            translation: Vec3::new(-3.0, 2.0, 0.3),
            rotation: Vec3::new(0.0, PI / 4.0, 0.0),
            scale: 0.7,
            vertex_array: obj.get_vertex_array(), // Reutilizando el mismo modelo
            id: 5.0,
        },

        SceneObject {
            translation: Vec3::new(-3.0, 0.0, -1.3),
            rotation: Vec3::new(0.0, PI / 4.0, 0.0),
            scale: 0.7,
            vertex_array: obj.get_vertex_array(), // Reutilizando el mismo modelo
            id: 6.0,
        },
        

        SceneObject {
            translation: Vec3::new(-1.3, 3.0, 5.3),
            rotation: Vec3::new(0.0, PI / 4.0, 0.0),
            scale: 1.3,
            vertex_array: obj.get_vertex_array(), // Reutilizando el mismo modelo
            id: 8.0,
        },

        SceneObject {
            translation: Vec3::new(4.3, 1.0, -3.3),
            rotation: Vec3::new(0.0, PI / 4.0, 0.0),
            scale: 1.7,
            vertex_array: obj.get_vertex_array(), // Reutilizando el mismo modelo
            id: 6.0,
        },
        SceneObject {
            translation: Vec3::new(4.3, 1.0, -3.3),
            rotation: Vec3::new(0.3, PI / 4.0, 0.0),
            scale: 0.8,
            vertex_array: obj2.get_vertex_array(), // Reutilizando el mismo modelo
            id: 5.0,
        },
    ];


    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Shaders",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x333355);

    // model position
    let translation = Vec3::new(5.0, 0.0, 0.0);
    let rotation = Vec3::new(0.0, 0.0, 0.0);
    let scale = 1.0f32;

    let vertex_arrays_nave = nave.get_vertex_array(); 


    // camera parameters
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let mut time = 0;

    // Ángulos de rotación
    let mut angles = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let rotation_speeds = vec![0.00001, 0.00002, 0.00002, 0.00001, 0.00002, 0.00005,0.00001, 0.00002, 0.00002];


    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
    
        time += 1;
    
        // Manejo de entrada (teclas para mover la cámara)
        handle_input(&window, &mut camera);
    
        // Limpia el framebuffer para el siguiente frame
        framebuffer.clear();
    
        // Calcula las matrices de cámara
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    
        // Renderizar la nave
        let model_matrix_nave = create_model_matrix(Vec3::new(5.0, 0.0, 0.0), 0.2, Vec3::new(0.0, 0.0, 0.0));
        let uniforms_nave = Uniforms { model_matrix: model_matrix_nave, view_matrix, projection_matrix, viewport_matrix, time };
        render(&mut framebuffer, &uniforms_nave, &vertex_arrays_nave, 0.0);
    
        // Renderizar los objetos con rotación orbital
        for (i, object) in objects.iter_mut().enumerate() {
            // Índice de rotación
            let angle = angles[i];
    
            // Matriz de rotación en torno al origen
            let rotation_matrix = Mat4::new_rotation(Vec3::new(0.0, angle, 0.0));
    
            // Transforma la posición del objeto
            let rotated_translation = rotation_matrix * Vec4::new(object.translation.x, object.translation.y, object.translation.z, 1.0);
    
            // Actualiza la posición del objeto
            object.translation = Vec3::new(rotated_translation.x, rotated_translation.y, rotated_translation.z);
    
            // Incrementa el ángulo para el próximo frame
            angles[i] = (angles[i] + rotation_speeds[i]) % (2.0 * PI);
    
            // Crea la matriz del modelo del objeto
            let model_matrix = create_model_matrix(object.translation, object.scale, object.rotation);
    
            // Define los uniformes
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
            };
    
            // Renderiza el objeto
            render(&mut framebuffer, &uniforms, &object.vertex_array, object.id);
        }
    
        // Actualiza la ventana con el framebuffer
        window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height).unwrap();
    }
    
}



fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;
   
   

    //  camera orbit controls
    if window.is_key_down(Key::Left) {
      camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
      camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
      camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
      camera.orbit(0.0, rotation_speed);
    }

    // Camera movement controls
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
      movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
      movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
      movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
      movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
      camera.move_center(movement);
    }

    // Camera zoom controls
    if window.is_key_down(Key::Up) {
      camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
      camera.zoom(-zoom_speed);
    }
}
