#version 330 core
out vec4 FragColor;

// Fragment
uniform vec3 star_pos;		// point light source position (Sun) [GCS]
uniform vec3 planet_pos;	// planet center position [GCS]
//uniform vec3 planet_r;		// planet radius

uniform sampler2D ambTex;
uniform sampler2D diffuseTex;
uniform sampler2D txratm; //Texture where gradient is stored
uniform vec4 atmoColorMod;

in vec3 lpos;			// point light source position (Sun) [camera space]
in vec3 ppos;			// planet center position [camera space]
in vec3 pixel_pos;		// fragment position [camera space]
in vec3 pixel_nor;		// fragment surface normal
in vec2 pixel_txy;		// fragment surface texture coord
in float angleIncidence;
in vec4 colAtmosphere;
in vec3 lightDir;

struct PointLight {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};
uniform PointLight pointLights[8];

uniform int nLights;


uniform int useAmbTex;

float minDiff = 1.0;

vec4 calcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir);

void main()
	{
	float li;
	vec3 c,lt_dir,c0;
	vec4 c1;


    vec3 norm = normalize(pixel_nor);
    vec3 viewDir = normalize(-pixel_pos);

    vec4 result = vec4(0.0);

    for(int i = 0; i < nLights; i++){
        result+=calcPointLight(pointLights[i], norm, pixel_pos, viewDir);
    }


    if(minDiff<0 && useAmbTex == 1){
            if(minDiff<-0.25){
                minDiff = -0.25;
            }

            minDiff = minDiff * -4;

            minDiff = 1.0 - minDiff;
            result = mix(texture(ambTex, pixel_txy),result, minDiff);
    }

    c0 = result.xyz;

    vec2 gradientLevel = vec2(angleIncidence, 0.5);
    c1 = colAtmosphere * texture(txratm, gradientLevel) * 1.4;
    c1 = c1 * atmoColorMod;



    c = c1.a * c1.rgb + (vec3(1.0, 1.0, 1.0) - c1.a) * c0; //Blend atmosphere with diffuse color


	FragColor=vec4(c,1.0);
//	gl_FragDepth=0.0;
	}

vec4 calcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
{
    vec3 lightDir = normalize(light.position - fragPos);
    // diffuse shading
    float diffBef = dot(normal, lightDir);

    float diff = max(diffBef, 0.0);

    if(diffBef<minDiff){
        minDiff = diffBef;
    }

    // combine results
    vec4 ambient  = vec4(light.ambient,1.0)  * texture(diffuseTex, pixel_txy);
    vec4 diffuse  = vec4(light.diffuse,1.0)  * diff * texture(diffuseTex, pixel_txy);

    return ambient + diffuse;
}