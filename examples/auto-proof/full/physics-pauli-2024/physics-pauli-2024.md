# Quantum simulation of Pauli channels and dynamical maps: Algorithm and implementation

## Abstract

Pauli channels are fundamental in the context of quantum computing as they model the simplest kind of noise in quantum devices. We propose a quantum algorithm for simulating Pauli channels and extend it to encompass Pauli dynamical maps (parametrized Pauli channels). A parametrized quantum circuit is employed to accommodate for dynamical maps. We also establish the mathematical conditions for an *N*-qubit transformation to be achievable using a parametrized circuit where only one single-qubit operation depends on the parameter. The implementation of the proposed circuit is demonstrated using IBM's quantum computers for the case of one qubit, and the fidelity of this implementation is reported.

## 1 Introduction

Since their inception, quantum computers were proposed as powerful tools for the simulation of quantum systems [[`ref01`]]. Being open quantum systems of fundamental [[`ref02`], [`ref03`]] and practical [[`ref04`]] interest, there has been efforts towards the simulation of the evolution of open quantum systems [[`ref05`]– [`ref07`]] and specifically for quantum channels [[`ref08`]– [`ref11`]], which can be used to study and model decoherence. Such quantum algorithms can be represented using what is known as a quantum circuit [[`ref12`]], which we will study in section 3.

Such systems have been simulated because of their many applications, such as studying the emergence of multipartite entanglement [[`ref13`], [`ref14`]], studying dissipative processes [[`ref15`]] and modeling non-Markovian dynamics [[`ref16`]]. Among quantum systems, the simplest case is that of a qubit [[`ref12`]], and within them, the simplest class of channels that produce decoherence are Pauli channels [[`ref17`]– [`ref19`]]. Indeed, they serve as effective models for the noise affecting quantum devices [[`ref20`]].

In this article we propose a quantum algorithm for simulating Pauli channels and implement it on one of IBM's quantum computers. The algorithm proposed is straightforward and can be used for any Pauli channel by only changing a couple of parameters in the operations it performs. To represent the algorithm, we use a quantum circuit, which is a common way of representing algorithms intended for quantum computers. [[`ref12`]]. Furthermore, we will also simulate Pauli dynamical maps, which are continuous parametrized curves of Pauli channels. These maps can be used to model a continuous change of a qubit instead of only discrete jumps. The generality of the algorithm proposed for Pauli channels will be very useful in this part, since then using for dynamical maps will come very naturally. Implementing these maps will lead us to study quantum algorithms with free parameters, something that is common in areas such as quantum machine learning [[`ref21`]]. This can be done using parametrized quantum circuits, which consist of quantum circuits in which one or more operations depend on a free parameter.

We start by providing the definition of quantum channels, the general framework used here, and multi-qubit Pauli channels in section 2. Our first objective is to present a general and straightforward quantum algorithm capable of simulating Pauli channels on quantum computers; we do this in section 3, where we also demonstrate its implementation using IBM's quantum computers for several single and two-qubit Pauli channels. Expanding beyond Pauli channels, we introduce the concept of Pauli dynamical maps, defined as a continuous parametrization of multi-qubit Pauli channels. In fact, in order to implement them, we study parametrized quantum circuits in chapter 4. Furthermore, we contribute to the body of work related to parametrized quantum circuits by establishing a theorem, which sets the mathematical conditions for the transformations that can be done using a parametrized circuit with the restriction that only a controlled single-qubit rotation in the circuit may depend on the parameter. Finally, in section 5, we conclude analyzing the Pauli dynamical maps that fulfill the conditions of theorem 2.

## 2 Pauli channels and dynamical maps

In this section we introduce the concept of quantum channels, focusing on a specific type called Pauli channels. Furthermore, we define Pauli dynamical maps, which are curves of Pauli channels parametrized by a variable.

### 2.1 Quantum channels

In quantum mechanics, a closed system's state is represented by a vector in a Hilbert space H. The state's evolution is unitary and given by Schrodinger's equation [[`ref22`]]. However, in real-world situations, quantum systems are usually open, which means that they interact with their environment [[`ref04`]]. For instance, the system's state may become entangled with the environment, leading to a loss of information about the system's state over time.

To describe open systems, instead of state vectors, we use matrices *$\rho$* that act on H. These matrices are called density matrices, and they include information about the system's interaction with its environment. For a density matrix *$\rho$* to be physically valid, it must satisfy two conditions: tr(*$\rho$*) = 1 and it must be positive semi-definite, which is denoted as *$\rho$* ≥ 0 [[`ref12`]].

Knowing this, we can now define quantum channels. Quantum channels are operators E that can describe the evolution of open quantum systems, such that $\rho$ → E ( $\rho$ ). Quantum channels are the most general linear operations that a quantum system can undergo independently of its past [[`ref23`], [`ref24`]]. These channels are constructed based on three fundamental properties: linearity, trace preservation, and complete positivity.

Linearity ensures that a quantum channel E maps any ensemble of density matrices into the corresponding ensemble of their evolution. The trace preserving property is given by tr ( E [ $\rho$ ] ) = tr ( $\rho$ ) = 1 and guarantees that the quantum channel does not change the condition that tr(*$\rho$*) = 1. Finally, the channel should also preserve the condition *$\rho$* ≥ 0, and a map that does this is called a positive map. However, positivity of E is not enough, and we actually require the more restrictive condition of complete positivity. Complete positivity means that E $\otimes$ I n is positive for any positive integer *n* (where I n is the *n* × *n* identity matrix). This ensures that even if the main system is entangled with another system, applying E to the main system while doing nothing to the other one still results in a positive semidefinite state for the main system [[`ref12`]].

