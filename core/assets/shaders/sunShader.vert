#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aNormal;

out vec2 TexCoord;
out vec3 Normal;
out vec3 FragPos;
out vec3 modelViewPos;

uniform mat4 modelView;
uniform mat4 projection;
uniform mat4 view;
uniform float og_farPlaneDistance;
uniform float u_logarithmicDepthConstant;


vec4 modelToClipCoordinates(vec4 position, mat4 modelViewPerspectiveMatrix, float depthConstant, float farPlaneDistance){
	vec4 clip = modelViewPerspectiveMatrix * position;

	clip.z = ((2.0 * log(depthConstant * clip.z + 1.0) / log(depthConstant * farPlaneDistance + 1.0)) - 1.0) * clip.w;
	return clip;
}

void main()
{
    modelViewPos = vec3(modelView*vec4(aPos,0.0));
    gl_Position = modelToClipCoordinates(vec4(aPos, 1.0), projection * modelView, u_logarithmicDepthConstant, og_farPlaneDistance);
    FragPos = vec3(modelView * vec4(aPos, 1.0));
    Normal = mat3(transpose(inverse(modelView))) * aNormal;
    TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}