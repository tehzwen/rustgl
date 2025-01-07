#version 330 core
struct PointLight {
    vec3 position;
    vec3 color;
    float strength;
};

uniform PointLight light;

in vec3 fragPosition; // Position of the fragment in world space
in vec3 normal;       // Normal of the fragment in world space

out vec4 final_color;

void main() {
    vec3 norm = normalize(normal);
    vec3 light_dir = normalize(light.position - fragPosition);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light.color * light.strength * vec3(0.5, 0.0, 0.0);

    final_color = vec4(diffuse, 1.0);
}
