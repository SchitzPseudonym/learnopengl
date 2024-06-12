#version 330 core
out vec4 FragColor;

in vec3 fcolor;
in vec2 ftex_coords;

uniform sampler2D our_texture;

void main()
{
    FragColor = texture(our_texture, ftex_coords) * vec4(fcolor, 1.0);
}