Given a quantum channel E, the condition of trace preservation is straightforward to verify but complete positivity is not as simple. To test complete positivity of a quantum channel, Jamiolkowski and Choi [[`ref25`], [`ref26`]] developed a simple algorithm that exploits the isomorphism between a channel E and the state D = ( I $\otimes$ E ) [ / $\Omega$ > < $\Omega$ / ], where / $\Omega$ > = 1 / dim ( H ) $\sum$ i dim ( H ) / i > / i > is a maximally entangled state between the original system and an ancilla. Remarkably, the map E is completely positive if and only if D (also known as the Choi or dynamical matrix of E) is positive semidefinite.

### 2.2 Pauli channels

We have discussed the main features of quantum channels and now we turn our attention to a specific type of channels for *N*-qubit systems called Pauli channels. First we will define these channels for single-qubit systems, whose most general density matrix can be written as [[`ref12`]]:
$\rho$ = 1 2 $\sum$ $\alpha$ = 0 3 r $\alpha$ $\sigma$ $\alpha$ , (1)
with $\sigma$ 0 = I, and *$\sigma$*$_{1,2,3}$ the usual Pauli matrices. The condition tr(*$\rho$*) = 1 requires that *r*$_{0}$ = 1 while *$\rho$* ≥ 0 implies that the remaining *r*$_{1,2,3}$ form a vector r → = ( r 1 , r 2 , r 3 ) inside a unit sphere known as the Bloch sphere [[`ref27`]]. That is, every possible density matrix for a one-qubit system is uniquely associated with a point in a unit sphere.

Given a one-qubit system described by *$\rho$*, a Pauli channel is defined as an operation that with probability *k*$\gamma$ applies the Pauli matrix *$\sigma$*$\gamma$ to the system, for *$\gamma$* = 0, 1, 2, 3 [[`ref17`]]. Mathematically, the Pauli channel is written in the following way:
E ( $\rho$ ) = $\sum$ $\gamma$ = 0 3 k $\gamma$ $\sigma$ $\gamma$ $\rho$ $\sigma$ $\gamma$ , (2)
where the probabilities *k*$\gamma$ of applying *$\sigma$*$\gamma$ are non-negative real numbers such that $\sum$$\gamma$ *k*$\gamma$ = 1 (these conditions also ensure that the channel is trace preserving and completely positive).

Pauli channels are some of the most fundamental noise models in quantum information science [[`ref28`]]. Some notable examples of Pauli channels are the following:

**Bit Flip Channel:** This is a channel that with probability 1 $-$ *p* leaves the qubit as it is and with probability *p* applies the *$\sigma$*$_{1}$ matrix (which flips the basis states /0> and /1> of the qubit), and so it is given by:
E ( $\rho$ ) = ( 1 - p ) $\rho$ + p $\sigma$ 1 $\rho$ $\sigma$ 1 .
Analogous channels exist using *$\sigma$*$_{3}$ (called the bit flip channel, which has a probability *p* of adding a relative phase *$\pi$* to the state) or using *$\sigma$*$_{2}$ (called the phase flip channel, which has a probability *p* of flipping the base states and also add a relative phase *$\pi$*).

**Depolarizing channel:** This channel has a probability 1 $-$ *p* of doing nothing to the qubit and a probability *p* of converting it into the maximally mixed state 1 2 I and it can be written as:
E ( $\rho$ ) = ( 1 - p ) $\rho$ + p 1 2 I = ( 1 - 3 p 4 ) $\sigma$ + p 4 $\sigma$ 1 $\rho$ $\sigma$ 1 + p 4 $\sigma$ 2 $\rho$ $\sigma$ 2 + p 4 $\sigma$ 3 $\rho$ $\sigma$ 3 . (3)

We can also see how an arbitrary Pauli channel acts on an arbitrary density matrix. To do it, we substitute Eq (1) in Eq (2):
E ( $\rho$ ) = 1 2 $\sum$ $\gamma$ , $\alpha$ = 0 3 k $\gamma$ r $\alpha$ $\sigma$ $\gamma$ $\sigma$ $\alpha$ $\sigma$ $\gamma$ . (4)
This can be simplified noting that
$\sigma$ $\gamma$ $\sigma$ $\alpha$ $\sigma$ $\gamma$ = A $\alpha$ , $\gamma$ $\sigma$ $\alpha$ , with A = ( 1 1 1 1 1 1 - 1 - 1 1 - 1 1 - 1 1 - 1 - 1 1 ) , (5)
which was obtained following the commutation relation of Pauli matrices [*$\sigma$*$\alpha$, *$\sigma$*$\beta$] = 2*i$\epsilon$*$\alpha$$\beta$$\gamma$*$\sigma$*$\gamma$ and *$\epsilon$*$\alpha$$\beta$$\gamma$ is the Levi-Civita tensor. Then, Eq (4) takes the form
E ( $\rho$ ) = 1 2 $\sum$ $\alpha$ ( $\sum$ $\gamma$ A $\alpha$ , $\gamma$ k $\gamma$ ) r $\alpha$ $\sigma$ $\alpha$ . (6) Eq (6) once again has the form of Eq (1) but with components ($\sum$$\gamma$ *A*$\alpha$,$\gamma$*k*$\gamma$)*r*$\alpha$. This gives us another way of understanding Pauli channels as operations that take each component *r*$\alpha$ of the density matrix and multiplies them by $\sum$$\gamma$ *A*$\alpha$,$\gamma$*k*$\gamma$, that is:
r $\alpha$ → Channel Pauli $\tau$ $\alpha$ r $\alpha$ , $\tau$ $\alpha$ $:=$ $\sum$ $\gamma$ A $\alpha$ , $\gamma$ k $\gamma$ . (7)
Notice that *$\tau$*$_{0}$ = 1, which is a consequence of $\sum$$\gamma$ *k*$\gamma$ = 1 and ensures that after the channel, the resulting density matrix still has trace one. Furthermore, reverting the definition of *$\tau$*$\alpha$ by using that A - 1 = 1 4 A, we get that k $\gamma$ = 1 4 $\sum$ $\alpha$ A $\alpha$ , $\gamma$ $\tau$ $\alpha$. Then, using that *k*$\gamma$ ≥ 0 we get the following conditions on the multipliers *$\tau$*$\alpha$:
1 + $\tau$ i - $\tau$ j - $\tau$ k ≥ 0 , for i , j , k different numbers in { 1 , 2 , 3 } , (8) 1 + $\tau$ 1 + $\tau$ 2 + $\tau$ 3 ≥ 0 . (9)

