#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aNormal;

// Vertex
uniform vec3 star_pos;		// point light source position (Sun) [GCS]
uniform vec3 planet_pos;	// planet center position [GCS]
uniform float planet_r;		// planet radius
uniform float overglow;		// cos(angle) of terminator propagation\

uniform mat4 modelView;
uniform mat4 projection;
uniform float og_farPlaneDistance;
uniform float u_logarithmicDepthConstant;

out vec3 lpos;			// point light source position (Sun) [camera space]
out vec3 ppos;			// planet center position [camera space]
out vec3 pixel_pos;		// fragment position [camera space]
out vec3 pixel_nor;		// fragment surface normal
out vec2 pixel_txy;		// fragment surface texture coord
out float atm1d_tx;		// fragment surface texture coord


vec4 modelToClipCoordinates(vec4 position, mat4 modelViewPerspectiveMatrix, float depthConstant, float farPlaneDistance){
	vec4 clip = modelViewPerspectiveMatrix * position;

	clip.z = ((2.0 * log(depthConstant * clip.z + 1.0) / log(depthConstant * farPlaneDistance + 1.0)) - 1.0) * clip.w;
	return clip;
}

void main()
	{
	lpos=vec3(modelView*vec4(star_pos,1.0));
	ppos=vec3(modelView*vec4(planet_pos,1.0));

	pixel_pos=vec3(modelView*vec4(aPos,1.0));
	pixel_nor=mat3(transpose(inverse(modelView))) * aNormal;
	pixel_txy=aTexCoord;

	atm1d_tx=length(vec2(pixel_pos.xy-ppos.xy))/planet_r;

	gl_Position = modelToClipCoordinates(vec4(aPos, 1.0), projection * modelView, u_logarithmicDepthConstant, og_farPlaneDistance);
	}
