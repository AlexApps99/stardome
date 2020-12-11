#version 330 core
out vec4 FragColor;

in vec4 gl_FragCoord;
in vec3 Norm;
in vec2 TexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main() {
	vec3 surfCol = texture(texture1, TexCoord).rgb;
	if (texture(texture2, TexCoord).r >= 0.05) {
		// Ocean
		surfCol *= (-acos(dot(vec3(0.0, 1.0, 0.0), Norm)) + (PI)) / 2.0;
	} else {
		// Ground
		surfCol *= (-acos(dot(vec3(0.0, 1.0, 0.0), Norm)) + (PI)) / 4.0 + 0.5;
	}
	vec3 atmCol = vec3(0.0, 0.0, 0.0);
	FragColor = vec4(surfCol + atmCol, 0.0);
}
