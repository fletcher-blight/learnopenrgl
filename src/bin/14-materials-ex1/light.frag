#version 330 core

out vec4 colour;

uniform vec3 light_colour;

void main() {
    colour = vec4(light_colour, 1.0);
}