These conditions imply that (*$\tau$*$_{1}$, *$\tau$*$_{2}$, *$\tau$*$_{3}$) has to be inside a tetrahedron with vertices (1, 1, 1), (1, $-$1, $-$1), ($-$1, 1, $-$1) and ($-$1, $-$1, 1). Therefore, the *$\tau$*$_{1,2,3}$ are numbers between $-$1 and 1, which means that the components *r*$\alpha$ of the density matrix are always attenuated and possibly sign flipped.

Having defined the one qubit case, we can now generalize to *N* qubits. In order to do it, we need to introduce the so-called *Pauli strings*, defined as
$\sigma$ $\alpha$ → = $\sigma$ $\alpha$ 1 $\otimes$ $\sigma$ $\alpha$ 2 $\otimes$ $\cdots$ $\otimes$ $\sigma$ $\alpha$ N , (10)
where $\alpha$ → denotes a multi-index (*$\alpha$*$_{1}$, $\cdots$, *$\alpha$*$_{N}$) and *$\alpha$*$_{i}$ $\in$ {0, 1, 2, 3}. These operators form an orthogonal basis in the space of operators acting on *N* qubits. Similarly to the single-qubit case, the density matrix *$\rho$* of a system of *N* qubits can be written using Pauli strings as:
$\rho$ = 1 2 N $\sum$ $\alpha$ → r $\alpha$ → $\sigma$ $\alpha$ → . (11)
Then, just as before, we define a Pauli channel as a transformation that applies the operator $\sigma$ $\gamma$ → to *$\rho$* with probability k $\gamma$ → and is therefore described mathematically by:
E ( $\rho$ ) = $\sum$ $\gamma$ → k $\gamma$ → $\sigma$ $\gamma$ → $\rho$ $\sigma$ $\gamma$ → , (12)
where just as before, k $\gamma$ → are non-negative real numbers such that $\sum$ $\gamma$ → k $\gamma$ → = 1.

As in the one qubit case, Pauli channels for *N* qubits attenuate the components r $\alpha$ → of the density matrix. This can be seen by substituting Eq (11) in Eq (12) and using the property of Eq (5):
E ( $\rho$ ) = 1 2 N $\sum$ $\gamma$ → , $\alpha$ → k $\gamma$ → r $\alpha$ → $\sigma$ $\gamma$ → $\sigma$ $\alpha$ → $\sigma$ $\gamma$ → = 1 2 N $\sum$ $\alpha$ → ( $\sum$ $\gamma$ → ( A $\otimes$ N ) $\alpha$ → , $\gamma$ → k $\gamma$ → ) r $\alpha$ → $\sigma$ $\alpha$ → ,
which means that applying the Pauli channel multiplies the components r $\alpha$ → by $\tau$ $\alpha$ → $:=$ $\sum$ $\gamma$ → ( A $\otimes$ N ) $\alpha$ → , $\gamma$ → k $\gamma$ →.

### 2.3 Pauli dynamical maps

As seen in the last section, Pauli channels and in general quantum channels are discrete maps that transform a density matrix *$\rho$* into E ( $\rho$ ). However, we could also define a continuous set of channels *$\varepsilon$*$_{p}$ with *p* a real parameter.

For the special case of Pauli channels, we define a Pauli dynamical map as a continuous parametrized curve drawn inside the set of Pauli channels and starting at the identity channel. Therefore, a Pauli dynamical map can be written as
E p ( $\rho$ ) = $\sum$ $\gamma$ → k $\gamma$ → ( p ) $\sigma$ $\gamma$ → $\rho$ $\sigma$ $\gamma$ → , (13)
where *p* is a parameter in an interval [*a*, *b*] and E p is a Pauli channel for every *p*, with E a being the identity channel.

## 3 Circuit for a Pauli channel

In this section we propose a quantum circuit that simulates *N*-qubit Pauli channels. A quantum circuit is a model for representing computation on a quantum system. This computation may include preparing the initial state of the system, applying unitary operations and measuring the whole system or parts of it [[`ref22`]]. Our systems always consists of some number of qubits, that can be divided into the ones to which we will apply the quantum channel, or main qubits and the ones that help us complete said operation, or ancilla qubits. The circuit implements a unitary operator *U* on the system consisting of all the qubits and then the effect on the main qubits is measured. Moreover, we implement the circuit for *N* = 1 on a quantum computer and analyze the results using the diamond norm. We find that close to the depolarizing channel, the general circuit simulator can implement such channels with the highest fidelity.

### 3.1 Description of the circuit for a Pauli channel

To design the circuit that implements Eq (12), we construct a state that includes the probabilities *k*$\gamma$ on the ancilla qubits and subsequently apply controlled Pauli operations on the main qubits. The circuit that does this is presented in Fig 1.

```mermaid {span=wide pos=top width=0.66}
info:doi/10.1371/journal.pone.0297210.g001
```

