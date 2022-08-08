#version 140

in vec3 normal;
in vec3 position;
in vec2 tex_coords;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_texcoord;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;


void main() {
    v_normal = normal;
    v_position = position;
    v_texcoord = tex_coords;

    mat4 vm = view * model;
    gl_Position = perspective * vm * vec4(position, 1.0);
}