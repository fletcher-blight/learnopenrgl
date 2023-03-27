#version 330 core

uniform mat4 projection;
uniform mat4 view;

layout (location = 0) in vec3 vertex_position;
layout (location = 1) in vec2 vertex_coordinate;

out vec2 fragment_coordinate;

void main() {
    fragment_coordinate = vertex_coordinate;
    gl_Position = projection * view * vec4(vertex_position, 1.0);
}