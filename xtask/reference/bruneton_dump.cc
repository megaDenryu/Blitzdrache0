// Bruneton 2017実装のCPU参照から、大気の物理量の期待値を焼き出す。
// このファイル自体はBlitzdrache0のコードであり、参照実装のヘッダを取り込んで呼ぶだけである。
// 出典・取得手順・リビジョンは xtask/src/gen_atmosphere_reference.rs の冒頭にある。
//
// 大気パラメータと各レコードの引数は、いったんfloatで作ってからdoubleへ広げて渡す。
// 比較相手のRust側がf32で同じ値を持つため、こうしないと入力そのものが1e-8程度ずれ、
// 出力の食い違いが式の違いによるものか入力の違いによるものか分けられなくなる。
//
// 透過率は波長ごとの成分を持つため、赤・緑・青それぞれについて定数スペクトルの大気を組み直して
// 参照実装の透過率関数を3回呼ぶ。密度・交差距離・光学距離・位相関数は波長に依存しない。

#include <cmath>
#include <cstdio>

#include "atmosphere/reference/functions.h"

namespace {

using namespace atmosphere::reference;

constexpr float kBottomRadius = 6360000.0f;
constexpr float kTopRadius = 6460000.0f;
constexpr float kRayleighScattering[3] = {5.802e-6f, 13.558e-6f, 33.100e-6f};
constexpr float kRayleighScaleHeight = 8000.0f;
constexpr float kMieExtinction = 4.440e-6f;
constexpr float kMieScaleHeight = 1200.0f;
constexpr float kAbsorptionExtinction[3] = {0.650e-6f, 1.881e-6f, 0.085e-6f};
constexpr float kAbsorptionBottom = 10000.0f;
constexpr float kAbsorptionPeak = 25000.0f;
constexpr float kAbsorptionTop = 40000.0f;
constexpr float kAsymmetry = 0.8f;
constexpr int kSampleCount = 500;

double Wide(float value) { return static_cast<double>(value); }

DensityProfileLayer EmptyLayer() {
  return DensityProfileLayer(0.0 * m, 0.0, 0.0 / m, 0.0 / m, 0.0);
}

// Rust側は減衰率を -1/尺度高度 としてf32で持つため、同じ順序でfloat演算してから広げる。
DensityProfileLayer ExponentialLayer(float scale_height) {
  const float decay = -(1.0f / scale_height);
  return DensityProfileLayer(0.0 * m, 1.0, Wide(decay) / m, 0.0 / m, 0.0);
}

void SetTentProfile(DensityProfile* profile) {
  const float up_width = kAbsorptionPeak - kAbsorptionBottom;
  const float down_width = kAbsorptionTop - kAbsorptionPeak;
  const float up_linear = 1.0f / up_width;
  const float up_constant = -kAbsorptionBottom / up_width;
  const float down_linear = -(1.0f / down_width);
  const float down_constant = kAbsorptionTop / down_width;
  profile->layers[0] = DensityProfileLayer(Wide(kAbsorptionPeak) * m, 0.0, 0.0 / m,
      Wide(up_linear) / m, Wide(up_constant));
  profile->layers[1] = DensityProfileLayer(0.0 * m, 0.0, 0.0 / m,
      Wide(down_linear) / m, Wide(down_constant));
}

AtmosphereParameters BuildAtmosphere(int channel) {
  AtmosphereParameters atmosphere;
  atmosphere.bottom_radius = Wide(kBottomRadius) * m;
  atmosphere.top_radius = Wide(kTopRadius) * m;
  atmosphere.rayleigh_density.layers[0] = EmptyLayer();
  atmosphere.rayleigh_density.layers[1] = ExponentialLayer(kRayleighScaleHeight);
  atmosphere.rayleigh_scattering =
      ScatteringSpectrum(Wide(kRayleighScattering[channel]) / m);
  atmosphere.mie_density.layers[0] = EmptyLayer();
  atmosphere.mie_density.layers[1] = ExponentialLayer(kMieScaleHeight);
  atmosphere.mie_scattering = ScatteringSpectrum(Wide(kMieExtinction) / m);
  atmosphere.mie_extinction = ScatteringSpectrum(Wide(kMieExtinction) / m);
  atmosphere.mie_phase_function_g = Wide(kAsymmetry);
  SetTentProfile(&atmosphere.absorption_density);
  atmosphere.absorption_extinction =
      ScatteringSpectrum(Wide(kAbsorptionExtinction[channel]) / m);
  atmosphere.ground_albedo = DimensionlessSpectrum(0.1);
  atmosphere.mu_s_min = -0.2;
  return atmosphere;
}

// 地平線の余弦は 1 - (下端半径 / 半径)^2 という2つの近い量の差を含む。観測点が地表のすぐ上にあると
// この差が1e-5級まで縮み、floatで計算すると有効数字が4桁失われる。期待値はdoubleで出す。
double HorizonCosine(float radius) {
  const double ratio = Wide(kBottomRadius) / Wide(radius);
  return -std::sqrt(1.0 - ratio * ratio);
}

void PrintParameters() {
  printf("param bottom_radius %.17g\n", Wide(kBottomRadius));
  printf("param top_radius %.17g\n", Wide(kTopRadius));
  const char* channel_name[3] = {"r", "g", "b"};
  for (int i = 0; i < 3; ++i) {
    printf("param rayleigh_scattering_%s %.17g\n", channel_name[i],
        Wide(kRayleighScattering[i]));
    printf("param absorption_extinction_%s %.17g\n", channel_name[i],
        Wide(kAbsorptionExtinction[i]));
  }
  printf("param rayleigh_scale_height %.17g\n", Wide(kRayleighScaleHeight));
  printf("param mie_extinction %.17g\n", Wide(kMieExtinction));
  printf("param mie_scale_height %.17g\n", Wide(kMieScaleHeight));
  printf("param absorption_bottom %.17g\n", Wide(kAbsorptionBottom));
  printf("param absorption_peak %.17g\n", Wide(kAbsorptionPeak));
  printf("param absorption_top %.17g\n", Wide(kAbsorptionTop));
  printf("param asymmetry %.17g\n", Wide(kAsymmetry));
  printf("param sample_count %d\n", kSampleCount);
}

void PrintDensities(const AtmosphereParameters& atmosphere) {
  const float altitudes[] = {0.0f, 500.0f, 1000.0f, 2000.0f, 5000.0f, 8000.0f,
      10000.0f, 12000.0f, 17500.0f, 25000.0f, 32500.0f, 40000.0f, 50000.0f,
      60000.0f, 80000.0f, 100000.0f};
  for (float altitude : altitudes) {
    const Length h = Wide(altitude) * m;
    printf("density %.17g %.17g %.17g %.17g\n", Wide(altitude),
        GetProfileDensity(atmosphere.rayleigh_density, h)(),
        GetProfileDensity(atmosphere.mie_density, h)(),
        GetProfileDensity(atmosphere.absorption_density, h)());
  }
}

void PrintRayRecords(const AtmosphereParameters atmospheres[3]) {
  const float offsets[] = {0.0f, 100.0f, 1000.0f, 10000.0f, 25000.0f, 50000.0f,
      99000.0f, 100000.0f};
  for (float offset : offsets) {
    const float radius = kBottomRadius + offset;
    const Length r = Wide(radius) * m;
    const double horizon_exact = HorizonCosine(radius);
    printf("horizon %.17g %.17g\n", Wide(radius), horizon_exact);

    // 標本点として使う天頂余弦はRust側がf32で持つため、floatへ落としてから組み立てる。
    const float horizon = static_cast<float>(horizon_exact);
    const float above[] = {1.0f, 0.7f, 0.3f, 0.05f, 0.0f, horizon * 0.9f,
        horizon * 0.5f, horizon * 0.999f};
    for (float mu_f : above) {
      if (mu_f < horizon) {
        continue;
      }
      const Number mu(Wide(mu_f));
      printf("top_distance %.17g %.17g %.17g\n", Wide(radius), Wide(mu_f),
          DistanceToTopAtmosphereBoundary(atmospheres[0], r, mu).to(m));
      printf("optical_length %.17g %.17g %.17g %.17g %.17g\n", Wide(radius),
          Wide(mu_f),
          ComputeOpticalLengthToTopAtmosphereBoundary(
              atmospheres[0], atmospheres[0].rayleigh_density, r, mu).to(m),
          ComputeOpticalLengthToTopAtmosphereBoundary(
              atmospheres[0], atmospheres[0].mie_density, r, mu).to(m),
          ComputeOpticalLengthToTopAtmosphereBoundary(
              atmospheres[0], atmospheres[0].absorption_density, r, mu).to(m));
      double transmittance[3];
      for (int channel = 0; channel < 3; ++channel) {
        transmittance[channel] = ComputeTransmittanceToTopAtmosphereBoundary(
            atmospheres[channel], r, mu)(550.0 * nm)();
      }
      printf("transmittance %.17g %.17g %.17g %.17g %.17g\n", Wide(radius),
          Wide(mu_f), transmittance[0], transmittance[1], transmittance[2]);
    }

    const float below[] = {-1.0f, -0.5f, horizon - 0.001f,
        (horizon - 1.0f) * 0.5f};
    for (float mu_f : below) {
      if (mu_f < -1.0f || mu_f >= horizon) {
        continue;
      }
      const Number mu(Wide(mu_f));
      printf("bottom_distance %.17g %.17g %.17g\n", Wide(radius), Wide(mu_f),
          DistanceToBottomAtmosphereBoundary(atmospheres[0], r, mu).to(m));
    }
  }
}

void PrintPhases() {
  const float cosines[] = {-1.0f, -0.9f, -0.5f, -0.2f, 0.0f, 0.2f, 0.5f, 0.9f,
      0.99f, 1.0f};
  for (float nu_f : cosines) {
    const Number nu(Wide(nu_f));
    printf("phase %.17g %.17g %.17g\n", Wide(nu_f),
        (RayleighPhaseFunction(nu) * sr)(),
        (MiePhaseFunction(Number(Wide(kAsymmetry)), nu) * sr)());
  }
}

}  // namespace

int main() {
  const AtmosphereParameters atmospheres[3] = {
      BuildAtmosphere(0), BuildAtmosphere(1), BuildAtmosphere(2)};
  PrintParameters();
  PrintDensities(atmospheres[0]);
  PrintRayRecords(atmospheres);
  PrintPhases();
  return 0;
}
