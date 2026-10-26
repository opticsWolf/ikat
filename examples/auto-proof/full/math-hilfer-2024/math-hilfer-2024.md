# Well-posedness analysis and pseudo-Galerkin approximations using Tau Legendre algorithm for fractional systems of delay differential models regarding Hilfer (*$\alpha$*,*$\beta$*)-framework set

## Abstract

Fractional calculus serves as a versatile and potent tool for the modeling and control of intricate systems. This discussion debates the system of DFDEs with two regimes; theoretically and numerically. For theoretical analysis, we have established the EUE by leveraging the definition of Hilfer (*$\alpha$*,*$\beta$*)-framework. Our investigation involved the examination of the possessions of the FRD, FCD, and FHD, utilizing their forcefulness and qualifications to convert the concerning delay system into an equivalent one of fractional DVIEs. By employing the CMT, we have successfully demonstrated the prescribed requirements. For numerical analysis, the Galerkin algorithm was implemented by leveraging OSLPs as a base function. This algorithm allows us to estimate the solution to the concerning system by transforming it into a series of algebraic equations. By employing the software MATHEMATICA 11, we have effortlessly demonstrated the requirements estimation of the nodal values. One of the key advantages of the deployed algorithm is its ability to achieve accurate results with fewer iterations compared to alternative methods. To validate the effectiveness and precision of our analysis, we conducted a comprehensive evaluation through various linear and nonlinear numerical applications. The results of these tests, accompanied by figures and tables, further support the superiority of our algorithm. Finally, an analysis of the numerical algorithm employed was provided along with insightful suggestions for potential future research directions.

## 1 Foundations

Fractional calculus is a discipline within modern mathematics and computational physics that focuses on studying derivatives and integrals with fractional orders. While initially introduced by Leibniz in the late 17th century, it did not gain significant attention until the late 20th century. This can be partly attributed to advancements in computing technology during this period, as they facilitated efficient solutions to fractional models [[`ref01`]– [`ref03`]]. The applications of fractional patterns span various domains including engineering, signal processing, statistics, and others. For instance, fractional calculus finds utility in modeling the behavior of viscoelastic materials, controlling robotic systems, predicting financial markets, and modeling biological systems [[`ref04`]– [`ref07`]].

The DFDEs are a class of ordinary calculus that emphasizes fractional derivatives and delays. The scope of DFDEs is more comprehensive than traditional delay models, allowing for a broader range of phenomena to be represented. Applications of DFDEs appear in diverse domains involving signal processing, statistics, engineering, and biology where various simulation techniques are used [[`ref08`]– [`ref19`]]. For instance, in physics, DFDEs have been used to model heat transfer, diffusion, and wave propagation in materials with memory properties [[`ref20`], [`ref21`]]. In engineering, DFDEs have proven useful for modeling and controlling systems such as robots, aircraft, and power networks [[`ref07`], [`ref22`]– [`ref24`]]. Within biology, DFDEs have been employed to formulate agent-based representations of systems like the cardiovascular, nervous, and immune systems [[`ref25`], [`ref26`]].

The DFDE is an equation in which the derivative of a function, $\theta$$\beta$(?) with $\beta$ = 1,2, at a certain time, ?, involves values of the function, *$\theta$*$\beta$(?$-$$\sigma$$\beta$) with $\beta$ = 1,2, at a previous time, ?$-$$\sigma$$\beta$ with $\beta$ = 1,2. The simplest constant delay equation takes the form $\theta$$\beta$'(?)=S$\beta$(?,$\wp$$\beta$($\theta$1(?),$\theta$2(?)),$\theta$$\beta$(?$-$$\sigma$$\beta$)) with $\beta$ = 1,2, where *$\theta$*$\beta$(?$-$$\sigma$$\beta$) with $\beta$ = 1,2 representing the value of $\theta$$\beta$ with $\beta$ = 1,2 at a constant time $\sigma$$\beta$ with $\beta$ = 1,2 units in the past, making the effect *$\theta$*$\beta$ with $\beta$ = 1,2 on the current rate of change of $\theta$$\beta$ delayed by a time $\sigma$$\beta$ with $\beta$ = 1,2.

This paper presents the necessary conditions for the EUE together with numerical simulation concerning the resulting set of DFDEs




 HD0$\alpha$,$\beta$$\theta$$\beta$(?)=S$\beta$(?,$\wp$$\beta$($\theta$1(?),$\theta$2(?)),$\theta$$\beta$(?$-$$\sigma$$\beta$)),$\beta$=1,2,


(1)

accordance with the constraints




{$\theta$$\beta$(?)=$\varphi$$\beta$(?),$-$$\sigma$$\beta$<?≤0,I0+1$-$$\gamma$$\varphi$$\beta$(0+)=$\rho$$\beta$.


(2)

Herein, ?$\in$ *J*: = [0,$\mathcal{F}$] with $\mathcal{F}$>0,*$\alpha$*$\in$(0,1) and *$\beta$*$\in$[0,1] providing *$\alpha$*<*$\gamma$*<1 with $\gamma$=(1$-$$\beta$)($\alpha$$-$1)+1,$\rho$$\beta$$\in$R,$\varphi$$\beta$$\in$L1p(J),p$\in$(0,1$-$$\gamma$+$\alpha$2),$\sigma$$\beta$>0,S$\beta$$\in$C(J×R2→R),$\wp$$\beta$$\in$(R2→R), and HD0$\alpha$,$\beta$$\theta$$\beta$ relate to the FHD [[`ref27`]– [`ref31`]] of *$\theta$*$\beta$ of fraction *$\alpha$* and type *$\beta$*.

Our primary concern is to obtain a quantitative estimate of (1) through examination of the elegances of the SGLA, which depends on orthogonal spline basis functions [[`ref32`]– [`ref36`]]. Previous investigations have employed the presented scheme to approximate various differential or integral problems, a few of which will be mentioned. In [[`ref36`]], researchers successfully characterized the telegraph fraction model leveraging the SGLA. Similarly, in [[`ref35`]], the SGLA was utilized to find approximation of Fredholm fraction integrodifferential problems.

In this particular research, we adopted the parameterization of OSLPs to substitute the necessary background functions in (1). Subsequently, the SGLA was employed to convert (1) into sets of algebraic equations. Upon handling the producing set, the desired approximate solution was obtained. One notable numerical optimality of the SGLA employed is its feasibility for any model formalism. Furthermore, it enumerates significantly valid approximations by leveraging a minimal number of OSLP terms.

The organization of the computations and algorithm development are ordered as next. Section 2 presents spectrum attributes, and necessary lemmas based on the definitions of FRD, FCD, and FHD. In Section 3, we transform the system described in (1) and (2) subjected to a congruent set of DVIEs. Additionally, this section outlines the development of proof for the EUE concerning (1) and (2). In Section 4, we introduce the SGLA as a solution technique for tackling (1) and (2) and proceed to characterize the theorems on convergence. Section 5 provides numerical applications and results. Finally, a summary of key findings is mentioned in Section 6.

## 2 Background and overview results

This section presents indispensable background and properties associated with FRD, FCD, and FHD approaches, simultaneously, with significant outcomes that will be exploited in the next portion. Anyhow, consider the following requirements:

The Banach topological manifold *L*$^{P}$(*J*,$\mathbb{R}$) is axiomatized as the collection of each Lebesgue measurable S:J→R equipped with $\Vert$S$\Vert$Lp=($\int$J/S(?))pd?)1p<$\infty$.

The topological manifold AC(J,R)={S:J→R;SabsolutelycontinuousonJ}. Indeed, the topological manifold AC$\hbar$(J,R)={S:S$\in$C$\hbar$$-$1(J,R)andS$\hbar$$-$1$\in$AC(J,R)}.

The weighted topological manifold C$\theta$(J)={S:J→R;?$\theta$S(?)$\in$C(J,R)} with $\Vert$S$\Vert$C$\theta$=max?$\in$J?$\theta$/S(?)/.

**Definition 1.** [[`ref37`]] The left-side integral of fraction *$\theta$*>0 for S$\in$L1(J) is




(I0+$\theta$S)(?)=1$\Gamma$($\theta$)$\int$0?(?$-$s)$\theta$$-$1S(s)ds.


(3)

**Definition 2.** [[`ref37`]] If S(?)$\in$AC$\hbar$(J,R), then the left-side FRD of fraction *$\theta$*$\in$($\hbar$$-$1,$\hbar$) with $\hbar$$\in$$\mathbb{N}$ exists a.e. on *J* with




( LD0+$\theta$S)(?)=1$\Gamma$($\hbar$$-$$\theta$)d$\hbar$d?$\hbar$$\int$0?(?$-$s)$\hbar$$-$$\theta$$-$1S(s)ds.


(4)

**Remark 1.** If *$\theta$*>0 and $\theta$^>0, then




[I0+$\theta$s$\theta$^$-$1](?)=$\Gamma$($\theta$^)$\Gamma$($\theta$^+$\theta$)?$\theta$^+$\theta$$-$1.







[ LD0+$\theta$s$\theta$^$-$1](?)=$\Gamma$($\theta$^)$\Gamma$($\theta$^$-$$\theta$)?$\theta$^$-$$\theta$$-$1.







[ LD0+$\theta$s$\theta$$-$$\beta$](?)=0,$\beta$=1,2,$\cdots$,[$\theta$]+1.

**Definition 3.** [[`ref38`]] The FCD of fraction *$\theta$*$\in$($\hbar$$-$1,$\hbar$) with $\hbar$$\in$$\mathbb{N}$ is




 CD0$\theta$S(?)=1$\Gamma$($\hbar$$-$$\theta$)$\int$0?S($\hbar$)(s)(?$-$s)$\theta$+1$-$$\hbar$ds.


(5)

**Definition 4.** [[`ref27`]] Let D=dd? and *$\gamma$* = *$\alpha$*+$\hbar$*$\beta$*$-$*$\alpha$$\beta$*. The left-side FHD of fraction *$\alpha$*$\in$($\hbar$$-$1,$\hbar$) with $\hbar$$\in$$\mathbb{N}$ and *$\beta$*$\in$[0,1] of S is




