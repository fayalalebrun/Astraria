#version 330 core
out vec4 FragColor;

in vec2 TexCoord;
in vec3 FragPos;
in vec3 Normal;

void main()
{
    //float n = (noise4(vec4(FragPos, 1.0), 4, 40.0, 0.7) + 1.0) * 0.5;

    float total = 1;
    FragColor = vec4(total, total, total, 1.0);
}
