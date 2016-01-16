#version 300 es
precision mediump float; // set the default precision quali er,

in vec4 v_color;

out vec4 fragColor;

void main()
{
  fragColor = v_color;
}
