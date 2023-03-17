#version 330 core

layout (location=0) in vec2 position;

out vec2 texture_coord;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    switch (gl_VertexID) {
        case 0: texture_coord = vec2(0.0, 0.0); break;
        case 1: texture_coord = vec2(0.5, 1.0); break;
        case 2: texture_coord = vec2(1.0, 0.0); break;
    }
}
