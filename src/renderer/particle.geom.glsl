#version 330

layout(points) in;

layout(line_strip, max_vertices = 2) out;

in vec3 vVelocity[];

out vec4 color;

void main()
{
    vec3 tmpColor = abs(vVelocity[0]) * 64.0;

    gl_Position = gl_in[0].gl_Position;
    color = vec4(tmpColor, 1.0);
    EmitVertex();

    gl_Position = gl_in[0].gl_Position - (vec4(vVelocity[0], 0.0) * 4.0);
    color = vec4(tmpColor, 0.0);
    EmitVertex();

    EndPrimitive();
}