The first part of the circuit involves the creation of the state
Ancilla state = $\sum$ $\gamma$ → b $\gamma$ → / $\gamma$ → > (14)
on the ancilla qubits, where b $\gamma$ → are numbers such that / b $\gamma$ → / 2 = k $\gamma$ → and the 2*N*-qubit state / $\gamma$ → > is defined as /*$\gamma$*$_{1}$>$\cdots$/*$\gamma$*$_{N}$>. When measured in the computational basis, the state given in Eq (15) collapses to / $\gamma$ → > with a probability / b $\gamma$ → / 2 = k $\gamma$ →. The circuit in Fig 1 uses this fact to apply $\sigma$ $\gamma$ → on the main qubits with a probability k $\gamma$ → by using controlled operations conditioned on the state of the system being / $\gamma$ → >, just as the Pauli channel is supposed to do.

### 3.2 Simulation for one-qubit Pauli channels

For the particular case of a Pauli channel on one qubit, the circuit that simulates it can be constructed as in Fig 2, which is a special case of Fig 1 but with all details explicitly shown. In said figure, the ancilla state of Eq (14) can be taken to be k 0 / 00 > + k 1 / 01 > + k 2 / 10 > + k 3 / 11 > and it is created on the ancilla qubits with the help of three rotations of angles defined by the following equations:
cos ( $\theta$ 0 2 ) = k 0 + k 1 , tan ( $\theta$ 1 + $\theta$ 2 2 ) = k 1 / k 0 , tan ( $\theta$ 2 - $\theta$ 1 2 ) = k 3 / k 2 . (15)

```mermaid {span=column pos=force width=1.0}
info:doi/10.1371/journal.pone.0297210.g002
```

We took a sample of one-qubit Pauli channels and evaluated their implementation on IBM's ibmq-lima quantum computer [[`ref29`]], as shown in Fig 3. For each of the channels sampled, we used quantum process tomography [[`ref29`], [`ref30`]] to obtain the operator *$\xi$*$_{I}$ corresponding to the implementation of the circuit in the quantum computer. Then, we compared *$\xi$*$_{I}$ with the theoretical operator *$\xi$*$_{T}$ of the Pauli channel we wanted to implement. To see how close the operators *$\xi$*$_{I}$ and *$\xi$*$_{T}$ are, we shall use the diamond distance [[`ref31`]], which is defined by
/ / $\xi$ I - $\xi$ T / / $\diamond$ = max $\rho$ / / ( $\xi$ I $\otimes$ I ) $\rho$ - ( $\xi$ T $\otimes$ I ) $\rho$ / / 1 , (16)
with *I* the identity map, // $\cdot$ //$_{1}$ the trace norm and the maximization done over all density matrices *$\rho$*. The calculation of this norm is done using the semi-definite program from reference [[`ref32`]]. When the two channels are the same, the diamond distance has a value of 0, while in the case that the channels are completely distinguishable, the distance reaches its maximum value of 2 [[`ref33`]]. For the analysis done in Fig 3, we define a sort of "diamond fidelity" as:
f = 1 - 1 2 / / $\varepsilon$ I - $\varepsilon$ T / / $\diamond$ , (17)
which ranges from 0, when the channels have a maximum distance, to 1, when they are exactly equal.

```mermaid {span=wide pos=top width=0.68}
info:doi/10.1371/journal.pone.0297210.g003
```

Finally, using the representation of Pauli channels in a tetrahedron as in Eq (8), we show in Fig 3 the diamond fidelity defined by Eq (17) for the channels analyzed. We can see that channels close to the completely depolarizing channel (that is close to the center of the tetrahedron) have a high *f*, while those close to unitary channels have much lower *f*. This is reasonable because quantum computers are prone to errors that depolarize qubits, which isn't very problematic when trying to simulate depolarization but it is when simulating unitary processes. Moreover, as can be seen in Fig 3, the algorithm of Fig 2 is not optimal for channels in the edges of the tetrahedron, which corresponds to unitary channels. These channels could be accomplished more efficiently by simply applying the corresponding unitary operator directly. In this case we would be trading generality for fidelity. The main advantage of the algorithm is that we can simulate with a unitary circuit any Pauli channel, only varying a couple of angles in the circuit. In fact, as we will see, sometimes only changing one angle in the circuit will be enough to simulate many non-unitary dynamical maps of physical interest.

### 3.3 Simulation of a two-qubit Pauli channel

For the general case of a two-qubit Pauli channel, the circuit of Fig 1 requires two main qubits and four ancilla qubits. However, the quantum computer which we are using has only 5 qubits. We could still simulate a two qubit Pauli channel that applies at most 8 Pauli operations, instead of 16 which would be the most general case, using the three ancilla qubits. For example, consider the following two-qubit dephasing channel:
$\varepsilon$ ( $\rho$ ) = ( 1 - p ) $\rho$ + p 3 ( I $\otimes$ $\sigma$ z ) $\rho$ ( I $\otimes$ $\sigma$ z ) + p 3 ( $\sigma$ z $\otimes$ I ) $\rho$ ( $\sigma$ z $\otimes$ I ) + p 3 ( $\sigma$ z $\otimes$ $\sigma$ z ) $\rho$ ( $\sigma$ z $\otimes$ $\sigma$ z ) , (18)
where *p* $\in$ [0, 1] is a parameter that defines the channel. Since this channel has four Pauli operations, only 2 ancilla qubits will be necessary, and thus it can be implemented using the circuit of Fig 4b.

```mermaid {span=column pos=here width=1.0}
info:doi/10.1371/journal.pone.0297210.g004
```

Just as for the one-qubit Pauli channels, this channel was implemented on IBM's ibmq-lima quantum computer. We obtained the Choi matrix by doing quantum process tomography on the main qubits. Then, we calculated the diamond fidelity for different values of *p* and obtained the results shown in Fig 4a. As can be seen in the figure, the fidelity doesn't vary much as we change the value of *p*. The average fidelity over all values of *p* is 0.758 and it is biggest when *p* = 0, corresponding to the identity channel, where it reaches a fidelity of 0.874.

## 4 One parameter circuits

Finally, we consider the simulation of Pauli dynamical maps. This is pretty much already solved, since just as Pauli channels, Pauli dynamical maps can be implemented using the circuit of Fig 1. However, there is one difference: the state to be created on the ancilla qubits now depends on a parameter *p*, and it is represented by the expression:
$\sum$ $\gamma$ → b $\gamma$ → ( p ) / $\gamma$ → > . (19)
Thus, we temporarily shift our focus from Pauli channels and dynamical maps to the general problem of creating a circuit to generate a curve of states like the one described in Eq (19).

In general, producing this curve of states for *N* qubits will require many rotations parametrized by *p*, such as the three rotations used for the ancilla qubits in Fig 2. However, it would be preferable to achieve the same effect using only one parametrized rotation. This would allow us to interpret said rotation as a knob that smoothly traverses the curve of states. Consequently, we are faced with the question of which curves of states, such as the one described in Eq (19), can be produced using just a single parametrized rotation. To clarify this, we provide the following definition for a circuit with one parametrized rotation.

**Definition 1** *1-Parameter Rotation Circuit:* *A 1-Parameter Rotation (1PR) circuit is a parametrized quantum circuit that includes only one gate dependent on a parameter p. Moreover, the parametrized gate is a one-qubit rotation about any axis, whether controlled or not*.

Therefore, a 1PR circuit implements a unitary operator *U*(*p*) on some number *N* of qubits, such that *U* depends on the parameter *p* only locally on one specific qubit.

Based on this definition, we aim to determine which curves of states can be generated using 1PR circuits. To accomplish this, we begin by proving that all 1PR circuits have the form depicted in Fig 5, where the parametrized rotation is around *$\sigma$*$_{3}$ and is applied to the last qubit.

```mermaid {span=column pos=force width=1.0}
info:doi/10.1371/journal.pone.0297210.g005
```

**Theorem 1** *An N* $-$ *qubit 1PR circuit can always be transformed into the form shown in* Fig 5.

**Proof:** First, we observe that according to the definition, a 1PR circuit always consists of an operation *B* followed by the parametrized rotation on a specific qubit and then another operation *A*, where *A* and *B* are not parametrized.

Next, we note that it is not necessary to consider rotations about an arbitrary axis, as a rotation about any axis n ^ parameterized by *p* can be transformed into a rotation about *$\sigma$*$_{3}$ without introducing gates that depend on *p*. To see this, consider the rotation R n ^ ( 2 s ), where 2*s* is a function of *p* (the factor of 2 is for convenience later on) and n ^ = ( n 1 , n 2 , n 3 ) represents the rotation axis. We can express n ^ as (sin *$\theta$* cos *$\phi$*, sin *$\theta$* sin *$\phi$*, cos *$\theta$*), where *$\theta$* and *$\phi$* are fixed angles dependent on n ^. The rotation can then be rewritten as follows:
R n ^ ( 2 s ) = R $\sigma$ 3 ( $\phi$ ) R $\sigma$ 2 ( $\theta$ ) R $\sigma$ 3 ( 2 s ) R $\sigma$ 2 ( - $\theta$ ) R $\sigma$ 3 ( - $\phi$ ) . (20)
Since the angles *$\theta$* and *$\phi$* do not depend on the parameter *p*, any 1PR circuit can be transformed into a circuit where the parametrized rotation is around *$\sigma$*$_{3}$ instead of an arbitrary axis. Moreover, without loss of generality, we can choose the last qubit as the target qubit for the rotation, since if it is not, we could use swap gates to move the rotation to the first qubit without adding gates that depend on *p*.

Therefore, a 1PR circuit can be transformed such that the rotation is around *$\sigma$*$_{3}$ and is applied to the last qubit (possibly controlled by other qubits), resulting in the form depicted in Fig 5.

With the aid of this theorem, we can now determine the curves of states of *N* qubits that can be generated using a 1PR circuit. This result is stated in the following theorem.

**Theorem 2** *Consider a 1PR circuit of N qubits parametrized by p and denote by U*(*p*) *the operator it implements on this system. Then, for every j* $\in$ {0, 1, $\cdots$, 2$^{N}$ $-$ 1}, *we have that*:
U ( p ) / j > = e i s ( p ) / a j > + e - i s ( p ) / b j > + / c j > (21) *with s*(*p*) *some function of p*, /*a*$^{j}$>, /*b*$^{j}$>, /*c*$^{j}$> *orthogonal states and* <*a*$^{j}$/*a*$^{j}$> + <*b*$^{j}$/*b*$^{j}$> + <*c*$^{j}$/*c*$^{j}$> = 1. *Here* /*j*> *represents an element of the N-qubit computational basis*.

**Proof:** As before, from the definition of a 1PR circuit, we know that *U*(*p*) can be decomposed as *ARB*, where *A*, *B* are unitary operators acting on the *N*-qubit system and *R* is a rotation applied to one qubit. From theorem 1, we conclude that *R* can be taken in general as a *$\sigma$*$_{3}$ rotation of angle 2*s*(*p*) applied to the last qubit and controlled by some of the other ones. To prove this theorem, we will apply *U*(*p*) to /*j*> by first applying *B*, then R $\sigma$ 3 ( 2 s ( p ) ) and finally *A*, with the goal of getting to Eq (21).

First, applying *B* to /*j*> results in B/j>=B0,j/0>+B1,j/1>+$\cdots$+B2n$-$1,j/2n$-$1>, with *B*$_{i,j}$ the entries of matrix *B*. This can be rewritten by separating the last qubit from the other *N* $-$ 1:
B / j > = $\sum$ k = 0 2 N - 1 - 1 ( B 2 k , j / k > / 0 > + B 2 k + 1 , j / k > / 1 > ) . (22)
After the operator *B*, the circuit applies the controlled rotation R $\sigma$ 3 ( 2 s ( p ) ). To simplify the analysis, we separate the states of the first *N* $-$ 1 qubits into those that fulfill the control conditions of the rotation (which we denote as the set C) and those that do not, and write it as
B / j > = $\sum$ k $\in$ C ( B 2 k , j / k > / 0 > + B 2 k + 1 , j / k > / 1 > ) + $\sum$ k $\notin$ C ( B 2 k , j / k > / 0 > + B 2 k + 1 , j / k > / 1 > ) . (23)

Then, the rotation *R* will only affect the states on the first sum (since they fulfill the control conditions) and not the others. Therefore, using that a R $\sigma$ 3 ( 2 s ( p ) ) rotation acts by adding a phase *e*$-$is(p) to /0> and a phase *e*$^{is(p)}$ to /1>, we have that,
R $\sigma$ 3 ( 2 s ( p ) ) B / j > = e - i s ( p ) $\sum$ k $\in$ C B 2 k , j / k > / 0 > + e i s ( p ) $\sum$ k $\in$ C B 2 k + 1 , j / k > / 1 > + $\sum$ k $\notin$ C ( B 2 k , j / k > / 0 > + B 2 k + 1 , j / k > / 1 > ) = e - i s ( p ) / b ~ j > + e i s ( p ) / a ~ j > + / c ~ j > , (24)
where we defined
/ a ~ j > = $\sum$ k $\in$ C B 2 k , j / k > / 1 > , / b ~ j > = $\sum$ k $\in$ C B 2 k + 1 , j / k > / 0 > , / c ~ j > = $\sum$ k $\notin$ C ( B 2 k , j / k > / 0 > + B 2 k + 1 , j / k > / 1 > ) .
These states are clearly orthogonal because they are each linear combinations of different orthogonal states of the computational basis. Moreover, they satisfy <*a*$^{j}$/*a*$^{j}$> + <*b*$^{j}$/*b*$^{j}$> + <*c*$^{j}$/*c*$^{j}$> = 1 because this quantity is the squared norm of the *j*th column of *B*, which is unitary.

Finally, after having applied the rotation, the circuit applies gate *A*, so that the result is given by:
U / j > = A R $\sigma$ 3 ( 2 s ( p ) ) B / j > = e - i s ( p ) A / a ~ j > + e i s ( p ) A / b ~ j > + A / c ~ j > = e - i s ( p ) / a j > + e i s ( p ) / b j > + / c j > , (25)
where / a j > = A / a ~ j > , / b j > = A / b ~ j > , / c j > = A / c ~ j > are still orthogonal states that satisfy <*a*/*a*> + <*b*/*b*> + <*c*/*c*> = 1 because *A* is unitary.

