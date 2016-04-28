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
	/*vec3 sun = vec3(cos(Time), sin(Time), cos(Time));

	vec3 normal = normalize(v_Normal);
	vec3 light_dir = normalize(light_pos - v_Position);

	float lambertian = max(dot(light_dir,normal), 0.0) * light_brightness;
	float sun_lambertian = max(dot(sun,normal), 0.0);

	float total = (lambertian + sun_lambertian);

	if (total > 0.05) {
		Target0 = total * v_Color;
	} else {
		Target0 = v_Color * 0.05;
	}*/

	vec3 normal = normalize(v_Normal);
	vec3 ambient = vec3(0.05, 0.05, 0.05);

	float brightness = 0.05;
	//vec3 color = vec3(0.0, 0.0, 0.0);
	for (int i = 0; i < MAX_LIGHTS; i++) { 
		Light light = u_Lights[i];

		//vec4 light_local = light.proj * vec4(v_Position, 1.0);

		vec3 light_dir = normalize(light.pos.xyz - v_Position);
		float diffuse = max(0.0, dot(normal, light_dir));

		brightness += diffuse * light.color.w;
		//color += light.color.xyz * diffuse * light.color.w;
	}

	Target0 = brightness * v_Color;
}