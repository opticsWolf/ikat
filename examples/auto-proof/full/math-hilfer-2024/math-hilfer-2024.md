# Well-posedness analysis and pseudo-Galerkin approximations using Tau Legendre algorithm for fractional systems of delay differential models regarding Hilfer ($\alpha$,$\beta$)-framework set

## Abstract

Fractional calculus serves as a versatile and potent tool for the modeling and control of intricate systems. This discussion debates the system of DFDEs with two regimes; theoretically and numerically. For theoretical analysis, we have established the EUE by leveraging the definition of Hilfer ($\alpha$,$\beta$)-framework. Our investigation involved the examination of the possessions of the FRD, FCD, and FHD, utilizing their forcefulness and qualifications to convert the concerning delay system into an equivalent one of fractional DVIEs. By employing the CMT, we have successfully demonstrated the prescribed requirements. For numerical analysis, the Galerkin algorithm was implemented by leveraging OSLPs as a base function. This algorithm allows us to estimate the solution to the concerning system by transforming it into a series of algebraic equations. By employing the software MATHEMATICA 11, we have effortlessly demonstrated the requirements estimation of the nodal values. One of the key advantages of the deployed algorithm is its ability to achieve accurate results with fewer iterations compared to alternative methods. To validate the effectiveness and precision of our analysis, we conducted a comprehensive evaluation through various linear and nonlinear numerical applications. The results of these tests, accompanied by figures and tables, further support the superiority of our algorithm. Finally, an analysis of the numerical algorithm employed was provided along with insightful suggestions for potential future research directions.

## 1 Foundations

Fractional calculus is a discipline within modern mathematics and computational physics that focuses on studying derivatives and integrals with fractional orders. While initially introduced by Leibniz in the late 17th century, it did not gain significant attention until the late 20th century. This can be partly attributed to advancements in computing technology during this period, as they facilitated efficient solutions to fractional models [[`ref01`]– [`ref03`]]. The applications of fractional patterns span various domains including engineering, signal processing, statistics, and others. For instance, fractional calculus finds utility in modeling the behavior of viscoelastic materials, controlling robotic systems, predicting financial markets, and modeling biological systems [[`ref04`]– [`ref07`]].

The DFDEs are a class of ordinary calculus that emphasizes fractional derivatives and delays. The scope of DFDEs is more comprehensive than traditional delay models, allowing for a broader range of phenomena to be represented. Applications of DFDEs appear in diverse domains involving signal processing, statistics, engineering, and biology where various simulation techniques are used [[`ref08`]– [`ref19`]]. For instance, in physics, DFDEs have been used to model heat transfer, diffusion, and wave propagation in materials with memory properties [[`ref20`], [`ref21`]]. In engineering, DFDEs have proven useful for modeling and controlling systems such as robots, aircraft, and power networks [[`ref07`], [`ref22`]– [`ref24`]]. Within biology, DFDEs have been employed to formulate agent-based representations of systems like the cardiovascular, nervous, and immune systems [[`ref25`], [`ref26`]].

The DFDE is an equation in which the derivative of a function, $\theta_{\beta}$($\zeta$) with $\beta$ = 1,2, at a certain time, $\zeta$, involves values of the function, $\theta_{\beta}$($\zeta-\sigma_{\beta}$) with $\beta$ = 1,2, at a previous time, $\zeta-\sigma_{\beta}$ with $\beta$ = 1,2. The simplest constant delay equation takes the form $\vartheta_{\beta}^{'}(\zeta)={\mathcal{S}}_{\beta}(\zeta,\wp_{\beta}(\vartheta_{1}(\zeta),\vartheta_{2}(\zeta)),\vartheta_{\beta}(\zeta-\sigma_{\beta}))$ with $\beta$ = 1,2, where $\theta_{\beta}$($\zeta-\sigma_{\beta}$) with $\beta$ = 1,2 representing the value of $\theta_{\beta}$ with $\beta$ = 1,2 at a constant time $\sigma_{\beta}$ with $\beta$ = 1,2 units in the past, making the effect $\theta_{\beta}$ with $\beta$ = 1,2 on the current rate of change of $\theta_{\beta}$ delayed by a time $\sigma_{\beta}$ with $\beta$ = 1,2.

This paper presents the necessary conditions for the EUE together with numerical simulation concerning the resulting set of DFDEs


$$^{H}D_{0}^{\alpha,\beta}\vartheta_{\beta}(\zeta)={\mathcal{S}}_{\beta}(\zeta,\wp_{\beta}(\vartheta_{1}(\zeta),\vartheta_{2}(\zeta)),\vartheta_{\beta}(\zeta-\sigma_{\beta})),\beta=1,2, \tag{1}$$


accordance with the constraints


$$\left\{\begin{matrix} {\vartheta_{\beta}(\zeta)=\varphi}_{\beta}(\zeta),-\sigma_{\beta}<\zeta\leq0, \\ I_{0^{+}}^{1-\gamma}\varphi_{\beta}(0^{+})=\rho_{\beta}. \end{matrix} \right. \tag{2}$$

Herein, $\zeta\in$ *J*: = [0,$\mathcal{F}$] with $\mathcal{F}$>0,$\alpha\in$(0,1) and $\beta\in$[0,1] providing $\alpha$<$\gamma$<1 with $\gamma=(1-\beta)(\alpha-1)+1,\rho_{\beta}\in\mathbb{R},\varphi_{\beta}\in L^{\frac{1}{p}}(J),p\in(0,\frac{1-\gamma+\alpha}{2}),\sigma_{\beta}>0,{\mathcal{S}}_{\beta}\in C(J\times{\mathbb{R}}^{2}\to\mathbb{R}),\wp_{\beta}\in({\mathbb{R}}^{2}\to\mathbb{R})$, and $^{H}D_{0}^{\alpha,\beta}\vartheta_{\beta}$ relate to the FHD [[`ref27`]– [`ref31`]] of $\theta_{\beta}$ of fraction $\alpha$ and type $\beta$.

Our primary concern is to obtain a quantitative estimate of (1) through examination of the elegances of the SGLA, which depends on orthogonal spline basis functions [[`ref32`]– [`ref36`]]. Previous investigations have employed the presented scheme to approximate various differential or integral problems, a few of which will be mentioned. In [[`ref36`]], researchers successfully characterized the telegraph fraction model leveraging the SGLA. Similarly, in [[`ref35`]], the SGLA was utilized to find approximation of Fredholm fraction integrodifferential problems.

In this particular research, we adopted the parameterization of OSLPs to substitute the necessary background functions in (1). Subsequently, the SGLA was employed to convert (1) into sets of algebraic equations. Upon handling the producing set, the desired approximate solution was obtained. One notable numerical optimality of the SGLA employed is its feasibility for any model formalism. Furthermore, it enumerates significantly valid approximations by leveraging a minimal number of OSLP terms.

The organization of the computations and algorithm development are ordered as next. Section 2 presents spectrum attributes, and necessary lemmas based on the definitions of FRD, FCD, and FHD. In Section 3, we transform the system described in (1) and (2) subjected to a congruent set of DVIEs. Additionally, this section outlines the development of proof for the EUE concerning (1) and (2). In Section 4, we introduce the SGLA as a solution technique for tackling (1) and (2) and proceed to characterize the theorems on convergence. Section 5 provides numerical applications and results. Finally, a summary of key findings is mentioned in Section 6.

## 2 Background and overview results

This section presents indispensable background and properties associated with FRD, FCD, and FHD approaches, simultaneously, with significant outcomes that will be exploited in the next portion. Anyhow, consider the following requirements:

The Banach topological manifold *L*$^{P}$(*J*,$\mathbb{R}$) is axiomatized as the collection of each Lebesgue measurable $\mathcal{S}:J\to\mathbb{R}$ equipped with ${\Vert\mathcal{S}\Vert}_{L^{p}}=(\int_{J}{/\mathcal{S}(\zeta))}^{p}d\zeta)^{\frac{1}{p}}<\infty$.

The topological manifold $AC(J,\mathbb{R})=\left\{\mathcal{S}:J\to\mathbb{R};\mathcal{S}\;\mathrm{a}\mathrm{b}\mathrm{s}\mathrm{o}\mathrm{l}\mathrm{u}\mathrm{t}\mathrm{e}\mathrm{l}\mathrm{y}\;\mathrm{c}\mathrm{o}\mathrm{n}\mathrm{t}\mathrm{i}\mathrm{n}\mathrm{u}\mathrm{o}\mathrm{u}\mathrm{s}\;\mathrm{o}\mathrm{n}\;J\right\}$. Indeed, the topological manifold ${AC}^{\hbar}(J,\mathbb{R})=\left\{\mathcal{S}:\mathcal{S}\in C^{\hbar-1}(J,\mathbb{R})\;\mathrm{a}\mathrm{n}\mathrm{d}\;{\mathcal{S}}^{\hbar-1}\in AC(J,\mathbb{R})\right\}$.

The weighted topological manifold $C_{\theta}(J)=\left\{\mathcal{S}:J\to\mathbb{R};\zeta^{\theta}\mathcal{S}(\zeta)\in C(J,\mathbb{R})\right\}$ with ${\Vert\mathcal{S}\Vert}_{C_{\theta}}=\max_{\zeta\in J}\zeta^{\theta}/\mathcal{S}(\zeta)/$.

**Definition 1.** [[`ref37`]] The left-side integral of fraction $\theta$>0 for $\mathcal{S}\in L^{1}(J)$ is


$$(I_{0^{+}}^{\theta}\mathcal{S})(\zeta)=\frac{1}{\Gamma(\theta)}\int_{0}^{\zeta}(\zeta-s)^{\theta-1}\mathcal{S}(s)ds. \tag{3}$$

**Definition 2.** [[`ref37`]] If $\mathcal{S}(\zeta)\in{AC}^{\hbar}(J,\mathbb{R})$, then the left-side FRD of fraction $\theta\in$($\hbar-$1,$\hbar$) with $\hbar\in\mathbb{N}$ exists a.e. on *J* with


$$(^{L}D_{0^{+}}^{\theta}\mathcal{S})(\zeta)=\frac{1}{\Gamma(\hbar-\theta)}\frac{d^{\hbar}}{d\zeta^{\hbar}}\int_{0}^{\zeta}(\zeta-s)^{\hbar-\theta-1}\mathcal{S}(s)ds. \tag{4}$$

**Remark 1.** If $\theta$>0 and $\hat{\theta}>0$, then


$$[I_{0^{+}}^{\theta}s^{\hat{\theta}-1}](\zeta)=\frac{\Gamma(\hat{\theta})}{\Gamma(\hat{\theta}+\theta)}\zeta^{\hat{\theta}+\theta-1}.$$




$$[^{L}D_{0^{+}}^{\theta}s^{\hat{\theta}-1}](\zeta)=\frac{\Gamma(\hat{\theta})}{\Gamma(\hat{\theta}-\theta)}\zeta^{\hat{\theta}-\theta-1}.$$




$$[^{L}D_{0^{+}}^{\theta}s^{\theta-\beta}](\zeta)=0,\beta=1,2,\cdots,[\theta]+1.$$

**Definition 3.** [[`ref38`]] The FCD of fraction $\theta\in$($\hbar-$1,$\hbar$) with $\hbar\in\mathbb{N}$ is


$$^{C}D_{0}^{\theta}\mathcal{S}(\zeta)=\frac{1}{\Gamma(\hbar-\theta)}\int_{0}^{\zeta}\frac{{\mathcal{S}}^{(\hbar)}(s)}{(\zeta-s)^{\theta+1-\hbar}}ds. \tag{5}$$

**Definition 4.** [[`ref27`]] Let $D=\frac{d}{d\zeta}$ and $\gamma$ = $\alpha$+$\hbar\beta-$*$\alpha\beta$*. The left-side FHD of fraction $\alpha\in$($\hbar-$1,$\hbar$) with $\hbar\in\mathbb{N}$ and $\beta\in$[0,1] of $\mathcal{S}$ is


$$(^{H}D_{0^{+}}^{\alpha,\beta}\mathcal{S})(\zeta)=(I_{0^{+}}^{\beta(\hbar-\alpha)}D^{\hbar}I_{0^{+}}^{(1-\beta)(\hbar-\alpha)}\mathcal{S})(\zeta)=(I_{0^{+}}^{\gamma-\alpha}D^{\hbar}I_{0^{+}}^{\hbar-\gamma}\mathcal{S})(\zeta),\zeta\in J. \tag{6}$$

The term $^{H}D_{0^{+}}^{\alpha,\beta}$ can be formatted as $^{H}D_{0^{+}}^{\alpha,\beta}=I_{0^{+}}^{\beta(\hbar-\alpha)}D^{\hbar}I_{0^{+}}^{(1-\beta)(\hbar-\alpha)}=I_{0^{+}}^{\beta(\hbar-\alpha)}{^{L}D}_{0^{+}}^{\gamma}=I_{0^{+}}^{(\gamma-\alpha)}{^{L}D}_{0^{+}}^{\gamma}$. Indeed, if $\beta$ = 0, then the left-side FRD is ${^{L}D}_{0^{+}}^{\alpha}={^{H}D}_{0^{+}}^{\alpha,0}$ and if $\beta$ = 1, then the left-side FCD is ${^{C}D}_{0^{+}}^{\alpha}={^{H}D}_{0^{+}}^{\alpha,1}.$

**Lemma 1.** [[`ref28`]] The fundamental solution of $^{H}D_{0^{+}}^{\alpha,\beta}\mathcal{S}(\zeta)=0$ is $\mathcal{S}(\zeta)=\sum_{\beta=1}^{\hbar}c_{\beta}(\zeta-0)^{\alpha+\hbar\beta-\alpha\beta-\beta}$ with $\zeta>0,\alpha+\hbar\beta-\alpha\beta\in(\hbar-1,\hbar)$, and $c_{\beta}\in\mathbb{R}$.

**Lemma 2.** [[`ref28`]] Let $\rho\in$(0,1), $\upsilon\in(-1,0]$, and $\rho$+$\upsilon$>0, then at $0<p<\frac{\rho+\upsilon}{2}$ and $\vartheta\in L^{\frac{1}{p}}$, one has $(I_{0^{+}}^{\rho}s^{\upsilon}\vartheta(s))(\zeta)\in AC(J,\mathbb{R})$.

**Lemma 3.** [[`ref28`]] Let $\theta\in$($\hbar-$1,$\hbar$) with $\hbar\in\mathbb{N}$. If $\theta\in$*L*$^{1}$(*J*) and $I_{0^{+}}^{\hbar-\theta}\vartheta\in AC^{\hbar}(J)$, then


$$(I_{0^{+}}^{\theta}{}^{L}D_{0^{+}}^{\theta}\vartheta)(\zeta)=\vartheta(\zeta)-\sum_{\beta=1}^{\hbar}\frac{(I_{0^{+}}^{\hbar-\theta}\vartheta)^{(\hbar-\beta)}(0^{+})}{\Gamma(\theta-\beta+1)}\zeta^{\theta-\beta}. \tag{7}$$

**Lemma 4.** [[`ref31`]] Let $\theta\in$*L*$^{1}$(*J*) and $^{L}D_{0^{+}}^{\gamma-\alpha}\vartheta\in L^{1}(J)$ exists. Then $(^{H}D_{0^{+}}^{\alpha,\beta}I_{0^{+}}^{\alpha}\vartheta)(\zeta)=I_{0^{+}}^{\gamma-\alpha}{}^{L}D_{0^{+}}^{\gamma-\alpha}\vartheta(\zeta)$.

**Lemma 5.** [[`ref31`]] Let $\alpha$>0 and 0≤$\gamma$<1. Then $I_{0^{+}}^{\alpha}$ is bounded from *C*$_{\gamma}$(*J*) into *C*$_{\gamma}$(*J*).

Here, we define the following topological manifold: $C_{1-\gamma}^{\alpha,\beta}(J)=\left\{\mathcal{S}\in C_{1-\gamma}(J);({^{H}D}_{0^{+}}^{\alpha,\beta}\mathcal{S})(\zeta)\in C_{1-\gamma}(J)\right\}$ and $C_{1-\gamma}^{\gamma}(J)=\left\{\mathcal{S}\in C_{1-\gamma}(J);({^{L}D}_{0^{+}}^{\gamma}\mathcal{S})(\zeta)\in C_{1-\gamma}(J)\right\}.$ Note that ${^{H}D}_{0^{+}}^{\alpha,\beta}\wp=I_{0^{+}}^{\beta(1-\alpha)}{}^{L}D_{0^{+}}^{\gamma}\wp$, so, by Lemma 6 $C_{1-\gamma}^{\gamma}(J)\subset C_{1-\gamma}^{\alpha,\beta}(J)$.

**Lemma 6.** [[`ref31`]] Let $\wp\in L^{1}(0,c)$ and $\theta$>0. Then


$$\lim_{\zeta\to c^{+}}\int_{0}^{c}(\zeta-s)^{\theta-1}\wp(s)ds=\int_{0}^{c}(c-s)^{\theta-1}\wp(s)ds=\Gamma(\theta)I_{0^{+}}^{\theta}\wp(c). \tag{8}$$

**Lemma 7.** [[`ref31`]] Let $\theta\in(0,1),0<c<\mathcal{F},\wp\in C_{\theta}[0,c]$, $\wp\in C[c,\mathcal{F}]$, and $\wp$ is continuous at $c$. Then $\wp\in$*C*$_{\theta}$(*J*).

## 3 Congruence fractional DVIE and the results concerning EUE

In this section, we investigate the examination of the possessions of the FRD, FCD, and FHD utilizing their forcefulness and qualifications to convert the concerning delay system into an equivalent one of fractional DVIEs. After that, we visualize and clarify the EUE concerning (1) and (2) leveraging its congruence system of fractional DVIEs (9) within $C_{1-\gamma}^{\gamma}(J)$.

**Theorem 1.** Let $0<p<\frac{1-\gamma+\alpha}{2},\wp\in C_{1-\gamma}(\mathbb{R},\mathbb{R}),\vartheta_{1},\vartheta_{2}\in C_{1-\gamma}(J,\mathbb{R})$, and ${\mathcal{S}}_{1},{\mathcal{S}}_{2}:J\times{\mathbb{R}}^{2}\times\to\mathbb{R}$ be as ${\mathcal{S}}_{1}(\cdot,\wp(\vartheta(\cdot)),\vartheta_{1}(\cdot-\sigma_{1}))\in L^{\frac{1}{p}}(J,\mathbb{R},\mathbb{R})$ and ${\mathcal{S}}_{2}(\cdot,\wp(\vartheta(\cdot)),\vartheta_{2}(\cdot-\sigma_{2}))\in L^{\frac{1}{p}}(J,\mathbb{R},\mathbb{R})$. Then $\theta$($\zeta$) fulfills Problem (1) and (2) iff $\theta$($\zeta$) fulfills


$$\vartheta_{\beta}(\zeta)=\frac{\rho_{\beta}\zeta^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta}))}{\Gamma(\alpha)}ds,\beta=1,2. \tag{9}$$

