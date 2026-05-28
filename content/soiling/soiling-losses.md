---
id: "soiling-losses"
title: "Soiling Losses"
category: "Soiling"
tags:
  - "Soiling"
  - "PV loss"
  - "cleaning"
  - "environmental loss"
severity: Medium
impact_factor: >-
    Dust, snow, dirt, and other deposited particles reduce irradiance reaching the module surface and can create geographically variable production losses.
detection_method: >-
    Use soiling stations, clean/dirty reference sensors, performance-ratio trend analysis, visual inspection, and weather-normalized production comparison.
mitigation: >-
    Optimize cleaning schedules, module tilt, anti-soiling approaches, and site-specific maintenance rules based on measured soiling rate and cleaning economics.
contributors:
  - name: Alexandre Mathieu
    url: https://github.com/AlexandreHugoMathieu
images:
  - path: /assets/page/soiling_images/overview_hsu.png
    caption: "HSU estimation, worldwide (SolarAnywhere)"
    alt: "HSU estimation, worldwide (SolarAnywhere)"
  - path: /assets/page/soiling_images/soiling_type_ilse.png
    caption: "Soiling types (Ilse et al., 2019)"
    alt: "Soiling types (Ilse et al., 2019)"
  - path: /assets/page/soiling_images/soiling_signature.png
    caption: "Soiling signatures (Maghami et al., 2016)"
    alt: "Soiling signatures (Maghami et al., 2016)"
  - path: /assets/page/soiling_images/soiling_estimation.png
    caption: "Soiling estimation with \"Stochastic Rate and Recovery\" (Deceglie et al., 2018)"
    alt: "Soiling estimation with \"Stochastic Rate and Recovery\" (Deceglie et al., 2018)"
  - path: /assets/page/soiling_images/cleaning_types.png
    caption: "Cleaning methods as exposed in (Ilse et al., 2019)"
    alt: "Cleaning methods as exposed in (Ilse et al., 2019)"
  - path: /assets/page/soiling_images/mitigation_technics.png
    caption: "Mitigations techics listed in (Ilse et al., 2019)"
    alt: "Mitigations techics listed in (Ilse et al., 2019)"
---

"**Soiling losses** refer to loss in power resulting from snow, dirt, dust and other particles that cover the surface of the PV module." (Maghami et al., 2016)

### I.	Overview

From a worldwide perspective, PV soiling losses were estimated to rise from 3-4% in 2019 to 4-7% in 2023 (Ilse et al., 2019). It has the specificity to be reversible (Köntges et al., 2017) and depends over time on weather and dust physical characteristics (Fernández-Solas et al., 2022; Maghami et al., 2016). As presented by Solar Anywhere with the HSU model (Coello and Boyle, 2019) just below, those features make it heterogenous over the world with significant performance losses in hot and dry climates with up to 1%/day of soiling rate in some dry-climate countries and up to 2%/day in China (Ilse et al., 2019). In contrast, France has its distribution median estimated to 0.02%/day.


![HSU estimation, worldwide (SolarAnywhere)](/assets/page/soiling_images/overview_hsu.png)

*HSU estimation, worldwide (SolarAnywhere)*


Soiling depends on several factors and are subject to (Ilse et al., 2019; Maghami et al., 2016):
- *Environmental factors*: rainfall frequency/quantity , wind, humidity…
- *Installation*: tilt angle, location, pv panel materials...
Concentrator PV especially are particularly sensitive to soiling.
Below 10 degrees of tilt, cleaning maintenance should be studied.
- *Dust characteristics*: particle matter air densities, type..
Especially, some “cementing” effect of the dust particles can make it extremely hard to remove.

According to the well-documented report from (Ilse et al., 2019) with the figure below, soiling types can be categorized into 6 categories.

 

![Soiling types (Ilse et al., 2019)](/assets/page/soiling_images/soiling_type_ilse.png)

*Soiling types (Ilse et al., 2019)*


### II.	Signature

In regards to the impact on the I-V characteristics of the module, two main signatures can be identified (Maghami et al., 2016):

- « Soft shading »: the dust decreases the optic transmisttance with an homogeneous effect.
- « Hard shading »: solid dust particules block sunlight with a more abrupt effect : most often non homogenous.

 

![Soiling signatures (Maghami et al., 2016)](/assets/page/soiling_images/soiling_signature.png)

*Soiling signatures (Maghami et al., 2016)*


Then, when connected in series, modules with different levels of soiling could also force the inverter MPP-tracker to operate in a non-optimal way (Maghami et al., 2016).

When looking at performance loss over time, several models or estimation methods with different complexities are used in the literature such as the HSU model (Coello and Boyle, 2019) or Kimber’s model (Kimber et al., 2006). For a more complete review,  (Ilse et al., 2019) or  (Schill et al., 2022) list a collection of those. One method which outstands to infer aging and soiling effects from operating variables is RdTools (Deceglie et al., n.d.). Overall, those models often express the drop in transmittance or, directly, power due to additional soiling in %/day.

 

