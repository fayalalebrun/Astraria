#version 330 core
out vec4 FragColor;

in vec2 TexCoord;
in vec3 FragPos;
in vec3 Normal;
in vec3 modelViewPos;

uniform sampler2D diffuseTex;

uniform vec3 camToSunDir;
uniform vec3 sunPos;
uniform mat4 modelView;
uniform mat4 view;

void main()
{
    vec3 norPos = normalize(modelViewPos);
    float theta = 1.0-(dot(normalize(vec3(view * vec4(sunPos,0.0))), -norPos));

    FragColor = vec4(vec3(texture(diffuseTex,TexCoord).xyz) - theta,1.0);

}
