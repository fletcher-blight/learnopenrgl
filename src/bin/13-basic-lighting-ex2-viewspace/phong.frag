#version 330 core

uniform vec3 object_colour;
uniform vec3 light_colour;
uniform vec3 light_pos;

in vec3 fragment_normal;
in vec3 fragment_position;

out vec4 fragment_colour;

void main() {
    vec3 normal_dir = normalize(fragment_normal);
    vec3 light_dir = normalize(light_pos - fragment_position);
    vec3 view_dir = normalize(-fragment_position);
    vec3 reflect_dir = reflect(-light_dir, normal_dir);

    float ambient_strength = 0.1;
    vec3 ambient_light = ambient_strength * light_colour;

    float diffuse_effect = max(dot(normal_dir, light_dir), 0.0);
    vec3 diffuse_light = diffuse_effect * light_colour;

    float specular_strength = 0.5;
    float specular_effect = specular_strength * pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular_light = specular_effect * light_colour;

    vec3 colour = (ambient_light + diffuse_light + specular_light) * object_colour;
    fragment_colour = vec4(colour, 1.0);
}