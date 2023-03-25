#version 330 core

layout (location = 0) in vec2 vertex_position;
layout (location = 1) in vec2 vertex_texture_coordinate;

out vec2 fragment_texture_coordinate;

void main() {
    fragment_texture_coordinate = vertex_texture_coordinate;
    gl_Position = vec4(vertex_position.xy, 0.0, 1.0);
}