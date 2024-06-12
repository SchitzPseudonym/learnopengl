#version 330 core
out vec4 FragColor;

in vec3 poscolor;
in vec2 TexCoords;

uniform float running_time;
uniform sampler2D our_texture;

void main()
{
    FragColor = texture(our_texture, TexCoords);
    //FragColor = vec4(poscolor + running_time, 1.0);
}