**Proof.** For $\zeta\in$*J* and $\gamma-\alpha\in(0,1-\alpha)$ it suffices from Lemma 2 at $\rho=1-(\gamma-\alpha)$ and $\upsilon=0$ that $1-(\gamma-\alpha)>0,0<p<\frac{1-(\gamma-\alpha)}{2}$, and ${\mathcal{S}}_{1},{\mathcal{S}}_{2}\in L^{\frac{1}{p}}$. So $I_{0^{+}}^{1-(\gamma-\alpha)}{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta}))(\zeta)\in AC(J\times{\mathbb{R}}^{2},\mathbb{R})$ and $^{L}D_{0^{+}}^{\alpha}{\mathcal{S}}_{\beta}\in L^{1}(J\times{\mathbb{R}}^{3})$.

From Lemma 3, take $\theta$ = $\gamma-\alpha$, one gained


$$(I_{0^{+}}^{\gamma-\alpha}{}^{L}D_{0^{+}}^{\gamma-\alpha}{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta})))(\zeta)={\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta}))-\frac{(I_{0^{+}}^{1-(\gamma-\alpha)}{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta}))(0^{+})}{\Gamma(\gamma-\alpha)}\zeta^{\gamma-\alpha-1}. \tag{10}$$

From Lemma 4, one concludes


$$I_{0^{+}}^{\gamma-\alpha}{}^{L}D_{0^{+}}^{\gamma-\alpha}{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta}))(\zeta)=^{H}D_{0^{+}}^{\alpha,\beta}I_{0^{+}}^{\alpha}({\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta})))(\zeta). \tag{11}$$

Then


$$^{H}D_{0^{+}}^{\alpha,\beta}({\vartheta_{\beta}(\zeta)-I}_{0^{+}}^{\alpha}{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta}))(\zeta))={\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta}))-{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta}))+\frac{(I_{0^{+}}^{1-(\gamma-\alpha)}{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta})))(0^{+})}{\Gamma(\gamma-\alpha)}\zeta^{\gamma-\alpha-1}. \tag{12}$$

Since $(I_{0^{+}}^{1-(\gamma-\alpha)}{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta})))(0^{+})=0.$ So, $^{H}D_{0^{+}}^{\alpha,\beta}(\vartheta_{\beta}(\zeta)-(I_{0^{+}}^{\alpha}{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta})))(\zeta))=0$. Using Lemma 1, one concludes whenever $c_{\beta}\in\mathbb{R}$ that


$$\vartheta_{\beta}(\zeta)=c_{\beta}\zeta^{\gamma-1}+(I_{0^{+}}^{\alpha}{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta})))(\zeta),\zeta>0. \tag{13}$$

Using ${\vartheta_{\beta}(\zeta)=\varphi}_{\beta}(\zeta)$ with $-\sigma_{\beta}<\zeta\leq0$ such that $I_{0^{+}}^{1-\gamma}\varphi_{\beta}(0^{+})=\rho_{\beta}$ and $I_{0^{+}}^{1-\gamma}\vartheta_{\beta}(0^{+})=c_{\beta}\Gamma(\gamma)+0,-\sigma_{\beta}\leq\zeta\leq0$, we have $c_{\beta}\Gamma(\gamma)=\rho_{\beta}$. So the fractional DVIEs are


$$\vartheta_{\beta}(\zeta)=\frac{\rho_{\beta}\zeta^{\gamma-1}}{\Gamma(\gamma)}+(I_{0^{+}}^{\alpha}{\mathcal{S}}_{\beta}(s,\wp_{\beta}(\vartheta(s)),\vartheta_{\beta}(s-\sigma_{\beta})))(\zeta),\beta=1,2. \tag{14}$$

In the consecutive results, $\vartheta(\zeta)=(\vartheta_{1}(\zeta),\vartheta_{2}(\zeta)),\overset{\sim}{\vartheta}(\zeta)=({\overset{\sim}{\vartheta}}_{1}(\zeta),{\overset{\sim}{\vartheta}}_{2}(\zeta)),^{H}D_{0}^{\alpha,\beta}\vartheta(\zeta)=(^{H}D_{0}^{\alpha,\beta}\vartheta_{1}(\zeta),^{H}D_{0}^{\alpha,\beta}\vartheta_{2}(\zeta))$, and $\vartheta(\zeta-\theta)=(\vartheta_{1}(\zeta-\sigma_{1}),\vartheta_{2}(\zeta-\sigma_{2}))$with $\theta$ = ($\sigma_{1}$, $\sigma_{2}$). Indeed, $\wp(\vartheta(\zeta))=(\wp_{1}(\vartheta(\zeta)),\wp_{2}(\vartheta(\zeta)))$ and $\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))=({\mathcal{S}}_{1}(\zeta,\wp_{1}(\vartheta(\zeta)),\vartheta_{1}(\zeta-\sigma_{1})),{\mathcal{S}}_{2}(\zeta,\wp_{2}(\vartheta(\zeta)),\vartheta_{2}(\zeta-\sigma_{2})))$.

**Theorem 2.** For any $\vartheta:J\to{\mathbb{R}}^{2},\wp\in C_{1-\gamma}({\mathbb{R}}^{2},\mathbb{R}),$ and $\vartheta(\zeta-\theta)\in C_{1-\gamma}(J)$, if $\mathcal{S}$ fulfills the Lipschitzian


$${\Vert\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))\Vert}_{C_{1-\gamma}(J)}\leq L({\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}(J)}+{\Vert\vartheta(\zeta-\theta)-\overset{\sim}{\vartheta}(\zeta-\theta)\Vert}_{C_{1-\gamma}(J)}), \tag{15}$$


then $\exists$!-Sol for (1) and (2) in $C_{1-\gamma}^{\gamma}(J)$.

**Proof.** Initially, we prove the EUE within *C*$_{1-\gamma}$(*J*). Our demonstration relies on the examination of three scenarios: the first encompasses $\zeta\in$(0,$\sigma_{1}$], the second encompasses $\zeta\in$($\sigma_{1}$, $\sigma_{2}$], and the third one encompasses $\zeta\in$($\sigma_{2}$,$\mathcal{F}$]. In each scenario, we divide the *J* into subintervals and validate *G*: *S*→*S* defined as


$$G\vartheta(\zeta)=(G\vartheta_{1}(\zeta),G\vartheta_{2}(\zeta)) \tag{16}$$




