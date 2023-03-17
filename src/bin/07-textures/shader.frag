#version 330 core

in vec2 texture_coord;

out vec4 colour;

uniform sampler2D tex1;
uniform sampler2D tex2;

void main() {
    colour = mix(texture(tex1, texture_coord), texture(tex2, texture_coord), 0.9);
}
