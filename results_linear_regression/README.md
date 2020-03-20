---
title: Linearna regresia
bibliography: ref.bib
---

*V texte su vynechane vektorove znacenia. Veliciny $\theta, y, \epsilon$ su vektorove veliciny.*


Linearna regresia je odhad vektoru parametrov $\theta$ pomocou linearneho modelu $m = \mathbf{A} \theta$, o ktorom dufame, ze pre odhadnute $\hat{\theta}$ bude $m \approx y$, kde $y$ su data.
$\mathbf{A}$ je modelova matica a pre tento problem ma tvar:
\begin{gather}
    \mathbf{A} = 
    \begin{pmatrix}
        1 & x_0^2 \\ 
        \vdots & \vdots \\
        1 & x_N^2
    \end{pmatrix}
\end{gather}

V linearnom pripade su odhady parametrov vypocitane ako:
\begin{gather}
    \hat{\theta} = \left(
                   \mathbf{A}^{\mathrm{T}} \mathbf{W} \mathbf{A} 
             \right)^{-1} 
             \mathbf{A}^{\mathrm{T}} \mathbf{W} y,
\end{gather}
kde $\mathbf{W}$ je vahova matica.
V tomto probleme sa neuplatini, bude identita, kedze body maju rovnaku vahu.
Vzorec je mozne odvodit z minimalizacie $g = \sum_{i=1}^{N} (y_i - m_i) w_{ij} (y_j - m_j)$.

Chyby a korelacie parametrov su dane kovariancnou maticou:
\begin{gather}
    c_{ij} \equiv \mathrm{Cov}(x_i,x_j) = E((x_i - E(x_i)) (x_j - E(x_j))) = E(x_i x_j) - E(x_i) E(x_j)
\end{gather}
Oznacim:
\begin{gather}
    \xi_{ij} \equiv \left( \left( \mathbf{A}^{\mathrm{T}} \mathbf{A} \right)^{-1} \mathbf{A}^{\mathrm{T}}  \right)_{ij}
\end{gather}
potom dosadenim odhadov $\hat{\theta}$ (uz s $W = I$):
\begin{gather}
    \mathrm{Cov} (\hat{\theta}_i, \hat{\theta}_j) = 
        E \left(\xi_{ki} y_i  \xi_{lj} y_j
        \right)
        - E \left(\xi_{ki} y_i \right)
        E \left(\xi_{lj} y_j \right) = \\
        \xi_{ki} \mathrm{Cov}(y_i, y_j) \xi_{jl}
\end{gather}
Predpokladame, ze $\mathbf{C}(y,y)$ je diagonalna, ale s neznamym rozpylom (rovnaky pre vsetky body).
Nevychyleny odhad rozptylu je:
\begin{gather}
    \sigma_y^2 \approx s_y^2 = \frac{\epsilon_i \epsilon_i}{N - p},
\end{gather}
kde $N$ je pocet bodov $y$ a $p$ je pocet fitovanych parametrov.


# Intervalovy odhad

Odhadovane parametre su tiez nahodne premenne.
Rozdelenie $\epsilon$ je nahodne s nulovym prvym momentom a rozpylom $\sigma_y^2$.
Z toho vyplyva, ze rozdelenie $\hat{\theta} - \theta$ je tiez normalne, kedze je dane linearnym vztahom z $\epsilon$.
Rozdelenie $\frac{\epsilon_i \epsilon_i}{\sigma_y^2} \equiv R$ je $\chi^2$ z definicie $\chi^2$.
Zaujima nas, z akeho rozdelenia bude $\theta$.
Odhadovany rozptyl parametrov $s_{\hat{\theta}}^2$ je dany diagonalou $\mathbf{C}(\hat{\theta}, \hat{\theta})$ a suvisi s odhadnutym rozpytlom $s_y^2$:
\begin{gather}
    s_{\hat{\theta}i}^2 = \mathrm{Cov}(\hat{\theta}_i, \hat{\theta}_i) = \xi_{ik} \mathrm{Cov}(y_k, y_l) \xi_{li} =  s_y^2 \xi_{ik} \xi_{ki}
\end{gather}

