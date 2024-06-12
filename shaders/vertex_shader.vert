#version 330 core

in vec3 position; // the position variable has attribute position 0
in vec3 color;

out vec3 poscolor;

void main()
{
    poscolor = position;
    gl_Position = vec4(position, 1.0); // see how we directly give a vec3 to vec4's constructor
}