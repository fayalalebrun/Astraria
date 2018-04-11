#version 330 core
out vec4 FragColor;

in vec4 primaryColor;
in vec4 secondaryColor;
in vec2 TexCoord;

uniform sampler2D diffuseTex;

void main (void)
{
	FragColor = primaryColor + 0.25 * secondaryColor;
	FragColor = texture(diffuseTex, TexCoord) + FragColor - texture(diffuseTex,TexCoord);
	//gl_FragColor = gl_Color + texture2D(s2Tex1, gl_TexCoord[0].st) * texture2D(s2Tex2, gl_TexCoord[1].st) * gl_SecondaryColor;
}
