#version 330 core

layout (location = 0) in vec3 vertex_position;

out vec2 fragment_uv;
out vec3 fragment_normal;
out vec3 fragment_position;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

vec3 calculate_fragment_normal(int index) {
    switch (index / 6) {
        case 0: return vec3(0.0, 0.0, 1.0);
        case 1: return vec3(-1.0, 0.0, 0.0);
        case 2: return vec3(0.0, 1.0, 0.0);
        case 3: return vec3(1.0, 0.0, 0.0);
        case 4: return vec3(0.0, -1.0, 0.0);
        case 5: return vec3(0.0, 0.0, -1.0);
    }
    return vec3(0.0);
}

vec2 calculate_fragment_uv(int index) {
    switch (index % 6) {
        case 0: case 3: return vec2(0.0, 0.0);
        case 1:         return vec2(0.0, 1.0);
        case 2: case 4: return vec2(1.0, 1.0);
        case 5:         return vec2(1.0, 0.0);
    }
    return vec2(0.0, 0.0);
}

void main() {
    fragment_uv = calculate_fragment_uv(gl_VertexID);
    fragment_normal = mat3(transpose(inverse(model))) * calculate_fragment_normal(gl_VertexID);
    fragment_position = vec3(model * vec4(vertex_position, 1.0));
    gl_Position = projection * view * model * vec4(vertex_position, 1.0);
}