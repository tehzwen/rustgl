#version 330 core
precision highp float;

struct PointLight {
    vec3 position;
    vec3 color;
    float strength;
};

struct DirectionalLight {
    vec3 direction;
    vec3 color;
};

struct MaterialTexture {
    sampler2D tex;
    int enabled;
    float scale;
};

// physical material
struct PhysicalMaterial {
    vec3 albedo;
    float metallic;
    float roughness;
    float ao;
    MaterialTexture diffuse_texture;
    MaterialTexture normal_texture;
    MaterialTexture arm_texture; // ao, roughness, metallic all in one
};

uniform PhysicalMaterial material;

uniform vec3 camera_position;
uniform vec2 resolution;

uniform int numPointLights;
uniform PointLight[4] pointLights;
uniform DirectionalLight dirLight;

in vec3 fragPosition;
in vec3 normal;
in vec2 oUVs;

// physical rendering components
const float PI = 3.14159265359;
const highp float NOISE_GRANULARITY = 1.0 / 255.0;

highp float random(highp vec2 coords) {
    return fract(sin(dot(coords.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

float DistributionGGX(vec3 N, vec3 H, float roughness);
float GeometrySchlickGGX(float NdotV, float roughness);
float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness);
vec3 fresnelSchlick(float cosTheta, vec3 F0);

vec3 fresnelSchlick(float cosTheta, vec3 F0) {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

float DistributionGGX(vec3 N, vec3 H, float roughness) {
    float a = roughness * roughness;
    float a2 = a * a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH * NdotH;

    float num = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

float GeometrySchlickGGX(float NdotV, float roughness) {
    float r = (roughness + 1.0);
    float k = (r * r) / 8.0;

    float num = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return num / denom;
}
float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness) {
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2 = GeometrySchlickGGX(NdotV, roughness);
    float ggx1 = GeometrySchlickGGX(NdotL, roughness);

    return ggx1 * ggx2;
}

vec3 getNormalFromMap() {
    vec3 tangentNormal = texture(material.normal_texture.tex, oUVs * material.normal_texture.scale).xyz * 2.0 - 1.0;

    vec3 Q1 = dFdx(fragPosition);
    vec3 Q2 = dFdy(fragPosition);
    vec2 st1 = dFdx(oUVs);
    vec2 st2 = dFdy(oUVs);

    vec3 N = normalize(normal);
    vec3 T = normalize(Q1 * st2.t - Q2 * st1.t);
    vec3 B = -normalize(cross(N, T));
    mat3 TBN = mat3(T, B, N);

    return normalize(TBN * tangentNormal);
}

out vec4 final_color;

void main() {
    vec3 N = normalize(normal);
    if(material.normal_texture.enabled == 1) {
        N = getNormalFromMap();
    }

    vec3 V = normalize(camera_position - fragPosition);

    vec3 total = vec3(0.0, 0.0, 0.0);

    // calculate reflectance at normal incidence; if dia-electric (like plastic) use F0
    // of 0.04 and if it's a metal, use the albedo color as F0 (metallic workflow)
    vec3 F0 = vec3(0.04);

    // check if we want to use diffuse map
    vec3 albedoColor = material.albedo;
    if(material.diffuse_texture.enabled == 1) {
        albedoColor = pow(texture(material.diffuse_texture.tex, oUVs * material.diffuse_texture.scale).rgb, vec3(2.2));
    }

    // check if we want to use roughness map
    float roughness = material.roughness;
    if (material.arm_texture.enabled == 1) {
        roughness = texture(material.arm_texture.tex, oUVs * material.arm_texture.scale).g;
    }

    F0 = mix(F0, albedoColor, material.metallic);

    // reflectance equation
    vec3 Lo = vec3(0.0);

    // Directional light contribution
    {
        vec3 L = normalize(-dirLight.direction); // Ensure the direction is normalized
        vec3 H = normalize(V + L);

        // Cook-Torrance BRDF
        float NDF = DistributionGGX(N, H, roughness);
        float G = GeometrySmith(N, V, L, roughness);
        vec3 F = fresnelSchlick(clamp(dot(H, V), 0.0, 1.0), F0);

        vec3 numerator = NDF * G * F;
        float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001; // Prevent divide by zero
        vec3 specular = numerator / denominator;

        vec3 kS = F;
        vec3 kD = vec3(1.0) - kS;
        kD *= 1.0 - material.metallic;

        float NdotL = max(dot(N, L), 0.0);
        vec3 radiance = dirLight.color;
        Lo += (kD * albedoColor / PI + specular) * radiance * NdotL;
    }

    for(int i = 0; i < numPointLights; i++) {
        // calculate per-light radiance
        vec3 L = normalize(pointLights[i].position - fragPosition);
        vec3 H = normalize(V + L);
        float distance = length(pointLights[i].position - fragPosition);
        float attenuation = 1.0 / (1.0 + 0.09 * distance + 0.0032 * (distance * distance));
        vec3 radiance = pointLights[i].color * pointLights[i].strength * attenuation;

        // Cook-Torrance BRDF
        float NDF = DistributionGGX(N, H, roughness);
        float G = GeometrySmith(N, V, L, roughness);
        vec3 F = fresnelSchlick(clamp(dot(H, V), 0.0, 1.0), F0);

        vec3 numerator = NDF * G * F;
        float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001; // + 0.0001 to prevent divide by zero
        vec3 specular = numerator / denominator;

        // kS is equal to Fresnel
        vec3 kS = F;
        // for energy conservation, the diffuse and specular light can't
        // be above 1.0 (unless the surface emits light); to preserve this
        // relationship the diffuse component (kD) should equal 1.0 - kS.
        vec3 kD = vec3(1.0) - kS;
        // multiply kD by the inverse metalness such that only non-metals
        // have diffuse lighting, or a linear blend if partly metal (pure metals
        // have no diffuse light).
        kD *= 1.0 - material.metallic;

        // scale light by NdotL
        float NdotL = max(dot(N, L), 0.0);

        // add to outgoing radiance Lo
        Lo += (kD * albedoColor / PI + specular) * radiance * NdotL;  // note that we already multiplied the BRDF by the Fresnel (kS) so we won't multiply by kS again

        total += Lo;
    }

    // ambient lighting (note that the next IBL tutorial will replace
    // this ambient lighting with environment lighting).
    float ao = material.ao;
    if (material.arm_texture.enabled == 1) {
        ao = texture(material.arm_texture.tex, oUVs).r;
    }

    vec3 ambient = vec3(0.03) * albedoColor * material.ao;
    vec3 color = ambient + total;

    // HDR tonemapping
    color = color / (color + vec3(1.0));
    // gamma correct
    color = pow(color, vec3(1.0 / 2.2));

    highp vec2 coordinates = gl_FragCoord.xy / resolution;
    highp float dither = random(coordinates);
    color += dither * NOISE_GRANULARITY;

    final_color = vec4(color, 1.0);
}
