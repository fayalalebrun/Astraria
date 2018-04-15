#version 330 core

out vec4 FragColor;

uniform vec4 color;

uniform float og_farPlaneDistance;
uniform float u_logarithmicDepthConstant;

in float logz;

void main() {
	FragColor = color;
	gl_FragDepth = logz;
}
