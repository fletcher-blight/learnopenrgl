#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct Attenuation {
    float constant;
    float linear;
    float quadratic;
};

struct PointLight {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    Attenuation attenuation;
};

uniform Material material;
uniform PointLight point_light;
uniform vec3 view_position;

in vec2 fragment_uv;
in vec3 fragment_position;
in vec3 fragment_normal;

out vec4 fragment_colour;

float calculate_attenuation(Attenuation attenuation, float distance) {
    return 1.0 / (attenuation.constant + (attenuation.linear * distance) + (attenuation.quadratic * distance * distance));
}

void main() {
    vec3 normal = normalize(fragment_normal);
    vec3 light_dir = normalize(point_light.position - fragment_position);
    vec3 view_dir = normalize(view_position - fragment_position);
    vec3 specular_dir = reflect(-light_dir, normal);
    float light_distance = length(point_light.position - fragment_position);
    float attenuation = calculate_attenuation(point_light.attenuation, light_distance);

    vec3 diffuse_colour = vec3(texture(material.diffuse, fragment_uv));
    vec3 specular_colour = vec3(texture(material.specular, fragment_uv));

    vec3 ambient = point_light.ambient * diffuse_colour;
    vec3 diffuse = point_light.diffuse * diffuse_colour * max(dot(normal, light_dir), 0.0);
    vec3 specular = point_light.specular * specular_colour * pow(max(dot(specular_dir, view_dir), 0.0), material.shininess);

    vec3 colour = attenuation * (ambient + diffuse + specular);
    fragment_colour = vec4(colour, 1.0);
}