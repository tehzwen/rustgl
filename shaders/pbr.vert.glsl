#version 330 core
layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 norm; // Normal of the vertex
layout(location = 2) in vec2 uvs;

uniform mat4 projection; // Orthogonal projection matrix
uniform mat4 model;      // Model transformation matrix
uniform mat4 view;       // View (camera) matrix
uniform mat3 normalMatrix;

out vec3 fragPosition;   // To pass to fragment shader
out vec3 normal;         // To pass to fragment shader
out vec2 oUVs;
out vec3 Normal;

void main() {
    vec4 worldPosition = model * vec4(pos, 1.0);
    fragPosition = vec3(worldPosition);

    normal = mat3(transpose(inverse(model))) * norm;

    oUVs = uvs;
    gl_Position = projection * view * model * vec4(pos, 1.0);
}
