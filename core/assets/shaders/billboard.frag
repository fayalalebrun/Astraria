#version 330 core
out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D tex;

void main() {
	gl_FragColor = texture(tex, TexCoord);
}
