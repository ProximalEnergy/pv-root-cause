---
id: "cell-interconnection-failure"
title: "Cell Interconnection Failure"
category: "Modules"
tags:
  - "PV cell"
  - "cell interconnection"
  - "PV failure"
  - "solder bond"
severity: High
impact_factor: >-
    Broken or resistive cell interconnections can block current, activate bypass diodes, and cause severe power loss or thermal stress.
detection_method: >-
    Use electroluminescence, infrared thermography, IV curve analysis, and visual inspection to separate interconnection issues from cracks or diode failures.
mitigation: >-
    Replace affected modules when interconnect failures are electrically active and reduce mechanical or thermal stressors that drive progression.
contributors:
  - name: Alexandre Mathieu
    url: https://github.com/AlexandreHugoMathieu
images:
  - path: /assets/page/interconnection_images/intro.PNG
    caption: >-
      Figure 1: Disconnected cell interconnect with delamination [1]
    alt: >-
      Figure 1: Disconnected cell interconnect with delamination [1]
  - path: /assets/page/interconnection_images/metal_grid.PNG
    caption: >-
      Figure 2: Typical PV module with three bypass diodes and its cell interconnections
    alt: >-
      Figure 2: Typical PV module with three bypass diodes and its cell interconnections
  - path: /assets/page/interconnection_images/apparition_rate.PNG
    caption: >-
      Figure 3: Annual trend in number of modules with cell interconnect ribbon failures [2]
    alt: >-
      Figure 3: Annual trend in number of modules with cell interconnect ribbon failures [2]
  - path: /assets/page/interconnection_images/final.PNG
    caption: >-
      Figure 4: Glass breakage (left) and burn marks (right) caused from broken cell interconnections [2]
    alt: >-
      Figure 4: Glass breakage (left) and burn marks (right) caused from broken cell interconnections [2]
  - path: /assets/page/interconnection_images/IR.PNG
    caption: >-
      Figure 5: Infrared (left) and electroluminescence (right) images of PV module with disconnected cell interconnections [2]
    alt: >-
      Figure 5: Infrared (left) and electroluminescence (right) images of PV module with disconnected cell interconnections [2]
  - path: /assets/page/interconnection_images/IV.PNG
    caption: >-
      Figure 6: Infrared image and IV curve of a module with an interconnection failure [2]
    alt: >-
      Figure 6: Infrared image and IV curve of a module with an interconnection failure [2]
  - path: /assets/page/interconnection_images/diode_activation.PNG
    caption: >-
      Figure 7: Infrared image and IV curve of a module with two interconnection failures blocking the current flow [1]
    alt: >-
      Figure 7: Infrared image and IV curve of a module with two interconnection failures blocking the current flow [1]
---

**Interconnection failures** in photovoltaic (PV) modules can lead to severe **power losses**, and **long-term damage** with safety risks.


![Figure 1: Disconnected cell interconnect with delamination [1]](/assets/page/interconnection_images/intro.PNG)

*Figure 1: Disconnected cell interconnect with delamination [1]*


This post provides an overview of interconnection failures in PV modules, explores its failure signatures, apparition times, concomitant failures, and detection techniques.

# I.	Overview & root causes

In a crystalline silicon PV module, wafer-cells are typically interconnected in series with busbars (or cell interconnect ribbons). These busbars are connected from the front side to the rear side of the solar cells. A series of interconnected cells with a bypass diode in parallel is called a cell string [2]. These cell strings are typically interconnected in series or sometimes in parallel by string interconnect ribbons [2]. The cell interconnections are considered to be both the busbars and string interconnect ribbons.


![Figure 2: Typical PV module with three bypass diodes and its cell interconnections](/assets/page/interconnection_images/metal_grid.PNG)

*Figure 2: Typical PV module with three bypass diodes and its cell interconnections*


The interconnection failure often come from a combination of reasons at different stages [3]:
- **Manufacturing process**: poor soldering, misplacement of ribbons, too intense deformation of the ribbon kink, narrow distance between the cells  [2], [3], [4]. The lamination process could also facilitate failures [1].
- **Transportation**: Mechanical stress [2], [4], [5]. 
- **Operation**: Thermomechanical stress, repetitive wind load,  electrochemical corrosion can also fatigue the interconnections [3]. Hails is also mentioned as a stress [5].

