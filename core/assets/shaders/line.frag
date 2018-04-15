#version 330 core

out vec4 FragColor;

uniform vec4 color;

uniform float og_farPlaneDistance;
uniform float u_logarithmicDepthConstant;

in float logz;

void main() {
	FragColor = color;
	gl_FragDepth = ((2.0 * log(u_logarithmicDepthConstant * logz + 1.0) /
	    log(u_logarithmicDepthConstant * og_farPlaneDistance + 1.0)) - 1.0);
}
