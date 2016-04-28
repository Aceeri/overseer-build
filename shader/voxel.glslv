#version 150 core

in vec4 vox_Color;
in ivec4 vox_Pos;

in ivec4 vert_Pos;
in ivec4 vert_Normal;

out vec3 v_Position;
out vec3 v_Normal;
out vec4 v_Color;

uniform mat4 c_Transform;

void main() {
	gl_Position = c_Transform * (vert_Pos + 2 * vox_Pos);

	v_Color = vox_Color;
	v_Normal = vec3(vert_Normal);
	v_Position = vec3(vert_Pos + 2 * vox_Pos);
}