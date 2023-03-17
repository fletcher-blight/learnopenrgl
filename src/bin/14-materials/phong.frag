#version 330 core

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec3 normal;
in vec3 fragment_pos;

out vec4 colour;

uniform Material material;
uniform Light light;
uniform vec3 view_pos;

void main() {
    vec3 normal_dir = normalize(normal);
    vec3 light_dir = normalize(light.position - fragment_pos);
    vec3 view_dir = normalize(view_pos - fragment_pos);
    vec3 reflect_dir = reflect(-light_dir, normal_dir);

    vec3 ambient = light.ambient * material.ambient;
    vec3 diffuse = light.diffuse * material.diffuse * max(dot(normal_dir, light_dir), 0.0);
    vec3 specular = light.specular * material.specular * pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);

    vec3 fragment_colour = ambient + diffuse + specular;
    colour = vec4(fragment_colour, 1.0);
}