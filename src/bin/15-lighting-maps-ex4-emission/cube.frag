#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    sampler2D emission;
    float shininess;
};

struct Light {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec2 fragment_uv;
in vec3 fragment_normal;
in vec3 fragment_position;

out vec4 fragment_colour;

uniform Material material;
uniform Light light;
uniform vec3 view_position;

void main() {
    vec3 normal = normalize(fragment_normal);
    vec3 light_dir = normalize(light.position - fragment_position);
    vec3 view_dir = normalize(view_position - fragment_position);
    vec3 specular_dir = reflect(-light_dir, normal);

    vec3 diffuse_colour = vec3(texture(material.diffuse, fragment_uv));
    vec3 specular_colour = vec3(texture(material.specular, fragment_uv));
    vec3 emission_colour = vec3(texture(material.emission, fragment_uv));

    float diffuse_strength = max(dot(normal, light_dir), 0.0);

    vec3 ambient = light.ambient * diffuse_colour;
    vec3 diffuse = light.diffuse * diffuse_colour * diffuse_strength;
    vec3 specular = light.specular * specular_colour * pow(max(dot(specular_dir, view_dir), 0.0), material.shininess);
    vec3 emission = mix(emission_colour, vec3(0.0), diffuse_strength);

    // Don't emit over the shiny specular border
    if (length(specular_colour) > 0.0) {
        emission = vec3(0.0);
    }

    fragment_colour = vec4(ambient + diffuse + specular + emission, 1.0);
}