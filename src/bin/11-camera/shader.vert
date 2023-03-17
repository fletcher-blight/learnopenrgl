#version 330 core

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

layout (location=0) in vec3 position;
out vec2 fragment_uv;

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
    gl_Position = projection * view * model * vec4(position, 1.0);
}