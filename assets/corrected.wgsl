struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

fn linear_to_nonlinear(x: f32) -> f32 {
    if x <= 0.0 {
        return x;
    }
    if x <= 0.0031308 {
        return x * 12.92;
    } else {
        return (1.055 * pow(x, 1.0 / 2.4)) - 0.055;
    }
}

fn nonlinear_to_linear(x: f32) -> f32 {
    if x <= 0.0 {
        return x;
    }
    if x <= 0.04045 {
        return x / 12.92;
    } else {
        return pow((x + 0.055) / 1.055, 2.4);
    }
}

fn unpremultiply(rgb: vec3<f32>, a: f32) -> vec3<f32> {
    if (a == 0.0) {
        return vec3(0.0, 0.0, 0.0);
    }
    return vec3(rgb.r / a, rgb.g / a, rgb.b / a);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    var texColor = textureSample(texture, texture_sampler, input.uv);
    var a = texColor.a;
    var s = vec3(linear_to_nonlinear(texColor.r), linear_to_nonlinear(texColor.g), linear_to_nonlinear(texColor.b));
    var s_non_premult = unpremultiply(s, a);
    var lin = vec3(nonlinear_to_linear(s_non_premult.r) * a, nonlinear_to_linear(s_non_premult.g) * a, nonlinear_to_linear(s_non_premult.b) * a);
    return vec4(lin, texColor.a);
}
