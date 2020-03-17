---
title: Linearna regresia
---

Linearna regresia je odhad vektoru parametrov $\vec{\theta}$:

\begin{gather}
    \vec{\theta} = \left(
                       \mathbf{A}^{\mathrm{T}} \mathbf{W} \mathbf{A} 
                   \right)^{-1} 
                   \mathbf{A}^{\mathrm{T}} \mathbf{W} \vec{y},
\end{gather}
kde $\vec{y}$ su realizacie nahodnej premennej,
$\mathbf{W}$ je vahova matica (v tomto probleme sa neuplatini, bude identita)
 a $\mathbf{A}$ je modelova matica linearneho modelu,
 t.j. $\vec{y} \approx \vec{m} = \mathbf{A} \vec{\theta}$.

Kedze model je linearny v parametroch $a, b$, metoda linearnej regresie bude mat unikatne riesenie, ktore bude najlepsim odhadom parametrov $a, b$.

Chyby a korelacie parametrov su dane kovariancnou maticou:

\begin{gather}
    \mathbf{C}(\vec{\theta},\vec{\theta}) = \frac{1}{k} \left( \mathbf{A}^\mathrm{T} \mathbf{W} \mathbf{A} \right)^{-1}
\end{gather}
Pokial nie su zname presne hodnoty chyb na datach $\vec{y}$ faktor $k$ je mozne odhadnut:
\begin{gather}
    k = \frac{N - p}{\vec{\epsilon}^\mathrm{T} \mathbf{W} \vec{\epsilon}},
\end{gather}
kde $N$ je pocet realizacii nahodnej premennej (pocet bodov), $p$ pocet parametrov a $\vec{\epsilon} = \vec{y} - \vec{m}$.

Modelova matica pre tento problem ma tvar:
\begin{gather}
    \mathbf{A} = 
    \begin{pmatrix}
        1 & x_0^2 \\ 
        \vdots & \vdots \\
        1 & x_N^2
    \end{pmatrix}
\end{gather}
