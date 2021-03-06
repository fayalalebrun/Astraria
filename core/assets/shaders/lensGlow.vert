#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;
out float dotView;

uniform float width;
uniform float height;
uniform float screenWidth;
uniform float screenHeight;
uniform vec3 uPos;
uniform vec3 camDir;

uniform mat4 modelView;
uniform mat4 projection;


void main()
{
    vec4 position = projection * modelView * vec4(uPos,1.0);
    position /= position.w;
    position.z = -0.99;

    float offsetX = (width/screenWidth);
    float offsetY = (height/screenHeight);


    gl_Position = position  + vec4(aPos.x * offsetX, aPos.y * offsetY, 0.0, 0.0);

    TexCoord = aTexCoord;

    vec3 dirFromCam = normalize(uPos);
    dotView = dot(dirFromCam, camDir);

}