#version 330 core

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inInstancePosition;

uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

void main() {
    vec3 cameraRight = vec3(viewMatrix[0][0], viewMatrix[1][0], viewMatrix[2][0]); // First column
    vec3 cameraUp = vec3(viewMatrix[0][1], viewMatrix[1][1], viewMatrix[2][1]);   // Second column
    vec3 worldPosition = inInstancePosition +
        inPosition.x * cameraRight * 5.0 + // Scale the quad
        inPosition.y * cameraUp * 5.0;

    gl_Position = projectionMatrix * viewMatrix * vec4(worldPosition, 1.0);
}