$$=(\frac{\rho_{1}{\boldsymbol{\zeta}}^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\boldsymbol{\zeta}}\frac{(\boldsymbol{\zeta}-s)^{\alpha-1}S_{1}(s,{\boldsymbol{\wp}}_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1}))}{\Gamma(\alpha)}ds,\frac{\rho_{2}{\boldsymbol{\zeta}}^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\boldsymbol{\zeta}}\frac{(\boldsymbol{\zeta}-s)^{\alpha-1}S_{2}(s,{\boldsymbol{\wp}}_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2}))}{\Gamma(\alpha)}ds),$$


where $S=\left\{\vartheta\in C_{1-\gamma}(J)\right\}$ is a contraction mapping.

Assuming that $\sigma_{1}\leq\sigma_{2}$. Then we consider three cases as follows:

**Case 1.** Let $\zeta\in$(0,$\sigma_{1}$], so $\vartheta(\zeta-\theta)=\overset{\sim}{\vartheta}(\zeta-\theta)=\varphi(\zeta-\theta)=(\varphi_{1}(\zeta-\sigma_{1}),\varphi_{2}(\zeta-\sigma_{2}))$ and choose $c_{1},c_{2}$ such that $0<c_{1}\leq c_{2}\leq\sigma_{1}.\;\mathrm{T}\mathrm{h}\mathrm{e}\;\mathrm{m}\mathrm{e}\mathrm{t}\mathrm{r}\mathrm{i}\mathrm{c}\;\mathrm{s}\mathrm{p}\mathrm{a}\mathrm{c}\mathrm{e}\;C_{1-\gamma}[c_{1},c_{2}]$ is considered complete with


$$d(\vartheta(\zeta),\overset{\sim}{\vartheta}(\zeta))={\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[c_{1},c_{2}]}=\max_{\zeta\in[c_{1},c_{2}]}\Vert\zeta^{1-\gamma}(\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta))\Vert. \tag{17}$$

Select $\zeta_{1}\in(0,\sigma_{1}]$ and note that $\frac{\rho_{1}}{\Gamma(\gamma)}\zeta^{\gamma-1},\frac{\rho_{2}}{\Gamma(\gamma)}\zeta^{\gamma-1}\in C_{1-\gamma}[0,\zeta_{1}]$, so by Lemma 5 due to $G\vartheta\in C_{1-\gamma}[0,\zeta_{1}]$, thus $G$ maps $C_{1-\gamma}[0,\zeta_{1}]$ into itself. Now, $\forall\vartheta(\zeta),\overset{\sim}{\vartheta}(\zeta)\in C_{1-\gamma}[0,\zeta_{1}]$, we have


$${\Vert G\vartheta(\zeta)-G\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[0,\zeta_{1}]}\leq|\frac{(\rho_{1}-\rho_{2})}{\Gamma(\gamma)}|{\Vert I_{0^{+}}^{\alpha}|\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))|\Vert}_{C_{1-\gamma}[0,\zeta_{1}]}. \tag{18}$$

Note that


$${\Vert\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))\Vert}_{C_{1-\gamma}[0,\zeta_{1}]}=\max_{\zeta\in[0,\zeta_{1}]}\Vert\zeta^{1-\gamma}(\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta)))\Vert. \tag{19}$$

So,


$$|\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta)-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))|\leq\zeta^{\gamma-1}{\Vert\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))\Vert}_{C_{1-\gamma}[0,\zeta_{1}]}. \tag{20}$$




$${\Vert G\vartheta(\zeta)-G\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[0,\zeta_{1}]}$$




$$\leq\frac{|\rho_{1}-\rho_{2}|}{\Gamma(\gamma)}\frac{\Gamma(\gamma)}{\Gamma(\gamma+\alpha)}{\zeta_{1}}^{\gamma+\alpha-1}L({\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[0,\zeta_{1}]}+{\Vert\vartheta(\zeta-\theta)-\overset{\sim}{\vartheta}(\zeta-\theta)\Vert}_{C_{1-\gamma}[0,\zeta_{1}]})$$




$$=\frac{|\rho_{1}-\rho_{2}|}{\Gamma(\gamma+\alpha)}{\zeta_{1}}^{\gamma+\alpha-1}L({\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[0,\zeta_{1}]}+{\Vert\varphi(\zeta-\theta)-\varphi(\zeta-\theta)\Vert}_{C_{1-\gamma}[0,\zeta_{1}]})$$




$$=\mu_{1}{\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[0,\zeta_{1}]}. \tag{21}$$


where $\mu_{1}=\frac{L/\rho_{1}-\rho_{2}/}{\Gamma(\gamma+\alpha)}{\zeta_{1}}^{\gamma+\alpha-1}<1$.

Given that $\mu_{1}$<1, the CMT guarantees the existence of a single fixed point, denoted as $\vartheta_{0}(\zeta)\in C_{1-\gamma}[0,\zeta_{1}]$ within (0,$\zeta_{1}$]. In the case where $\zeta_{1}\neq\sigma_{1}$, we examine [$\zeta_{1}$, $\sigma_{1}$]. It is important to note that


$$G\vartheta(\zeta)=(\frac{\rho_{1}\zeta^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1}))}{\Gamma(\alpha)}ds,\frac{\rho_{2}\zeta^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2}))}{\Gamma(\alpha)}ds) \tag{22}$$




$$=(\frac{{\rho_{1}\zeta}^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta_{1}}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1}))}{\Gamma(\alpha)}ds+(I_{\zeta_{1}}^{\alpha}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1})))(\zeta),\frac{{\rho_{2}\zeta}^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta_{1}}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2}))}{\Gamma(\alpha)}ds+(I_{\zeta_{1}}^{\alpha}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2})))(\zeta)).$$

Let $\vartheta(\zeta),\overset{\sim}{\vartheta}(\zeta)\in C[\zeta_{1},\zeta_{2}]$ for some $\zeta_{1}<\zeta_{2}\leq\sigma_{1}$, we see that


$${\Vert G\vartheta(\zeta)-G\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}$$




$$={\Vert\frac{\zeta^{\gamma-1}(\rho_{1}-\rho_{2})}{\Gamma(\gamma)}(I_{\zeta_{1}}^{\alpha}\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-I_{\zeta_{1}}^{\alpha}\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta)))\Vert}_{C[\zeta_{1},\zeta_{2}]}$$




$$\leq\frac{|(\rho_{1}-\rho_{2})|}{\Gamma(\gamma)}\frac{{\Gamma(\gamma)(\zeta_{2}-\zeta_{1})}^{\gamma+\alpha-1}}{\Gamma(\gamma+\alpha)}{\Vert\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))\Vert}_{C[\zeta_{1},\zeta_{2}]}$$




$$\leq\frac{L{|\rho_{1}-\rho_{2}|(\zeta_{2}-\zeta_{1})}^{\gamma+\alpha-1}}{\Gamma(\gamma+\alpha)}({\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}+{\Vert\vartheta(\zeta-\theta)-\overset{\sim}{\vartheta}(\zeta-\theta)\Vert}_{C[\zeta_{1},\zeta_{2}]})$$




$$=\frac{L{|\rho_{1}-\rho_{2}|(\zeta_{2}-\zeta_{1})}^{\gamma+\alpha-1}}{\Gamma(\gamma+\alpha)}({\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}+{\Vert\varphi(\zeta-\theta)-\varphi(\zeta-\theta)\Vert}_{C[\zeta_{1},\zeta_{2}]})$$




$$=\frac{L{|\rho_{1}-\rho_{2}|(\zeta_{2}-\zeta_{1})}^{\gamma+\alpha-1}}{\Gamma(\gamma+\alpha)}{\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}$$




$$=\mu_{2}{\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}, \tag{23}$$


where $\mu_{2}=\frac{L{/\rho_{1}-\rho_{2}/(\zeta_{2}-\zeta_{1})}^{\gamma+\alpha-1}}{\Gamma(\gamma+\alpha)}<1$. It results that *G* is a contraction on [$\zeta_{1}$, $\zeta_{2}$]. Consequently, $\exists$!-Sol $\theta_{1}$($\zeta$) for $\zeta\in$[$\zeta_{1}$, $\zeta_{2}$]. According to Lemma 6, it is evident that $\vartheta_{0}(\zeta_{1})=\vartheta_{1}(\zeta_{1})$. Thus,


$$\vartheta(\zeta)=\left\{\begin{matrix} \vartheta_{0}(\zeta),0<\zeta\leq\zeta_{1}, \\ \vartheta_{1}(\zeta),\zeta_{1}<\zeta\leq\zeta_{2}. \end{matrix} \right. \tag{24}$$

According to Lemma 7; $\vartheta(\zeta)\in C_{1-\gamma}[0,\zeta_{2}]$. Additionally, $\theta$($\zeta$) represents the only solution to (1) and (2) within [0,$\zeta_{2}$]. In the case where $\zeta_{2}\neq\sigma_{1}$, we repeat the necessary steps. Let's assume that *N*$-$2 additional steps are required. Following that, we determine a! -Sol $\theta_{k}$($\zeta$) for $\zeta$ within $[\zeta_{k},\zeta_{k+1}]$, where *k* takes on values *k* = 2,3,$\cdots$,*N*. Here, we have $0=\zeta_{0}<\zeta_{1}<\cdots<\zeta_{N}=\sigma_{1}$ satisfying the condition that


$$\mu_{k+1}=\frac{L|\rho_{1}-\rho_{2}|}{\Gamma(\gamma+\alpha)}(\zeta_{k+1}-\zeta_{k})^{\gamma+\alpha-1}<1. \tag{25}$$

We possess a distinctive solution $\vartheta(\zeta)\in C_{1-\gamma}[0,\sigma_{1}]$ ensuring its uniqueness as


$$\vartheta(\zeta)=\left\{\begin{matrix} \vartheta_{0}(\zeta),0<\zeta<\zeta_{1}, \\ \vartheta_{1}(\zeta),\zeta_{1}<\zeta\leq\zeta_{2}, \\ \vdots \\ \vartheta_{N}(\zeta),\zeta_{N-1}<\zeta<\sigma_{1}. \end{matrix} \right. \tag{26}$$

Since $\vartheta(\zeta)\in C_{1-\gamma}[0,\tau]$ is the! -Sol, so it fulfills (9) and


$$\vartheta(\zeta)=(\frac{\rho_{1}\zeta^{\gamma-1}}{\Gamma(\gamma)}+(I_{0^{+}}^{\alpha}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1})))(\zeta),\frac{\rho_{2}\zeta^{\gamma-1}}{\Gamma(\gamma)}+(I_{0^{+}}^{\alpha}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2})))(\zeta).) \tag{27}$$

Applying $^{L}D_{0^{+}}^{\gamma}$ to both sides results in the following:


$$^{L}D_{0^{+}}^{\gamma}\vartheta(\zeta)=(^{L}D_{0^{+}}^{\gamma}\frac{\rho_{1}\zeta^{\gamma-1}}{\Gamma(\gamma)}+^{L}D_{0^{+}}^{\gamma}(I_{0^{+}}^{\alpha}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1})))(\zeta),^{L}D_{0^{+}}^{\gamma}\frac{\rho_{2}\zeta^{\gamma-1}}{\Gamma(\gamma)}+^{L}D_{0^{+}}^{\gamma}(I_{0^{+}}^{\alpha}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2})))(\zeta))$$




$$=((^{L}D_{0^{+}}^{\gamma-\alpha}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1})))(\zeta),(^{L}D_{0^{+}}^{\gamma-\alpha}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2})))(\zeta))$$




$$=((^{L}D_{0^{+}}^{\beta(1-\alpha)}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1})))(\zeta),(^{L}D_{0^{+}}^{\beta(1-\alpha)}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2})))(\zeta)). \tag{28}$$

It is evident that the RHS belongs to *C*$_{1-\gamma}$[0, $\sigma_{1}$]. Therefore, $^{L}D_{0^{+}}^{\gamma}\vartheta(\zeta)\in C_{1-\gamma}[0,\sigma_{1}]$. Consequently, according to (9); $\vartheta(\zeta)\in C_{1-\gamma}^{\gamma}[0,\sigma_{1}]$ and this is the only solution in $[0,\sigma_{1}]$.