The connection between busbars and string interconnect ribbon is even more subject to fatigue than the rest of the interconnections [2]. 

# II.	Signature

If redundant electrical interconnections are available, the failure of one them might have an negligible effect on the overall power output [3].  Nevertheless, if all interconnections of a cell are interrupted so the current flow is blocked, electric arcs might appear if the bypass diode does not operate correctly [3]. 

Overall, Kontges et al. [2] set up three stages to describe the interconnection failure effects:
1. **Stage 1:** Only a small number of cell interconnections are disconnected. The resistance in series is increased [2], [4] but the current still flows through the cells.
2. **Stage 2:** The cell interconnection ribbons are fully disconnected and the current flows constantly through the bypass diode. At this failure stage, safety risk heavily depends on the durability of this bypass diode [2].
Safety risks are also quite limited while the hot spot temperature does not reach more than 100°C [3]. 
3. **Final stage:** Safety is at stake where, for instance, the bypass has blown up which results in more damage to the module such as burn marks, glass breakage etc...[2]

In stage 1, the resistance in series seems to be mostly responsible for the power decrease which results in a decrease of fill factor [2], [6]. Colvin et al. particularly highlights that this loss is more severe during more sunny conditions [6]. When having 4 busbars, the power loss per interconnection cut is in the order of 0.1% due to an increase of the resistance in series around 1% [6]. With 3 busbars, the redundancy is decreased and the power loss important around 1%/cut [7]. Both studies [6][7] showed that cutting the outer connections lead to more losses than when cutting the inner connections.

# III.	Apparition

PV cell interconnection failure occurs over the whole PV installation lifetime. The figure below represents the distribution of the cell interconnection stages over time from a 1080-module system installed in 2004. Note that the first final stages occur 6 years after installation.


![Figure 3: Annual trend in number of modules with cell interconnect ribbon failures [2]](/assets/page/interconnection_images/apparition_rate.PNG)

*Figure 3: Annual trend in number of modules with cell interconnect ribbon failures [2]*


According to Kontges et al.  [1], cell interconnection failures seem to appear after 4 years of operating and keep occurring over the whole PV system lifetime.

Bansal et al. [4] emphasize that hot and dry, hot & humid and composite climates might be more prone to cell interconnection failures.

Kontges et al. [2] report that « defect cell interconnect » accounts for 10% of all PV module failures from a pool of customer complaints in the first two years after delivery (2 million PV modules in Germany). In another article, interconnection failure accounts for 3 failures among the 115 module failures from a group of 1-3 year-old modules in USA [8].

# IV.	Concomitant failures

Several failures usually occur at the same time as interconnection failure.

The corrosion of the cell metallization leads to interconnection fatigue and, potentially, cell interconnection failures [3], [5]. It is usually correlated with a visible light yellow to dark discoloration of the electric parts [3]. 

If all interconnection disconnections block the current flow in a cell string, the bypass diode will be activate, and will prematurely age. A failed bypass diode will result in open-circuit or short-circuit state. In case of open-circuited diode, the current will go through the failed cell string and dangerously generate heat which can lead to high temperatures, electric arcs and even fires [3]. Then, in progressed stages, the interconnection failures might cause glass breakage and burn marks [2], [3], [5], [9] as shown in the Figure below.


![Figure 4: Glass breakage (left) and burn marks (right) caused from broken cell interconnections [2]](/assets/page/interconnection_images/final.PNG)

*Figure 4: Glass breakage (left) and burn marks (right) caused from broken cell interconnections [2]*


 


A defect or detached junction box might also facilitate humidity ingress, resulting in higher corrosion levels with weaker cell interconnections [3].

Hotspots caused from partial shading might accentuate the thermomechanical stress and cause more fatigue on the cell interconnections [2].

# V.	Detection

Cell interconnection failures are not easy to see with the naked eye [3]. However, burn marks and glass breakage can occur in progressed stages and might reveal them [3]. 

