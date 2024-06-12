#version 330 core

in vec3 position; // the position variable has attribute position 0
in vec2 tex_coords;
//in vec3 color;

out vec3 ourColor;
out vec2 TexCoords;

void main()
{
    gl_Position = vec4(position, 1.0);
    TexCoords = tex_coords;
    //ourColor = aColor;
}