![Soiling estimation with "Stochastic Rate and Recovery" (Deceglie et al., 2018)](/assets/page/soiling_images/soiling_estimation.png)

*Soiling estimation with "Stochastic Rate and Recovery" (Deceglie et al., 2018)*


### III.	Cleaning

Natural cleaning from rainfalls is an open debate in the literature with different granted thresholds (Schill et al., 2022). On one hand, according to (Maghami et al., 2016), at least 20 mm of rainfall is necessary to completely clean PV modules while more conservative thresholds (0.5-5mm) are considered by (Coello and Boyle, 2019). Also, a small rainfall could potentially participate to additional dust particle depositions. Another one could also argue that rainfall do not get rid of those "cemented” particles and active cleaning becomes a necessity. As illustrated by (Ilse et al., 2019) below, the different types of cleaning go from manual to fully automatic with different costs, location habits and scalabilities.

 

![Cleaning methods as exposed in (Ilse et al., 2019)](/assets/page/soiling_images/cleaning_types.png)

*Cleaning methods as exposed in (Ilse et al., 2019)*


Automatic cleaning appears to clean more than 95% and is estimated to a cost of 2.4 – 8.2 euros/m². One must also note that for building-mounted PV installations, cleaning soiling is 3 to 8 times more expensive than ground-mounted installation.

### IV.	Prevention

The Anti-Soiling Coating is among the most spread methods to mitigate soiling but some other methods also exists as presented by (Ilse et al., 2019) below.

 

![Mitigations techics listed in (Ilse et al., 2019)](/assets/page/soiling_images/mitigation_technics.png)

*Mitigations techics listed in (Ilse et al., 2019)*


Also, the IEC 61724-1 (IEC, 2021) provides recommendations in order to mitigate soiling. As for instance, to get class « A », a soiling station should be installed on-site.

To conclude, soiling is a site-specific PV loss which can significantly entail performances and should be considered carefully. All in all, in the cases where soiling has a severe impact on productions, different approaches exist to mitigate its effects.


### References

Coello, M., Boyle, L., 2019. Simple Model for Predicting Time Series Soiling of Photovoltaic Panels. IEEE J. Photovolt. 9, 1382–1387. https://doi.org/10.1109/JPHOTOV.2019.2919628

Deceglie, M.G., Ambarish Nag, Adam Shinn, Kimball, G., Ruth, D., Jordan, D., Yan, J., Anderson, K., Perry, K., Mikofski, M., Muller, M., Vining, W., Deline, C., n.d. RdTools.

Deceglie, M.G., Micheli, L., Muller, M., 2018. Quantifying Soiling Loss Directly From PV Yield. IEEE J. Photovolt. 8, 547–551. https://doi.org/10.1109/JPHOTOV.2017.2784682

Fernández-Solas, Á., Montes-Romero, J., Micheli, L., Almonacid, F., Fernández, E.F., 2022. Estimation of soiling losses in photovoltaic modules of different technologies through analytical methods. Energy 244, 123173. https://doi.org/10.1016/J.ENERGY.2022.123173

IEC, 2021. IEC 61724-1: Photovoltaic system performance monitoring – Monitoring.

Ilse, K., Micheli, L., Figgis, B.W., Lange, K., Daßler, D., Hanifi, H., Wolfertstetter, F., Naumann, V., Hagendorf, C., Gottschalg, R., Bagdahn, J., 2019. Techno-Economic Assessment of Soiling Losses and Mitigation Strategies for Solar Power Generation. Joule 3, 2303–2321. https://doi.org/10.1016/J.JOULE.2019.08.019

Kimber, A., Mitchell, L., Nogradi, S., Wenger, H., 2006. The Effect of Soiling on Large Grid-Connected Photovoltaic Systems in California and the Southwest Region of the United States, in: 2006 IEEE 4th World Conference on Photovoltaic Energy Conference. pp. 2391–2395. https://doi.org/10.1109/WCPEC.2006.279690

Köntges, M., Oreski, G., Jahn, U., Herz, M., Hacke, P., Weiss, K.-A., 2017. Assessment of photovoltaic module failures in the field (No. IEA-PVPS T13-09:2017). IEA PVPS.

Maghami, M.R., Hizam, H., Gomes, C., Radzi, M.A., Rezadad, M.I., Hajighorbani, S., 2016. Power loss due to soiling on solar panel: A review. Renew. Sustain. Energy Rev. 59, 1307–1316. https://doi.org/10.1016/J.RSER.2016.01.044

Schill, C., Anderson, A., Baldus-Jeursen, C., Burnham, L., Micheli, L., Parlevliet, D., Pilat, E., Stridh, B., Urrejola, E., 2022. Soiling Losses – Impact on the Performance of Photovoltaic Power Plants (Technical Report No. IEA-PVPS T13-21:2022). IEA.
