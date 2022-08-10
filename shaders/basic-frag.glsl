#version 140

in vec3 v_position;
in vec3 v_normal;
in vec2 v_texcoord;

out vec4 f_color;

uniform sampler2D tex;
uniform vec3 camera_position;

float mod289(float x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 mod289(vec4 x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 perm(vec4 x){return mod289(((x * 34.0) + 1.0) * x);}

float noise(vec3 p){
    vec3 a = floor(p);
    vec3 d = p - a;
    d = d * d * (3.0 - 2.0 * d);

    vec4 b = a.xxyy + vec4(0.0, 1.0, 0.0, 1.0);
    vec4 k1 = perm(b.xyxy);
    vec4 k2 = perm(k1.xyxy + b.zzww);

    vec4 c = k2 + a.zzzz;
    vec4 k3 = perm(c);
    vec4 k4 = perm(c + 1.0);

    vec4 o1 = fract(k3 * (1.0 / 41.0));
    vec4 o2 = fract(k4 * (1.0 / 41.0));

    vec4 o3 = o2 * d.z + o1 * (1.0 - d.z);
    vec2 o4 = o3.yw * d.x + o3.xz * (1.0 - d.x);

    return o4.y * d.y + o4.x * (1.0 - d.y);
}

void main() {
    float ambient_strength = 0.1;
    vec3 light_color = vec3(0.8, 0.95, 1.0);
    vec3 ambient = ambient_strength * light_color; // Not yet implemented

    vec3 light_direction = vec3(-0.2, 0.8, 0.1);

    float lum = max(dot(normalize(v_normal), normalize(light_direction)), 0.5);
    vec3 color_r = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0) * light_color;

    // Add a fog effect depending on the distance from the camera
    float fog_factor = (1.0 - clamp(length(v_position - camera_position) / 50.0, 0.0, 1.0));

    // Add a little bit of noise to the color
    float noise_factor = noise(v_position * 500000.0) / 4.0 + 0.75;

    f_color = (texture(tex, v_texcoord) * vec4(color_r, 1.0) * fog_factor) * noise_factor + (vec4(0.0, 0.0, 0.0, 1.0) * (1.0 - fog_factor));
}