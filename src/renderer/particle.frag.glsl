#version 330

in vec4 color;

out vec4 outColor;

void main()
{
    vec2 coord = gl_PointCoord - vec2(0.5);
    if (length(coord) > 0.5)
        discard;
    outColor = vec4(1.0, 0.0, 1.0, color.a);
}