( HD0+$\alpha$,$\beta$S)(?)=(I0+$\beta$($\hbar$$-$$\alpha$)D$\hbar$I0+(1$-$$\beta$)($\hbar$$-$$\alpha$)S)(?)=(I0+$\gamma$$-$$\alpha$D$\hbar$I0+$\hbar$$-$$\gamma$S)(?),?$\in$J.


(6)

The term HD0+$\alpha$,$\beta$ can be formatted as HD0+$\alpha$,$\beta$=I0+$\beta$($\hbar$$-$$\alpha$)D$\hbar$I0+(1$-$$\beta$)($\hbar$$-$$\alpha$)=I0+$\beta$($\hbar$$-$$\alpha$) LD0+$\gamma$=I0+($\gamma$$-$$\alpha$) LD0+$\gamma$. Indeed, if *$\beta$* = 0, then the left-side FRD is LD0+$\alpha$= HD0+$\alpha$,0 and if *$\beta$* = 1, then the left-side FCD is CD0+$\alpha$= HD0+$\alpha$,1.

**Lemma 1.** [[`ref28`]] The fundamental solution of HD0+$\alpha$,$\beta$S(?)=0 is S(?)=$\sum$$\beta$=1$\hbar$c$\beta$(?$-$0)$\alpha$+$\hbar$$\beta$$-$$\alpha$$\beta$$-$$\beta$ with ?>0,$\alpha$+$\hbar$$\beta$$-$$\alpha$$\beta$$\in$($\hbar$$-$1,$\hbar$), and c$\beta$$\in$R.

**Lemma 2.** [[`ref28`]] Let *$\rho$*$\in$(0,1), $\upsilon$$\in$($-$1,0], and *$\rho$*+*$\upsilon$*>0, then at 0<p<$\rho$+$\upsilon$2 and $\theta$$\in$L1p, one has (I0+$\rho$s$\upsilon$$\theta$(s))(?)$\in$AC(J,R).

**Lemma 3.** [[`ref28`]] Let *$\theta$*$\in$($\hbar$$-$1,$\hbar$) with $\hbar$$\in$$\mathbb{N}$. If *$\theta$*$\in$*L*$^{1}$(*J*) and I0+$\hbar$$-$$\theta$$\theta$$\in$AC$\hbar$(J), then




(I0+$\theta$ LD0+$\theta$$\theta$)(?)=$\theta$(?)$-$$\sum$$\beta$=1$\hbar$(I0+$\hbar$$-$$\theta$$\theta$)($\hbar$$-$$\beta$)(0+)$\Gamma$($\theta$$-$$\beta$+1)?$\theta$$-$$\beta$.


(7)

**Lemma 4.** [[`ref31`]] Let *$\theta$*$\in$*L*$^{1}$(*J*) and LD0+$\gamma$$-$$\alpha$$\theta$$\in$L1(J) exists. Then ( HD0+$\alpha$,$\beta$I0+$\alpha$$\theta$)(?)=I0+$\gamma$$-$$\alpha$ LD0+$\gamma$$-$$\alpha$$\theta$(?).

**Lemma 5.** [[`ref31`]] Let *$\alpha$*>0 and 0≤*$\gamma$*<1. Then I0+$\alpha$ is bounded from *C*$\gamma$(*J*) into *C*$\gamma$(*J*).

Here, we define the following topological manifold: C1$-$$\gamma$$\alpha$,$\beta$(J)={S$\in$C1$-$$\gamma$(J);( HD0+$\alpha$,$\beta$S)(?)$\in$C1$-$$\gamma$(J)} and C1$-$$\gamma$$\gamma$(J)={S$\in$C1$-$$\gamma$(J);( LD0+$\gamma$S)(?)$\in$C1$-$$\gamma$(J)}. Note that HD0+$\alpha$,$\beta$$\wp$=I0+$\beta$(1$-$$\alpha$) LD0+$\gamma$$\wp$, so, by Lemma 6 C1$-$$\gamma$$\gamma$(J)$\subset$C1$-$$\gamma$$\alpha$,$\beta$(J).

**Lemma 6.** [[`ref31`]] Let $\wp$$\in$L1(0,c) and *$\theta$*>0. Then




lim?→c+$\int$0c(?$-$s)$\theta$$-$1$\wp$(s)ds=$\int$0c(c$-$s)$\theta$$-$1$\wp$(s)ds=$\Gamma$($\theta$)I0+$\theta$$\wp$(c).


(8)

**Lemma 7.** [[`ref31`]] Let $\theta$$\in$(0,1),0<c<F,$\wp$$\in$C$\theta$[0,c], $\wp$$\in$C[c,F], and $\wp$ is continuous at c. Then $\wp$$\in$*C*$\theta$(*J*).

## 3 Congruence fractional DVIE and the results concerning EUE

In this section, we investigate the examination of the possessions of the FRD, FCD, and FHD utilizing their forcefulness and qualifications to convert the concerning delay system into an equivalent one of fractional DVIEs. After that, we visualize and clarify the EUE concerning (1) and (2) leveraging its congruence system of fractional DVIEs (9) within C1$-$$\gamma$$\gamma$(J).

**Theorem 1.** Let 0<p<1$-$$\gamma$+$\alpha$2,$\wp$$\in$C1$-$$\gamma$(R,R),$\theta$1,$\theta$2$\in$C1$-$$\gamma$(J,R), and S1,S2:J×R2×→R be as S1($\cdot$,$\wp$($\theta$($\cdot$)),$\theta$1($\cdot$$-$$\sigma$1))$\in$L1p(J,R,R) and S2($\cdot$,$\wp$($\theta$($\cdot$)),$\theta$2($\cdot$$-$$\sigma$2))$\in$L1p(J,R,R). Then *$\theta$*(?) fulfills Problem (1) and (2) iff *$\theta$*(?) fulfills




$\theta$$\beta$(?)=$\rho$$\beta$?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?(?$-$s)$\alpha$$-$1S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$))$\Gamma$($\alpha$)ds,$\beta$=1,2.


(9)

**Proof.** For ?$\in$*J* and $\gamma$$-$$\alpha$$\in$(0,1$-$$\alpha$) it suffices from Lemma 2 at $\rho$=1$-$($\gamma$$-$$\alpha$) and $\upsilon$=0 that 1$-$($\gamma$$-$$\alpha$)>0,0<p<1$-$($\gamma$$-$$\alpha$)2, and S1,S2$\in$L1p. So I0+1$-$($\gamma$$-$$\alpha$)S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$))(?)$\in$AC(J×R2,R) and LD0+$\alpha$S$\beta$$\in$L1(J×R3).

From Lemma 3, take *$\theta$* = *$\gamma$*$-$*$\alpha$*, one gained




(I0+$\gamma$$-$$\alpha$ LD0+$\gamma$$-$$\alpha$S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$)))(?)=S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$))$-$(I0+1$-$($\gamma$$-$$\alpha$)S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$))(0+)$\Gamma$($\gamma$$-$$\alpha$)?$\gamma$$-$$\alpha$$-$1.


(10)

From Lemma 4, one concludes




I0+$\gamma$$-$$\alpha$ LD0+$\gamma$$-$$\alpha$S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$))(?)= HD0+$\alpha$,$\beta$I0+$\alpha$(S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$)))(?).


(11)

Then




 HD0+$\alpha$,$\beta$($\theta$$\beta$(?)$-$I0+$\alpha$S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$))(?))=S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$))$-$S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$))+(I0+1$-$($\gamma$$-$$\alpha$)S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$)))(0+)$\Gamma$($\gamma$$-$$\alpha$)?$\gamma$$-$$\alpha$$-$1.


(12)

Since (I0+1$-$($\gamma$$-$$\alpha$)S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$)))(0+)=0. So, HD0+$\alpha$,$\beta$($\theta$$\beta$(?)$-$(I0+$\alpha$S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$)))(?))=0. Using Lemma 1, one concludes whenever c$\beta$$\in$R that




$\theta$$\beta$(?)=c$\beta$?$\gamma$$-$1+(I0+$\alpha$S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$)))(?),?>0.


(13)

Using $\theta$$\beta$(?)=$\varphi$$\beta$(?) with $-$$\sigma$$\beta$<?≤0 such that I0+1$-$$\gamma$$\varphi$$\beta$(0+)=$\rho$$\beta$ and I0+1$-$$\gamma$$\theta$$\beta$(0+)=c$\beta$$\Gamma$($\gamma$)+0,$-$$\sigma$$\beta$≤?≤0, we have c$\beta$$\Gamma$($\gamma$)=$\rho$$\beta$. So the fractional DVIEs are




$\theta$$\beta$(?)=$\rho$$\beta$?$\gamma$$-$1$\Gamma$($\gamma$)+(I0+$\alpha$S$\beta$(s,$\wp$$\beta$($\theta$(s)),$\theta$$\beta$(s$-$$\sigma$$\beta$)))(?),$\beta$=1,2.


(14)

In the consecutive results, $\theta$(?)=($\theta$1(?),$\theta$2(?)),$\theta$~(?)=($\theta$~1(?),$\theta$~2(?)), HD0$\alpha$,$\beta$$\theta$(?)=( HD0$\alpha$,$\beta$$\theta$1(?), HD0$\alpha$,$\beta$$\theta$2(?)), and $\theta$(?$-$$\theta$)=($\theta$1(?$-$$\sigma$1),$\theta$2(?$-$$\sigma$2))with *$\theta$* = (*$\sigma$*$_{1}$, *$\sigma$*$_{2}$). Indeed, $\wp$($\theta$(?))=($\wp$1($\theta$(?)),$\wp$2($\theta$(?))) and S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))=(S1(?,$\wp$1($\theta$(?)),$\theta$1(?$-$$\sigma$1)),S2(?,$\wp$2($\theta$(?)),$\theta$2(?$-$$\sigma$2))).

**Theorem 2.** For any $\theta$:J→R2,$\wp$$\in$C1$-$$\gamma$(R2,R), and $\theta$(?$-$$\theta$)$\in$C1$-$$\gamma$(J), if S fulfills the Lipschitzian




