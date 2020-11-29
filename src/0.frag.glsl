#version 330 core
out vec4 FragColor;

in vec4 gl_FragCoord;
in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D texture1;

void main() {
	FragColor = texture(texture1, TexCoord);
}
