#version 330 core

uniform float near;
uniform float far;

out vec4 fragment_colour;

float to_linear_depth(float nonlinear_depth) {
    float ndc = nonlinear_depth * 2.0 - 1.0;
    float linear_depth = (2.0 * near * far) / (far + near - ndc * (far - near));
    return linear_depth;
}

void main() {
    float linear_depth = to_linear_depth(gl_FragCoord.z);
    float visual_linear_depth = linear_depth / far; // apply for only within frutsum for better visualisation
    fragment_colour = vec4(vec3(visual_linear_depth), 1.0);
}