#version 330 core

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

layout (location=0) in vec3 vertex_position;

out vec3 fragment_normal;
out vec3 fragment_position;

vec3 get_normal() {
    switch (gl_VertexID / 6) {
        case 0: return vec3(0.0, 0.0, 1.0);
        case 1: return vec3(-1.0, 0.0, 0.0);
        case 2: return vec3(0.0, 1.0, 0.0);
        case 3: return vec3(1.0, 0.0, 0.0);
        case 4: return vec3(0.0, -1.0, 0.0);
        case 5: return vec3(0.0, 0.0, -1.0);
    }
    return vec3(0.0);
}

void main() {
    fragment_position = vec3(view * model * vec4(vertex_position, 1.0));
    fragment_normal = mat3(transpose(inverse(view * model))) * get_normal();
    gl_Position = projection * view * model * vec4(vertex_position, 1.0);
}