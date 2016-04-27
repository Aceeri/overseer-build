#version 150 core

in vec4 v_Color;
out vec4 Target0;

uniform vec4 t_Color;

void main() {
  Target0 = v_Color;
}