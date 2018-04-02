#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;

uniform float billboardWidth;
uniform float billboardHeight;
uniform float screenWidth;
uniform float screenHeight;
uniform vec3 billboardOrigin;

uniform mat4 modelView;
uniform mat4 projection;
uniform float og_farPlaneDistance;
uniform float u_logarithmicDepthConstant;


vec4 modelToClipCoordinates(vec4 position, mat4 modelViewPerspectiveMatrix, float depthConstant, float farPlaneDistance){
	vec4 clip = modelViewPerspectiveMatrix * position;

	clip.z = ((2.0 * log(depthConstant * clip.z + 1.0) / log(depthConstant * farPlaneDistance + 1.0)) - 1.0) * clip.w;
	return clip;
}

void main()
{
    vec4 position = modelToClipCoordinates(vec4(billboardOrigin, 1.0), projection * modelView, u_logarithmicDepthConstant, og_farPlaneDistance);

    float offsetX = (billboardWidth/screenWidth)  - 1;
    float offsetY = (billboardHeight/screenHeight)  - 1;

    gl_Position = position  + vec4(aPos.x * offsetX, aPos.y * offsetY, 0.0, 0.0);

    TexCoord = aTexCoord;
}