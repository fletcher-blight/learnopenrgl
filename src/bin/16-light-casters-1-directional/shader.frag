#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct DirectionalLight {
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

uniform Material material;
uniform DirectionalLight directional_light;
uniform vec3 view_position;

in vec2 fragment_uv;
in vec3 fragment_position;
in vec3 fragment_normal;

out vec4 fragment_colour;

void main() {
    vec3 normal = normalize(fragment_normal);
    vec3 light_dir = normalize(-directional_light.direction);
    vec3 view_dir = normalize(view_position - fragment_position);
    vec3 specular_dir = reflect(-light_dir, normal);

    vec3 diffuse_colour = vec3(texture(material.diffuse, fragment_uv));
    vec3 specular_colour = vec3(texture(material.specular, fragment_uv));

    vec3 ambient = directional_light.ambient * diffuse_colour;
    vec3 diffuse = directional_light.diffuse * diffuse_colour * max(dot(normal, light_dir), 0.0);
    vec3 specular = directional_light.specular * specular_colour * pow(max(dot(specular_dir, view_dir), 0.0), material.shininess);

    fragment_colour = vec4(ambient + diffuse + specular, 1.0);
}