$\Vert$S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))$\Vert$C1$-$$\gamma$(J)≤L($\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C1$-$$\gamma$(J)+$\Vert$$\theta$(?$-$$\theta$)$-$$\theta$~(?$-$$\theta$)$\Vert$C1$-$$\gamma$(J)),


(15)

then $\exists$!-Sol for (1) and (2) in C1$-$$\gamma$$\gamma$(J).

**Proof.** Initially, we prove the EUE within *C*1$-$$\gamma$(*J*). Our demonstration relies on the examination of three scenarios: the first encompasses ?$\in$(0,*$\sigma$*$_{1}$], the second encompasses ?$\in$(*$\sigma$*$_{1}$, *$\sigma$*$_{2}$], and the third one encompasses ?$\in$(*$\sigma$*$_{2}$,$\mathcal{F}$]. In each scenario, we divide the *J* into subintervals and validate *G*: *S*→*S* defined as




G$\theta$(?)=(G$\theta$1(?),G$\theta$2(?))


(16)





=($\rho$1?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?(?$-$s)$\alpha$$-$1S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1))$\Gamma$($\alpha$)ds,$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?(?$-$s)$\alpha$$-$1S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2))$\Gamma$($\alpha$)ds),



where S={$\theta$$\in$C1$-$$\gamma$(J)} is a contraction mapping.

Assuming that $\sigma$1≤$\sigma$2. Then we consider three cases as follows:

**Case 1.** Let ?$\in$(0,*$\sigma$*$_{1}$], so $\theta$(?$-$$\theta$)=$\theta$~(?$-$$\theta$)=$\varphi$(?$-$$\theta$)=($\varphi$1(?$-$$\sigma$1),$\varphi$2(?$-$$\sigma$2)) and choose c1,c2 such that 0<c1≤c2≤$\sigma$1.ThemetricspaceC1$-$$\gamma$[c1,c2] is considered complete with




d($\theta$(?),$\theta$~(?))=$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C1$-$$\gamma$[c1,c2]=max?$\in$[c1,c2]$\Vert$?1$-$$\gamma$($\theta$(?)$-$$\theta$~(?))$\Vert$.


(17)

Select ?1$\in$(0,$\sigma$1] and note that $\rho$1$\Gamma$($\gamma$)?$\gamma$$-$1,$\rho$2$\Gamma$($\gamma$)?$\gamma$$-$1$\in$C1$-$$\gamma$[0,?1], so by Lemma 5 due to G$\theta$$\in$C1$-$$\gamma$[0,?1], thus G maps C1$-$$\gamma$[0,?1] into itself. Now, $\forall$$\theta$(?),$\theta$~(?)$\in$C1$-$$\gamma$[0,?1], we have




$\Vert$G$\theta$(?)$-$G$\theta$~(?)$\Vert$C1$-$$\gamma$[0,?1]≤/($\rho$1$-$$\rho$2)$\Gamma$($\gamma$)/$\Vert$I0+$\alpha$/S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))/$\Vert$C1$-$$\gamma$[0,?1].


(18)

Note that




$\Vert$S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))$\Vert$C1$-$$\gamma$[0,?1]=max?$\in$[0,?1]$\Vert$?1$-$$\gamma$(S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$)))$\Vert$.


(19)

So,




/S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$)$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))/≤?$\gamma$$-$1$\Vert$S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))$\Vert$C1$-$$\gamma$[0,?1].


(20)





$\Vert$G$\theta$(?)$-$G$\theta$~(?)$\Vert$C1$-$$\gamma$[0,?1]







≤/$\rho$1$-$$\rho$2/$\Gamma$($\gamma$)$\Gamma$($\gamma$)$\Gamma$($\gamma$+$\alpha$)?1$\gamma$+$\alpha$$-$1L($\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C1$-$$\gamma$[0,?1]+$\Vert$$\theta$(?$-$$\theta$)$-$$\theta$~(?$-$$\theta$)$\Vert$C1$-$$\gamma$[0,?1])







=/$\rho$1$-$$\rho$2/$\Gamma$($\gamma$+$\alpha$)?1$\gamma$+$\alpha$$-$1L($\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C1$-$$\gamma$[0,?1]+$\Vert$$\varphi$(?$-$$\theta$)$-$$\varphi$(?$-$$\theta$)$\Vert$C1$-$$\gamma$[0,?1])







=$\mu$1$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C1$-$$\gamma$[0,?1].


(21)

where $\mu$1=L/$\rho$1$-$$\rho$2/$\Gamma$($\gamma$+$\alpha$)?1$\gamma$+$\alpha$$-$1<1.

Given that *$\mu$*$_{1}$<1, the CMT guarantees the existence of a single fixed point, denoted as $\theta$0(?)$\in$C1$-$$\gamma$[0,?1] within (0,?$_{1}$]. In the case where ?$_{1}$$\neq$*$\sigma$*$_{1}$, we examine [?$_{1}$, *$\sigma$*$_{1}$]. It is important to note that




G$\theta$(?)=($\rho$1?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?(?$-$s)$\alpha$$-$1S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1))$\Gamma$($\alpha$)ds,$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?(?$-$s)$\alpha$$-$1S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2))$\Gamma$($\alpha$)ds)


(22)





=($\rho$1?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?1(?$-$s)$\alpha$$-$1S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1))$\Gamma$($\alpha$)ds+(I?1$\alpha$S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1)))(?),$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?1(?$-$s)$\alpha$$-$1S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2))$\Gamma$($\alpha$)ds+(I?1$\alpha$S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2)))(?)).

Let $\theta$(?),$\theta$~(?)$\in$C[?1,?2] for some ?1<?2≤$\sigma$1, we see that




$\Vert$G$\theta$(?)$-$G$\theta$~(?)$\Vert$C[?1,?2]







=$\Vert$?$\gamma$$-$1($\rho$1$-$$\rho$2)$\Gamma$($\gamma$)(I?1$\alpha$S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$I?1$\alpha$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$)))$\Vert$C[?1,?2]







≤/($\rho$1$-$$\rho$2)/$\Gamma$($\gamma$)$\Gamma$($\gamma$)(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\gamma$+$\alpha$)$\Vert$S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))$\Vert$C[?1,?2]







≤L/$\rho$1$-$$\rho$2/(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\gamma$+$\alpha$)($\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[?1,?2]+$\Vert$$\theta$(?$-$$\theta$)$-$$\theta$~(?$-$$\theta$)$\Vert$C[?1,?2])







=L/$\rho$1$-$$\rho$2/(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\gamma$+$\alpha$)($\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[?1,?2]+$\Vert$$\varphi$(?$-$$\theta$)$-$$\varphi$(?$-$$\theta$)$\Vert$C[?1,?2])







=L/$\rho$1$-$$\rho$2/(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\gamma$+$\alpha$)$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[?1,?2]







=$\mu$2$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[?1,?2],


(23)

where $\mu$2=L/$\rho$1$-$$\rho$2/(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\gamma$+$\alpha$)<1. It results that *G* is a contraction on [?$_{1}$, ?$_{2}$]. Consequently, $\exists$!-Sol *$\theta$*$_{1}$(?) for ?$\in$[?$_{1}$, ?$_{2}$]. According to Lemma 6, it is evident that $\theta$0(?1)=$\theta$1(?1). Thus,




$\theta$(?)={$\theta$0(?),0<?≤?1,$\theta$1(?),?1<?≤?2.


(24)

According to Lemma 7; $\theta$(?)$\in$C1$-$$\gamma$[0,?2]. Additionally, *$\theta$*(?) represents the only solution to (1) and (2) within [0,?$_{2}$]. In the case where ?$_{2}$$\neq$*$\sigma$*$_{1}$, we repeat the necessary steps. Let's assume that *N*$-$2 additional steps are required. Following that, we determine a! -Sol *$\theta$*$_{k}$(?) for ? within [?k,?k+1], where *k* takes on values *k* = 2,3,$\cdots$,*N*. Here, we have 0=?0<?1<$\cdots$<?N=$\sigma$1 satisfying the condition that




$\mu$k+1=L/$\rho$1$-$$\rho$2/$\Gamma$($\gamma$+$\alpha$)(?k+1$-$?k)$\gamma$+$\alpha$$-$1<1.


(25)

We possess a distinctive solution $\theta$(?)$\in$C1$-$$\gamma$[0,$\sigma$1] ensuring its uniqueness as




$\theta$(?)={$\theta$0(?),0<?<?1,$\theta$1(?),?1<?≤?2,$\vdots$$\theta$N(?),?N$-$1<?<$\sigma$1.


(26)

Since $\theta$(?)$\in$C1$-$$\gamma$[0,$\tau$] is the! -Sol, so it fulfills (9) and




$\theta$(?)=($\rho$1?$\gamma$$-$1$\Gamma$($\gamma$)+(I0+$\alpha$S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1)))(?),$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)+(I0+$\alpha$S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2)))(?).)


(27)

Applying LD0+$\gamma$ to both sides results in the following:




 LD0+$\gamma$$\theta$(?)=( LD0+$\gamma$$\rho$1?$\gamma$$-$1$\Gamma$($\gamma$)+ LD0+$\gamma$(I0+$\alpha$S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1)))(?), LD0+$\gamma$$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)+ LD0+$\gamma$(I0+$\alpha$S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2)))(?))







=(( LD0+$\gamma$$-$$\alpha$S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1)))(?),( LD0+$\gamma$$-$$\alpha$S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2)))(?))







=(( LD0+$\beta$(1$-$$\alpha$)S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1)))(?),( LD0+$\beta$(1$-$$\alpha$)S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2)))(?)).


(28)

It is evident that the RHS belongs to *C*1$-$$\gamma$[0, *$\sigma$*$_{1}$]. Therefore, LD0+$\gamma$$\theta$(?)$\in$C1$-$$\gamma$[0,$\sigma$1]. Consequently, according to (9); $\theta$(?)$\in$C1$-$$\gamma$$\gamma$[0,$\sigma$1] and this is the only solution in [0,$\sigma$1].

