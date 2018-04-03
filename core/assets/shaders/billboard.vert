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
    position /= position.w;

    float offsetX = (billboardWidth/screenWidth);
    float offsetY = (billboardHeight/screenHeight);

    // Move the vertex in directly screen space. No need for CameraUp/Right_worlspace here.
    //gl_Position += vec4( position.xy + vec2(aPos.x * offsetX, aPos.y * offsetY),position.z,position.w);

    gl_Position = position  + vec4(aPos.x * offsetX, aPos.y * offsetY, 0.0, 0.0);

    TexCoord = aTexCoord;
}