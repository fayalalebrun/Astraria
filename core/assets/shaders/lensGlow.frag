#version 330 core
out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D tex;

void main() {
    float alpha = texture(tex,TexCoord).r;
	FragColor = vec4(1.0,1.0,1.0,alpha);
}
