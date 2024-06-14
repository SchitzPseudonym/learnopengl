#version 450 core

uniform mat4 transform;

in vec3 position;
in vec2 tex_coords;

out vec2 tex_coordsf;

void main()
{
    tex_coordsf = tex_coords;
    gl_Position = transform * vec4(position, 1.0);
}
