#version 330 core
out vec4 FragColor;

// Fragment
uniform vec3 star_pos;		// point light source position (Sun) [GCS]
uniform vec3 planet_pos;	// planet center position [GCS]
//uniform vec3 planet_r;		// planet radius
uniform float overglow;		// cos(angle) of terminator propagation

uniform sampler2D diffuseTex;
uniform sampler2D txratm;
in vec3 lpos;			// point light source position (Sun) [camera space]
in vec3 ppos;			// planet center position [camera space]
in vec3 pixel_pos;		// fragment position [camera space]
in vec3 pixel_nor;		// fragment surface normal
in vec2 pixel_txy;		// fragment surface texture coord
in float atm1d_tx;		// fragment surface texture coord

void main()
	{
	float li;
	vec3 c,c0,c1,lt_dir;
	lt_dir=normalize(star_pos-planet_pos); // vector from fragment to point light source
	li=dot(pixel_nor,lt_dir)+overglow;
	if (li<0.0) li=0.0;
	if (li>1.0) li=1.0;

	c0=texture(diffuseTex,pixel_txy).rgb;
	c1=texture(txratm,vec2(atm1d_tx,0.5)).rgb;

	c=((c0)+c1)*li;

	FragColor=vec4(c,1.0);
//	gl_FragDepth=0.0;
	}