**Case 2.** Assuming that $\zeta\in$ ($\sigma_{1}$,$\sigma_{2}$] and $\vartheta(\zeta),\overset{\sim}{\vartheta}(\zeta)\in C_{1-\gamma}[\sigma_{1},\sigma_{2}]$, we can divide [0, $\sigma_{1}$] into $[0,\sigma_{1}]\cup\cdots\cup[(k_{0}-1)\sigma_{1},k_{0}\sigma_{1}]\cup[k_{0}\sigma_{1},\sigma_{2}]$. Here, *k*$_{0}$ is a natural number and fulfills $0\leq\sigma_{2}-k_{0}\sigma_{1}\leq\sigma_{2}$. In a previous case (Case 1), we have already proven (1) and (2) have a! -Sol on [0,$\sigma_{1}$], denoted as $\vartheta_{0}^{\ast}(\zeta)$. Now, let's assume that (1) and (2) also has a! -Sol on [$\sigma_{1}$, *k$\sigma$*$_{1}$], where 1≤*k*≤*k*$_{0}$, denoted as $\vartheta_{k}^{\ast}(\zeta)$. Our goal is to axiomatize the existence of a! -Sol $\vartheta_{k+1}^{\ast}(\zeta)$ on $[k\sigma_{1},(k+1)\sigma_{1}].$ Anyhow, $\forall\zeta\in[k\sigma_{1},(k+1)\sigma_{1}]$ define


$$G\vartheta(\zeta)=(\frac{\rho_{1}\zeta^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1}))}{\Gamma(\alpha)}ds,\frac{\rho_{2}\zeta^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2}))}{\Gamma(\alpha)}ds). \tag{29}$$

Assume that $\mathrm{G}$ is a contraction for $\zeta\in$[$\sigma_{1}$, *k$\sigma$*$_{1}$] such that ${\Vert G\vartheta(\zeta)-G\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\sigma_{1},k\sigma_{1}]}\leq\mu_{k}{\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\sigma_{1},k\sigma_{1}]}$. Therefore, $\exists$!-Sol $\varphi_{k}$($\zeta$) for all $\zeta\in$[$\sigma_{1}$, *k$\sigma$*$_{1}$]. Now, consider $\zeta\in[k\sigma_{1},(k+1)\sigma_{1}]$. In this case, $\vartheta(\zeta-\sigma_{1})=\overset{\sim}{\vartheta}(\zeta-\sigma_{1})=\varphi_{k}(\zeta-\sigma_{1})$. Choose two values $c_{1},c_{2}$ such that $\sigma_{1}<c_{1}\leq c_{2}\leq k\sigma_{1}$. This entails that the metric space $C_{1-\gamma}[c_{1},c_{2}]$ is complete with


$$d(\vartheta(\zeta),\overset{\sim}{\vartheta}(\zeta))={\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[c_{1},c_{2}]}=\max_{\zeta\in[c_{1},c_{2}]}\Vert\zeta^{1-\gamma}(\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta))\Vert. \tag{30}$$

Select $\zeta_{1}\in(k\sigma_{1},(k+1)\sigma_{1}]$ such that


$$\mu_{1k}=\frac{|\rho_{1}-\rho_{2}|L}{\Gamma(\gamma+\alpha)}{\zeta_{1}}^{\gamma+\alpha-1}<1. \tag{31}$$

Note that $\frac{\rho_{1}\zeta^{\gamma-1}}{\Gamma(\gamma)},\frac{\rho_{2}\zeta^{\gamma-1}}{\Gamma(\gamma)}\in C_{1-\gamma}[k\sigma_{1},\zeta_{1}]$, so by Lemma 5 due to $G\vartheta\in C_{1-\gamma}[k\sigma_{1},\zeta_{1}]$; *G* maps $C_{1-\gamma}[k\sigma_{1},\zeta_{1}]$ into itself. Now, $\forall\vartheta(\zeta),\overset{\sim}{\vartheta}(\zeta)\in C_{1-\gamma}[k\sigma_{1},\zeta_{1}]$, we have


$${\Vert G\vartheta(\zeta)-G\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[k\sigma_{1},\zeta_{1}]}\leq\frac{|\rho_{1}-\rho_{2}|}{\Gamma(\gamma)}{\Vert I_{0^{+}}^{\alpha}|\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\sigma_{1}))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\sigma_{1}))|\Vert}_{C_{1-\gamma}[k\sigma_{1},\zeta_{1}]}. \tag{32}$$

Note that


$${\Vert\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\sigma_{1}))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\sigma_{1}))\Vert}_{C_{1-\gamma}[k\sigma_{1},\zeta_{1}]}=\max_{\zeta\in[k\sigma_{1},\zeta_{1}]}\Vert\zeta^{1-\gamma}\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\sigma_{1}))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\sigma_{1}))\Vert. \tag{33}$$

Similarly, by doing the same procedure as in Case 1, we can find that


$${\Vert G\vartheta(\zeta)-G\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[k\sigma_{1},\zeta_{1}]}\leq\frac{|\rho_{1}-\rho_{2}|}{\Gamma(\gamma+\alpha)}{\zeta_{1}}^{\gamma+\alpha-1}L{\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[k\sigma_{1},\zeta_{1}]}$$




$$=\mu_{1k}{\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[k\sigma_{1},\zeta_{1}]}, \tag{34}$$


where $\mu_{1k}=\frac{/\rho_{1}-\rho_{2}/L}{\Gamma(\gamma+\alpha)}{\zeta_{1}}^{\gamma+\alpha-1}<1$.

According to the CMT, there exists a single fixed point that serves as the solution $\vartheta_{0}(\zeta)\in C_{1-\gamma}[k\sigma_{1},\zeta_{1}]$ within $(k\sigma_{1},\zeta_{1}]$. In the case where $\zeta_{1}\neq(k+1)\sigma_{1}$, we will focus on $[\zeta_{1},(k+1)\sigma_{1}]$. It is important to note that


$$G\vartheta(\zeta)=(\frac{{\rho_{1}\zeta}^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta_{1}}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1}))}{\Gamma(\alpha)}ds+(I_{\zeta_{1}}^{\alpha}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1})))(\zeta),\frac{{\rho_{2}\zeta}^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta_{1}}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2}))}{\Gamma(\alpha)}ds+(I_{\zeta_{1}}^{\alpha}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2})))(\zeta)). \tag{35}$$

Let $\vartheta(\zeta),\overset{\sim}{\vartheta}(\zeta)\in C[\zeta_{1},\zeta_{2}]$ for some $\zeta_{1}<\zeta_{2}\leq(k+1)\sigma_{1}$, we can see that


$${\Vert G\vartheta(\zeta)-G\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}$$




$$\leq|\frac{(\rho_{1}-\rho_{2})}{\Gamma(\gamma)}|{\Vert I_{{\zeta_{1}}^{+}}^{\alpha}|\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))|\Vert}_{C[\zeta_{1},\zeta_{2}]}$$




$$\leq\frac{L{|\rho_{1}-\rho_{2}|(\zeta_{2}-\zeta_{1})}^{\gamma+\alpha-1}}{\Gamma(\gamma+\alpha)}({\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}+{\Vert\vartheta(\zeta-\theta)-\overset{\sim}{\vartheta}(\zeta-\theta)\Vert}_{C[\zeta_{1},\zeta_{2}]})$$




$$=\frac{L{|\rho_{1}-\rho_{2}|(\zeta_{2}-\zeta_{1})}^{\gamma+\alpha-1}}{\Gamma(\gamma+\alpha)}({\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}+{\Vert\varphi_{k}(\zeta-\theta)-\varphi_{k}(\zeta-\theta)\Vert}_{C[\zeta_{1},\zeta_{2}]})$$




$$=\mu_{2k}{\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}. \tag{36}$$

such that $\mu_{2k}=\frac{L{/\rho_{1}-\rho_{2}/(\zeta_{2}-\zeta_{1})}^{\gamma+\alpha-1}}{\Gamma(\gamma+\alpha)}<1$, it results that *G* is a contraction on [$\zeta_{1}$,$\zeta_{2}$].

Consequently, $\exists$!-Sol $\theta_{1}$($\zeta$) for $\zeta\in$[$\zeta_{1}$,$\zeta_{2}$]. According to Lemma 6, it can be observed that $\vartheta_{0}(\zeta_{1})=\vartheta_{1}(\zeta_{1})$. Therefore,


$$\vartheta(\zeta)=\left\{\begin{matrix} \vartheta_{0}(\zeta),0<\zeta\leq\zeta_{1}, \\ \vartheta_{1}(\zeta),\zeta_{1}<\zeta\leq\zeta_{2}. \end{matrix} \right. \tag{37}$$

Using Lemma 7, $\vartheta(\zeta)\in C_{1-\gamma}[k\sigma_{1},\zeta_{2}]$. Therefore, $\theta$($\zeta$) is the only solution of (1) and (2) over $[k\sigma_{1},\zeta_{2}]$. If $\zeta_{2}\neq(k+1)\sigma_{1}$, we proceed with the steps as required. Let's presume that we require *N*$-$2 additional steps. Following, we discover the distinct solution $\theta_{k}$($\zeta$) for $\zeta\in[\zeta_{k},\zeta_{k+1}],k=2,3,\cdots,N$, where $k\sigma_{1}=\zeta_{0}<\zeta_{1}<\cdots<\zeta_{N}=(k+1)\sigma_{1}$ such that


$$\mu_{k+1}=\frac{L|\rho_{1}-\rho_{2}|}{\Gamma(\gamma+\alpha)}(\zeta_{k+1}-\zeta_{k})^{\gamma+\alpha-1}<1. \tag{38}$$

We possess the exclusive solution $\vartheta(\zeta)\in C_{1-\gamma}[k\sigma_{1},(k+1)\sigma_{1}]$ such that


$$\vartheta(\zeta)=\left\{\begin{matrix} \vartheta_{0}(\zeta),k\sigma_{1}<\zeta<\zeta_{1}, \\ \vartheta_{1}(\zeta),\zeta_{1}<\zeta\leq\zeta_{2}, \\ \vdots \\ \vartheta_{N}(\zeta),\zeta_{N-1}<\zeta<(k+1)\sigma_{1}. \end{matrix} \right. \tag{39}$$

Since $\vartheta(\zeta)\in C_{1-\gamma}[k\sigma_{1},(k+1)\sigma_{1}]$ is the! -Sol, so it fulfills (9) and


$$\vartheta(\zeta)=(\frac{\rho_{1}\zeta^{\gamma-1}}{\Gamma(\gamma)}+(I_{k\tau}^{\alpha}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1})))(\zeta),\frac{\rho_{2}\zeta^{\gamma-1}}{\Gamma(\gamma)}+(I_{k\tau}^{\alpha}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2})))(\zeta)). \tag{40}$$

Applying $^{L}D_{{k\sigma_{1}}^{+}}^{\gamma}$ to (40) gives


$$^{L}D_{{k\tau}^{+}}^{\gamma}\vartheta(\zeta)=((^{L}D_{{k\sigma_{1}}^{+}}^{\beta(1-\alpha)}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1})))(\zeta),(^{L}D_{{k\sigma_{1}}^{+}}^{\beta(1-\alpha)}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2})))(\zeta)). \tag{41}$$

It is observed that the RHS lies within the range of $C_{1-\gamma}[k\sigma_{1},(k+1)\sigma_{1}]$ which entails that $^{L}D_{{k\sigma_{1}}^{+}}^{\gamma}\vartheta(\zeta)\in C_{1-\gamma}[k\sigma_{1},(k+1)\sigma_{1}]$. Consequently, according to (9); $\vartheta(\zeta)\in C_{1-\gamma}^{\gamma}[k\sigma_{1},(k+1)\sigma_{1}]$ and this is the only solution for $[k\sigma_{1},(k+1)\sigma_{1}]$. By applying induction, a! -Sol exists for [$\sigma_{1}$, $\sigma_{2}$].

**Case 3.** When $\zeta\in(\sigma_{2},\mathcal{F}]$ and $\vartheta(\zeta),\overset{\sim}{\vartheta}(\zeta)\in C_{1-\gamma}[\sigma_{2},\mathcal{F}]$ we can divide [0,$\mathcal{F}$] into $[0,\sigma_{2}]\cup\cdots\cup[(k_{0}-1)\sigma_{2},k_{0}\sigma_{2}]\cup[k_{0}\sigma_{2},\mathcal{F}]$, where *k*$_{0}\in\mathbb{N}$ and fulfills $0\leq\mathcal{F}-k_{0}\sigma_{2}\leq\mathcal{F}$. In Case 1, we have already proven that (1) and (2) have a! -Sol on [0, $\sigma_{2}$], denoted as $\vartheta_{0}^{\ast}(\zeta)$. Now, let's assume that (1) and (2) have a! -Sol on [$\sigma_{2}$, *k$\sigma$*$_{2}$], where 1≤*k*≤*k*$_{0}$ say $\vartheta_{k}^{\ast}(\zeta)$. We aim to ensure the existence of a! -Sol $\vartheta_{k+1}^{\ast}(\zeta)$ on $[k\sigma_{2},(k+1)\sigma_{2}].$ Consider $\zeta\in[k\sigma_{2},(k+1)\sigma_{2}]$ and


