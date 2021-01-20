#version 330 core
out vec4 FragColor;

in vec4 gl_FragCoord;
in vec3 FragPos;
in vec3 Norm;
in vec2 TexCoord;

uniform sampler2D texture1;
uniform vec3 sun;
uniform vec3 cam_pos;

// I am assuming the sun is white (cos it is)

void main() {
    // TODO Perhaps alpha channel should be specular
    // Would need one less massive texture to be loaded :)
    // TODO moonlight would be cool
    vec4 data = texture(texture1, TexCoord);
    // If lighting is enabled
    // Sun comes pre-normalized
    vec3 result = data.rgb; // albedo
    if (sun != vec3(0.0, 0.0, 0.0)) {
        vec3 norm = normalize(Norm);
        float diff = max(dot(norm, sun), 0.0);
        float spec = 0.0;
        if (data.a <= 0.95) {
            // Specular
            vec3 viewDir = normalize(cam_pos - FragPos);
            vec3 reflectDir = reflect(-sun, norm);
            // (see learnopengl for meaning of this)
            spec = 1.25 * pow(max(dot(viewDir, reflectDir), 0.0), 256);
        }

        result *= vec3(0.05 + diff + spec);
	}

    FragColor = vec4(result, 1.0);
}
