#version 330 core
out vec4 FragColor;

in vec2 TexCoord;
in float dotView;

uniform sampler2D tex;
uniform sampler2D spectrumTex;
uniform float temperature;

void main() {
    if(dotView < 0 ){
        discard;
    }

    float u = (temperature - 800.0f) / 29200.0f;
    float v = 1.0 - texture(tex,TexCoord).r + 0.125;

    float alpha = texture(tex,TexCoord).r;
	FragColor = vec4(vec3(texture(spectrumTex, vec2(u, v))),alpha);
}