Potom vyraz:
\begin{gather}
    t_i = \frac{\theta_i - \hat{\theta}_i}{s_{\theta i}} =
        \frac{\theta_i - \hat{\theta}_i}{\xi_{ik} \xi_{ki} s_y} =
        \frac{\theta_i - \hat{\theta}_i}{\xi_{ik} \xi_{ki}} \sqrt{\frac{(N - p) \sigma_y^2}{R}}
\end{gather}
je zo Studentovho $t$-rozdelenia a je mozne ho pouzit pre konstrukciu intervalu spolahlivosti.
Konkretne:
\begin{gather}
    \mathrm{Pr}(-\tau < t < \tau) = 1 - \alpha,
\end{gather}
kde $\tau$ je hodnota (kvantil), pre ktoru $T(\tau) = 1 - \frac{\alpha}{2}$, kde $T$ je (kumulativna) Studentova distribucna funkcia.
Vyraz je mozne pretvorit na tvrdenie o $\theta$:
\begin{gather}
    \mathrm{Pr}(\hat{\theta} - s_\theta \tau < \theta < \hat{\theta} + s_\theta \tau) = 1 - \alpha,
\end{gather}
co by malo byt ekvivalentne tvdeniu, ze $\theta$ lezia s pravdepodobnostou $1 - \alpha$ v intervale:
\begin{gather}
    \theta \in [ \hat{\theta} - s_\theta \tau, \hat{\theta} + s_\theta \tau ]
\end{gather}


# Pas spolahlivosti

Pre pas spolahlivosti okolo celej krivky existuje podobny postup ako vyssie (podrobne v [@casella] a [@kutner], alebo [tiez](https://en.wikipedia.org/wiki/Working–Hotelling_procedure)).

\begin{gather}
    \mathrm{Var}(a_{ij} \theta_j) = a_{ij} a_{ik} \mathrm{Cov}(\theta_j, \theta_k)
\end{gather}
alebo v maticovom zapise:
\begin{gather}
    s_m^2 = \mathrm{diag} \left( \mathbf{A} \mathbf{C}(\theta, \theta) \mathbf{A}^\mathrm{T} \right)
\end{gather}

Znova potrebujeme najst rozdelenie vyrazu:

\begin{gather}
    f_i = \frac{A \theta - A \hat{\theta}}{s_m}
\end{gather}

pre vycislenie tvrdenia:
\begin{gather}
    \mathrm{Pr}(\mathbf{A} \hat{\theta} - s_m \phi < \mathbf{A} \theta < \mathbf{A} \hat{\theta} + s_m \phi~~\mathrm{for~all}~x) = 1 - \alpha
\end{gather}

Ukazuje sa, ze $\phi^2$ je z $F$-rozdelenia:
\begin{gather}
    \phi = \sqrt{2 F_{\alpha; p, N-p}}
\end{gather}

# Intervalovy odhad chyby

Ako bolo pouzite vyssie rozptyl data je odhadnuty z:
\begin{gather}
    \sigma_y^2 \approx s_y^2 = \frac{\epsilon_i \epsilon_i}{N - p}
\end{gather}
a vyraz:
\begin{gather}
    c = \frac{\epsilon_i \epsilon_i}{\sigma_y^2} = \frac{s_y^2 (N - p)}{\sigma_y^2}
\end{gather}
je z $\chi^2$-rozdelenia ($N-p$ stupnov volnosti).
To znamena, ze:
\begin{gather}
    \mathrm{Pr} \left( \sigma_l \le \sigma_y \le \sigma_u \right) = 1 - \alpha \\
    \mathrm{Pr} \left( \sigma_l^2 \le \sigma_y^2 \le \sigma_u^2 \right) = 1 - \alpha \\
    \mathrm{Pr} \left( \frac{(N - p) s_y^2}{\sigma_l^2} \ge \frac{(N - p) s_y^2}{\sigma_y^2} \ge \frac{(N - p) s_y^2}{\sigma_u^2} \right) = 1 - \alpha
\end{gather}

Krajne hodnoty teda budu:
\begin{gather}
    \sigma_u = \sqrt{\frac{(N - p) s_y^2}{c_{\alpha/2}}} \\
    \sigma_l = \sqrt{\frac{(N - p) s_y^2}{c_{1 - \alpha/2}}},
\end{gather}
kde $c_\beta$ je definovana ako:
\begin{gather}
    \beta = \int_0^{c_\beta} \chi^2(x) \mathrm{d} x
\end{gather}



