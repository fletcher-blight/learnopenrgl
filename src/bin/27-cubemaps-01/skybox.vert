#version 330 core

uniform mat4 projection;
uniform mat4 view;

layout (location = 0) in vec3 vertex_position;

out vec3 fragment_coordinate;

void main() {
    fragment_coordinate = vertex_position;
    vec4 position = projection * view * vec4(vertex_position, 1.0);
    gl_Position = position.xyww; // force depth to be the maximum 1.0
}