#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 view;
uniform mat4 projection;
uniform float og_farPlaneDistance;
uniform float u_logarithmicDepthConstant;

uniform float FC;

out float logz;

vec4 modelToClipCoordinates(vec4 position, mat4 modelViewPerspectiveMatrix, float depthConstant, float farPlaneDistance){
	vec4 clip = modelViewPerspectiveMatrix * position;

	clip.z = ((2.0 * log(depthConstant * clip.z + 1.0) / log(depthConstant * farPlaneDistance + 1.0)) - 1.0) * clip.w;
	return clip;
}

void main()
{
    //gl_Position = modelToClipCoordinates(vec4(aPos, 1.0), projection * view, u_logarithmicDepthConstant, og_farPlaneDistance);
    gl_Position = projection * view * vec4(aPos,1.0);

    logz = gl_Position.z;
}