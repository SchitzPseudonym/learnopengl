#version 330 core
out vec4 FragColor;

in vec3 fcolor;
in vec2 ftex_coords;

uniform sampler2DArray texture_array;

void main()
{
    vec4 texture1 = texture(texture_array, vec3(ftex_coords, 0));
    vec4 texture2 = texture(texture_array, vec3(ftex_coords, 1));
    FragColor = texture1 * texture2;
}
