#version 330 core

in vec3 vertex_light_colour;

out vec4 colour;

uniform vec3 object_colour;

void main() {
    colour = vec4(vertex_light_colour * object_colour, 1.0);
}