**Case 2.** Assuming that ?$\in$ (*$\sigma$*$_{1}$,*$\sigma$*$_{2}$] and $\theta$(?),$\theta$~(?)$\in$C1$-$$\gamma$[$\sigma$1,$\sigma$2], we can divide [0, *$\sigma$*$_{1}$] into [0,$\sigma$1]$\cup$$\cdots$$\cup$[(k0$-$1)$\sigma$1,k0$\sigma$1]$\cup$[k0$\sigma$1,$\sigma$2]. Here, *k*$_{0}$ is a natural number and fulfills 0≤$\sigma$2$-$k0$\sigma$1≤$\sigma$2. In a previous case (Case 1), we have already proven (1) and (2) have a! -Sol on [0,*$\sigma$*$_{1}$], denoted as $\theta$0*(?). Now, let's assume that (1) and (2) also has a! -Sol on [*$\sigma$*$_{1}$, *k$\sigma$*$_{1}$], where 1≤*k*≤*k*$_{0}$, denoted as $\theta$k*(?). Our goal is to axiomatize the existence of a! -Sol $\theta$k+1*(?) on [k$\sigma$1,(k+1)$\sigma$1]. Anyhow, $\forall$?$\in$[k$\sigma$1,(k+1)$\sigma$1] define




G$\theta$(?)=($\rho$1?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?(?$-$s)$\alpha$$-$1S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1))$\Gamma$($\alpha$)ds,$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?(?$-$s)$\alpha$$-$1S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2))$\Gamma$($\alpha$)ds).


(29)

Assume that G is a contraction for ?$\in$[*$\sigma$*$_{1}$, *k$\sigma$*$_{1}$] such that $\Vert$G$\theta$(?)$-$G$\theta$~(?)$\Vert$C[$\sigma$1,k$\sigma$1]≤$\mu$k$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[$\sigma$1,k$\sigma$1]. Therefore, $\exists$!-Sol *$\varphi$*$_{k}$(?) for all ?$\in$[*$\sigma$*$_{1}$, *k$\sigma$*$_{1}$]. Now, consider ?$\in$[k$\sigma$1,(k+1)$\sigma$1]. In this case, $\theta$(?$-$$\sigma$1)=$\theta$~(?$-$$\sigma$1)=$\varphi$k(?$-$$\sigma$1). Choose two values c1,c2 such that $\sigma$1<c1≤c2≤k$\sigma$1. This entails that the metric space C1$-$$\gamma$[c1,c2] is complete with




d($\theta$(?),$\theta$~(?))=$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C1$-$$\gamma$[c1,c2]=max?$\in$[c1,c2]$\Vert$?1$-$$\gamma$($\theta$(?)$-$$\theta$~(?))$\Vert$.


(30)

Select ?1$\in$(k$\sigma$1,(k+1)$\sigma$1] such that




$\mu$1k=/$\rho$1$-$$\rho$2/L$\Gamma$($\gamma$+$\alpha$)?1$\gamma$+$\alpha$$-$1<1.


(31)

Note that $\rho$1?$\gamma$$-$1$\Gamma$($\gamma$),$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)$\in$C1$-$$\gamma$[k$\sigma$1,?1], so by Lemma 5 due to G$\theta$$\in$C1$-$$\gamma$[k$\sigma$1,?1]; *G* maps C1$-$$\gamma$[k$\sigma$1,?1] into itself. Now, $\forall$$\theta$(?),$\theta$~(?)$\in$C1$-$$\gamma$[k$\sigma$1,?1], we have




$\Vert$G$\theta$(?)$-$G$\theta$~(?)$\Vert$C1$-$$\gamma$[k$\sigma$1,?1]≤/$\rho$1$-$$\rho$2/$\Gamma$($\gamma$)$\Vert$I0+$\alpha$/S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\sigma$1))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\sigma$1))/$\Vert$C1$-$$\gamma$[k$\sigma$1,?1].


(32)

Note that




$\Vert$S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\sigma$1))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\sigma$1))$\Vert$C1$-$$\gamma$[k$\sigma$1,?1]=max?$\in$[k$\sigma$1,?1]$\Vert$?1$-$$\gamma$S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\sigma$1))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\sigma$1))$\Vert$.


(33)

Similarly, by doing the same procedure as in Case 1, we can find that




$\Vert$G$\theta$(?)$-$G$\theta$~(?)$\Vert$C1$-$$\gamma$[k$\sigma$1,?1]≤/$\rho$1$-$$\rho$2/$\Gamma$($\gamma$+$\alpha$)?1$\gamma$+$\alpha$$-$1L$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C1$-$$\gamma$[k$\sigma$1,?1]







=$\mu$1k$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C1$-$$\gamma$[k$\sigma$1,?1],


(34)

where $\mu$1k=/$\rho$1$-$$\rho$2/L$\Gamma$($\gamma$+$\alpha$)?1$\gamma$+$\alpha$$-$1<1.

According to the CMT, there exists a single fixed point that serves as the solution $\theta$0(?)$\in$C1$-$$\gamma$[k$\sigma$1,?1] within (k$\sigma$1,?1]. In the case where ?1$\neq$(k+1)$\sigma$1, we will focus on [?1,(k+1)$\sigma$1]. It is important to note that




G$\theta$(?)=($\rho$1?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?1(?$-$s)$\alpha$$-$1S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1))$\Gamma$($\alpha$)ds+(I?1$\alpha$S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1)))(?),$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?1(?$-$s)$\alpha$$-$1S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2))$\Gamma$($\alpha$)ds+(I?1$\alpha$S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2)))(?)).


(35)

Let $\theta$(?),$\theta$~(?)$\in$C[?1,?2] for some ?1<?2≤(k+1)$\sigma$1, we can see that




$\Vert$G$\theta$(?)$-$G$\theta$~(?)$\Vert$C[?1,?2]







≤/($\rho$1$-$$\rho$2)$\Gamma$($\gamma$)/$\Vert$I?1+$\alpha$/S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))/$\Vert$C[?1,?2]







≤L/$\rho$1$-$$\rho$2/(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\gamma$+$\alpha$)($\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[?1,?2]+$\Vert$$\theta$(?$-$$\theta$)$-$$\theta$~(?$-$$\theta$)$\Vert$C[?1,?2])







=L/$\rho$1$-$$\rho$2/(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\gamma$+$\alpha$)($\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[?1,?2]+$\Vert$$\varphi$k(?$-$$\theta$)$-$$\varphi$k(?$-$$\theta$)$\Vert$C[?1,?2])







=$\mu$2k$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[?1,?2].


(36)

such that $\mu$2k=L/$\rho$1$-$$\rho$2/(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\gamma$+$\alpha$)<1, it results that *G* is a contraction on [?$_{1}$,?$_{2}$].

Consequently, $\exists$!-Sol *$\theta$*$_{1}$(?) for ?$\in$[?$_{1}$,?$_{2}$]. According to Lemma 6, it can be observed that $\theta$0(?1)=$\theta$1(?1). Therefore,




$\theta$(?)={$\theta$0(?),0<?≤?1,$\theta$1(?),?1<?≤?2.


(37)

Using Lemma 7, $\theta$(?)$\in$C1$-$$\gamma$[k$\sigma$1,?2]. Therefore, *$\theta$*(?) is the only solution of (1) and (2) over [k$\sigma$1,?2]. If ?2$\neq$(k+1)$\sigma$1, we proceed with the steps as required. Let's presume that we require *N*$-$2 additional steps. Following, we discover the distinct solution *$\theta$*$_{k}$(?) for ?$\in$[?k,?k+1],k=2,3,$\cdots$,N, where k$\sigma$1=?0<?1<$\cdots$<?N=(k+1)$\sigma$1 such that




$\mu$k+1=L/$\rho$1$-$$\rho$2/$\Gamma$($\gamma$+$\alpha$)(?k+1$-$?k)$\gamma$+$\alpha$$-$1<1.


(38)

We possess the exclusive solution $\theta$(?)$\in$C1$-$$\gamma$[k$\sigma$1,(k+1)$\sigma$1] such that




$\theta$(?)={$\theta$0(?),k$\sigma$1<?<?1,$\theta$1(?),?1<?≤?2,$\vdots$$\theta$N(?),?N$-$1<?<(k+1)$\sigma$1.


(39)

Since $\theta$(?)$\in$C1$-$$\gamma$[k$\sigma$1,(k+1)$\sigma$1] is the! -Sol, so it fulfills (9) and




$\theta$(?)=($\rho$1?$\gamma$$-$1$\Gamma$($\gamma$)+(Ik$\tau$$\alpha$S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1)))(?),$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)+(Ik$\tau$$\alpha$S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2)))(?)).


(40)

Applying LDk$\sigma$1+$\gamma$ to (40) gives




 LDk$\tau$+$\gamma$$\theta$(?)=(( LDk$\sigma$1+$\beta$(1$-$$\alpha$)S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1)))(?),( LDk$\sigma$1+$\beta$(1$-$$\alpha$)S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2)))(?)).


(41)

It is observed that the RHS lies within the range of C1$-$$\gamma$[k$\sigma$1,(k+1)$\sigma$1] which entails that LDk$\sigma$1+$\gamma$$\theta$(?)$\in$C1$-$$\gamma$[k$\sigma$1,(k+1)$\sigma$1]. Consequently, according to (9); $\theta$(?)$\in$C1$-$$\gamma$$\gamma$[k$\sigma$1,(k+1)$\sigma$1] and this is the only solution for [k$\sigma$1,(k+1)$\sigma$1]. By applying induction, a! -Sol exists for [*$\sigma$*$_{1}$, *$\sigma$*$_{2}$].

**Case 3.** When ?$\in$($\sigma$2,F] and $\theta$(?),$\theta$~(?)$\in$C1$-$$\gamma$[$\sigma$2,F] we can divide [0,$\mathcal{F}$] into [0,$\sigma$2]$\cup$$\cdots$$\cup$[(k0$-$1)$\sigma$2,k0$\sigma$2]$\cup$[k0$\sigma$2,F], where *k*$_{0}$$\in$$\mathbb{N}$ and fulfills 0≤F$-$k0$\sigma$2≤F. In Case 1, we have already proven that (1) and (2) have a! -Sol on [0, *$\sigma$*$_{2}$], denoted as $\theta$0*(?). Now, let's assume that (1) and (2) have a! -Sol on [*$\sigma$*$_{2}$, *k$\sigma$*$_{2}$], where 1≤*k*≤*k*$_{0}$ say $\theta$k*(?). We aim to ensure the existence of a! -Sol $\theta$k+1*(?) on [k$\sigma$2,(k+1)$\sigma$2]. Consider ?$\in$[k$\sigma$2,(k+1)$\sigma$2] and




