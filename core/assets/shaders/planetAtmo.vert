#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aNormal;

// Vertex
uniform vec3 star_pos;		// point light source position (Sun) [GCS]
uniform vec3 planet_pos;	// planet center position [GCS]
uniform float overglow;		// cos(angle) of terminator propagation\

uniform mat4 view;
uniform mat4 modelView;
uniform mat4 projection;
uniform float og_farPlaneDistance;
uniform float u_logarithmicDepthConstant;

out vec3 lpos;			// point light source position (Sun) [camera space]
out vec3 ppos;			// planet center position [camera space]
out vec3 pixel_pos;		// fragment position [camera space]
out vec3 pixel_nor;		// fragment surface normal
out vec2 pixel_txy;		// fragment surface texture coord
out float angleIncidence;
out vec4 colAtmosphere; //color of the atmosphere
out vec3 lightDir; //direction of light in camera space

const float PI = 3.14159265;
const float transitionWidth = 0.1; //How prominent the atmosphere is
const float fresnelExponent = 20;


vec4 modelToClipCoordinates(vec4 position, mat4 modelViewPerspectiveMatrix, float depthConstant, float farPlaneDistance){
	vec4 clip = modelViewPerspectiveMatrix * position;

	clip.z = ((2.0 * log(depthConstant * clip.z + 1.0) / log(depthConstant * farPlaneDistance + 1.0)) - 1.0) * clip.w;
	return clip;
}

void main()
	{


	lpos=(view*vec4(star_pos,1.0)).xyz;
	ppos=(view*vec4(planet_pos,1.0)).xyz;

	lightDir =normalize(lpos-ppos);

	pixel_pos=vec3(modelView*vec4(aPos,1.0));
	pixel_nor=normalize(mat3(transpose(inverse(modelView))) * aNormal);
	pixel_txy=aTexCoord;

	vec3 viewDir = normalize(-pixel_pos);

    float dotProd = dot(lightDir, pixel_nor);
    dotProd = clamp(dotProd,-1.0,1.0);

	angleIncidence = acos(dotProd) / PI;

    float shadeFactor = 0.1 * (1 - angleIncidence) + 0.9 * (1 - (clamp(angleIncidence, 0.5, 0.5 + transitionWidth) - 0.5) / transitionWidth);

    float dotProd2 = dot(pixel_nor, viewDir);
    dotProd2 = clamp(dotProd2,-1.0,1.0);

    float angleToViewer = sin(acos(dotProd2));

    float perspectiveFactor = 0.3 + 0.2 * pow(angleToViewer, fresnelExponent) + 0.5 * pow(angleToViewer, fresnelExponent * 20);

    colAtmosphere = vec4(perspectiveFactor*shadeFactor);

	gl_Position = modelToClipCoordinates(vec4(aPos, 1.0), projection * modelView, u_logarithmicDepthConstant, og_farPlaneDistance);
	}
