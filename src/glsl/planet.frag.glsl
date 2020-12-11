#version 330 core
out vec4 FragColor;

in vec4 gl_FragCoord;
in vec3 Norm;
in vec2 TexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;

// Based on:
// https://www.scratchapixel.com/lessons/procedural-generation-virtual-worlds/simulating-sky/simulating-colors-of-the-sky
const float earthRadius = 6360e3;
const float atmosphereRadius = 6420e3;
const float Hr = 7994.0;
const float Hm = 1200.0;
const float PI = radians(180.0);
const vec3 betaR = vec3(3.8e-6, 13.5e-6, 33.1e-6);
const vec3 betaM = vec3(21e-6);
const float kInfinity = uintBitsToFloat(0x7F800000u);
//#define angle -0.95 * PI * ((iMouse.y / iResolution.y) - 0.5)
#define angle (0.0)
#define sunDirection vec3(0.0, cos(angle), -sin(angle))
// At high altitudes mie scattering is negligible
// Removing it might be worth it

bool solveQuadratic(float a, float b, float c, inout float x1, inout float x2) 
{ 
    if (b == 0.0) { 
        // Handle special case where the the two vector ray.dir and V are perpendicular
        // with V = ray.orig - sphere.centre
        if (a == 0.0) return false; 
        x1 = 0.0; x2 = sqrt(-c / a); 
        return true; 
    } 
    float discr = b * b - 4.0 * a * c; 
 
    if (discr < 0.0) return false; 
 
    float q = (b < 0.f) ? -0.5f * (b - sqrt(discr)) : -0.5f * (b + sqrt(discr)); 
    x1 = q / a; 
    x2 = c / q; 
 
    return true; 
} 

bool raySphereIntersect(const vec3 orig, const vec3 dir, const float radius, inout float t0, inout float t1) 
{ 
    // They ray dir is normalized so A = 1 
    float A = dir.x * dir.x + dir.y * dir.y + dir.z * dir.z; 
    float B = 2.0 * (dir.x * orig.x + dir.y * orig.y + dir.z * orig.z); 
    float C = orig.x * orig.x + orig.y * orig.y + orig.z * orig.z - radius * radius; 
 
    if (!solveQuadratic(A, B, C, t0, t1)) return false; 
 
    if (t0 > t1) t0 = t1, t1 = t0; // Verify this
 
    return true; 
} 

vec3 computeIncidentLight(const vec3 orig, const vec3 dir, float tmin, float tmax)
{ 
    float t0, t1;
    if (!raySphereIntersect(orig, dir, atmosphereRadius, t0, t1) || t1 < 0.0) return vec3(0.0); 
    if (t0 > tmin && t0 > 0.0) tmin = t0; 
    if (t1 < tmax) tmax = t1; 
    uint numSamples = 16u; 
    uint numSamplesLight = 8u; 
    float segmentLength = (tmax - tmin) / float(numSamples); 
    float tCurrent = tmin; 
    vec3 sumR = vec3(0.0), sumM = vec3(0.0); // mie and rayleigh contribution 
    float opticalDepthR = 0.0, opticalDepthM = 0.0; 
    float mu = dot(dir, sunDirection); // mu in the paper which is the cosine of the angle between the sun direction and the ray direction 
    float phaseR = 3.f / (16.f * PI) * (1.0 + mu * mu); 
    float g = 0.76f; 
    float phaseM = 3.f / (8.f * PI) * ((1.f - g * g) * (1.f + mu * mu)) / ((2.f + g * g) * pow(1.f + g * g - 2.f * g * mu, 1.5f)); 
    for (uint i = 0u; i < numSamples; ++i) { 
        vec3 samplePosition = orig + (tCurrent + segmentLength * 0.5f) * dir; 
        float height = length(samplePosition) - earthRadius; 
        // compute optical depth for light
        float hr = exp(-height / Hr) * segmentLength; 
        float hm = exp(-height / Hm) * segmentLength; 
        opticalDepthR += hr; 
        opticalDepthM += hm; 
        // light optical depth
        float t0Light, t1Light; 
        raySphereIntersect(samplePosition, sunDirection, atmosphereRadius, t0Light, t1Light); 
        float segmentLengthLight = t1Light / float(numSamplesLight), tCurrentLight = 0.0; 
        float opticalDepthLightR = 0.0, opticalDepthLightM = 0.0; 
        uint j; 
        for (j = 0u; j < numSamplesLight; ++j) { 
            vec3 samplePositionLight = samplePosition + (tCurrentLight + segmentLengthLight * 0.5f) * sunDirection; 
            float heightLight = length(samplePositionLight) - earthRadius; 
            if (heightLight < 0.0) break; 
            opticalDepthLightR += exp(-heightLight / Hr) * segmentLengthLight; 
            opticalDepthLightM += exp(-heightLight / Hm) * segmentLengthLight; 
            tCurrentLight += segmentLengthLight; 
        } 
        if (j == numSamplesLight) { 
            vec3 tau = betaR * (opticalDepthR + opticalDepthLightR) + betaM * 1.1f * (opticalDepthM + opticalDepthLightM); 
            vec3 attenuation = vec3(exp(-tau.x), exp(-tau.y), exp(-tau.z)); 
            sumR += attenuation * hr; 
            sumM += attenuation * hm; 
        } 
        tCurrent += segmentLength; 
    } 
 
    // We use a magic number here for the intensity of the sun (20). We will make it more
    // scientific in a future revision of this lesson/code
    
    return (sumR * betaR * phaseR + sumM * betaM * phaseM) * 20.0;
}

void main() {
	//vec3 dir = vec3(1.0);
	//vec3 atmCol = computeIncidentLight(vec3(0.0, earthRadius + 10000.0, 0.0), dir, 0.0, kInfinity);
	vec3 surfCol = texture(texture1, TexCoord).rgb;
	if (texture(texture2, TexCoord).r >= 0.05) {
		// Ocean
		surfCol *= (-acos(dot(vec3(0.0, 1.0, 0.0), Norm)) + (PI)) / 2.0;
	} else {
		// Ground
		surfCol *= (-acos(dot(vec3(0.0, 1.0, 0.0), Norm)) + (PI)) / 4.0 + 0.5;
	}
	vec3 atmCol = vec3(0.0, 0.0, 0.0);
	FragColor = vec4(surfCol + atmCol, 0.0);
}