G$\theta$(?)=($\rho$1?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?(?$-$s)$\alpha$$-$1S1(s,$\wp$1($\theta$(s)v),$\theta$1(s$-$$\sigma$1))$\Gamma$($\alpha$)ds,$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?(?$-$s)$\alpha$$-$1S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2))$\Gamma$($\alpha$)ds).


(42)

Suppose *G* is a contraction for ? in [*$\sigma$*$_{2}$, *k$\sigma$*$_{2}$] such that $\Vert$G$\theta$(?)$-$G$\theta$~(?)$\Vert$C[$\sigma$2,k$\sigma$2]≤$\mu$k$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[$\sigma$2,k$\sigma$2]. This entails that $\exists$!-Sol $\varphi$k(?)=($\varphi$1k(?),$\varphi$2k(?)) for all ?$\in$[$\sigma$2,k$\sigma$2]. Consider ?$\in$[k$\sigma$2,(k+1)$\sigma$2], so, $\theta$(?$-$$\sigma$2)=$\theta$~(?$-$$\sigma$2)=$\varphi$k(?$-$$\sigma$2)=($\varphi$1k(?$-$$\sigma$2),$\varphi$2k(?$-$$\sigma$2)). Let c1 and c2 be chosen such that $\sigma$<c1≤c2≤k$\sigma$2. Then, C1$-$$\gamma$[c1,c2] equipped




d($\theta$(?),$\theta$~(?))=$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C1$-$$\gamma$[c1,c2]=max?$\in$[c1,c2]$\Vert$?1$-$$\gamma$($\theta$(?)$-$$\theta$~(?))$\Vert$.


(43)

Select ?1$\in$(k$\sigma$2,(k+1)$\sigma$2] and due to $\rho$1?$\gamma$$-$1$\Gamma$($\gamma$),$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)$\in$C1$-$$\gamma$[k$\sigma$2,?1], so leveraging Lemma 5 due to G$\theta$$\in$C1$-$$\gamma$[k$\sigma$2,?1]; G maps C1$-$$\gamma$[k$\sigma$2,?1] into itself. Therefore, $\forall$$\theta$(?),$\theta$~(?)$\in$C1$-$$\gamma$[k$\sigma$2,?1], we have




$\Vert$G$\theta$(?)$-$G$\theta$~(?)$\Vert$C1$-$$\gamma$[k$\sigma$2,?1]=$\Vert$I0+$\alpha$S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$I0+$\alpha$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))$\Vert$C1$-$$\gamma$[k$\sigma$2,?1]







≤$\Vert$I0+$\alpha$/S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))/$\Vert$C1$-$$\gamma$[k$\sigma$2,?1].


(44)

Note that




$\Vert$S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))$\Vert$C1$-$$\gamma$[k$\sigma$2,?1]=max?$\in$[k$\sigma$2,?1]$\Vert$?1$-$$\gamma$S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))$\Vert$.


(45)

Similarly, by applying the same procedure in Cases 1 and 2, one finds




$\Vert$G$\theta$(?)$-$G$\theta$~(?)$\Vert$C1$-$$\gamma$[k$\sigma$2,?1]≤/$\rho$1$-$$\rho$2/$\Gamma$($\gamma$+$\alpha$)?1$\gamma$+$\alpha$$-$1L$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C1$-$$\gamma$[k$\sigma$2,?1]=$\mu$1k$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C1$-$$\gamma$[k$\sigma$2,?1],


(46)

where $\mu$1k=L/$\rho$1$-$$\rho$2/$\Gamma$($\gamma$+$\alpha$)?1$\gamma$+$\alpha$$-$1<1.

According to the CMT, there exists a single fixed point that serves as the solution $\theta$0(?)$\in$C1$-$$\gamma$[k$\sigma$2,?1] within (*k$\sigma$*, ?$_{1}$]. In the case where ?1$\neq$(k+1)$\sigma$2 we examine [?1,(k+1)$\sigma$2]. It is important to note that




G$\theta$(?)=($\rho$1?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?1(?$-$s)$\alpha$$-$1S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1))$\Gamma$($\alpha$)ds+(I?1$\alpha$S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1)))(?),$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)+$\int$0?1(?$-$s)$\alpha$$-$1S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2))$\Gamma$($\alpha$)ds+(I?1$\alpha$S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2)))(?)).


(47)

Let $\theta$(?),$\theta$~(?)$\in$C[?1,?2] for some ?1<?2≤(k+1)$\sigma$2, we see that




$\Vert$G$\theta$(?)$-$G$\theta$~(?)$\Vert$C[?1,?2]







≤/$\rho$1$-$$\rho$2/$\Gamma$($\gamma$)$\Gamma$($\gamma$)(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\alpha$+$\gamma$)$\Vert$S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))$-$S(?,$\wp$($\theta$~(?)),$\theta$~(?$-$$\theta$))$\Vert$C[?1,?2]







≤L/$\rho$1$-$$\rho$2/(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\alpha$+$\gamma$)($\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[?1,?2)+$\Vert$$\theta$(?$-$$\theta$)$-$$\theta$~(?$-$$\theta$)$\Vert$C[?1,?2])







=L/$\rho$1$-$$\rho$2/(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\alpha$+$\gamma$)($\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[?1,?2]+$\Vert$$\varphi$k(?$-$$\theta$)$-$$\varphi$k(?$-$$\theta$)$\Vert$C[?1,?2])







=$\mu$2k$\Vert$$\theta$(?)$-$$\theta$~(?)$\Vert$C[?1,?2].


(48)

where $\mu$2k=L/$\rho$1$-$$\rho$2/(?2$-$?1)$\gamma$+$\alpha$$-$1$\Gamma$($\alpha$+$\gamma$)<1. It results that *G* is a contraction over [?$_{1}$,?$_{2}$], and there exists a single solution *$\theta$*$_{1}$(?) for ?$\in$[?$_{1}$,?$_{2}$]. As per Lemma 6, it can be observed that *$\theta$*$_{0}$(?$_{1}$) = *$\theta$*$_{1}$(?$_{1}$). Consequently, the function *$\theta$*$_{1}$(?) can be expressed as




$\theta$(?)={$\theta$0(?),0<?≤?1,$\theta$1(?),?1<?≤?2.


(49)

Furthermore, according to Lemma 7; $\theta$(?)$\in$C1$-$$\gamma$[k$\sigma$2,?2]. Therefore, *$\theta$*(?) represents the! -Sol to (1) and (2) within [*k$\sigma$*, ?$_{2}$]. If ?2$\neq$(k+1)$\sigma$2 the aforementioned steps can be repeated as necessary. Assuming that *N*$-$2 additional steps are required; the! -Sol *$\theta$*$_{k}$(?) can be determined for ?$\in$[?k,?k+1], where *k* = 2,3,…,*N*. Here, k$\sigma$2=?0<?1<$\cdots$<?N=(k+1)$\sigma$2 ensuring the resulting conditions are satisfied




$\mu$k+1=L/$\rho$1$-$$\rho$2/$\Gamma$($\gamma$+$\alpha$)(?k+1$-$?k)$\gamma$+$\alpha$$-$1<1.


(50)

We possess a! -Sol $\theta$(?)$\in$C1$-$$\gamma$[k$\sigma$2,(k+1)$\sigma$2] which can be defined as




$\theta$(?)={$\theta$0(?),k$\sigma$2<?<?1,$\theta$1(?),?1<?≤?2,$\vdots$$\theta$N(?),?N$-$1<?<(k+1)$\sigma$2.


(51)

Since $\theta$(?)$\in$C1$-$$\gamma$[k$\sigma$2,(k+1)$\sigma$2] is the! -Sol, it fulfills (9) and




$\theta$(?)=($\rho$1?$\gamma$$-$1$\Gamma$($\gamma$)+(Ik$\sigma$2$\alpha$S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1)))(?),$\rho$2?$\gamma$$-$1$\Gamma$($\gamma$)+(Ik$\sigma$2$\alpha$S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2)))(?)).


(52)

Applying LDk$\sigma$2+$\gamma$ to both sides of (52) gives




 LDk$\sigma$+$\gamma$$\theta$(?)=(( LDk$\sigma$2+$\beta$(1$-$$\alpha$)S1(s,$\wp$1($\theta$(s)),$\theta$1(s$-$$\sigma$1)))(?),( LDk$\sigma$2+$\beta$(1$-$$\alpha$)S2(s,$\wp$2($\theta$(s)),$\theta$2(s$-$$\sigma$2)))(?)).


(53)

We can see that the RHS is in *C*1$-$$\gamma$. Consequently, LDk$\sigma$2+$\gamma$$\theta$(?)$\in$C1$-$$\gamma$[k$\sigma$2,(k+1)$\sigma$2], so, based on (9); $\theta$(?)$\in$C1$-$$\gamma$$\gamma$[k$\sigma$2,(k+1)$\sigma$2]]istheonlysolutionfor[k$\sigma$2,(k+1)$\sigma$2]. Applying induction, we can infer that a!-Sol exists for [*$\sigma$*$_{2}$, $\mathcal{F}$ ].

Finally, considering Cases (1–3), one can conclude that a! -Sol exists for (1) and (2) within C1$-$$\gamma$$\gamma$(J).

## 4 The SGLA: Assembly and outcomes

This section introduces the SGLA numerical scheme for solving (1) and (2). This approach is a type of weighted residual numerical technique that uses a finite set of basis polynomials as weighting functions. Based on our proposed algorithm, we employ orthogonal spline local polynomial functions as the weighting functions. Indeed, we prove theorems regarding the convergence and error estimation of the SGLA.

Broadly, the SGLA is a numerical technique used to solve various classes of differential/integral problems including several types by approximating the solution by leveraging a finite collection of orthogonal basis functions. In implementing the Galerkin scheme, the orthogonal basis functions are selected to satisfy the initial or boundary constraints. The effectiveness of the Galerkin approach relies on several prerequisites as follows:

