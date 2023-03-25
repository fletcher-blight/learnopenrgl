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

void greyscale() {
    vec4 texture_colour = texture(fragment_texture, fragment_texture_coordinate);
    float average = (texture_colour.r + texture_colour.g + texture_colour.b) / 3.0;
    fragment_colour = vec4(vec3(average), 1.0);
}

void greyscale_weighted() {
    vec4 texture_colour = texture(fragment_texture, fragment_texture_coordinate);
    float average = 0.2126 * texture_colour.r + 0.7152 * texture_colour.g + 0.0722 * texture_colour.b;
    fragment_colour = vec4(vec3(average), 1.0);
}

void apply_kernel(const float[9] kernel) {
    const float offset = 1.0 / 300.0;
    const vec2 offsets[9] = vec2[](
        vec2(-offset, offset),
        vec2(0.0f, offset),
        vec2(offset, offset),
        vec2(-offset, 0.0f),
        vec2(0.0f, 0.0f),
        vec2(offset, 0.0f),
        vec2(-offset, -offset),
        vec2(0.0f, -offset),
        vec2(offset, -offset)
    );

    vec3 sample_texture[9];
    for (int i = 0; i != 9; ++i) {
        sample_texture[i] = vec3(texture(fragment_texture, fragment_texture_coordinate.st + offsets[i]));
    }

    vec3 colour = vec3(0.0);
    for (int i = 0; i != 9; ++i)
        colour += sample_texture[i] * kernel[i];

    fragment_colour = vec4(colour, 1.0);
}

void sharpen() {
    float kernel[9] = float[](-1, -1, -1, -1, 9, -1, -1, -1, -1);
    apply_kernel(kernel);
}

void main() {
    switch (fragment_function) {
        case 1:
            inversion();
            break;
        case 2:
            greyscale();
            break;
        case 3:
            greyscale_weighted();
            break;
        case 4:
            sharpen();
            break;
        default:
            plain();
            break;
    }
}