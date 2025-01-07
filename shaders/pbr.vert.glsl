#version 330 core
layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 norm;
layout(location = 2) in vec2 uvs;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 view;

out vec3 fragPosition;
out vec3 normal;
out vec2 oUVs;

void main() {
    vec4 worldPosition = model * vec4(pos, 1.0);
    fragPosition = vec3(worldPosition);

    normal = mat3(transpose(inverse(model))) * norm;

    oUVs = uvs;
    gl_Position = projection * view * model * vec4(pos, 1.0);
}