This theorem implies that when starting from the state /0> or any other initial state, the only possible curves of states that can be created using a 1PR circuit are of the following form:
/ $\eta$ ( p ) > = / c > + e i s ( p ) / a > + e - i s ( p ) / b > , (26)
with conditions defined by the equations:
< a / a > + < b / b > + < c / c > = 1 , < a / b > = < a / c > = < b / c > = 0 . (27)

Moreover, it is possible to construct a 1PR circuit to generate any given curve of states described by Eq (26). One approach to achieve this is by utilizing the circuit depicted in Fig 5, with the parametrized rotation *R* applied to the last qubit controlled by all the other qubits. There is some freedom when choosing the operators *A* and *B*, we only need to make sure that:
B / 0 > = < a / a > / 2 N - 1 > + < b / b > / 2 N - 2 > + < c / c > / 2 N - 3 > , A / 2 N - 1 > = 1 < a / a > / a > , A / 2 N - 2 > = 1 < b / b > / b > , A / 2 N - 3 > = 1 < c / c > / c > . (28)
The remaining part of the operators *A* and *B* can be chosen in any arbitrary manner as long as they are unitary. Examples of these matrices are shown for particular dynamical maps in the next section.

To see that A R $\sigma$ 3 ( s ( p ) ) B / 0 > in fact creates the curve of states described in Eq (26), we can rewrite the expression of *B*/0> by separating the first *N* $-$ 1 qubits from the last one:
B / 0 > = < a / a > / 2 N - 1 - 1 > / 1 > + < b / b > / 2 N - 1 - 1 > / 0 > + < c / c > / 2 N - 1 - 2 > / 1 > .
Since the parametrized rotation R $\sigma$ 3 ( 2 s ( p ) ) is controlled by all the first *N* $-$ 1 qubits, it only applies to the first two terms of *B*/0>. As a result, we obtain:
R $\sigma$ 3 ( 2 s ( p ) ) B / 0 > = < a / a > e i s ( p ) / 2 N - 1 - 1 > / 1 > + < b / b > e - i s ( p ) / 2 N - 1 - 1 > / 0 > + < c / c > / 2 N - 1 - 2 > / 1 > .

