#version 150 core

const int MAX_LIGHTS = 10;

struct Light {
	vec4 pos;
	vec4 color;
	mat4 proj;
};

in vec3 v_Position;
in vec3 v_Normal;
in vec4 v_Color;

out vec4 Target0;

uniform float Time;

uniform b_Lights {
	Light u_Lights[MAX_LIGHTS];
};

void main() {
	vec3 normal = normalize(v_Normal);
	vec3 ambient = vec3(0.05, 0.05, 0.05);

	float brightness = 0.05;
	for (int i = 0; i < MAX_LIGHTS; i++) { 
		Light light = u_Lights[i];

		//vec4 light_local = light.proj * vec4(v_Position, 1.0);

		vec3 light_dir = normalize(light.pos.xyz - v_Position);
		float diffuse = max(0.0, dot(normal, light_dir));

		brightness += diffuse * light.color.w;
	}

	Target0 = brightness * v_Color;
}