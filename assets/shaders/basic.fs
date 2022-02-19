#import "version_directive.glsl"

in vec3 color;
out vec4 frag_color;

void main()
{
  frag_color = vec4(color.xyz, 1.0);
}