$$G\vartheta(\zeta)=(\frac{\rho_{1}\zeta^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)v),\vartheta_{1}(s-\sigma_{1}))}{\Gamma(\alpha)}ds,\frac{\rho_{2}\zeta^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2}))}{\Gamma(\alpha)}ds). \tag{42}$$

Suppose *G* is a contraction for $\zeta$ in [$\sigma_{2}$, *k$\sigma$*$_{2}$] such that ${\Vert G\vartheta(\zeta)-G\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\sigma_{2},k\sigma_{2}]}\leq\mu_{k}{\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\sigma_{2},k\sigma_{2}]}$. This entails that $\exists$!-Sol $\varphi_{k}(\zeta)=(\varphi_{1k}(\zeta),\varphi_{2k}(\zeta))$ for all $\zeta\in[\sigma_{2},k\sigma_{2}]$. Consider $\zeta\in[k\sigma_{2},(k+1)\sigma_{2}]$, so, $\vartheta(\zeta-\sigma_{2})=\overset{\sim}{\vartheta}(\zeta-\sigma_{2})=\varphi_{k}(\zeta-\sigma_{2})=(\varphi_{1k}(\zeta-\sigma_{2}),\varphi_{2k}(\zeta-\sigma_{2}))$. Let $c_{1}$ and $c_{2}$ be chosen such that $\sigma<c_{1}\leq c_{2}\leq k\sigma_{2}$. Then, $C_{1-\gamma}[c_{1},c_{2}]$ equipped


$$d(\vartheta(\zeta),\overset{\sim}{\vartheta}(\zeta))={\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[c_{1},c_{2}]}=\max_{\zeta\in[c_{1},c_{2}]}\Vert\zeta^{1-\gamma}(\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta))\Vert. \tag{43}$$

Select $\zeta_{1}\in(k\sigma_{2},(k+1)\sigma_{2}]$ and due to $\frac{\rho_{1}\zeta^{\gamma-1}}{\Gamma(\gamma)},\frac{\rho_{2}\zeta^{\gamma-1}}{\Gamma(\gamma)}\in C_{1-\gamma}[k\sigma_{2},\zeta_{1}]$, so leveraging Lemma 5 due to $G\vartheta\in C_{1-\gamma}[k\sigma_{2},\zeta_{1}]$; $G$ maps $C_{1-\gamma}[k\sigma_{2},\zeta_{1}]$ into itself. Therefore, $\forall\vartheta(\zeta),\overset{\sim}{\vartheta}(\zeta)\in C_{1-\gamma}[k\sigma_{2},\zeta_{1}]$, we have


$${\Vert G\vartheta(\zeta)-G\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[k\sigma_{2},\zeta_{1}]}={\Vert I_{0^{+}}^{\alpha}\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-I_{0^{+}}^{\alpha}\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))\Vert}_{C_{1-\gamma}[k\sigma_{2},\zeta_{1}]}$$




$$\leq{\Vert I_{0^{+}}^{\alpha}|\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))|\Vert}_{C_{1-\gamma}[k\sigma_{2},\zeta_{1}]}. \tag{44}$$

Note that


$${\Vert\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))\Vert}_{C_{1-\gamma}[k\sigma_{2},\zeta_{1}]}=\max_{\zeta\in[k\sigma_{2},\zeta_{1}]}\Vert\zeta^{1-\gamma}\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))\Vert. \tag{45}$$

Similarly, by applying the same procedure in Cases 1 and 2, one finds


$${\Vert G\vartheta(\zeta)-G\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[k\sigma_{2},\zeta_{1}]}\leq\frac{|\rho_{1}-\rho_{2}|}{\Gamma(\gamma+\alpha)}{\zeta_{1}}^{\gamma+\alpha-1}L{\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[k\sigma_{2},\zeta_{1}]}=\mu_{1k}{\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C_{1-\gamma}[k\sigma_{2},\zeta_{1}]}, \tag{46}$$


where $\mu_{1k}=\frac{L/\rho_{1}-\rho_{2}/}{\Gamma(\gamma+\alpha)}{\zeta_{1}}^{\gamma+\alpha-1}<1$.

According to the CMT, there exists a single fixed point that serves as the solution $\vartheta_{0}(\zeta)\in C_{1-\gamma}[k\sigma_{2},\zeta_{1}]$ within (*k$\sigma$*, $\zeta_{1}$]. In the case where $\zeta_{1}\neq(k+1)\sigma_{2}$ we examine $[\zeta_{1},(k+1)\sigma_{2}]$. It is important to note that


$$G\vartheta(\zeta)=(\frac{{\rho_{1}\zeta}^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta_{1}}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1}))}{\Gamma(\alpha)}ds+(I_{\zeta_{1}}^{\alpha}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1})))(\zeta),\frac{{\rho_{2}\zeta}^{\gamma-1}}{\Gamma(\gamma)}+\int_{0}^{\zeta_{1}}\frac{(\zeta-s)^{\alpha-1}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2}))}{\Gamma(\alpha)}ds+(I_{\zeta_{1}}^{\alpha}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2})))(\zeta)). \tag{47}$$

Let $\vartheta(\zeta),\overset{\sim}{\vartheta}(\zeta)\in C[\zeta_{1},\zeta_{2}]$ for some $\zeta_{1}<\zeta_{2}\leq(k+1)\sigma_{2}$, we see that


$${\Vert G\vartheta(\zeta)-G\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}$$




$$\leq\frac{|\rho_{1}-\rho_{2}|}{\Gamma(\gamma)}\frac{\Gamma(\gamma)(\zeta_{2}-\zeta_{1})^{\gamma+\alpha-1}}{\Gamma(\alpha+\gamma)}{\Vert\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))-\mathcal{S}(\zeta,\wp(\overset{\sim}{\vartheta}(\zeta)),\overset{\sim}{\vartheta}(\zeta-\theta))\Vert}_{C[\zeta_{1},\zeta_{2}]}$$




$$\leq\frac{L|\rho_{1}-\rho_{2}|(\zeta_{2}-\zeta_{1})^{\gamma+\alpha-1}}{\Gamma(\alpha+\gamma)}({\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2})}+{\Vert\vartheta(\zeta-\theta)-\overset{\sim}{\vartheta}(\zeta-\theta)\Vert}_{C[\zeta_{1},\zeta_{2}]})$$




$$=\frac{L|\rho_{1}-\rho_{2}|(\zeta_{2}-\zeta_{1})^{\gamma+\alpha-1}}{\Gamma(\alpha+\gamma)}({\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}+{\Vert\varphi_{k}(\zeta-\theta)-\varphi_{k}(\zeta-\theta)\Vert}_{C[\zeta_{1},\zeta_{2}]})$$




$$=\mu_{2k}{\Vert\vartheta(\zeta)-\overset{\sim}{\vartheta}(\zeta)\Vert}_{C[\zeta_{1},\zeta_{2}]}. \tag{48}$$


where $\mu_{2k}=\frac{L/\rho_{1}-\rho_{2}/(\zeta_{2}-\zeta_{1})^{\gamma+\alpha-1}}{\Gamma(\alpha+\gamma)}<1$. It results that *G* is a contraction over [$\zeta_{1}$,$\zeta_{2}$], and there exists a single solution $\theta_{1}$($\zeta$) for $\zeta\in$[$\zeta_{1}$,$\zeta_{2}$]. As per Lemma 6, it can be observed that $\theta_{0}$($\zeta_{1}$) = $\theta_{1}$($\zeta_{1}$). Consequently, the function $\theta_{1}$($\zeta$) can be expressed as


$$\vartheta(\zeta)=\left\{\begin{matrix} \vartheta_{0}(\zeta),0<\zeta\leq\zeta_{1}, \\ \vartheta_{1}(\zeta),\zeta_{1}<\zeta\leq\zeta_{2}. \end{matrix} \right. \tag{49}$$

Furthermore, according to Lemma 7; $\vartheta(\zeta)\in C_{1-\gamma}[k\sigma_{2},\zeta_{2}]$. Therefore, $\theta$($\zeta$) represents the! -Sol to (1) and (2) within [*k$\sigma$*, $\zeta_{2}$]. If $\zeta_{2}\neq(k+1)\sigma_{2}$ the aforementioned steps can be repeated as necessary. Assuming that *N*$-$2 additional steps are required; the! -Sol $\theta_{k}$($\zeta$) can be determined for $\zeta\in[\zeta_{k},\zeta_{k+1}]$, where *k* = 2,3,…,*N*. Here, $k\sigma_{2}=\zeta_{0}<\zeta_{1}<\cdots<\zeta_{N}=(k+1)\sigma_{2}$ ensuring the resulting conditions are satisfied


$$\mu_{k+1}=\frac{L|\rho_{1}-\rho_{2}|}{\Gamma(\gamma+\alpha)}(\zeta_{k+1}-\zeta_{k})^{\gamma+\alpha-1}<1. \tag{50}$$

We possess a! -Sol $\vartheta(\zeta)\in C_{1-\gamma}[k\sigma_{2},(k+1)\sigma_{2}]$ which can be defined as


$$\vartheta(\zeta)=\left\{\begin{matrix} \vartheta_{0}(\zeta),k\sigma_{2}<\zeta<\zeta_{1}, \\ \vartheta_{1}(\zeta),\zeta_{1}<\zeta\leq\zeta_{2}, \\ \vdots \\ \vartheta_{N}(\zeta),\zeta_{N-1}<\zeta<(k+1)\sigma_{2}. \end{matrix} \right. \tag{51}$$

Since $\vartheta(\zeta)\in C_{1-\gamma}[k\sigma_{2},(k+1)\sigma_{2}]$ is the! -Sol, it fulfills (9) and


$$\vartheta(\zeta)=(\frac{\rho_{1}\zeta^{\gamma-1}}{\Gamma(\gamma)}+(I_{k\sigma_{2}}^{\alpha}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1})))(\zeta),\frac{\rho_{2}\zeta^{\gamma-1}}{\Gamma(\gamma)}+(I_{k\sigma_{2}}^{\alpha}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2})))(\zeta)). \tag{52}$$

Applying $^{L}D_{{k\sigma_{2}}^{+}}^{\gamma}$ to both sides of (52) gives


$$^{L}D_{{k\sigma}^{+}}^{\gamma}\vartheta(\zeta)=((^{L}D_{{k\sigma_{2}}^{+}}^{\beta(1-\alpha)}{\mathcal{S}}_{1}(s,\wp_{1}(\vartheta(s)),\vartheta_{1}(s-\sigma_{1})))(\zeta),(^{L}D_{{k\sigma_{2}}^{+}}^{\beta(1-\alpha)}{\mathcal{S}}_{2}(s,\wp_{2}(\vartheta(s)),\vartheta_{2}(s-\sigma_{2})))(\zeta)). \tag{53}$$

We can see that the RHS is in *C*$_{1-\gamma}$. Consequently, $^{L}D_{{k\sigma_{2}}^{+}}^{\gamma}\vartheta(\zeta)\in C_{1-\gamma}[k\sigma_{2},(k+1)\sigma_{2}]$, so, based on (9); $\vartheta(\zeta)\in C_{1-\gamma}^{\gamma}[k\sigma_{2},(k+1)\sigma_{2}]]\;\mathrm{i}\mathrm{s}\;\mathrm{t}\mathrm{h}\mathrm{e}\;\mathrm{o}\mathrm{n}\mathrm{l}\mathrm{y}\;\mathrm{s}\mathrm{o}\mathrm{l}\mathrm{u}\mathrm{t}\mathrm{i}\mathrm{o}\mathrm{n}\;\mathrm{f}\mathrm{o}\mathrm{r}\;[k\sigma_{2},(k+1)\sigma_{2}]$. Applying induction, we can infer that a!-Sol exists for [$\sigma_{2}$, $\mathcal{F}$ ].

Finally, considering Cases (1–3), one can conclude that a! -Sol exists for (1) and (2) within $C_{1-\gamma}^{\gamma}(J)$.

## 4 The SGLA: Assembly and outcomes

This section introduces the SGLA numerical scheme for solving (1) and (2). This approach is a type of weighted residual numerical technique that uses a finite set of basis polynomials as weighting functions. Based on our proposed algorithm, we employ orthogonal spline local polynomial functions as the weighting functions. Indeed, we prove theorems regarding the convergence and error estimation of the SGLA.

Broadly, the SGLA is a numerical technique used to solve various classes of differential/integral problems including several types by approximating the solution by leveraging a finite collection of orthogonal basis functions. In implementing the Galerkin scheme, the orthogonal basis functions are selected to satisfy the initial or boundary constraints. The effectiveness of the Galerkin approach relies on several prerequisites as follows:

The basis functions must be orthogonal, facilitating straightforward computation of solution coefficients.

The selected basis functions should have regular behavior and adequate smoothness to ensure precise approximation.

