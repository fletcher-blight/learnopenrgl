#version 330 core

out vec4 colour;

uniform vec3 object_colour;
uniform vec3 light_colour;

void main() {
    colour = vec4(object_colour * light_colour, 1.0);
}