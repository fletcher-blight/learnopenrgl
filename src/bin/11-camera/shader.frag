#version 330 core

uniform sampler2D tex;

in vec2 fragment_uv;

out vec4 fragment_colour;

void main() {
    fragment_colour = texture(tex, fragment_uv);
}