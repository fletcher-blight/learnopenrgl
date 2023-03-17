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

struct DirectionalLight {
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    Attenuation attenuation;
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

#define NUM_DIRECTIONAL_LIGHTS 2
#define NUM_POINT_LIGHTS 4

uniform Material material;
uniform bool enable_directionals;
uniform DirectionalLight directional_lights[NUM_DIRECTIONAL_LIGHTS];
uniform bool enable_point_lights;
uniform PointLight point_lights[NUM_POINT_LIGHTS];
uniform bool enable_spotlight;
uniform Spotlight spotlight;
uniform vec3 view_position;

in vec2 fragment_uv;
in vec3 fragment_position;
in vec3 fragment_normal;

out vec4 fragment_colour;

float calculate_attenuation(Attenuation attenuation, float distance) {
    return 1.0 / (attenuation.constant + (attenuation.linear * distance) + (attenuation.quadratic * distance * distance));
}

vec3 calculate_direction_light_colour(DirectionalLight light) {
    vec3 normal = normalize(fragment_normal);
    vec3 light_dir = normalize(-light.direction);
    vec3 view_dir = normalize(view_position - fragment_position);
    vec3 specular_dir = reflect(-light_dir, normal);

    vec3 diffuse_colour = vec3(texture(material.diffuse, fragment_uv));
    vec3 specular_colour = vec3(texture(material.specular, fragment_uv));

    vec3 ambient = light.ambient * diffuse_colour;
    vec3 diffuse = light.diffuse * diffuse_colour * max(dot(normal, light_dir), 0.0);
    vec3 specular = light.specular * specular_colour * pow(max(dot(specular_dir, view_dir), 0.0), material.shininess);

    return ambient + diffuse + specular;
}

vec3 calculate_point_light(PointLight light) {
    vec3 normal = normalize(fragment_normal);
    vec3 light_dir = normalize(light.position - fragment_position);
    vec3 view_dir = normalize(view_position - fragment_position);
    vec3 specular_dir = reflect(-light_dir, normal);
    float light_distance = length(light.position - fragment_position);
    float attenuation = calculate_attenuation(light.attenuation, light_distance);

    vec3 diffuse_colour = vec3(texture(material.diffuse, fragment_uv));
    vec3 specular_colour = vec3(texture(material.specular, fragment_uv));

    vec3 ambient = light.ambient * diffuse_colour;
    vec3 diffuse = light.diffuse * diffuse_colour * max(dot(normal, light_dir), 0.0);
    vec3 specular = light.specular * specular_colour * pow(max(dot(specular_dir, view_dir), 0.0), material.shininess);

    return attenuation * (ambient + diffuse + specular);
}

vec3 calculate_spotlight(Spotlight light) {
    vec3 normal = normalize(fragment_normal);
    vec3 light_dir = normalize(light.position - fragment_position);
    vec3 view_dir = normalize(view_position - fragment_position);
    vec3 specular_dir = reflect(-light_dir, normal);

    float light_distance = length(light.position - fragment_position);
    float attenuation = calculate_attenuation(light.attenuation, light_distance);
    float theta = dot(light_dir, normalize(-light.direction));
    float epsilon = light.cutoff - light.outer_cutoff;
    float intensity = clamp((theta - light.outer_cutoff) / epsilon, 0.0, 1.0);

    vec3 diffuse_colour = vec3(texture(material.diffuse, fragment_uv));
    vec3 specular_colour = vec3(texture(material.specular, fragment_uv));

    vec3 ambient = light.ambient * diffuse_colour;
    vec3 diffuse = intensity * light.diffuse * diffuse_colour * max(dot(normal, light_dir), 0.0);
    vec3 specular = intensity * light.specular * specular_colour * pow(max(dot(specular_dir, view_dir), 0.0), material.shininess);

    return attenuation * (ambient + diffuse + specular);
}

void main() {
    vec3 colour = vec3(0.0);

    if (enable_directionals) {
        for (int i = 0; i != NUM_DIRECTIONAL_LIGHTS; ++i) {
            colour += calculate_direction_light_colour(directional_lights[i]);
        }
    }

    if (enable_point_lights) {
        for (int i = 0; i != NUM_POINT_LIGHTS; ++i) {
            colour += calculate_point_light(point_lights[i]);
        }
    }

    if (enable_spotlight) {
        colour += calculate_spotlight(spotlight);
    }

    fragment_colour = vec4(colour, 1.0);
}