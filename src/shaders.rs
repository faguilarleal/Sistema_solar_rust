use nalgebra_glm::{atan2, dot, mat4_to_mat3, Mat3, Vec2, Vec3, Vec4};
use crate::vertex::Vertex;
use crate::{framebuffer, Uniforms};
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  // apply viewport matrix
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transform normal
  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, id:f32) -> Color {
  match id {
    1.0 => planet1(fragment, uniforms),
    2.0 => moon_shader(fragment, uniforms),

    3.0 => sun_shader(fragment, uniforms),

    4.0 => earth_shader(fragment, uniforms),
    5.0 => vibrant_blue_planet_shader(fragment, uniforms),
    
    6.0 => rocky_planet_shader(fragment, uniforms),
    7.0 => ring_shader(fragment, uniforms),
    8.0 => planet2(fragment, uniforms),
    // Agregar más shaders aquí según sea necesario
    _ => {
        // Por defecto, renderiza un shader genérico
        Color::new(69, 146, 173 ) // Negro como fallback
    }
}
    
  }

// planeta 1, planeta gaseoso
fn planet1(fragment: &Fragment, uniforms: &Uniforms) -> Color {
   let color1 = Color::new( 85, 117, 114 );   
   let color2 = Color::new(112, 147, 144  );   
 
   let stripe_width = 0.2;  // Width of each stripe
   let speed = 0.001;        // Speed of stripe movement
 
   let moving_y = fragment.vertex_position.y + uniforms.time as f32 * speed;
 
   let stripe_factor = ((moving_y / stripe_width) * PI).sin() * 0.5 + 0.5;
 
   color1.lerp(&color2, stripe_factor) * fragment.intensity
}

// planeta 1, planeta gaseoso
fn planet2(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let color1 = Color::new( 161, 117, 79);   
  let color2 = Color::new(173, 118, 69  );   

  let stripe_width = 0.2;  // Width of each stripe
  let speed = 0.001;        // Speed of stripe movement

  let moving_y = fragment.vertex_position.y + uniforms.time as f32 * speed;

  let stripe_factor = ((moving_y / stripe_width) * PI).sin() * 0.5 + 0.5;

  color1.lerp(&color2, stripe_factor) * fragment.intensity
}


fn sun_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {


  // Coordenadas del fragmento normalizadas al rango [-1, 1]
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let center = (0.0, 0.0); // Centro del degradado
  let radius = ((x - center.0).powi(2) + (y - center.1).powi(2)).sqrt(); // Distancia al centro

  let color1 = Color::new(255, 255, 255); // Amarillo muy claro
  let color2 = Color::new(255, 230, 28); // Amarillo pastel
  let color3 = Color::new(255, 178, 51);  // Amarillo intenso
  let color4 = Color::new(204, 102, 0);   // Naranja oscuro

  let t = radius.clamp(0.0, 1.0); // Radio normalizado entre 0 y 1
  let blended_color = if t < 0.53 {
      // Mezcla entre el color1 y color2
      color1.lerp(&color2, t / 0.33)
  } else if t < 0.66 {
      // Mezcla entre el color2 y color3
      color2.lerp(&color3, (t - 0.33) / 0.33)
  } else {
      // Mezcla entre el color3 y color4
      color3.lerp(&color4, (t - 0.66) / 0.34)
  };

  blended_color
}


fn earth_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let z = fragment.vertex_position.z;

  let theta = (y / 0.5).asin(); // Latitud
  let phi = z.atan2(x);         // Longitud
  let u = (phi / (2.0 * PI)) + 0.5; // Coordenada u [0, 1]
  let v = (theta / PI) + 0.5;      // Coordenada v [0, 1]

  let scale = 7.2;
  let noise = ((u * scale).sin() * (v * scale).cos()).abs();
  let continent_threshold = 0.55;

  let land_color = Color::new(34, 139, 34); // Verde
  let ocean_color = Color::new(0, 105, 148); // Azul
  let base_color = if noise > continent_threshold { land_color } else { ocean_color };

  // Agregar polos
  let circle1_center = Vec2::new(0.5, 0.7); // Coordenadas UV del primer círculo
  let circle2_center = Vec2::new(0.2, 0.3); // Coordenadas UV del segundo círculo
  let circle_radius = 0.1;                 // Radio de los círculos

  let frag_position = Vec2::new(u, v);
  let distance_to_circle1 = (frag_position - circle1_center).norm();
  let distance_to_circle2 = (frag_position - circle2_center).norm();

  // Dibujar círculos: Si la distancia es menor al radio, aplica un color
  let circle_color = Color::new(255, 255, 255); // Blanco para los círculos
  let is_in_circle1 = distance_to_circle1 < circle_radius;
  let is_in_circle2 = distance_to_circle2 < circle_radius;

  // Simular nubes en la atmósfera
  let time = uniforms.time as f32 * 0.1; // Escala del tiempo para velocidad
  let cloud_scale = 8.0;          // Escala de las nubes
  let cloud_pattern = ((u * cloud_scale + time).sin() * (v * cloud_scale + time).cos()).abs();
  let cloud_intensity = (cloud_pattern - 0.5).clamp(0.0, 1.0) * 0.3; // Intensidad y opacidad de las nubes
  let cloud_color = Color::new(255, 255, 255) * cloud_intensity;

  // Nubes en movimiento circular hacia afuera
  let cloud_radius = 0.9; // Límite de la atmósfera con nubes
  let distance_from_center = frag_position.norm();
  let is_in_atmosphere = distance_from_center < cloud_radius;

  // Combinar colores
  let planet_color = if is_in_circle1 || is_in_circle2 {
      circle_color
  } else {
      base_color
  };

  let final_color = if is_in_atmosphere {
      planet_color * (1.0 - cloud_intensity) + cloud_color
  } else {
      planet_color
  };

  final_color
}

