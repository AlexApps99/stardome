#version 300 es
precision highp float;
out vec4 FragColor;

in vec3 FragPos;

// Based on https://www.scratchapixel.com/lessons/procedural-generation-virtual-worlds/simulating-sky/simulating-colors-of-the-sky

// Direction of sun (not position)
uniform vec3 sun_dir;
// Position relative to world
uniform vec3 cam_pos;
// Position relative to body
uniform vec3 pos;
// Radius of Earth
uniform float Re;
// Radius of Atmosphere
uniform float Ra;
// Rayleigh Scale height
uniform float Hr;
// Rayleigh attenuation coefficients
uniform vec3 betaR;
// Mie Scale height
//uniform float Hm;
// Mie attenuation coefficients
//uniform vec3 betaM;
// Mie asymmetry factor
//uniform float g;
// Sun intensity factor
uniform float intensity;

#define PI radians(180.0)
#define INFINITY uintBitsToFloat(0x7F800000u)

//#define MIE
#define SAMPLES (1u)
#define SAMPLES_LIGHT (1u)
#define MIE_EXTINCTION_MUL (1.1)

// Dir must be normalized
// Orig must be centered on sphere
bool raySphereIntersect(const in vec3 orig, const in vec3 dir, const in float radius, out float t0, out float t1) {
    float b = dot(dir, orig);
    float c = dot(orig, orig) - (radius * radius);
	float test = b*b - c;
    // Intersection should have two points
    if (test <= 0.0) return false;
	test = sqrt(test);
	t0 = -b - test;
	t1 = -b + test;
	if (t0 > t1) t0 = t1, t1 = t0;
	return true;
}

vec4 computeIncidentLight(const in vec3 orig, const in vec3 dir, in float tmin, in float tmax, const in vec3 sunDirection) {
    float t0, t1;
    if (!raySphereIntersect(orig, dir, Ra, t0, t1) || t1 < 0.0) discard;
    if (t0 > tmin && t0 > 0.0) tmin = t0;
    if (t1 < tmax) tmax = t1;
    float segmentLength = (tmax - tmin) / float(SAMPLES);
    float tCurrent = tmin;
    vec3 sumR = vec3(0.0); // rayleigh contribution
    float opticalDepthR = 0.0;
    float mu = dot(dir, sunDirection); // mu in the paper which is the cosine of the angle between the sun direction and the ray direction
    float phaseR = 3.0 / (16.0 * PI) * (1.0 + mu * mu);
    #ifdef MIE
    vec3 sumM = vec3(0.0); // mie contribution
    float opticalDepthM = 0.0;
    float phaseM = 3.0 / (8.0 * PI) * ((1.0 - g * g) * (1.0 + mu * mu)) / ((2.0 + g * g) * pow(1.0 + g * g - 2.0 * g * mu, 1.5));
    #endif
    for (uint i = 0u; i < SAMPLES; ++i) {
        vec3 samplePosition = orig + (tCurrent + segmentLength * 0.5) * dir;
        float height = length(samplePosition) - Re;
        // compute optical depth for light
        float hr = exp(-height / Hr) * segmentLength;
        opticalDepthR += hr;
        #ifdef MIE
        float hm = exp(-height / Hm) * segmentLength;
        opticalDepthM += hm;
        #endif
        // light optical depth
        float t0Light, t1Light;
        raySphereIntersect(samplePosition, sunDirection, Ra, t0Light, t1Light);
        float segmentLengthLight = t1Light / float(SAMPLES_LIGHT), tCurrentLight = 0.0;
        float opticalDepthLightR = 0.0;
        #ifdef MIE
        float opticalDepthLightM = 0.0;
        #endif
        uint j;
        for (j = 0u; j < SAMPLES_LIGHT; ++j) {
            vec3 samplePositionLight = samplePosition + (tCurrentLight + segmentLengthLight * 0.5) * sunDirection;
            float heightLight = length(samplePositionLight) - Re;
            if (heightLight < 0.0) break;
            opticalDepthLightR += exp(-heightLight / Hr) * segmentLengthLight;
            #ifdef MIE
            opticalDepthLightM += exp(-heightLight / Hm) * segmentLengthLight;
            #endif
            tCurrentLight += segmentLengthLight;
        }
        if (j == SAMPLES_LIGHT) {
            #ifdef MIE
            vec3 tau = betaR * (opticalDepthR + opticalDepthLightR) + betaM * MIE_EXTINCTION_MUL * (opticalDepthM + opticalDepthLightM);
            #else
            vec3 tau = betaR * (opticalDepthR + opticalDepthLightR);
            #endif
            vec3 attenuation = vec3(exp(-tau.x), exp(-tau.y), exp(-tau.z));
            sumR += attenuation * hr;
            #ifdef MIE
            sumM += attenuation * hm;
            #endif
        }
        tCurrent += segmentLength;
    }

    #ifdef MIE
    return vec4((sumR * betaR * phaseR + sumM * betaM * phaseM) * intensity, 1.0);
    #else
    return vec4((sumR * betaR * phaseR) * intensity, 1.0);
    #endif
}

void main() {
    vec3 dir = normalize(FragPos * 1e6 - cam_pos);
	float t0, t1, tMax = INFINITY;
    if (raySphereIntersect(pos, dir, Re, t0, t1) && t0 > 0.0) {
        tMax = t0;
    }

    FragColor = computeIncidentLight(pos, dir, 0.0, tMax, sun_dir);
}
