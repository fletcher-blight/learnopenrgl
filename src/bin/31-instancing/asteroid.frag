#version 330 core

uniform sampler2D diffuse;

in vec2 fragment_uv;

out vec4 fragment_colour;

void main() {
    fragment_colour = texture(diffuse, fragment_uv);
}
