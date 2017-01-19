# Shader
---
##1. first line
```
#version 300 es
```
Line 1 provides the version of the Shading Language—information that must appear on the  rst line of the shader (#version 300 es indicates the OpenGL ES Shading Language v3.00).

##. in out
```
in vec4 v_color;

out vec4 fragColor;
```
input and output

##. main
```
void main()
{
  v_color = a_color;
  gl_Position = u_mvpMatrix * a_position;
}
```
