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
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    // Añadimos una segunda luz
    let light_dir2 = normalize(vec3<f32>(-1.0, 0.5, 0.0));
    let diffuse1 = max(dot(normalize(normal), light_dir), 0.0);
    let diffuse2 = max(dot(normalize(normal), light_dir2), 0.0) * 0.5;
    let ambient = 0.2;
    return vec4<f32>((ambient + diffuse1 + diffuse2) * color.rgb, color.a);
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
        let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
        let diffuse = max(dot(normalize(normal), light_dir), 0.0);
        let ambient = 0.5;
        let radial_gradient = 1.0 - length(normal.xy);
        let emissive = vec3<f32>(1.0, 0.8, 0.0);
        return vec4<f32>((ambient + diffuse) * color.rgb * radial_gradient + emissive, color.a);
    }
    "#;