The Galerkin numerical scheme will provide accurate results if the problem has a! -Sol that depends continuously on the data and problem constraints. Failure to meet these constraints may result in imprecise or erroneous outcomes from the Galerkin approach.

Initially, we define the OSLPs as basis functions for representing functions within a predefined interval. Following that, we use this concept to define the essential functions in (1) in terms of OSLPs. Next, we calculate the residual required in the Galerkin numerical process. Leveraging the orthogonality of the residual with the OSLPs, the sets of FDDSs can be simplified into an algebraic one. Finally, this system can be solved by leveraging MATHEMATICA 11 to obtain the needed approximation.

**Definition 5.** [[`ref34`]] The OSLP of multiplicity $\beta$ is


$$\phi_{\beta}(\zeta)=\sum_{k=0}^{\beta}(-1)^{\beta+k}\frac{\Gamma(\hbar+k+1)}{\Gamma(\hbar-k+1)(\Gamma(k+1))^{2}}\zeta^{k}, \tag{54}$$


where $k=0,1,\cdots,\beta,\zeta\in[0,1],\phi_{\beta}(0)=(-1)^{\beta}$, and $\phi_{\beta}$(1) = 1.

The condition of orthogonality is defined as


$$\int_{0}^{1}\phi_{\beta}(\zeta)\phi_{\mathrm{k}}(\zeta)d\zeta=\left\{\begin{matrix} \frac{1}{2\beta+1},\beta=\mathrm{k}, \\ 0,\beta\neq\mathrm{k}. \end{matrix} \right. \tag{55}$$

This entails that $\forall\Delta\in C([0,1]\to\mathbb{R})$ may characterized in OSLPs with $\beta$ = 0,1,$\cdots$,$\hbar$ as


$$\left\{\begin{matrix} \Delta(\zeta)\approx\Delta_{\hbar}(\zeta)=(\Delta_{1}(\zeta),\Delta_{2}(\zeta))=(\sum_{\beta=0}^{\hbar}a_{\beta}\phi_{\beta}(\zeta),\sum_{\beta=0}^{\hbar}b_{\beta}\phi_{\beta}(\zeta)), \\ a_{\beta}=\langle\Delta_{1}(\zeta),\phi_{\beta}(\zeta)\rangle=\int_{0}^{1}(2\beta+1)\Delta_{1}(\zeta)\phi_{\beta}(\zeta)d\zeta, \\ b_{\beta}=\langle\Delta_{2}(\zeta),\phi_{\beta}(\zeta)\rangle=\int_{0}^{1}(2\beta+1)\Delta_{2}(\zeta)\phi_{\beta}(\zeta)d\zeta. \end{matrix} \right. \tag{56}$$

Now, interpolating OSLP for $\theta$($\zeta$) in (1), we get


$$\vartheta(\zeta)\approx\vartheta_{\hbar}(\zeta)=(\sum_{\beta=0}^{\hbar}a_{\beta}\phi_{\beta}(\zeta),\sum_{\beta=0}^{\hbar}b_{\beta}\phi_{\beta}(\zeta)), \tag{57}$$


where $a_{\beta}$ and $b_{\beta},$ $\beta$ = 0,1,$\cdots$,$\hbar$ are the unknown parameters.

Again, $^{H}D_{0^{+}}^{\alpha,\beta}\vartheta(\zeta),\wp(\vartheta(\zeta))$, and $\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))$ can be approximated with


$$^{H}D_{0^{+}}^{\alpha,\beta}\vartheta(\zeta)\approx^{H}D_{0^{+}}^{\alpha,\beta}\vartheta_{\hbar}(\zeta)=(\sum_{\beta=0}^{\hbar}a_{\beta}^{H}D_{0^{+}}^{\alpha,\beta}\phi_{\beta}(\zeta),\sum_{\beta=0}^{\hbar}b_{\beta}^{H}D_{0^{+}}^{\alpha,\beta}\phi_{\beta}(\zeta)). \tag{58}$$




$$\wp(\vartheta(\zeta))\approx\wp_{\hbar}(\vartheta_{\hbar}(\zeta))=(\wp_{1\hbar}(\vartheta_{\hbar}(\zeta)),\wp_{2\hbar}(\vartheta_{\hbar}(\zeta))). \tag{59}$$




$$\mathcal{S}(\zeta,\wp(\vartheta(\zeta)),\vartheta(\zeta-\theta))\approx{\mathcal{S}}_{\hbar}(\zeta,\wp_{\hbar}(\vartheta_{\hbar}(\zeta)),\vartheta_{\hbar}(\zeta-\theta))=({\mathcal{S}}_{1}(\zeta,\wp_{1\hbar}(\vartheta_{\hbar}(\zeta)),\sum_{\beta=0}^{\hbar}a_{\beta}\phi_{\beta}(\zeta-\sigma_{1})),{\mathcal{S}}_{2}(\zeta,\wp_{2\hbar}(\vartheta_{\hbar}(\zeta)),\sum_{\beta=0}^{\hbar}b_{\beta}\phi_{\beta}(\zeta-\sigma_{2}))). \tag{60}$$

Swapping out (57), (58), (59) and (60) into (1) gives


$$^{H}D_{0^{+}}^{\alpha,\beta}\vartheta_{\hbar}(\zeta)={\mathcal{S}}_{\hbar}(\zeta,\wp_{\hbar}(\vartheta_{\hbar}(\zeta)),\vartheta_{\hbar}(\zeta-\theta)). \tag{61}$$

To execute the SGLA, simulate the residual, *R*, out of (61) as


$$R(\zeta)=(R_{1}(\zeta)-R_{2}(\zeta))=^{H}D_{0^{+}}^{\alpha,\beta}\vartheta_{\hbar}(\zeta)-{\mathcal{S}}_{\hbar}(\zeta,\wp_{\hbar}(\vartheta_{\hbar}(\zeta)),\vartheta_{\hbar}(\zeta-\theta)). \tag{62}$$

When *R*($\zeta$) = (0,0), the result solves (1) and fulfills the presumed constraints, Therefore, in our research, we intend to optimize *R* to be zero in light of the selected OSLP that is chosen as the basis to axiomatize $\theta_{\hbar}$($\zeta$).

To fix the unknowable parameters $a_{0},a_{1},\cdots,a_{\hbar},b_{0},b_{1},\cdots,b_{\hbar}$ in (57), we select weight functions as the granted basis and integrate constraint controls concerning *R* = 0. By leveraging the orthogonality of *R* at ($\hbar$+1) mapping $\phi_{0}(\zeta),\phi_{1}(\zeta),\cdots,\phi_{\hbar}(\zeta)$ as


