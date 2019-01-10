#version 140
#define SIZE 1024

uniform vec2 windowSize;
uniform vec2 mouse;
out vec4 color;

layout(std140) uniform Buffer {
    vec4 array[SIZE/4];
};

float get(int ind) {
    return array[ind >> 2][ind & 3];
}

float dist_line(vec2 line, vec2 point) {
    return length(point - line * dot(line, point) / dot(line, line));
}

vec2 viewPort[2];

void main() {
    viewPort[0] = vec2(0.0);
    viewPort[1] = vec2(1.0);

    vec2 relpos = gl_FragCoord.xy / windowSize;
    vec2 viewpos = viewPort[0] + relpos * (viewPort[1] - viewPort[0]);

    if (length(mouse - relpos) < 0.05) {
        color = vec4(1.,0.,0.,1.);
        return;
    }

    if (viewpos.x * viewpos.y > 0.5) {
        color = vec4(1.,1.,1.,1.);
    } else {
        color = vec4(0.,0.,0.,1.);
    }
}