The basis functions must be orthogonal, facilitating straightforward computation of solution coefficients.

The selected basis functions should have regular behavior and adequate smoothness to ensure precise approximation.

The Galerkin numerical scheme will provide accurate results if the problem has a! -Sol that depends continuously on the data and problem constraints. Failure to meet these constraints may result in imprecise or erroneous outcomes from the Galerkin approach.

Initially, we define the OSLPs as basis functions for representing functions within a predefined interval. Following that, we use this concept to define the essential functions in (1) in terms of OSLPs. Next, we calculate the residual required in the Galerkin numerical process. Leveraging the orthogonality of the residual with the OSLPs, the sets of FDDSs can be simplified into an algebraic one. Finally, this system can be solved by leveraging MATHEMATICA 11 to obtain the needed approximation.

**Definition 5.** [[`ref34`]] The OSLP of multiplicity $\beta$ is




$\phi$$\beta$(?)=$\sum$?=0$\beta$($-$1)$\beta$+?$\Gamma$($\hbar$+?+1)$\Gamma$($\hbar$$-$?+1)($\Gamma$(?+1))2??,


(54)

where ?=0,1,$\cdots$,$\beta$,?$\in$[0,1],$\phi$$\beta$(0)=($-$1)$\beta$, and *$\phi$*$\beta$(1) = 1.

The condition of orthogonality is defined as




$\int$01$\phi$$\beta$(?)$\phi$?(?)d?={12$\beta$+1,$\beta$=?,0,$\beta$$\neq$?.


(55)

This entails that $\forall$$\Delta$$\in$C([0,1]→R) may characterized in OSLPs with $\beta$ = 0,1,$\cdots$,$\hbar$ as




{$\Delta$(?)≈$\Delta$$\hbar$(?)=($\Delta$1(?),$\Delta$2(?))=($\sum$$\beta$=0$\hbar$a$\beta$$\phi$$\beta$(?),$\sum$$\beta$=0$\hbar$b$\beta$$\phi$$\beta$(?)),a$\beta$=<$\Delta$1(?),$\phi$$\beta$(?)>=$\int$01(2$\beta$+1)$\Delta$1(?)$\phi$$\beta$(?)d?,b$\beta$=<$\Delta$2(?),$\phi$$\beta$(?)>=$\int$01(2$\beta$+1)$\Delta$2(?)$\phi$$\beta$(?)d?.


(56)

Now, interpolating OSLP for *$\theta$*(?) in (1), we get




$\theta$(?)≈$\theta$$\hbar$(?)=($\sum$$\beta$=0$\hbar$a$\beta$$\phi$$\beta$(?),$\sum$$\beta$=0$\hbar$b$\beta$$\phi$$\beta$(?)),


(57)

where a$\beta$ and b$\beta$, $\beta$ = 0,1,$\cdots$,$\hbar$ are the unknown parameters.

Again, HD0+$\alpha$,$\beta$$\theta$(?),$\wp$($\theta$(?)), and S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$)) can be approximated with




 HD0+$\alpha$,$\beta$$\theta$(?)≈ HD0+$\alpha$,$\beta$$\theta$$\hbar$(?)=($\sum$$\beta$=0$\hbar$a$\beta$ HD0+$\alpha$,$\beta$$\phi$$\beta$(?),$\sum$$\beta$=0$\hbar$b$\beta$ HD0+$\alpha$,$\beta$$\phi$$\beta$(?)).


(58)





$\wp$($\theta$(?))≈$\wp$$\hbar$($\theta$$\hbar$(?))=($\wp$1$\hbar$($\theta$$\hbar$(?)),$\wp$2$\hbar$($\theta$$\hbar$(?))).


(59)





S(?,$\wp$($\theta$(?)),$\theta$(?$-$$\theta$))≈S$\hbar$(?,$\wp$$\hbar$($\theta$$\hbar$(?)),$\theta$$\hbar$(?$-$$\theta$))=(S1(?,$\wp$1$\hbar$($\theta$$\hbar$(?)),$\sum$$\beta$=0$\hbar$a$\beta$$\phi$$\beta$(?$-$$\sigma$1)),S2(?,$\wp$2$\hbar$($\theta$$\hbar$(?)),$\sum$$\beta$=0$\hbar$b$\beta$$\phi$$\beta$(?$-$$\sigma$2))).


(60)

Swapping out (57), (58), (59) and (60) into (1) gives




 HD0+$\alpha$,$\beta$$\theta$$\hbar$(?)=S$\hbar$(?,$\wp$$\hbar$($\theta$$\hbar$(?)),$\theta$$\hbar$(?$-$$\theta$)).


(61)

To execute the SGLA, simulate the residual, *R*, out of (61) as




R(?)=(R1(?)$-$R2(?))= HD0+$\alpha$,$\beta$$\theta$$\hbar$(?)$-$S$\hbar$(?,$\wp$$\hbar$($\theta$$\hbar$(?)),$\theta$$\hbar$(?$-$$\theta$)).


(62)

When *R*(?) = (0,0), the result solves (1) and fulfills the presumed constraints, Therefore, in our research, we intend to optimize *R* to be zero in light of the selected OSLP that is chosen as the basis to axiomatize *$\theta$*$\hbar$(?).

To fix the unknowable parameters a0,a1,$\cdots$,a$\hbar$,b0,b1,$\cdots$,b$\hbar$ in (57), we select weight functions as the granted basis and integrate constraint controls concerning *R* = 0. By leveraging the orthogonality of *R* at ($\hbar$+1) mapping $\phi$0(?),$\phi$1(?),$\cdots$,$\phi$$\hbar$(?) as




