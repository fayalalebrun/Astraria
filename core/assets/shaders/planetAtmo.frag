#version 330 core
out vec4 FragColor;

// Fragment
uniform vec3 star_pos;		// point light source position (Sun) [GCS]
uniform vec3 planet_pos;	// planet center position [GCS]
//uniform vec3 planet_r;		// planet radius

uniform sampler2D diffuseTex;
uniform sampler2D txratm; //Texture where gradient is stored
in vec3 lpos;			// point light source position (Sun) [camera space]
in vec3 ppos;			// planet center position [camera space]
in vec3 pixel_pos;		// fragment position [camera space]
in vec3 pixel_nor;		// fragment surface normal
in vec2 pixel_txy;		// fragment surface texture coord
in float angleIncidence;
in vec4 colAtmosphere;
in vec3 lightDir;

void main()
	{
	float li;
	vec3 c,lt_dir,c0;
	vec4 c1;
	lt_dir=lightDir; // vector from fragment to point light source
	li=dot(pixel_nor,lt_dir);
	if (li<0.0) {
	    li=0.0;
	}
	if (li>1.0) {
	    li=1.0;
	}

    vec2 gradientLevel = vec2(angleIncidence, 0.5);
    c1 = colAtmosphere * texture(txratm, gradientLevel) * 1.4;

	c0=texture(diffuseTex,pixel_txy).rgb * li;


    c = c1.a * c1.rgb + (vec3(1.0, 1.0, 1.0) - c1.a) * c0; //Blend atmosphere with diffuse color


	FragColor=vec4(c,1.0);
//	gl_FragDepth=0.0;
	}
