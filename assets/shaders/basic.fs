#import "version_directive.glsl"

in vec2 tex_coords;
in vec3 color;

out vec4 frag_color;

uniform sampler2D tex;

void main()
{
  frag_color = texture(tex, tex_coords) * vec4(color.xyz, 1.0);
}
