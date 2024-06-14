#version 450 core

uniform sampler2DArray texture_array;

in vec2 tex_coordsf;

out vec4 FragColor;

void main()
{
    FragColor = texture(texture_array, vec3(tex_coordsf, 0));
}
