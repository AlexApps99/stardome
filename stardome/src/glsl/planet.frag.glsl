#version 330 core
out vec4 FragColor;

in vec4 gl_FragCoord;
in vec3 FragPos;
in vec3 Norm;
in vec2 TexCoord;

uniform sampler2D texture1;
uniform vec3 sun;
uniform vec3 cam_pos;

#define DIFF_THRSH (0.05)
#define SPEC_THRSH (0.95)
#define SPEC_COEFF (1.0)
#define AMBIENT (0.0)
#define SHININESS (4.0)

// TODO Night lighting (cities)
// TODO Moonlight
void main() {
    vec4 data = texture(texture1, TexCoord);
    vec3 result = data.rgb;

    if (sun != vec3(0.0, 0.0, 0.0)) {
        // Diffuse
        vec3 light = normalize(sun);
        vec3 normal = normalize(Norm);
        float diffuse = max(dot(normal, light), 0.0);
        float specular = 0.0;

        // TODO fade into specular/diffuse (sharp edge looks bad)
        // Potentially determine using ocean color as well as bathymetry
        // TODO pick a nicer ocean color
        if (data.a <= SPEC_THRSH && diffuse > DIFF_THRSH) {
            discard;
            // Specular
            vec3 view = normalize(cam_pos - FragPos);
            vec3 halfway = normalize(light + view);
            specular = SPEC_COEFF * pow(max(dot(normal, halfway), 0.0), SHININESS);
        }

        result *= vec3(AMBIENT + diffuse + specular);
	}

    FragColor = vec4(result, 1.0);
}