Finally, applying the operator *A* to this state yields:
A R $\sigma$ 3 ( 2 s ( p ) ) B = / c > + e i s ( p ) / a > + e - i s ( p ) / b > .
With this, we prove that any curve of states as that in Eq (26) can be constructed by the circuit in Fig 5 with the parametrized rotation controlled by all the other qubits by correctly choosing matrices *A* and *B*.

## 5 1PR circuit for a Pauli map

We can now use the previous results to conclude directly which Pauli dynamical maps can be implemented with a 1PR circuit. For this, the curve of states of Eq (19) has to be constructed with only one parametrized rotation, so it has to satisfy the conditions of theorem 2. Therefore, this implies that the map
$\varepsilon$ p ( $\rho$ ) = $\sum$ $\gamma$ → k $\gamma$ → ( p ) $\sigma$ $\gamma$ → $\rho$ $\sigma$ $\gamma$ → , (29)
can be implemented if there are numbers $\beta$ $\gamma$ → ( p ) such that / $\beta$ $\gamma$ → ( p ) / 2 = k $\gamma$ → ( p ) and
$\sum$ $\gamma$ → $\beta$ $\gamma$ → ( p ) / $\gamma$ → > = / c > + e i s ( p ) / a > + e - i s ( p ) / b > , (30)
where /*a*>, /*b*>, /*c*> fulfill the conditions of Eq (27).

For the particular case of one qubit, we can show some examples of Pauli dynamical maps implementable with a 1PR circuit, which are plotted in Fig 6. The examples we show include some of the most common maps: the bit flip, phase flip, bit-phase flip and depolarizing. However, we also include the parabolic dynamical map, defined in Eq (32) and shown in Fig 6. This map traces a parabola inside the tetrahedron connecting two of its vertices and it describes a frontier in the tetrahedron between Pauli channels that are reachable by Lindbladian dynamics and those that are not [[`ref19`]].

**Depolarizing:** This dynamical map is given by
$\varepsilon$ p ( $\rho$ ) = ( 1 - 3 p / 4 ) $\rho$ + ( p / 4 ) $\sigma$ 1 $\rho$ $\sigma$ 1 + ( p / 4 ) $\sigma$ 2 $\rho$ $\sigma$ 2 + ( p / 4 ) $\sigma$ 3 $\rho$ $\sigma$ 3 , (31)
with *p* $\in$ [0, 1]. Therefore, the curve of states /*$\beta$*(*p*)> needed on the ancilla qubits is such that /*$\beta$*$_{0}$(*p*)/$^{2}$ = (1 $-$ 3*p*/4), /*$\beta$*$_{1}$(*p*)/$^{2}$ = /*$\beta$*$_{2}$(*p*)/$^{2}$ = /*$\beta$*$_{3}$(*p*)/$^{2}$ = *p*/4. Then, taking the *$\beta$*$_{j}$ to be real, the curve of states can be
/ $\beta$ ( p ) > = 1 - 3 p / 4 / 0 > + p / 4 / 1 > + p / 4 / 2 > + p / 4 / 3 > .
This state can be rewritten as:
/ $\beta$ ( p ) > = e i s ( 1 2 / 0 > - i 2 3 / 1 > - i 2 3 / 2 > - i 2 3 / 3 > ) + e - i s ( 1 2 / 0 > + i 2 3 / 1 > + i 2 3 / 2 > + i 2 3 / 3 > ) $\equiv$ e i s / a > + e - i s / b >
with sin s = 3 p / 4. We can see that this curve satisfies the conditions of Eq (27), meaning that it can be created with a 1PR circuit. Explicitly, following the discussion after theorem 2, we know that this curve of states can be constructed by the circuit of Fig 7.
The circuit requires defining *A* and *B* as in Eq (28), and for the particular vectors /*a*>, /*b*>, /*c*> of this dynamical map, we have that
A = ( 0 0 1 / 2 1 / 2 1 / 6 - 1 / 2 i / 6 - i / 6 1 / 6 1 / 2 i / 6 - i / 6 - 2 / 3 0 i / 6 - i / 6 ) , B = ( 0 0 0 1 0 0 1 0 1 / 2 - 1 / 2 0 0 1 / 2 1 / 2 0 0 ) ,
where, as mentioned before, the first column of *A* and the last three of *B* can be chosen arbitrarily as long as the resultant matrices are unitary. We ran this circuit in IBM's quantum computer ibmq-lima for 25 different values of *p* between 0 and 1, with 20 repetitions for each value of *p*. The diamond fidelity was calculated in each case and is shown in Fig 8.

**Parabolic dynamical map:** We define the parabolic dynamical map as:
$\epsilon$ ( $\rho$ ) = ( 1 - p ) 2 $\rho$ + ( p - p 2 ) $\sigma$ 1 $\rho$ $\sigma$ 1 + ( p - p 2 ) $\sigma$ 2 $\rho$ $\sigma$ 2 + p 2 $\sigma$ 3 $\rho$ $\sigma$ 3 , (32)
with *p* $\in$ [0, 1]. If we take the *$\beta$*$_{j}$ to be real, the curve of states needed on the ancilla qubits can be:
/ $\beta$ ( p ) > = ( 1 - p ) / 0 > + p - p 2 / 1 > + p - p 2 / 2 > + p / 3 > . (33)
This can be rewritten as
/ $\beta$ ( p ) > = ( 1 2 / 0 > + 1 2 / 3 > ) + e i s ( i 4 / 0 > + 1 4 / 1 > + 1 4 / 2 > - i 4 / 3 > ) + e - i s ( - i 4 / 0 > + 1 4 / 1 > + 1 4 / 2 > + i 4 / 3 > ) $\equiv$ / c > + e i s / a > + e - i s / b > ,
with sin *s* = 2*p* $-$ 1, so that this curve fulfills the conditions of Eq (27). This means that it can be created with the 1PR circuit of figure Fig 7. In this case, matrices *A* and *B* can be taken to be:
A = ( 0 1 / 2 - i / 2 i / 2 - 1 / 2 0 1 / 2 1 / 2 1 / 2 0 1 / 2 1 / 2 0 1 / 2 i / 2 - i / 2 ) , B = ( 0 1 0 0 1 / 2 0 0 - 1 / 2 1 / 2 0 1 / 2 1 / 2 1 / 2 0 - 1 / 2 1 / 2 ) ,
where again, the first column of *A* and the last three of *B* can be chosen arbitrarily. As before, we ran this circuit for 25 values of *p* between 0 and 1, with 20 repetitions for each, obtaining as a result the fidelities shown in Fig 8.

**Bit flip map:** This dynamical map is defined as
$\varepsilon$ p ( $\rho$ ) = ( 1 - p ) $\rho$ + p $\sigma$ 1 $\rho$ $\sigma$ 1 ,
for *p* $\in$ [0, 1]. In particular, if we take the *$\beta$*$_{j}$ to be real, we need to create the curve of states:
/ $\beta$ ( p ) > = 1 - p / 0 > + p / 1 > .
This can be rewritten as
/ $\beta$ ( p ) > = e i s ( 1 2 / 0 > - i 2 / 1 > ) + e - i s ( 1 2 / 0 > + i 2 / 1 > ) ,
with sin s = p. Therefore, we can see that the curve satisfies the conditions of Eq (27) and it can be created with a 1PR circuit. Matrices *A* and *B* can be obtained as for the other dynamical maps and we ran the resulting circuit in ibmq-lima with the same specifications as for the other examples, with the results are shown in Fig 8.
The exact same thing can be done for the phase flip and bit phase flip dynamical maps by changing *$\sigma$*$_{1}$ to *$\sigma$*$_{3}$ and *$\sigma$*$_{2}$ respectively. For example, the bit phase flip map was implemented in [[`ref13`], [`ref34`]] using an optical arrangement, and it was indeed done by varying only one angle that depends on the parameter *p* (the angle of a half waveplate).

```mermaid {span=column pos=force width=1.0}
info:doi/10.1371/journal.pone.0297210.g006
```

```mermaid {span=column pos=force width=1.0}
info:doi/10.1371/journal.pone.0297210.g007
```

```mermaid {span=wide pos=top width=0.84}
info:doi/10.1371/journal.pone.0297210.g008
```

From the results of Fig 8, we can see that just as in figure Fig 3, fidelities are high for points close to the depolarizing channel (*p* = 1 for depolarizing map, *p* = 1/2 for parabolic map) and lower for points close to unitary channels (*p* = 0 for all maps and *p* = 1 for parabolic and bit flip). Furthermore, we can see that the actual values of the fidelities are comparable to those of Fig 3, which means that using the 1PR circuit as opposed to implementing each channel with Fig 2 does not considerably affect fidelity.

Furthermore, we can construct other examples of Pauli dynamical maps such that they can be implemented with a 1PR circuit. To do it, we only need to choose the three states /*a*>, /*b*>, /*c*> that satisfy the conditions of Eq (27). For example, this can be done systematically for the case of curves of states of two qubits (that is, for Pauli dynamical maps of one qubit) with the following procedure:

We first choose the norms /*a*/, /*b*/, /*c*/ such that <*a*/*a*> + <*b*/*b*> + <*c*/*c*> = 1. This can be done by selecting two angles *$\mu$* $\in$ [0, *$\pi$*/2], *$\nu$* $\in$ [0, *$\pi$*/2] and defining:
/ a / = sin $\nu$ cos $\mu$ , / b / = sin $\nu$ sin $\mu$ , / c / = cos $\nu$ .