$$\left\{\begin{matrix} \int_{0}^{1}R_{1}(\zeta)\phi_{\beta}(\zeta)d\zeta=0,\beta=0,1,\cdots,\hbar-1, \\ \sum_{\beta=0}^{\hbar}a_{\beta}\phi_{\beta}(0)=\phi_{1}(0). \end{matrix} \right. \tag{63}$$




$$\left\{\begin{matrix} \int_{0}^{1}R_{2}(\zeta)\phi_{\beta}(\zeta)d\zeta=0,\beta=0,1,\cdots,\hbar-1, \\ \sum_{\beta=0}^{\hbar}b_{\beta}\phi_{\beta}(0)=\phi_{2}(0). \end{matrix} \right. \tag{64}$$

So, one has a set of 2($\hbar$+1) algebraic equations including 2($\hbar$+1) unknown $a_{0},a_{1},\cdots,a_{\hbar},b_{0},b_{1},\cdots,b_{\hbar}$. After substituting the estimated $a_{0},a_{1},\cdots,a_{\hbar},b_{0},b_{1},\cdots,b_{\hbar}$ parameters in (57) we get the approximated solution for (1).

Next, similar to symbolization in Theorem 2; $\vartheta_{\hbar}(\zeta)=(\vartheta_{1\hbar}(\zeta),\vartheta_{2\hbar}(\zeta))=(\sum_{\beta=0}^{\hbar}a_{\beta}\phi_{\beta}(\zeta),\sum_{\beta=0}^{\hbar}b_{\beta}\phi_{\beta}(\zeta)),\vartheta_{\beta}(\zeta)=(\vartheta_{1\beta}(\zeta),\vartheta_{2\beta}(\zeta))=(\sum_{\beta=0}^{\beta}a_{\beta}\phi_{\beta}(\zeta),\sum_{\beta=0}^{\beta}b_{\beta}\phi_{\beta}(\zeta))$, and $\vartheta(\zeta)=(\vartheta_{1}(\zeta),\vartheta_{2}(\zeta))$.

**Theorem 3.** Let $(\sum_{\beta=0}^{\infty}a_{\beta}\phi_{\beta}(\zeta),\sum_{\beta=0}^{\infty}b_{\beta}\phi_{\beta}(\zeta))$ be the Legendre interpolation of $\theta$($\zeta$) with $\vartheta_{\beta}(\zeta)\in L^{2}[0,1]$. Then $\vartheta_{\hbar}(\zeta)\to\vartheta(\zeta)$ as $\hbar$→$\infty$.

**Proof.** Set $\theta_{\beta}$($\zeta$) be a partial sum in $(\sum_{\beta=0}^{\infty}a_{\beta}\phi_{\beta}(\zeta),\sum_{\beta=0}^{\infty}b_{\beta}\phi_{\beta}(\zeta))$. Then at $\hbar$>$\beta$, it is straightforward to verify that


$${\Vert\vartheta_{\hbar}(\zeta)-\vartheta_{\beta}(\zeta)\Vert}^{2}={\Vert[\begin{matrix} \vartheta_{1\hbar}(\zeta) \\ \vartheta_{2\hbar}(\zeta) \end{matrix}]-[\begin{matrix} \vartheta_{1\beta}(\zeta) \\ \vartheta_{2\beta}(\zeta) \end{matrix}]\Vert}^{2}={\Vert[\begin{matrix} \sum_{\beta=\beta+1}^{\hbar}{a_{\beta}\varphi}_{\beta}(\zeta) \\ \sum_{\beta=\beta+1}^{\hbar}b_{\beta}\varphi_{\beta}(\zeta) \end{matrix}]\Vert}^{2}<[\begin{matrix} \sum_{\beta=\beta+1}^{\hbar}{|a_{\beta}|}^{2} \\ \sum_{\beta=\beta+1}^{\hbar}{|b_{\beta}|}^{2} \end{matrix}]. \tag{65}$$

Utilizing the inequality of Bessel's, one finds $\sum_{\beta=\beta+1}^{\hbar}{/a_{\beta}/}^{2}\leq\sum_{\beta=\beta+1}^{\infty}{/a_{\beta}/}^{2}\leq{\Vert\vartheta\Vert}^{2}<\infty$ and $\sum_{\beta=\beta+1}^{\hbar}{/b_{\beta}/}^{2}\leq\sum_{\beta=\beta+1}^{\infty}{/b_{\beta}/}^{2}\leq{\Vert\vartheta\Vert}^{2}<\infty$. So, ${\Vert\vartheta_{\hbar}(\zeta)-\vartheta_{\beta}(\zeta)\Vert}^{2}\to0$ as $\hbar$,$\beta$→$\infty$. Thus, the sequence $\theta_{\hbar}$($\zeta$) is Cauchy and converges to $\mathcal{S}(\zeta)\in L^{2}([0,1)]$. Anyhow,


$$\langle[\begin{matrix} {\mathcal{S}}_{1}(\zeta) \\ {\mathcal{S}}_{2}(\zeta) \end{matrix}]-[\begin{matrix} \vartheta_{1}(\zeta) \\ \vartheta_{2}(\zeta) \end{matrix}],[\begin{matrix} \varphi_{\beta}(\zeta) \\ \varphi_{\beta}(\zeta) \end{matrix}]\rangle=\langle[\begin{matrix} {\mathcal{S}}_{1}(\zeta) \\ {\mathcal{S}}_{2}(\zeta) \end{matrix}],[\begin{matrix} \varphi_{\beta}(\zeta) \\ \varphi_{\beta}(\zeta) \end{matrix}]\rangle-\langle[\begin{matrix} \vartheta_{1}(\zeta) \\ \vartheta_{2}(\zeta) \end{matrix}],[\begin{matrix} \varphi_{\beta}(\zeta) \\ \varphi_{\beta}(\zeta) \end{matrix}]\rangle$$




$$=\lim_{\hbar\to\infty}\langle[\begin{matrix} \vartheta_{1\hbar}(\zeta) \\ \vartheta_{2\hbar}(\zeta) \end{matrix}],[\begin{matrix} \varphi_{\beta}(\zeta) \\ \varphi_{\beta}(\zeta) \end{matrix}]\rangle-(a_{\beta},b_{\beta})$$




$$=(a_{\beta},b_{\beta})-(a_{\beta},b_{\beta})$$




$$=(0,0). \tag{66}$$

Hence, $({\mathcal{S}}_{1}(\zeta),{\mathcal{S}}_{2}(\zeta))$ and $(\vartheta_{1\hbar}(\zeta),\vartheta_{2\hbar}(\zeta))$ converges to $\theta$($\zeta$) as $\hbar$→$\infty$.

**Theorem 4.** An $\varepsilon_{\hbar}(\zeta)=\vartheta(\zeta)-\vartheta_{\hbar}(\zeta)$ can approximate the solution of (1) and (2) by the SGLA.

**Proof.** By leveraging the SGLA, $\theta_{\hbar}$($\zeta$) fulfills (1). Hence,


$$\left\{\begin{matrix} ^{H}D_{0^{+}}^{\alpha,\beta}\vartheta_{\hbar}(\zeta)=[\begin{matrix} ^{H}D_{0^{+}}^{\alpha,\beta}\vartheta_{1\hbar}(\zeta) \\ ^{H}D_{0^{+}}^{\alpha,\beta}\vartheta_{2\hbar}(\zeta) \end{matrix}]=[\begin{matrix} {\mathcal{S}}_{1\hbar}(\zeta,\wp_{1\hbar}(\vartheta_{\hbar}(\zeta)),\vartheta_{1\hbar}(\zeta-\sigma_{1})) \\ {\mathcal{S}}_{2\hbar}(\zeta,\wp_{2\hbar}(\vartheta_{\hbar}(\zeta)),\vartheta_{2\hbar}(\zeta-\sigma_{2})) \end{matrix}],0<\zeta\leq\mathcal{F}, \\ \vartheta_{\hbar}(\zeta)=[\begin{matrix} \phi_{1}(\zeta) \\ \phi_{2}(\zeta) \end{matrix}],-\sigma_{1}\leq\zeta\leq0. \end{matrix} \right. \tag{67}$$

Now, by subtracting (67) from (1), one has


$$\left\{\begin{matrix} ^{H}D_{0^{+}}^{\alpha,\beta}\varepsilon_{\hbar}(\zeta)=h(\zeta,\varepsilon_{\hbar}(\zeta),\varepsilon_{\hbar}(\zeta-\theta)),0<\zeta\leq\mathcal{F}, \\ \varepsilon_{\hbar}(\zeta)=0,-\sigma_{1}\leq\zeta\leq0, \end{matrix} \right. \tag{68}$$




$$h(\zeta,\varepsilon_{\hbar}(\zeta),\varepsilon_{\hbar}(\zeta-\tau))=[\begin{matrix} {\mathcal{S}}_{1\hbar}(\zeta,\wp_{1\hbar}(\vartheta_{\hbar}(\zeta)),\vartheta_{1\hbar}(\zeta-\sigma_{1}))-{\mathcal{S}}_{1}(\zeta,\wp_{1}(\vartheta(\zeta)),\vartheta_{1}(\zeta-\sigma_{1})) \\ {\mathcal{S}}_{2\hbar}(\zeta,\wp_{2\hbar}(\vartheta_{\hbar}(\zeta)),\vartheta_{2\hbar}(\zeta-\sigma_{2}))-{\mathcal{S}}_{2}(\zeta,\wp_{2}(\vartheta(\zeta)),\vartheta_{2}(\zeta-\sigma_{2})) \end{matrix}] \tag{69}$$

Thus, to solve (68) and (69), we can use SGLA to gain a set of $2(\hbar+1)$ algebraic equations, and by handling this set one gets $\varepsilon_{\hbar}$($\zeta$).

## 5 Computational demonstrations

Herein, we solve two systems of DFDEs in the FHD sense that involve various instances by leveraging the proposed algorithm. While the second example is nonlinear, the first one is linear. Our methodology involves controlling the relative error ${\mathcal{R}}_{\hbar}(\zeta)=({\mathcal{R}}_{\hbar,1}(\zeta),{\mathcal{R}}_{\hbar,2}(\zeta))$ and the absolute error ${\mathcal{A}}_{\hbar}(\zeta)=({\mathcal{A}}_{\hbar,1}(\zeta),{\mathcal{A}}_{\hbar,2}(\zeta))$ for various values of $\alpha$, $\beta$, and $\hbar$. We provide a thorough examination of the effectiveness through plots and data that contrast the exact solution with the approximations.

### 5.1 SGLA: Steps and examples

The SGLA provides several benefits and utilities when handling a set of FDEs as

The simulated outcomes achieved closely approximates the exact solution. As evidenced by the obtained $\Re_{\hbar}$($\zeta$) and ${\mathcal{A}}_{\hbar}(\zeta)$, SGLA delivers highly sufficient and accurate results.

The SGLA can achieve high validity by leveraging tiny iterations in the OSLP expansions.

The SGLA represents a straightforward scheme to incorporate that does not require sophisticated mathematical apparatuses or a proficient programmer.

The utilized SGLA serves as a universal technique that can be implemented to exhibit other instances of fractional systems.

The core characteristic of SGLA is its applicability to other orthogonal basis functions.

Algorithm 1 clearly outlines the steps to derive a solution leveraging the presented SGLA and the specified FHD. At this point, either a programmer or a specialized mathematician with relevant expertise could implement this algorithm into a program by leveraging the MATHEMATICA 11 environment, consistent with how we demonstrated this approach in this paper.

**Algorithm 1.** The SGLA iteration steps for calculations of $\theta_{\hbar}$(*x*) concerning DFDE (1) and (2) in the FHD sense.

Requirement:

Primary parameters: $\alpha$,$\beta$,$\tau$, and $\sigma$;

Nonhomogenous terms: ${\mathcal{S}}_{1}$ and ${\mathcal{S}}_{2}$;

OSLPs of $^{H}D_{0^{+}}^{\alpha,\beta}\vartheta_{1}(\zeta)$;

Magnitude of accuracy: $\hbar$.

Action A:

While $\beta$ = 0,1,$\cdots$,$\hbar$ find *R*($\zeta$) from (62).

Action B:

While $\beta$ = 1,2,$\cdots$,$\hbar-$1 find

$\left\{\begin{matrix} \int_{0}^{1}R_{1}(\zeta)\phi_{\beta}(\zeta)d\zeta=0, \\ \sum_{\beta=0}^{\hbar}a_{\beta}\phi_{\beta}(0)=\phi_{1}(0). \end{matrix} \right.$

$\left\{\begin{matrix} \int_{0}^{1}R_{2}(\zeta)\phi_{\beta}(\zeta)d\zeta=0, \\ \sum_{\beta=0}^{\hbar}b_{\beta}\phi_{\beta}(0)=\phi_{2}(0). \end{matrix} \right.$

Action C:

While $\beta$ = 0,1,$\cdots$,$\hbar$ find $\sum_{\beta=0}^{\hbar}a_{\beta}(-1)^{\beta}$ and $\sum_{\beta=0}^{\hbar}b_{\beta}(-1)^{\beta}$.

Action D:

Solve 2($\hbar$+1) set of equations generated from Action B to find $c_{\beta},\beta=0,1,\cdots,\hbar$.

Result:

Assign $a_{\beta}$ and $b_{\beta}$ with $\beta$ = 0,1,$\cdots$,$\hbar$ on OSLP of $\theta$($\zeta$) to get $\theta_{\hbar}$(*x*).

Let us begin our calculations with the following two suitable instances, both of which have precise solutions throughout the chosen integral domain, ensuring the accuracy of the data outputs.

**Example 1.** Examine the fraction approximations of the succeeding linear sets of DFDEs in the FHD sense:


$$\left\{\begin{matrix} ^{H}D_{0^{+}}^{\alpha,\beta}\vartheta_{1}(\zeta)=\vartheta_{1}(\zeta-0.25)+\vartheta_{2}(\zeta)+u_{1}(\zeta),0<\zeta\leq2, \\ ^{H}D_{0^{+}}^{\alpha,\beta}\vartheta_{2}(\zeta)=\vartheta_{2}(\zeta-1)+\vartheta_{2}(\zeta)+\vartheta_{1}(\zeta)+u_{2}(\zeta), \\ u_{1}(\zeta)=-(\zeta-0.25)-\zeta^{2}+\frac{1}{\Gamma(2-\alpha)}\zeta^{1-\alpha}, \\ u_{2}(\zeta)=-(\zeta-1)-\zeta^{2}-\zeta+\frac{2}{\Gamma(3-\alpha)}\zeta^{2-\alpha}, \\ \vartheta_{1}(\zeta)=\zeta,-0.25\leq\zeta\leq0, \\ \vartheta_{2}(\zeta)=\zeta^{2},-1\leq\zeta\leq0. \end{matrix} \right. \tag{70}$$

Hither, $\theta_{1}$($\zeta$) = $\zeta$ over $\zeta\in$[$-$0.25,2] and $\theta_{1}$($\zeta$) = $\zeta^{2}$ over $\zeta\in$[$-$1,2]. Indded, 0<$\alpha$<1 and 0<$\beta$<1.

**Example 2.** Examine the fraction approximations of the succeeding nonlinear sets of DFDEs in the FHD sense:


$$\left\{\begin{matrix} ^{H}D_{0^{+}}^{\alpha,\beta}\vartheta_{1}(\zeta)+\vartheta_{1}(\zeta-1)+{\vartheta_{1}(\zeta)\vartheta}_{2}(\zeta)=u_{1}(\zeta),0<\zeta\leq2, \\ ^{H}D_{0^{+}}^{\alpha,\beta}\vartheta_{2}(\zeta)+\vartheta_{2}(\zeta-0.7)+\vartheta_{1}(\zeta)\vartheta_{2}(\zeta)+{\vartheta_{1}}^{2}(\zeta)=u_{2}(\zeta), \\ u_{1}(\zeta)=(\zeta-1)+\zeta^{3}+\frac{1}{\Gamma(2-\alpha)}\zeta^{1-\alpha}, \\ u_{2}(\zeta)=(\zeta-0.7)+\zeta^{2}+\zeta^{3}+\frac{2}{\Gamma(3-\alpha)}\zeta^{2-\alpha}, \\ \vartheta_{1}(\zeta)=\zeta,-1\leq\zeta\leq0, \\ \vartheta_{2}(\zeta)=\zeta^{2},-0.7\leq\zeta\leq0. \end{matrix} \right. \tag{71}$$

Hither, $\theta_{1}$($\zeta$) = $\zeta$ over $\zeta\in$[$-$1,1] and $\theta_{2}$($\zeta$) = $\zeta^{2}$ over $\zeta\in$[$-$0.7,1]. Indded, 0<$\alpha$<1 and 0<$\beta$<1.

### 5.2 Analysis of the findings

This study applies the preceding numerical technique to approximate solutions to (1) and (2). Graphical and tabulated outputs are generated for each example scenario under varied configurations of {$\alpha$,$\beta$} and $\hbar$ parameters. Additionally, computations of ${\mathcal{A}}_{\hbar}(\zeta)$ and $\Re_{\hbar}$($\zeta$) are carried out and compared extensively, to evaluate the efficacy of the SGLA numerical approach.

For Example 1; in Table 1, the correlation between data operations ${\mathcal{A}}_{\hbar,1}(\zeta)$ and $\Re_{\hbar,1}$($\zeta$) are presented at $\hbar$ = 3, $\alpha$ = 0.5, and $\beta\in${0.3,0.8}. In Table 2, the correlation between data operations ${\mathcal{A}}_{\hbar,2}(\zeta)$ and $\Re_{\hbar,2}$($\zeta$) are presented at $\hbar$ = 3, $\alpha$ = 0.5, and $\beta\in${0.3,0.8}. To provide additional information, Fig 1 displays ${\mathcal{A}}_{\hbar,1}(\zeta)$ and $\Re_{\hbar,1}$($\zeta$) at $(\hbar,\alpha,\beta)\in\left\{(3,0.3,0.3),(3,0.4,0.9),(3,0.9,0.4)\right\}$ that generated from the SGLA. Fig 2 displays ${\mathcal{A}}_{\hbar,2}(\zeta)$ and $\Re_{\hbar,2}$($\zeta$) at $(\hbar,\alpha,\beta)\in\left\{(3,0.3,0.3),(3,0.4,0.9),(3,0.9,0.4)\right\}$ that generated from the SGLA.

```mermaid {span=wide pos=page width=0.9}
info:doi/10.1371/journal.pone.0305259.g001
```

```mermaid {span=wide pos=page width=0.89}
info:doi/10.1371/journal.pone.0305259.g002
```

%% table

| *$\beta$* | 0.3 | 0.8 |  |  |
|---|---|---|---|---|
| $\zeta$$_{\beta}$ | $A_{\hbar,1}(\zeta_{\beta})$ | $\Re$$_{\hbar,1}$($\zeta$$_{\beta}$) | $A_{\hbar,1}(\zeta_{\beta})$ | $\Re$$_{\hbar,1}$($\zeta$$_{\beta}$) |
| $0.1$ | $9.7144514654\times{10}^{-17}$ | $8.8817841970\times{10}^{-16}$ | $1.1102230246\times{10}^{-16}$ | $1.1102230246\times{10}^{-15}$ |
| $0.2$ | $2.7755575615\times{10}^{-17}$ | $2.2204460492\times{10}^{-16}$ | $2.2204460492\times{10}^{-16}$ | $1.1102230246\times{10}^{-15}$ |
| $0.3$ | $1.6653345369\times{10}^{-16}$ | $6.6613381477\times{10}^{-16}$ | $2.7755575615\times{10}^{-16}$ | $8.8817841970\times{10}^{-16}$ |
| $0.4$ | $2.7755575615\times{10}^{-16}$ | $6.6613381477\times{10}^{-16}$ | $3.8857805861\times{10}^{-16}$ | $1.1102230246\times{10}^{-15}$ |
| $0.5$ | $4.4408920985\times{10}^{-16}$ | $8.8817841970\times{10}^{-16}$ | $5.5511151231\times{10}^{-16}$ | $1.1102230246\times{10}^{-15}$ |
| $0.6$ | $5.5511151231\times{10}^{-16}$ | $8.8817841970\times{10}^{-16}$ | $6.6613381477\times{10}^{-16}$ | $1.1102230246\times{10}^{-15}$ |
| $0.7$ | $8.8817841970\times{10}^{-16}$ | $1.3322676295\times{10}^{-15}$ | $7.7715611723\times{10}^{-16}$ | $1.1102230246\times{10}^{-15}$ |
| $0.8$ | $9.9920072216\times{10}^{-16}$ | $1.3322676295\times{10}^{-15}$ | $8.8817841970\times{10}^{-16}$ | $1.1102230246\times{10}^{-15}$ |
| $0.9$ | $1.22124532708\times{10}^{-15}$ | $1.3322676295\times{10}^{-15}$ | $9.9920072216\times{10}^{-16}$ | $1.1102230246\times{10}^{-15}$ |
| $1$ | $1.3322676295\times{10}^{-15}$ | $1.3322676295\times{10}^{-15}$ | $1.1102230246\times{10}^{-15}$ | $1.1102230246\times{10}^{-15}$ |

%% table

| *$\beta$* | 0.3 | 0.8 |  |  |
|---|---|---|---|---|
| $\zeta$$_{\beta}$ | $A_{\hbar,2}(\zeta_{\beta})$ | $\Re$$_{\hbar,2}$($\zeta$$_{\beta}$) | $A_{\hbar,2}(\zeta_{\beta})$ | $\Re$$_{\hbar,2}$($\zeta$$_{\beta}$) |
| $0.1$ | $2.1510571102\times{10}^{-16}$ | $2.1538326677\times{10}^{-14}$ | $2.4286128663\times{10}^{-16}$ | $2.4424906541\times{10}^{-14}$ |
| $0.2$ | $4.5102810375\times{10}^{-16}$ | $1.1324274851\times{10}^{-14}$ | $4.4408920985\times{10}^{-16}$ | $1.1102230246\times{10}^{-14}$ |
| $0.3$ | $6.8001160258\times{10}^{-16}$ | $7.5495165674\times{10}^{-15}$ | $5.8286708792\times{10}^{-16}$ | $6.4392935428\times{10}^{-15}$ |
| $0.4$ | $7.4940054162\times{10}^{-16}$ | $4.6629367034\times{10}^{-15}$ | $5.5511151231\times{10}^{-16}$ | $3.5527136788\times{10}^{-15}$ |
| $0.5$ | $8.3266726846\times{10}^{-16}$ | $3.3306690738\times{10}^{-15}$ | $6.6613381477\times{10}^{-16}$ | $2.6645352591\times{10}^{-15}$ |
| $0.6$ | $8.881784197\times{10}^{-16}$ | $2.4424906541\times{10}^{-15}$ | $7.2164496600\times{10}^{-16}$ | $1.9984014443\times{10}^{-15}$ |
| $0.7$ | $8.8817841970\times{10}^{-16}$ | $1.9984014443\times{10}^{-15}$ | $6.6613381477\times{10}^{-16}$ | $1.5543122344\times{10}^{-15}$ |
| $0.8$ | $7.7715611723\times{10}^{-16}$ | $1.3322676295\times{10}^{-15}$ | $5.5511151231\times{10}^{-16}$ | $8.8817841970\times{10}^{-16}$ |
| $0.9$ | $8.8817841970\times{10}^{-16}$ | $1.1102230246\times{10}^{-15}$ | $5.5511151231\times{10}^{-16}$ | $6.6613381477\times{10}^{-16}$ |
| $1$ | $8.8817841970\times{10}^{-16}$ | $8.8817841970\times{10}^{-16}$ | $6.6613381477\times{10}^{-16}$ | $6.6613381477\times{10}^{-16}$ |

For Example 2; in Table 3, the correlation between data operations ${\mathcal{A}}_{\hbar,2}(\zeta)$ and $\Re_{\hbar,1}$($\zeta$) are presented at $\hbar$ = 3, $\beta$ = 0.5, and $\alpha\in${0.1,0.7}. In Table 4, the correlation between data operations ${\mathcal{A}}_{\hbar,2}(\zeta)$ and $\Re_{\hbar,2}$($\zeta$) are presented at $\hbar$ = 3, $\beta$ = 0.5, and $\alpha\in${0.1,0.7}. To provide additional information, Fig 3 displays ${\mathcal{A}}_{\hbar,1}(\zeta)$ and $\Re_{\hbar,1}$($\zeta$) at $(\hbar,\alpha,\beta)\in\left\{(3,0.3,0.6),(3,0.6,0.3),(3,0.4,0.4)\right\}$ that generated from the SGLA. Fig 4 displays ${\mathcal{A}}_{\hbar,2}(\zeta)$ and ${\mathcal{R}}_{\hbar,2}(\zeta)$ at $(\hbar,\alpha,\beta)\in\left\{(3,0.3,0.6),(3,0.6,0.3),(3,0.4,0.4)\right\}$ that generated from the SGLA.

```mermaid {span=wide pos=page width=0.89}
info:doi/10.1371/journal.pone.0305259.g003
```

```mermaid {span=wide pos=page width=0.89}
info:doi/10.1371/journal.pone.0305259.g004
```

%% table

| *$\alpha$* | 0.1 | 0.7 |  |  |
|---|---|---|---|---|
| $\zeta$$_{\beta}$ | $A_{\hbar,1}(\zeta_{\beta})$ | $\Re$$_{\hbar,1}$($\zeta$$_{\beta}$) | $A_{\hbar,1}(\zeta_{\beta})$ | $\Re$$_{\hbar,1}$($\zeta$$_{\beta}$) |
| 0.1 | $1.7624790515\times{10}^{-15}$ | $1.7541523789\times{10}^{-14}$ | $8.3266726846\times{10}^{-17}$ | $7.7715611723\times{10}^{-16}$ |
| 0.2 | $2.0816681711\times{10}^{-15}$ | $1.0325074129\times{10}^{-14}$ | $8.3266726846\times{10}^{-17}$ | $3.3306690738\times{10}^{-16}$ |
| 0.3 | $2.1094237467\times{10}^{-15}$ | $6.9944050551\times{10}^{-15}$ | $1.1102230246\times{10}^{-16}$ | $3.3306690738\times{10}^{-16}$ |
| 0.4 | $1.9984014443\times{10}^{-15}$ | $4.8849813083\times{10}^{-15}$ | $1.1102230246\times{10}^{-16}$ | $2.2204460492\times{10}^{-16}$ |
| 0.5 | $1.6653345369\times{10}^{-15}$ | $3.3306690738\times{10}^{-15}$ | $1.1102230246\times{10}^{-16}$ | $2.2204460492\times{10}^{-16}$ |
| 0.6 | $1.4432899320\times{10}^{-15}$ | $2.4424906541\times{10}^{-15}$ | $1.1102230246\times{10}^{-16}$ | $2.2204460492\times{10}^{-16}$ |
| 0.7 | $1.2212453270\times{10}^{-15}$ | $1.7763568394\times{10}^{-15}$ | 0 | 0 |
| 0.8 | $1.2212453270\times{10}^{-15}$ | $1.4432899320\times{10}^{-15}$ | 0 | 0 |
| 0.9 | $1.4432899320\times{10}^{-15}$ | $1.5543122344\times{10}^{-15}$ | $1.1102230246\times{10}^{-16}$ | 0 |
| 1 | $2.2204460492\times{10}^{-15}$ | $2.2204460492\times{10}^{-15}$ | 0 | 0 |

%% table

| *$\alpha$* | 0.1 | 0.7 |  |  |
|---|---|---|---|---|
| $\zeta$$_{\beta}$ | $A_{\hbar,2}(\zeta_{\beta})$ | $\Re$$_{\hbar,2}$($\zeta$$_{\beta}$) | $A_{\hbar,2}(\zeta_{\beta})$ | $\Re$$_{\hbar,2}$($\zeta$$_{\beta}$) |
| 0.1 | $5.6205040621\times{10}^{-16}$ | $5.6177285046\times{10}^{-14}$ | $6.2450045135\times{10}^{-17}$ | $6.2172489379\times{10}^{-15}$ |
| 0.2 | $4.1633363423\times{10}^{-16}$ | $1.0325074129\times{10}^{-14}$ | 0 | 0 |
| 0.3 | $8.3266726846\times{10}^{-17}$ | $9.9920072216\times{10}^{-16}$ | $4.1633363423\times{10}^{-17}$ | $4.4408920985\times{10}^{-16}$ |
| 0.4 | $2.4980018054\times{10}^{-16}$ | $1.5543122344\times{10}^{-15}$ | 0 | 0 |
| 0.5 | $7.7715611723\times{10}^{-16}$ | $3.1086244689\times{10}^{-15}$ | $5.5511151231\times{10}^{-17}$ | $2.2204460492\times{10}^{-16}$ |
| 0.6 | $1.2767564783\times{10}^{-15}$ | $3.5527136788\times{10}^{-15}$ | $2.2204460492\times{10}^{-16}$ | $4.4408920985\times{10}^{-16}$ |
| 0.7 | $1.7208456881\times{10}^{-15}$ | $3.5527136788\times{10}^{-15}$ | $1.6653345369\times{10}^{-16}$ | $4.4408920985\times{10}^{-16}$ |
| 0.8 | $1.7763568394\times{10}^{-15}$ | $2.8865798640\times{10}^{-15}$ | $1.1102230246\times{10}^{-16}$ | $2.2204460492\times{10}^{-16}$ |
| 0.9 | $1.9984014443\times{10}^{-15}$ | $2.4424906541\times{10}^{-15}$ | $2.2204460492\times{10}^{-16}$ | $2.2204460492\times{10}^{-16}$ |
| 1 | $1.9984014443\times{10}^{-15}$ | $1.9984014443\times{10}^{-15}$ | $2.2204460492\times{10}^{-16}$ | $2.2204460492\times{10}^{-16}$ |

From the charts in the antecedent figures, we interestingly that the apparent drawing curves are very low towards the $\zeta$-axis. This confirms the data of the driving tables. In addition, these curves contain oscillations at their beginning or end, which confirms the occurrence of a delay in the fraction solutions, and also confirms the shape of the previously selected examples.

## 6 Summary of key findings

Throughout the work, we established the EUE concerning a set of DFDEs in the sense of the FHD approach. Our investigation involved examining the properties of the FRD, FCD, and FHD, utilizing their hallmarks to convert our system of DFDEs into an equivalent one of fractional DVIE with the help of the CMT. For numerical analysis, we implemented the SGLA leveraging OSLPs. This algorithm allows us to approximate the solution to the considered system by transforming it into a series of algebraic equations. One of the key advantages of this algorithm is its ability to achieve accurate results with fewer iterations compared to alternative methods. The numerical convergence and error estimates of the approach are examined and to validate the effectiveness and accuracy of our algorithm, we conducted a comprehensive evaluation through various linear and nonlinear numerical applications. The data from these evaluations, accompanied by charts and tables, further validate the strength of our algorithmic approach. Future research directions for our simulations could include applying it to additional types of DFDEs, such as those containing multiple delays or nonlinear terms. The algorithm could also be analyzed for solving DFDEs and fractional delay integral models. Furthermore, this research emphasizes the adaptability of the SGLA, as it can be extended to other complex fractional problems and systems by utilizing alternative orthogonal basis functions like Chebyshev or Fourier polynomials. Additionally, the algorithm may be parallelized to improve computational efficiency.

## Acknowledgements

The authors would like to express their gratitude to the unknown referees for carefully reading the paper and for their helpful comments.
