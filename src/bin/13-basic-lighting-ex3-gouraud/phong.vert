#version 330 core

layout (location=0) in vec3 position;

out vec3 vertex_light_colour;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform vec3 light_colour;
uniform vec3 light_pos;
uniform vec3 view_pos;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);

    vec3 normal;
    switch (gl_VertexID / 6) {
        case 0: normal = vec3(0.0, 0.0, 1.0); break;
        case 1: normal = vec3(-1.0, 0.0, 0.0); break;
        case 2: normal = vec3(0.0, 1.0, 0.0); break;
        case 3: normal = vec3(1.0, 0.0, 0.0); break;
        case 4: normal = vec3(0.0, -1.0, 0.0); break;
        case 5: normal = vec3(0.0, 0.0, -1.0); break;
    }
    normal = mat3(transpose(inverse(model))) * normal;
    vec3 fragment_pos = vec3(model * vec4(position, 1.0));

    vec3 normal_dir = normalize(normal);
    vec3 light_dir = normalize(light_pos - fragment_pos);
    vec3 view_dir = normalize(view_pos - fragment_pos);
    vec3 reflect_dir = reflect(-light_dir, normal_dir);

    float ambient_strength = 0.1;
    vec3 ambient_light = ambient_strength * light_colour;

    float diffuse_effect = max(dot(normal_dir, light_dir), 0.0);
    vec3 diffuse_light = diffuse_effect * light_colour;

    float specular_strength = 0.5;
    float specular_effect = specular_strength * pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular_light = specular_effect * light_colour;

    vertex_light_colour = vec3(ambient_light + diffuse_light + specular_light);
}