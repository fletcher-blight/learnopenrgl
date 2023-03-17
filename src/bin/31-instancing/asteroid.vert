#version 330 core

uniform mat4 rotate_model;
uniform mat4 view;
uniform mat4 projection;

layout (location=0) in vec3 vertex_position;
layout (location=1) in vec2 vertex_uv;
layout (location=3) in mat4 model;

out vec2 fragment_uv;

void main() {
    fragment_uv = vertex_uv;
    gl_Position = projection * view * rotate_model * model * vec4(vertex_position, 1.0);
}
