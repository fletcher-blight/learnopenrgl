#version 330 core

in vec3 normal;
in vec3 fragment_pos;

out vec4 colour;

uniform vec3 object_colour;
uniform vec3 light_colour;
uniform vec3 light_pos;
uniform vec3 view_pos;

void main() {
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

    vec3 result_colour = (ambient_light + diffuse_light + specular_light) * object_colour;
    colour = vec4(result_colour, 1.0);
}