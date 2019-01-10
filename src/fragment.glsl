#version 140
#define SIZE 1024

uniform vec2 windowSize;
uniform vec2 mouse;
out vec4 color;

float distLine(vec2 line, vec2 point) {
    float s = dot(line, point) / dot(line, line);
    vec2 x = point-line*s;
    return length(x);
}

layout(packed) uniform Buffer {
    vec4 array[SIZE/4];
};

void main() {
    int idx = int(gl_FragCoord.x/windowSize.x*float(SIZE));
    float val = array[idx >> 2][idx & 3]/2.0+0.25;
    float nval = (idx+1==SIZE)
        ? -100.0
        : array[idx+1 >> 2][idx+1 & 3]/2.0+0.25;

    vec2 diffline = vec2(1, windowSize.y*(nval - val));
    vec2 point = vec2(0, gl_FragCoord.y - val*windowSize.y);

    float dist = distLine(diffline, point);

    vec2 mousepos = vec2(
            sqrt(2)*windowSize.x * mouse.x,
            windowSize.y * (1-sqrt(2)*mouse.y)
            );
    if (length(mousepos - gl_FragCoord.xy) < 10.0) {
        color = vec4(1., 1., 1., 1.);
        return;
    }
    if (dist < 3) {
        color = vec4(0.2, 1., 0.2, 1.);
    } else {
        color = vec4(0., 0.2, 0., 1.);
    }
}
