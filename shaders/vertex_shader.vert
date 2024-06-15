#version 450 core

uniform mat4 transform;
uniform mat4 orthographic;
uniform mat4 perspective;

in vec3 position;
in vec2 tex_coords;

out vec2 tex_coordsf;

void main()
{
    tex_coordsf = tex_coords;
    mat4 projection = orthographic * perspective;
    gl_Position = transform * projection * vec4(position, 1.0);
}
