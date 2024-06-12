#version 330 core

in vec3 position; // the position variable has attribute position 0
in vec2 tex_coords;
in vec3 color;

out vec3 fcolor;
out vec2 ftex_coords;

void main()
{
    gl_Position = vec4(position, 1.0);
    ftex_coords = tex_coords;
    fcolor = color;
}
