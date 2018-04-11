#version 330 core
out vec4 FragColor;

uniform vec3 v3LightPos;
uniform float g;
uniform float g2;

in vec3 v3Direction;
in vec4 primaryColor;
in vec4 secondaryColor;

void main (void)
{
	float fCos = dot(v3LightPos, v3Direction) / length(v3Direction);
	float fMiePhase = 1.5 * ((1.0 - g2) / (2.0 + g2)) * (1.0 + fCos*fCos) / pow(1.0 + g2 - 2.0*g*fCos, 1.5);
    FragColor = primaryColor + fMiePhase * secondaryColor;
	FragColor.a = FragColor.b;
}
