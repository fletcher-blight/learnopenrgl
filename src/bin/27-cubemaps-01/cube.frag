#version 330 core

uniform sampler2D fragment_texture;

in vec2 fragment_coordinate;

out vec4 fragment_colour;

void main() {
    fragment_colour = texture(fragment_texture, fragment_coordinate);
}