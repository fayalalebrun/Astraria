#version 330 core
out vec4 FragColor;

in vec2 TexCoord;
in float dotView;

uniform sampler2D tex;

void main() {
    if(dotView < 0 ){
        discard;
    }

    float alpha = texture(tex,TexCoord).r;
	FragColor = vec4(1.0,1.0,1.0,alpha);
}
