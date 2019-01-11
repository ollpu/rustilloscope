#version 140

const int BUF_SIZE = 1024;

uniform vec2 windowSize;
uniform vec2 mouse;
uniform float scroll;
out vec4 color;

layout(std140) uniform Buffer {
    vec4 array[BUF_SIZE/4];
};

float get(int ind) {
    return array[ind >> 2][ind & 3];
}

float dist_line(vec2 line, vec2 point) {
    return length(point - line * dot(line, point) / dot(line, line));
}

vec2 l2g(vec2 point, vec2 vp1, vec2 vp2) {
    return point * (vp2 - vp1) + vp1;
}

vec2 l2g2(vec2 point, vec4 vp) {
    return point * (vp.zw - vp.xy) + vp.xy;
}

vec2 mandelbrot_iter(vec2 z, vec2 c) {
    vec2 znext = vec2(z.x * z.x - z.y * z.y, 2 * z.x * z.y);
    return znext + c;
}

void main() {
    vec2 vp1 = vec2(-2.0, -1.0);
    vec2 vp2 = vec2(1.0, 1.0);
    vec4 vp = vec4(vp1, vp2);
    float zoom = pow(2.0, scroll);
    vec2 c = l2g2(
        gl_FragCoord.xy / windowSize,
        vec4(zoom * vp1 - 2.0*mouse + vec2(1.0), zoom * vp2 - 2.0*mouse + vec2(1.0))
    );
    vec2 z0 = vec2(0.0);
    int i = 0;
    const int max_iter = 600;
    for (; i < max_iter; ++i) {
        z0 = mandelbrot_iter(z0, c);
        if (length(z0) > 2) break;
    }

    color = vec4(float(i) / float(max_iter));

    // vec2 viewstart = vec2(-1.0, -1.0) * windowSize;
    // vec2 viewend = vec2(2.0, 2.0) * windowSize;
    // vec2 globalpos = l2g(gl_FragCoord.xy, viewstart, viewend);

    // if (length(mouse * windowSize - gl_FragCoord.xy) < 10) {
    //     color = vec4(1.,0.,0.,1.);
    //     return;
    // }

    // int ind = int(float(BUF_SIZE) * globalpos.x / windowSize.x);
    // if (ind < 0 || ind >= BUF_SIZE) {
    //     color = vec4(0.,1.,float(ind)/float(BUF_SIZE),1.);
    //     return;
    // }

    // float val = get(ind);
    // float val_next = get(ind + 1);

    // vec2 point = vec2(
    //     windowSize.x / float(BUF_SIZE) / 2.0,
    //     gl_FragCoord.y - val
    // );

    // if (l2g(gl_FragCoord.xy, viewstart, viewend).x > 0.0) {
    //     color = vec4(1.);
    // }

   // vec2 viewPort[2];
   // viewPort[0] = vec2(-1.0, -1.0);
   // viewPort[1] = vec2(1.0, 1.0);
   // vec2 viewPortDiff = viewPort[1] - viewPort[0];
   // vec2 viewSize = viewPortDiff * windowSize;

   // vec2 relpos = gl_FragCoord.xy / windowSize;
   // vec2 viewpos = viewPort[0] + relpos * viewPortDiff;

   // if (length(mouse - relpos) < 0.05) {
   //     color = vec4(1.,0.,0.,1.);
   //     return;
   // }

   // int ind = int(float(BUF_SIZE) * viewpos.x);
   // float val = get(ind);
   // float val_next = get(ind + 1);
   // vec2 point = l2g(
   //     vec2(
   //         0.5 / float(BUF_SIZE),
   //         val - relpos.y
   //     ),
   //     viewPort[0],
   //     viewPort[1]
   // );
   // vec2 line = l2g(
   //     vec2(
   //         1.0 / float(BUF_SIZE),
   //         val_next - val
   //     ),
   //     viewPort[0],
   //     viewPort[1]
   // );

   // if (dist_line(line, point) < 0.1) {
   //     color = vec4(1.0);
   // } else {
   //     color = vec4(0.0);
   // }
}