pub fn vibrant_blue_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 3.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let time = uniforms.time as f32 * 0.05;

    // Crear un patrón basado en ondas para un efecto gaseoso dinámico
    let pattern1 = ((x * zoom + time).sin() * (y * zoom + time).cos()).abs();
    let pattern2 = ((x * zoom * 0.5 - time).cos() * (y * zoom * 0.7 + time).sin()).abs();
    let combined_pattern = (pattern1 + pattern2 * 0.5).min(1.0);

    // Colores de base con tonos más azulados y menos fucsia
    let r = (combined_pattern * 100.0) as u8;
    let g = ((1.0 - combined_pattern) * 170.0) as u8;
    let b = 240;

    let base_color = Color::new(r, g, b);

    // Ajuste de iluminación ambiental para un aspecto de gas disperso
    let ambient_intensity = 0.5;
    let ambient_color = Color::new(200, 80, 198);

    // Mezcla del color base y el color ambiental para dar una apariencia gaseosa en toda la superficie
    base_color * fragment.intensity + ambient_color * ambient_intensity
}

fn rocky_planet_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
  let scale = 10.0; // Escala del patrón de ruido
  let light_adjust = 0.6; // Ajuste de intensidad lumínica ambiental

  // Coordenadas esféricas del fragmento
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let z = fragment.vertex_position.z;

  // Introducir pseudoaleatoriedad en la textura
  let randomness = (x * 12.9898 + y * 78.233 + z * 37.719).sin() * 43758.5453;
  let random_factor = randomness.fract(); // Tomamos solo la parte decimal

  // Coordenadas ajustadas con ruido aleatorio
  let adjusted_x = x + random_factor * 0.1;
  let adjusted_y = y + random_factor * 0.1;

  // Patrón de ruido basado en seno y coseno para variaciones rocosas
  let noise_pattern = ((adjusted_x * scale).sin() * (adjusted_y * scale).cos()).abs();

  // Colores base para las regiones del terreno
  let base_color = Color::new(120, 85, 60); // Color terroso
  let highlight_color = Color::new(200, 170, 140); // Tonos más claros para áreas elevadas

  // Combina colores en función del patrón de ruido
  let surface_color = base_color.lerp(&highlight_color, noise_pattern);

  // Iluminación ambiental simple
  let ambient_intensity = 0.8;
  let ambient_color = Color::new(50, 30, 20); // Luz cálida difusa

  // Ajuste de la intensidad de luz
  let final_color = surface_color * fragment.intensity * light_adjust
      + ambient_color * ambient_intensity;

  final_color
}

fn moon_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para la superficie lunar
  let color1 = Color::new( 197, 199, 200);
  let color2 = Color::new( 220, 221, 222);
  let color3 = Color::new(137, 149, 154);

  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let frequency = 10.0;

  let wave1= (x*7.0*frequency + y * 5.0 * frequency).sin() * 0.5 + 0.5;
  let wave2= (x*5.0*frequency - y * 8.0 * frequency + PI / 3.0).sin() * 0.5 + 0.5;
  let wave3= (x*6.0*frequency + x * 4.0 * frequency + 2.0 * PI/3.0).sin() * 0.5 + 0.5;

  let mut final_color = color1.lerp(&color2, wave1);
  final_color = final_color.lerp(&color3, wave2);
  final_color = final_color.lerp(&color1, wave3);

  final_color * fragment.intensity
}


fn ring_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para la superficie lunar
  let base_color = Color::new(184, 162, 42 ); // Gris claro para la superficie
  let crater_color = Color::new(184, 177, 42 ); // Gris oscuro para los cráteres

  // Coordenadas del fragmento
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let z = fragment.vertex_position.z;

  // Coordenadas esféricas para la superficie
  let longitude = z.atan2(x); // Longitud
  let latitude = y.asin();    // Latitud

  // Ruido para cráteres
  let scale = 10.0; // Escala para los patrones de ruido
  let noise_value = ((longitude * scale).cos() * (latitude * scale).sin()).abs();

  // Generar cráteres (regiones más oscuras)
  let crater_threshold = 0.4;
  let is_crater = noise_value > crater_threshold;

  // Interpolación de colores entre cráteres y la superficie base
  let surface_color = if is_crater {
      crater_color
  } else {
      base_color
  };

  // Iluminación básica para simular sombras
  let light_direction = Vec3::new(1.0, 1.0, 1.0).normalize(); // Dirección de la luz
  let normal = fragment.vertex_position.normalize(); // Normal de la esfera
  let light_intensity = (normal.dot(&light_direction)).clamp(0.2, 1.0); // Intensidad de la luz
  let shadow_color = Color::new(50, 50, 50); // Sombra suave

  // Aplicar iluminación
  let final_color = surface_color * light_intensity + shadow_color * (1.0 - light_intensity);

  final_color
}
