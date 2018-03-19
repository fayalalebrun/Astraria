#version 330 core
out vec4 FragColor;

in vec2 TexCoord;
in vec3 FragPos;
in vec3 Normal;

struct PointLight {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};
uniform PointLight pointLights[8];

uniform sampler2D diffuseTex;
uniform int nLights;

vec3 calcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir);

void main()
{


    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(-FragPos);

    vec3 result = vec3(0.0);

    for(int i = 0; i < nLights; i++){
        result+=calcPointLight(pointLights[i], norm, FragPos, viewDir);
    }

    FragColor = vec4(result,1.0);

    if(nLights==0){
            FragColor = texture(diffuseTex, TexCoord);
    }
}

vec3 calcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
{
    vec3 lightDir = normalize(light.position - fragPos);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // combine results
    vec3 ambient  = light.ambient  * vec3(texture(diffuseTex, TexCoord));
    vec3 diffuse  = light.diffuse  * diff * vec3(texture(diffuseTex, TexCoord));

    return (ambient + diffuse);
}