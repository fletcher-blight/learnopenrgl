#version 330 core

uniform sampler2D fragment_texture;
uniform int fragment_function;

in vec2 fragment_texture_coordinate;

out vec4 fragment_colour;

void plain() {
    fragment_colour = texture(fragment_texture, fragment_texture_coordinate);
}

void inversion() {
    vec4 texture_colour = texture(fragment_texture, fragment_texture_coordinate);
    fragment_colour = vec4(vec3(1.0 - texture_colour), 1.0);
}

void main() {
    switch (fragment_function) {
        case 1:
            inversion();
            break;
        default:
            plain();
            break;
    }
}