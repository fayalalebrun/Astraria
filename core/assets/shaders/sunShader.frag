#version 330 core
out vec4 FragColor;

in vec2 TexCoord;
in vec3 FragPos;
in vec3 Normal;
in vec3 modelViewPos;

uniform sampler2D diffuseTex;
uniform sampler2D sunGradient;

uniform float temperature;
uniform vec3 camToSunDir;
uniform vec3 sunPos;
uniform mat4 modelView;
uniform mat4 view;

void main()
{
    vec3 norPos = normalize(modelViewPos);
    float theta = 1.0-(dot(normalize(vec3(view * vec4(sunPos,0.0))), -norPos));

    float u = (temperature - 800.0f) / 29200.0f;

    vec3 col = vec3(texture(diffuseTex,TexCoord)) - theta;
    col = col * texture(sunGradient,vec2(u, 0.5)).rgb;

    FragColor = vec4(col,1.0);

}
