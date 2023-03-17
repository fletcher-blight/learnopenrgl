#version 330 core

layout (location=0) in vec3 position;

out vec3 normal;
out vec3 fragment_pos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    fragment_pos = vec3(model * vec4(position, 1.0));
    gl_Position = projection * view * model * vec4(position, 1.0);

    switch (gl_VertexID / 6) {
        case 0: normal = vec3(0.0, 0.0, 1.0); break;
        case 1: normal = vec3(-1.0, 0.0, 0.0); break;
        case 2: normal = vec3(0.0, 1.0, 0.0); break;
        case 3: normal = vec3(1.0, 0.0, 0.0); break;
        case 4: normal = vec3(0.0, -1.0, 0.0); break;
        case 5: normal = vec3(0.0, 0.0, -1.0); break;
    }
    normal = mat3(transpose(inverse(model))) * normal;
}