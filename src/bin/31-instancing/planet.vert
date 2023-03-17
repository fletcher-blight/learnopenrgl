#version 330 core

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

layout (location=0) in vec3 vertex_position;
layout (location=1) in vec2 vertex_uv;

out vec2 fragment_uv;

void main() {
    fragment_uv = vertex_uv;
    gl_Position = projection * view * model * vec4(vertex_position, 1.0);
}
