pub const VERTEX_SHADER: &str = r#"
struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    color: vec4<f32>,
};
@binding(0) @group(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    // La posición es transformada por la matriz modelo-vista-proyección
    out.position = uniforms.view_proj * uniforms.model * vec4<f32>(position, 1.0);
    // La normal es la misma que la posición para una esfera centrada en el origen
    out.normal = normalize(position);
    // Pasamos el color uniforme al fragment shader
    out.color = uniforms.color;
    return out;
}
"#;



pub const FRAGMENT_SHADER_2: &str = r#"
@fragment
fn fs_main(
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>
) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diffuse = max(dot(normalize(normal), light_dir), 0.0);
    let ambient = 0.2;
    // Aplicamos un efecto especular adicional
    let specular_color = vec3<f32>(1.0, 1.0, 1.0);
    let specular = pow(max(dot(reflect(-light_dir, normalize(normal)), vec3<f32>(0.0, 0.0, 1.0)), 0.0), 32.0);
    return vec4<f32>((ambient + diffuse) * color.rgb + specular * specular_color * 0.5, color.a);
}
"#;

pub const FRAGMENT_SHADER_3: &str = r#"
@fragment
fn fs_main(
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>
) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diffuse = max(dot(normalize(normal), light_dir), 0.0);
    let ambient = 0.3; // Más ambiente para este shader
    // Añadimos un efecto de fresnel
    let view_dir = vec3<f32>(0.0, 0.0, 1.0);
    let fresnel = pow(1.0 - max(dot(normalize(normal), view_dir), 0.0), 3.0);
    return vec4<f32>((ambient + diffuse) * color.rgb + fresnel * vec3<f32>(0.5), color.a);
}
"#;

pub const FRAGMENT_SHADER_4: &str = r#"
@fragment
fn fs_main(
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>
) -> @location(0) vec4<f32> {
    // Simulación del tiempo (canal alpha del color para tiempo dinámico)
    let time = color.a;

    // Direcciones de luz
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diffuse = max(dot(normalize(normal), light_dir), 0.0);
    let ambient = 0.3;

    // **Generación de continentes y océanos (capa base)**
    let noise = sin(normal.x * 8.0 + normal.y * 8.0 + normal.z * 8.0) *
                cos(normal.x * 10.0 + normal.y * 10.0);
    let terrain_pattern = smoothstep(-0.2, 0.2, noise);

    let land_color = vec3<f32>(0.2, 0.6, 0.2); // Verde para los continentes
    let water_color = vec3<f32>(0.1, 0.3, 0.8); // Azul para los océanos
    let base_surface = mix(water_color, land_color, terrain_pattern);

    // **Generación de montañas (nueva capa)**
    let mountain_noise = sin(normal.x * 15.0 + normal.y * 15.0 + normal.z * 15.0) *
                         cos(normal.x * 20.0 + normal.z * 20.0);
    let mountain_pattern = smoothstep(0.4, 0.6, mountain_noise); // Suavizamos las montañas
    let mountain_color = vec3<f32>(0.5, 0.4, 0.3); // Marrón para las montañas
    let surface_with_mountains = mix(base_surface, mountain_color, mountain_pattern);

    // **Generación de nubes dinámicas (segunda capa)**
    let cloud_pattern = sin((normal.x + time * 0.2) * 8.0) *
                        cos((normal.y + time * 0.2) * 8.0);
    let cloud_density = smoothstep(0.5, 0.7, cloud_pattern); // Más contraste
    let cloud_color = vec3<f32>(1.0, 1.0, 1.0); // Blanco puro para las nubes

    // Combinación de nubes con la superficie y montañas
    let final_surface = mix(surface_with_mountains, cloud_color, cloud_density);

    // **Iluminación aplicada**
    let final_color = (ambient + diffuse) * final_surface;

    return vec4<f32>(final_color, 1.0);
}
"#;



pub const FRAGMENT_SHADER_5: &str = r#"
@fragment
fn fs_main(
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>
) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diffuse = max(dot(normalize(normal), light_dir), 0.0);
    // Efecto de bandas
    let bands = floor(diffuse * 4.0) / 4.0;
    let ambient = 0.2;
    return vec4<f32>((ambient + bands) * color.rgb, color.a);
}
"#;

pub const FRAGMENT_SHADER_6: &str = r#"
@fragment
fn fs_main(
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>
) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diffuse = max(dot(normalize(normal), light_dir), 0.0);
    let ambient = 0.2;
    // Patrón de rayas basado en la normal
    let stripe = sin(normal.x * 20.0) * 0.5 + 0.5;
    return vec4<f32>((ambient + diffuse * stripe) * color.rgb, color.a);
}
"#;

pub const FRAGMENT_SHADER_7: &str = r#"
@fragment
fn fs_main(
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>
) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diffuse = max(dot(normalize(normal), light_dir), 0.0);
    // Efecto de remolino
    let swirl = sin(atan2(normal.y, normal.x) * 5.0) * 0.5 + 0.5;
    let ambient = 0.2;
    return vec4<f32>((ambient + diffuse * swirl) * color.rgb, color.a);
}
"#;

pub const FRAGMENT_SHADER_8: &str = r#"
@fragment
fn fs_main(
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>
) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diffuse = max(dot(normalize(normal), light_dir), 0.0);
    let ambient = 0.2;
    // Efecto de viñeta basado en la distancia al centro
    let vignette = 1.0 - length(normal.xy);
    return vec4<f32>((ambient + diffuse * vignette) * color.rgb, color.a);
}
"#;

pub const FRAGMENT_SHADER_SUN: &str = r#"
  @fragment
fn fs_main(
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>
) -> @location(0) vec4<f32> {
    // Cálculo de la distancia desde el centro para gradientes
    let distance_from_center = length(normal.xy);

    // **Núcleo brillante**: Blanco puro en el centro
    let core_intensity = 1.0 - smoothstep(0.0, 0.3, distance_from_center);
    let core_color = vec3<f32>(1.0, 1.0, 1.0) * core_intensity;

    // **Halo cálido**: Gradiente cálido extendido alrededor del núcleo
    let halo_intensity = 1.0 - smoothstep(0.3, 1.2, distance_from_center);
    let halo_color = vec3<f32>(1.0, 0.8, 0.2) * halo_intensity;

    // **Combinar los efectos**: Core + Halo
    let final_color = core_color + halo_color;

    return vec4<f32>(final_color, 1.0);
}

 "#;