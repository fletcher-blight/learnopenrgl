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

struct Spotlight {
    vec3 position;
    vec3 direction;
    float cutoff;
    float outer_cutoff;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    Attenuation attenuation;
};

uniform Material material;
uniform Spotlight spotlight;
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
    vec3 light_dir = normalize(spotlight.position - fragment_position);
    vec3 view_dir = normalize(view_position - fragment_position);
    vec3 specular_dir = reflect(-light_dir, normal);

    float light_distance = length(spotlight.position - fragment_position);
    float attenuation = calculate_attenuation(spotlight.attenuation, light_distance);
    float theta = dot(light_dir, normalize(-spotlight.direction));
    float epsilon = spotlight.cutoff - spotlight.outer_cutoff;
    float intensity = clamp((theta - spotlight.outer_cutoff) / epsilon, 0.0, 1.0);

    vec3 diffuse_colour = vec3(texture(material.diffuse, fragment_uv));
    vec3 specular_colour = vec3(texture(material.specular, fragment_uv));

    vec3 ambient = spotlight.ambient * diffuse_colour;
    vec3 diffuse = intensity * spotlight.diffuse * diffuse_colour * max(dot(normal, light_dir), 0.0);
    vec3 specular = intensity * spotlight.specular * specular_colour * pow(max(dot(specular_dir, view_dir), 0.0), material.shininess);

    vec3 colour = attenuation * (ambient + diffuse + specular);
    fragment_colour = vec4(colour, 1.0);
}