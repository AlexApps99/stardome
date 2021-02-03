#version 330 core
out vec4 FragColor;

in vec3 TexCoords;

uniform samplerCube skybox;
uniform vec3 sun_dir;
uniform float sun_angle_rad;

// Fade multiplier
const float n = 5.0;

void main() {
    // TODO sun fades off either gaussian, or by invnerse square law lol idk
    float sun = acos(dot(normalize(sun_dir), normalize(TexCoords)));
    float sun_mix = 0.0;
    if (sun < sun_angle_rad) {
        sun_mix = 1.0;
    } else if (sun < sun_angle_rad * n) {
        float m = -1.0 / (n * sun_angle_rad - sun_angle_rad);
        float c = 1.0 - m * sun_angle_rad;
        sun_mix = m * sun + c;
    }
    FragColor = vec4(texture(skybox, TexCoords).rgb + vec3(20.0 * sun_mix), 1.0);
}