We define /*a*'> = /*a*//0>, /*b*'> = /*b*//1>, /*c*'> = /*c*//2>.

Finally, we choose a unitary matrix *V* with the condition that its first row is equal to *e*i$\theta$(/*a*/, /*b*/, /*c*/, 0) with *$\theta$* a uniform random phase. That way, we can define /*a*> = *V*/*a*'>, /*b*> = *V*/*b*'>, /*c*> = *V*/*c*'> and since *V* is unitary, these unprimed vectors will fulfill the conditions of Eq (27). Furthermore, the form of the first row ensures that the dynamical map begins at the identity, since it implies that when *s* = 0, the state created in Eq (30) is /*a*> + /*b*> + /*c*> = *e*i$\theta$/0>, which corresponds with applying the identity channel.
Such a matrix *V* can be randomly constructed by first finding three vectors w → 1 , w → 2 , w → 3 orthogonal to the first row using the Gram-Schmidt process. Then selecting random complex numbers *r*$_{1}$, *r*$_{2}$, *r*$_{3}$ such that /*r*$_{1}$/$^{2}$ + /*r*$_{2}$/$^{2}$ + /*r*$_{3}$/$^{2}$ = 1 and defining the second row of *V* to be r 1 w → 1 + r 2 w → 2 + r 3 w → 3. Once the first two rows are chosen, use Gram-Schmidt to find two vectors v → 1 , v → 2 orthogonal to them and similarly define the third row as q 1 v → 1 + q 2 v → 2 with /*q*$_{1}$/$^{2}$+ /*q*$_{2}$/$^{2}$ and *q*$_{1}$, *q*$_{2}$ selected at random. Finally, there is only one choice for the fourth row so that it is orthonormal to the first three and a random phase can be given to it.

Following this procedure for random angles and unitary matrices *V*, we plot four Pauli dynamical maps selected at random that can be implemented with a 1PR circuit in Fig 6.

## 6 Conclusion

In this work, we found a quantum algorithm for simulating Pauli channels in *N*-qubit systems and generalized it to Pauli dynamical maps by using parametrized quantum circuits. Furthermore, we implemented single-qubit Pauli channels on one of IBM's quantum computers and obtained their fidelities.

Then, when working with Pauli dynamical maps, we searched for a way of minimizing the amount of parametric operations in the circuit by requiring that only one single-qubit rotation depends on the parameter. In theorem 2 we found the general mathematical conditions for this, applicable to any parametrized circuit. Given this condition, we considered the space of Pauli maps that are possible to parametrize with one parameter, and gave some explicit examples for a system of one qubit. As the number of qubits gets bigger, the degrees of freedom for choosing a Pauli dynamical map also gets bigger, so we would expect that the space of maps that can be done with only one parameter becomes a relatively smaller part of the whole space of Pauli dynamical maps. However, some paradigmatic maps as the dephasing one we studied for 2 qubits will still be possible to do with a 1PR circuit (for this case, this can be seen by noticing that said dephasing channel has the same form as the depolarizing channel for one qubit, which we proved to be possible to implement with a 1PR circuit).

In conclusion, this work presents yet another example of the current exploration into simulating open quantum systems in quantum computers, and we observe the effect that the error of a quantum computer have on these simulations, quantified by the fidelity. On the other hand, the result of theorem 2 shows what can be done with the condition of using only one parametrized rotation and can be applied to any quantum algorithm that requires parametrized circuits, such as those used for quantum machine learning [[`ref21`]].
