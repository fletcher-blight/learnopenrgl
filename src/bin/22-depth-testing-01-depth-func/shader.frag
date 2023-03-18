#version 330 core

uniform sampler2D texture_sample;

in vec2 fragment_texture_coordinate;

out vec4 fragment_colour;

void main() {
    fragment_colour = texture(texture_sample, fragment_texture_coordinate);
}