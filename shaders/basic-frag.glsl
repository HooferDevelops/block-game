#version 140

in vec3 v_position;
in vec3 v_normal;
in vec2 v_texcoord;

out vec4 f_color;

uniform sampler2D tex;

void main() {
    float ambient_strength = 0.1;
    vec3 light_color = vec3(0.8, 0.95, 1.0);
    vec3 ambient = ambient_strength * light_color;

    vec3 light_direction = vec3(-0.2, 0.8, 0.1);

    float lum = max(dot(normalize(v_normal), normalize(light_direction)), 0.5);
    vec3 color_r = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0) * light_color;

    f_color = texture(tex, v_texcoord) * vec4(color_r, 1.0);
}