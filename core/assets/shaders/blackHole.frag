#version 330 core
out vec4 FragColor;

in vec2 TexCoord;
in vec3 FragPos;
in vec3 Normal;
in vec3 modelViewPos;


uniform vec3 holePos;
uniform mat4 modelView;
uniform mat4 view;
uniform samplerCube skybox;

void main()
{
    float ratio = 1.00 / 1.52;
    vec3 I = normalize(-FragPos);
    vec3 R = refract(I, normalize(Normal), ratio);


    vec3 norPos = normalize(modelViewPos);
    float theta = -(dot(normalize(vec3(view * vec4(holePos,0.0))), norPos)) - 0.7;
    theta = max(theta, 0.0);


    vec3 col = texture(skybox, R).rgb;


    FragColor = vec4(col-theta,1.0);

}
