#version 330 core

uniform samplerCube fragment_texture;

in vec3 fragment_coordinate;

out vec4 fragment_colour;

void main() {
    fragment_colour = texture(fragment_texture, fragment_coordinate);
}