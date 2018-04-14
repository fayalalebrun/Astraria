#version 330 core

out vec4 FragColor;

uniform vec4 color;

in float logz;

void main() {
	FragColor = color;
	gl_FragDepth = logz;
}
