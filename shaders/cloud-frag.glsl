#version 140

in vec3 v_position;
in vec3 v_normal;
in vec2 v_texcoord;
in vec4 v_offset_position;

out vec4 f_color;

uniform sampler2D tex;
uniform vec3 camera_position;

void main() {
    f_color = vec4(1.0, 1.0, 1.0, 1.0);
}