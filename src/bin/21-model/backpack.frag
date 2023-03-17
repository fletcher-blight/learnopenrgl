#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct LightColour {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct LightAttenuation {
    float constant;
    float linear;
    float quadratic;
};

struct DirectionalLight {
    vec3 direction;
    LightColour colour;
};

struct PointLight {
    vec3 position;
    LightColour colour;
    LightAttenuation attenuation;
};

uniform Material material;
uniform DirectionalLight directional_light_from_left;
uniform DirectionalLight directional_light_from_right;
uniform PointLight point_light_horizontal_rotating;
uniform vec3 view_position;

in vec3 fragment_position;
in vec3 fragment_normal;
in vec2 fragment_uv;

out vec4 fragment_colour;

float calculate_attenuation(LightAttenuation attenuation, vec3 position) {
    float distance = length(position - fragment_position);
    return 1.0 / (attenuation.constant + (attenuation.linear * distance) + (attenuation.quadratic * distance * distance));
}

vec3 calculate_directional_light_colour(DirectionalLight light) {
    vec3 diffuse_colour = texture(material.diffuse, fragment_uv).rgb;
    vec3 specular_colour = texture(material.specular, fragment_uv).rgb;

    vec3 normal = normalize(fragment_normal);
    vec3 light_normal = normalize(-light.direction);
    vec3 view_normal = normalize(view_position - fragment_position);
    vec3 light_bounce_normal = normalize(reflect(light.direction, normal));

    vec3 ambient = light.colour.ambient * diffuse_colour;
    vec3 diffuse = light.colour.diffuse * diffuse_colour * max(dot(normal, light_normal), 0.0);
    vec3 specular = light.colour.specular * specular_colour * pow(max(dot(view_normal, light_bounce_normal), 0.0), material.shininess);

    return ambient + diffuse + specular;
}

vec3 calculate_point_light_colour(PointLight light) {
    vec3 diffuse_colour = texture(material.diffuse, fragment_uv).rgb;
    vec3 specular_colour = texture(material.specular, fragment_uv).rgb;

    vec3 normal = normalize(fragment_normal);
    vec3 light_normal = normalize(light.position - fragment_position);
    vec3 view_normal = normalize(view_position - fragment_position);
    vec3 light_bounce_normal = normalize(reflect(-light_normal, normal));

    vec3 ambient = light.colour.ambient * diffuse_colour;
    vec3 diffuse = light.colour.diffuse * diffuse_colour * max(dot(normal, light_normal), 0.0);
    vec3 specular = light.colour.specular * specular_colour * pow(max(dot(view_normal, light_bounce_normal), 0.0), material.shininess);

    float attenuation = calculate_attenuation(light.attenuation, light.position);
    return attenuation * (ambient + diffuse + specular);
}

void main() {
    vec3 colour = vec3(0.0);

    colour += calculate_directional_light_colour(directional_light_from_left);
    colour += calculate_directional_light_colour(directional_light_from_right);
    colour += calculate_point_light_colour(point_light_horizontal_rotating);

    fragment_colour = vec4(colour, 1.0);
}