{$\int$01R1(?)$\phi$$\beta$(?)d?=0,$\beta$=0,1,$\cdots$,$\hbar$$-$1,$\sum$$\beta$=0$\hbar$a$\beta$$\phi$$\beta$(0)=$\phi$1(0).


(63)





{$\int$01R2(?)$\phi$$\beta$(?)d?=0,$\beta$=0,1,$\cdots$,$\hbar$$-$1,$\sum$$\beta$=0$\hbar$b$\beta$$\phi$$\beta$(0)=$\phi$2(0).


(64)

So, one has a set of 2($\hbar$+1) algebraic equations including 2($\hbar$+1) unknown a0,a1,$\cdots$,a$\hbar$,b0,b1,$\cdots$,b$\hbar$. After substituting the estimated a0,a1,$\cdots$,a$\hbar$,b0,b1,$\cdots$,b$\hbar$ parameters in (57) we get the approximated solution for (1).

Next, similar to symbolization in Theorem 2; $\theta$$\hbar$(?)=($\theta$1$\hbar$(?),$\theta$2$\hbar$(?))=($\sum$$\beta$=0$\hbar$a$\beta$$\phi$$\beta$(?),$\sum$$\beta$=0$\hbar$b$\beta$$\phi$$\beta$(?)),$\theta$?(?)=($\theta$1?(?),$\theta$2?(?))=($\sum$$\beta$=0?a$\beta$$\phi$$\beta$(?),$\sum$$\beta$=0?b$\beta$$\phi$$\beta$(?)), and $\theta$(?)=($\theta$1(?),$\theta$2(?)).

**Theorem 3.** Let ($\sum$$\beta$=0$\infty$a$\beta$$\phi$$\beta$(?),$\sum$$\beta$=0$\infty$b$\beta$$\phi$$\beta$(?)) be the Legendre interpolation of *$\theta$*(?) with $\theta$$\beta$(?)$\in$L2[0,1]. Then $\theta$$\hbar$(?)→$\theta$(?) as $\hbar$→$\infty$.

**Proof.** Set *$\theta$*$_{?}$(?) be a partial sum in ($\sum$$\beta$=0$\infty$a$\beta$$\phi$$\beta$(?),$\sum$$\beta$=0$\infty$b$\beta$$\phi$$\beta$(?)). Then at $\hbar$>?, it is straightforward to verify that




$\Vert$$\theta$$\hbar$(?)$-$$\theta$?(?)$\Vert$2=$\Vert$[$\theta$1$\hbar$(?)$\theta$2$\hbar$(?)]$-$[$\theta$1?(?)$\theta$2?(?)]$\Vert$2=$\Vert$[$\sum$$\beta$=?+1$\hbar$a$\beta$$\varphi$$\beta$(?)$\sum$$\beta$=?+1$\hbar$b$\beta$$\varphi$$\beta$(?)]$\Vert$2<[$\sum$$\beta$=?+1$\hbar$/a$\beta$/2$\sum$$\beta$=?+1$\hbar$/b$\beta$/2].


(65)

Utilizing the inequality of Bessel's, one finds $\sum$$\beta$=?+1$\hbar$/a$\beta$/2≤$\sum$$\beta$=?+1$\infty$/a$\beta$/2≤$\Vert$$\theta$$\Vert$2<$\infty$ and $\sum$$\beta$=?+1$\hbar$/b$\beta$/2≤$\sum$$\beta$=?+1$\infty$/b$\beta$/2≤$\Vert$$\theta$$\Vert$2<$\infty$. So, $\Vert$$\theta$$\hbar$(?)$-$$\theta$?(?)$\Vert$2→0 as $\hbar$,?→$\infty$. Thus, the sequence *$\theta$*$\hbar$(?) is Cauchy and converges to S(?)$\in$L2([0,1)]. Anyhow,




<[S1(?)S2(?)]$-$[$\theta$1(?)$\theta$2(?)],[$\varphi$$\beta$(?)$\varphi$$\beta$(?)]>=<[S1(?)S2(?)],[$\varphi$$\beta$(?)$\varphi$$\beta$(?)]>$-$<[$\theta$1(?)$\theta$2(?)],[$\varphi$$\beta$(?)$\varphi$$\beta$(?)]>







=lim$\hbar$→$\infty$<[$\theta$1$\hbar$(?)$\theta$2$\hbar$(?)],[$\varphi$$\beta$(?)$\varphi$$\beta$(?)]>$-$(a$\beta$,b$\beta$)







=(a$\beta$,b$\beta$)$-$(a$\beta$,b$\beta$)







=(0,0).


(66)

Hence, (S1(?),S2(?)) and ($\theta$1$\hbar$(?),$\theta$2$\hbar$(?)) converges to *$\theta$*(?) as $\hbar$→$\infty$.

**Theorem 4.** An $\varepsilon$$\hbar$(?)=$\theta$(?)$-$$\theta$$\hbar$(?) can approximate the solution of (1) and (2) by the SGLA.

**Proof.** By leveraging the SGLA, *$\theta$*$\hbar$(?) fulfills (1). Hence,




{ HD0+$\alpha$,$\beta$$\theta$$\hbar$(?)=[ HD0+$\alpha$,$\beta$$\theta$1$\hbar$(?) HD0+$\alpha$,$\beta$$\theta$2$\hbar$(?)]=[S1$\hbar$(?,$\wp$1$\hbar$($\theta$$\hbar$(?)),$\theta$1$\hbar$(?$-$$\sigma$1))S2$\hbar$(?,$\wp$2$\hbar$($\theta$$\hbar$(?)),$\theta$2$\hbar$(?$-$$\sigma$2))],0<?≤F,$\theta$$\hbar$(?)=[$\phi$1(?)$\phi$2(?)],$-$$\sigma$1≤?≤0.


(67)

Now, by subtracting (67) from (1), one has




{ HD0+$\alpha$,$\beta$$\varepsilon$$\hbar$(?)=h(?,$\varepsilon$$\hbar$(?),$\varepsilon$$\hbar$(?$-$$\theta$)),0<?≤F,$\varepsilon$$\hbar$(?)=0,$-$$\sigma$1≤?≤0,


(68)





h(?,$\varepsilon$$\hbar$(?),$\varepsilon$$\hbar$(?$-$$\tau$))=[S1$\hbar$(?,$\wp$1$\hbar$($\theta$$\hbar$(?)),$\theta$1$\hbar$(?$-$$\sigma$1))$-$S1(?,$\wp$1($\theta$(?)),$\theta$1(?$-$$\sigma$1))S2$\hbar$(?,$\wp$2$\hbar$($\theta$$\hbar$(?)),$\theta$2$\hbar$(?$-$$\sigma$2))$-$S2(?,$\wp$2($\theta$(?)),$\theta$2(?$-$$\sigma$2))]


(69)

Thus, to solve (68) and (69), we can use SGLA to gain a set of 2($\hbar$+1) algebraic equations, and by handling this set one gets *$\varepsilon$*$\hbar$(?).

## 5 Computational demonstrations

Herein, we solve two systems of DFDEs in the FHD sense that involve various instances by leveraging the proposed algorithm. While the second example is nonlinear, the first one is linear. Our methodology involves controlling the relative error R$\hbar$(?)=(R$\hbar$,1(?),R$\hbar$,2(?)) and the absolute error A$\hbar$(?)=(A$\hbar$,1(?),A$\hbar$,2(?)) for various values of *$\alpha$*, *$\beta$*, and $\hbar$. We provide a thorough examination of the effectiveness through plots and data that contrast the exact solution with the approximations.

### 5.1 SGLA: Steps and examples

The SGLA provides several benefits and utilities when handling a set of FDEs as

The simulated outcomes achieved closely approximates the exact solution. As evidenced by the obtained $\Re$$\hbar$(?) and A$\hbar$(?), SGLA delivers highly sufficient and accurate results.

The SGLA can achieve high validity by leveraging tiny iterations in the OSLP expansions.

The SGLA represents a straightforward scheme to incorporate that does not require sophisticated mathematical apparatuses or a proficient programmer.

The utilized SGLA serves as a universal technique that can be implemented to exhibit other instances of fractional systems.

The core characteristic of SGLA is its applicability to other orthogonal basis functions.

Algorithm 1 clearly outlines the steps to derive a solution leveraging the presented SGLA and the specified FHD. At this point, either a programmer or a specialized mathematician with relevant expertise could implement this algorithm into a program by leveraging the MATHEMATICA 11 environment, consistent with how we demonstrated this approach in this paper.

**Algorithm 1.** The SGLA iteration steps for calculations of *$\theta$*$\hbar$(*x*) concerning DFDE (1) and (2) in the FHD sense.

Requirement:

Primary parameters: *$\alpha$*,*$\beta$*,*$\tau$*, and *$\sigma$*;

Nonhomogenous terms: S1 and S2;

OSLPs of HD0+$\alpha$,$\beta$$\theta$1(?);

Magnitude of accuracy: $\hbar$.

Action A:

While $\beta$ = 0,1,$\cdots$,$\hbar$ find *R*(?) from (62).

Action B:

While $\beta$ = 1,2,$\cdots$,$\hbar$$-$1 find

{$\int$01R1(?)$\phi$$\beta$(?)d?=0,$\sum$$\beta$=0$\hbar$a$\beta$$\phi$$\beta$(0)=$\phi$1(0).

{$\int$01R2(?)$\phi$$\beta$(?)d?=0,$\sum$$\beta$=0$\hbar$b$\beta$$\phi$$\beta$(0)=$\phi$2(0).

Action C:

While $\beta$ = 0,1,$\cdots$,$\hbar$ find $\sum$$\beta$=0$\hbar$a$\beta$($-$1)$\beta$ and $\sum$$\beta$=0$\hbar$b$\beta$($-$1)$\beta$.

Action D:

Solve 2($\hbar$+1) set of equations generated from Action B to find c$\beta$,$\beta$=0,1,$\cdots$,$\hbar$.

Result:

Assign a$\beta$ and b$\beta$ with $\beta$ = 0,1,$\cdots$,$\hbar$ on OSLP of *$\theta$*(?) to get *$\theta$*$\hbar$(*x*).

Let us begin our calculations with the following two suitable instances, both of which have precise solutions throughout the chosen integral domain, ensuring the accuracy of the data outputs.

**Example 1.** Examine the fraction approximations of the succeeding linear sets of DFDEs in the FHD sense:




{ HD0+$\alpha$,$\beta$$\theta$1(?)=$\theta$1(?$-$0.25)+$\theta$2(?)+u1(?),0<?≤2, HD0+$\alpha$,$\beta$$\theta$2(?)=$\theta$2(?$-$1)+$\theta$2(?)+$\theta$1(?)+u2(?),u1(?)=$-$(?$-$0.25)$-$?2+1$\Gamma$(2$-$$\alpha$)?1$-$$\alpha$,u2(?)=$-$(?$-$1)$-$?2$-$?+2$\Gamma$(3$-$$\alpha$)?2$-$$\alpha$,$\theta$1(?)=?,$-$0.25≤?≤0,$\theta$2(?)=?2,$-$1≤?≤0.


(70)

Hither, *$\theta$*$_{1}$(?) = ? over ?$\in$[$-$0.25,2] and *$\theta$*$_{1}$(?) = ?$^{2}$ over ?$\in$[$-$1,2]. Indded, 0<*$\alpha$*<1 and 0<*$\beta$*<1.

**Example 2.** Examine the fraction approximations of the succeeding nonlinear sets of DFDEs in the FHD sense:




{ HD0+$\alpha$,$\beta$$\theta$1(?)+$\theta$1(?$-$1)+$\theta$1(?)$\theta$2(?)=u1(?),0<?≤2, HD0+$\alpha$,$\beta$$\theta$2(?)+$\theta$2(?$-$0.7)+$\theta$1(?)$\theta$2(?)+$\theta$12(?)=u2(?),u1(?)=(?$-$1)+?3+1$\Gamma$(2$-$$\alpha$)?1$-$$\alpha$,u2(?)=(?$-$0.7)+?2+?3+2$\Gamma$(3$-$$\alpha$)?2$-$$\alpha$,$\theta$1(?)=?,$-$1≤?≤0,$\theta$2(?)=?2,$-$0.7≤?≤0.


(71)

Hither, *$\theta$*$_{1}$(?) = ? over ?$\in$[$-$1,1] and *$\theta$*$_{2}$(?) = ?$^{2}$ over ?$\in$[$-$0.7,1]. Indded, 0<*$\alpha$*<1 and 0<*$\beta$*<1.

### 5.2 Analysis of the findings

This study applies the preceding numerical technique to approximate solutions to (1) and (2). Graphical and tabulated outputs are generated for each example scenario under varied configurations of {*$\alpha$*,*$\beta$*} and $\hbar$ parameters. Additionally, computations of A$\hbar$(?) and $\Re$$\hbar$(?) are carried out and compared extensively, to evaluate the efficacy of the SGLA numerical approach.

For Example 1; in Table 1, the correlation between data operations A$\hbar$,1(?) and $\Re$$\hbar$,1(?) are presented at $\hbar$ = 3, *$\alpha$* = 0.5, and *$\beta$*$\in${0.3,0.8}. In Table 2, the correlation between data operations A$\hbar$,2(?) and $\Re$$\hbar$,2(?) are presented at $\hbar$ = 3, *$\alpha$* = 0.5, and *$\beta$*$\in${0.3,0.8}. To provide additional information, Fig 1 displays A$\hbar$,1(?) and $\Re$$\hbar$,1(?) at ($\hbar$,$\alpha$,$\beta$)$\in${(3,0.3,0.3),(3,0.4,0.9),(3,0.9,0.4)} that generated from the SGLA. Fig 2 displays A$\hbar$,2(?) and $\Re$$\hbar$,2(?) at ($\hbar$,$\alpha$,$\beta$)$\in${(3,0.3,0.3),(3,0.4,0.9),(3,0.9,0.4)} that generated from the SGLA.

```mermaid {span=wide pos=top width=0.9}
info:doi/10.1371/journal.pone.0305259.g001
```

```mermaid {span=wide pos=top width=0.89}
info:doi/10.1371/journal.pone.0305259.g002
```

%% table

| *$\beta$* | 0.3 | 0.8 |  |  |
|---|---|---|---|---|
| ?$\beta$ | A$\hbar$,1(?$\beta$) | $\Re$$\hbar$,1(?$\beta$) | A$\hbar$,1(?$\beta$) | $\Re$$\hbar$,1(?$\beta$) |
| 0.1 | 9.7144514654×10$-$17 | 8.8817841970×10$-$16 | 1.1102230246×10$-$16 | 1.1102230246×10$-$15 |
| 0.2 | 2.7755575615×10$-$17 | 2.2204460492×10$-$16 | 2.2204460492×10$-$16 | 1.1102230246×10$-$15 |
| 0.3 | 1.6653345369×10$-$16 | 6.6613381477×10$-$16 | 2.7755575615×10$-$16 | 8.8817841970×10$-$16 |
| 0.4 | 2.7755575615×10$-$16 | 6.6613381477×10$-$16 | 3.8857805861×10$-$16 | 1.1102230246×10$-$15 |
| 0.5 | 4.4408920985×10$-$16 | 8.8817841970×10$-$16 | 5.5511151231×10$-$16 | 1.1102230246×10$-$15 |
| 0.6 | 5.5511151231×10$-$16 | 8.8817841970×10$-$16 | 6.6613381477×10$-$16 | 1.1102230246×10$-$15 |
| 0.7 | 8.8817841970×10$-$16 | 1.3322676295×10$-$15 | 7.7715611723×10$-$16 | 1.1102230246×10$-$15 |
| 0.8 | 9.9920072216×10$-$16 | 1.3322676295×10$-$15 | 8.8817841970×10$-$16 | 1.1102230246×10$-$15 |
| 0.9 | 1.22124532708×10$-$15 | 1.3322676295×10$-$15 | 9.9920072216×10$-$16 | 1.1102230246×10$-$15 |
| 1 | 1.3322676295×10$-$15 | 1.3322676295×10$-$15 | 1.1102230246×10$-$15 | 1.1102230246×10$-$15 |

%% table

| *$\beta$* | 0.3 | 0.8 |  |  |
|---|---|---|---|---|
| ?$\beta$ | A$\hbar$,2(?$\beta$) | $\Re$$\hbar$,2(?$\beta$) | A$\hbar$,2(?$\beta$) | $\Re$$\hbar$,2(?$\beta$) |
| 0.1 | 2.1510571102×10$-$16 | 2.1538326677×10$-$14 | 2.4286128663×10$-$16 | 2.4424906541×10$-$14 |
| 0.2 | 4.5102810375×10$-$16 | 1.1324274851×10$-$14 | 4.4408920985×10$-$16 | 1.1102230246×10$-$14 |
| 0.3 | 6.8001160258×10$-$16 | 7.5495165674×10$-$15 | 5.8286708792×10$-$16 | 6.4392935428×10$-$15 |
| 0.4 | 7.4940054162×10$-$16 | 4.6629367034×10$-$15 | 5.5511151231×10$-$16 | 3.5527136788×10$-$15 |
| 0.5 | 8.3266726846×10$-$16 | 3.3306690738×10$-$15 | 6.6613381477×10$-$16 | 2.6645352591×10$-$15 |
| 0.6 | 8.881784197×10$-$16 | 2.4424906541×10$-$15 | 7.2164496600×10$-$16 | 1.9984014443×10$-$15 |
| 0.7 | 8.8817841970×10$-$16 | 1.9984014443×10$-$15 | 6.6613381477×10$-$16 | 1.5543122344×10$-$15 |
| 0.8 | 7.7715611723×10$-$16 | 1.3322676295×10$-$15 | 5.5511151231×10$-$16 | 8.8817841970×10$-$16 |
| 0.9 | 8.8817841970×10$-$16 | 1.1102230246×10$-$15 | 5.5511151231×10$-$16 | 6.6613381477×10$-$16 |
| 1 | 8.8817841970×10$-$16 | 8.8817841970×10$-$16 | 6.6613381477×10$-$16 | 6.6613381477×10$-$16 |

For Example 2; in Table 3, the correlation between data operations A$\hbar$,2(?) and $\Re$$\hbar$,1(?) are presented at $\hbar$ = 3, *$\beta$* = 0.5, and *$\alpha$*$\in${0.1,0.7}. In Table 4, the correlation between data operations A$\hbar$,2(?) and $\Re$$\hbar$,2(?) are presented at $\hbar$ = 3, *$\beta$* = 0.5, and *$\alpha$*$\in${0.1,0.7}. To provide additional information, Fig 3 displays A$\hbar$,1(?) and $\Re$$\hbar$,1(?) at ($\hbar$,$\alpha$,$\beta$)$\in${(3,0.3,0.6),(3,0.6,0.3),(3,0.4,0.4)} that generated from the SGLA. Fig 4 displays A$\hbar$,2(?) and R$\hbar$,2(?) at ($\hbar$,$\alpha$,$\beta$)$\in${(3,0.3,0.6),(3,0.6,0.3),(3,0.4,0.4)} that generated from the SGLA.

```mermaid {span=wide pos=top width=0.89}
info:doi/10.1371/journal.pone.0305259.g003
```

```mermaid {span=wide pos=top width=0.89}
info:doi/10.1371/journal.pone.0305259.g004
```

%% table

| *$\alpha$* | 0.1 | 0.7 |  |  |
|---|---|---|---|---|
| ?$\beta$ | A$\hbar$,1(?$\beta$) | $\Re$$\hbar$,1(?$\beta$) | A$\hbar$,1(?$\beta$) | $\Re$$\hbar$,1(?$\beta$) |
| 0.1 | 1.7624790515×10$-$15 | 1.7541523789×10$-$14 | 8.3266726846×10$-$17 | 7.7715611723×10$-$16 |
| 0.2 | 2.0816681711×10$-$15 | 1.0325074129×10$-$14 | 8.3266726846×10$-$17 | 3.3306690738×10$-$16 |
| 0.3 | 2.1094237467×10$-$15 | 6.9944050551×10$-$15 | 1.1102230246×10$-$16 | 3.3306690738×10$-$16 |
| 0.4 | 1.9984014443×10$-$15 | 4.8849813083×10$-$15 | 1.1102230246×10$-$16 | 2.2204460492×10$-$16 |
| 0.5 | 1.6653345369×10$-$15 | 3.3306690738×10$-$15 | 1.1102230246×10$-$16 | 2.2204460492×10$-$16 |
| 0.6 | 1.4432899320×10$-$15 | 2.4424906541×10$-$15 | 1.1102230246×10$-$16 | 2.2204460492×10$-$16 |
| 0.7 | 1.2212453270×10$-$15 | 1.7763568394×10$-$15 | 0 | 0 |
| 0.8 | 1.2212453270×10$-$15 | 1.4432899320×10$-$15 | 0 | 0 |
| 0.9 | 1.4432899320×10$-$15 | 1.5543122344×10$-$15 | 1.1102230246×10$-$16 | 0 |
| 1 | 2.2204460492×10$-$15 | 2.2204460492×10$-$15 | 0 | 0 |

%% table

| *$\alpha$* | 0.1 | 0.7 |  |  |
|---|---|---|---|---|
| ?$\beta$ | A$\hbar$,2(?$\beta$) | $\Re$$\hbar$,2(?$\beta$) | A$\hbar$,2(?$\beta$) | $\Re$$\hbar$,2(?$\beta$) |
| 0.1 | 5.6205040621×10$-$16 | 5.6177285046×10$-$14 | 6.2450045135×10$-$17 | 6.2172489379×10$-$15 |
| 0.2 | 4.1633363423×10$-$16 | 1.0325074129×10$-$14 | 0 | 0 |
| 0.3 | 8.3266726846×10$-$17 | 9.9920072216×10$-$16 | 4.1633363423×10$-$17 | 4.4408920985×10$-$16 |
| 0.4 | 2.4980018054×10$-$16 | 1.5543122344×10$-$15 | 0 | 0 |
| 0.5 | 7.7715611723×10$-$16 | 3.1086244689×10$-$15 | 5.5511151231×10$-$17 | 2.2204460492×10$-$16 |
| 0.6 | 1.2767564783×10$-$15 | 3.5527136788×10$-$15 | 2.2204460492×10$-$16 | 4.4408920985×10$-$16 |
| 0.7 | 1.7208456881×10$-$15 | 3.5527136788×10$-$15 | 1.6653345369×10$-$16 | 4.4408920985×10$-$16 |
| 0.8 | 1.7763568394×10$-$15 | 2.8865798640×10$-$15 | 1.1102230246×10$-$16 | 2.2204460492×10$-$16 |
| 0.9 | 1.9984014443×10$-$15 | 2.4424906541×10$-$15 | 2.2204460492×10$-$16 | 2.2204460492×10$-$16 |
| 1 | 1.9984014443×10$-$15 | 1.9984014443×10$-$15 | 2.2204460492×10$-$16 | 2.2204460492×10$-$16 |

From the charts in the antecedent figures, we interestingly that the apparent drawing curves are very low towards the ?-axis. This confirms the data of the driving tables. In addition, these curves contain oscillations at their beginning or end, which confirms the occurrence of a delay in the fraction solutions, and also confirms the shape of the previously selected examples.

## 6 Summary of key findings

Throughout the work, we established the EUE concerning a set of DFDEs in the sense of the FHD approach. Our investigation involved examining the properties of the FRD, FCD, and FHD, utilizing their hallmarks to convert our system of DFDEs into an equivalent one of fractional DVIE with the help of the CMT. For numerical analysis, we implemented the SGLA leveraging OSLPs. This algorithm allows us to approximate the solution to the considered system by transforming it into a series of algebraic equations. One of the key advantages of this algorithm is its ability to achieve accurate results with fewer iterations compared to alternative methods. The numerical convergence and error estimates of the approach are examined and to validate the effectiveness and accuracy of our algorithm, we conducted a comprehensive evaluation through various linear and nonlinear numerical applications. The data from these evaluations, accompanied by charts and tables, further validate the strength of our algorithmic approach. Future research directions for our simulations could include applying it to additional types of DFDEs, such as those containing multiple delays or nonlinear terms. The algorithm could also be analyzed for solving DFDEs and fractional delay integral models. Furthermore, this research emphasizes the adaptability of the SGLA, as it can be extended to other complex fractional problems and systems by utilizing alternative orthogonal basis functions like Chebyshev or Fourier polynomials. Additionally, the algorithm may be parallelized to improve computational efficiency.

## Acknowledgements

The authors would like to express their gratitude to the unknown referees for carefully reading the paper and for their helpful comments.