Infrared images would highlight hotspots caused by interconnect failures [3]. More specifically, poor soldering lead to higher power dissipation and localized heating [3].


![Figure 5: Infrared (left) and electroluminescence (right) images of PV module with disconnected cell interconnections [2]](/assets/page/interconnection_images/IR.PNG)

*Figure 5: Infrared (left) and electroluminescence (right) images of PV module with disconnected cell interconnections [2]*


 


Some other technics are listed to detect interconnection failures [2]:
- Electroluminescence can identify it with darker regions at the failed interconnects [3].
- UV imaging can also detect ribbon breakages [2]. 
- The signal transmission method is also a cheap method to identify interconnection failures [2].

IV curves also enable to detect the performance loss from cell interconnection failures. In the Figure below, the interconnections are redundant, and the interconnection failure causes an increase of the resistance in series with a more “round” knee aspect. It is also combined with some aging effect which decreases the expected photocurrent.


![Figure 6: Infrared image and IV curve of a module with an interconnection failure [2]](/assets/page/interconnection_images/IV.PNG)

*Figure 6: Infrared image and IV curve of a module with an interconnection failure [2]*


At more advanced stages, the I-V curve can also detect these issues. When multiple interconnection failures block the flow of current, the bypass diode activates, as illustrated in the figure below.


![Figure 7: Infrared image and IV curve of a module with two interconnection failures blocking the current flow [1]](/assets/page/interconnection_images/diode_activation.PNG)

*Figure 7: Infrared image and IV curve of a module with two interconnection failures blocking the current flow [1]*


Replacing the module is then advised in advances stages and when burn marks are visible [5].

Finally, interconnection failures might lead to critical performance losses and safety risks. Then, it is crucial to closely monitor PV modules to avoid those situations.

**References**

[1]	M. Köntges, G. Oreski, U. Jahn, M. Herz, P. Hacke, and K.-A. Weiss, ‘Assessment of photovoltaic module failures in the field’, IEA PVPS, IEA-PVPS T13-09:2017, 2017.

[2]	M. Köntges et al., ‘Review of Failures of Photovoltaic Modules’, IEA PVPS T13, Technical Report IEA-PVPS T13-01:2014, 2014.

[3]	M. Herz, G. Friesen, U. Jahn, M. Köntges, S. Lindig, and D. Moser, ‘Quantification of Technical Risks in PV power Systems’, IEA PVPS, Technical Report IEA-PVPS T13-23:2021, Feb. 2022.

[4]	N. Bansal, S. P. Jaiswal, and G. Singh, ‘Comparative investigation of performance evaluation, degradation causes, impact and corrective measures for ground mount and rooftop solar PV plants – A review’, Sustain. Energy Technol. Assess., vol. 47, p. 101526, Oct. 2021, doi: 10.1016/j.seta.2021.101526.

[5]	C. Miquel, C. Stravrou, N. Lebert, and J. Sarantou, ‘Dysfonctionnement électriques des installations photovoltaïques: points de vigilance.’, AQC - HESPUL, Technical Report PTVIGI1801, Oct. 2018.

[6]	D. J. Colvin, E. J. Schneller, and K. O. Davis, ‘Impact of interconnection failure on photovoltaic module performance’, Prog. Photovolt. Res. Appl., vol. 29, no. 5, pp. 524–532, 2021, doi: 10.1002/pip.3401.

[7]	E. Annigoni et al., ‘Quantifying and modeling the impact of interconnection failures on the electrical performance of crystalline silicon photovoltaic modules’, Prog. Photovolt. Res. Appl., vol. 27, no. 5, pp. 424–432, 2019, doi: 10.1002/pip.3111.

[8]	S. Deng et al., ‘Research on hot spot risk for high-efficiency solar module’, Energy Procedia, vol. 130, pp. 77–86, 2017, doi: 10.1016/j.egypro.2017.09.399.

[9]	E. Sarquis Filho, ‘Toward a Predictive Strategy for the Operation and Maintenance of PV Systems’, PhD Thesis, 2023.
