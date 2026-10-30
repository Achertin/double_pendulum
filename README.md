# Double Pendulum

Real-time simulation of a chaotic double pendulum in Rust. The equations of motion are integrated with a hand-written fourth-order Runge–Kutta (RK4) scheme and drawn with [macroquad](https://docs.rs/macroquad).

![Double pendulum animation](double_pendulum.gif)

## What it does

- Describes the pendulum by its state $(\theta_1, \theta_2, \omega_1, \omega_2)$: the angle of each arm from the vertical and its angular velocity
- Advances the state every frame with one RK4 step, using [nalgebra](https://docs.rs/nalgebra) vectors
- Draws both arms in real time
- Starts two identical arms (1 m, 2 kg) almost horizontal and 1° apart, which quickly shows the chaotic motion

## Equations of motion

Writing the problem in terms of the two angles gives a system of four first-order ODEs:

$$\dot\theta_1 = \omega_1, \qquad \dot\theta_2 = \omega_2, \qquad \dot\omega_1 = \alpha_1(\theta_1, \theta_2, \omega_1, \omega_2), \qquad \dot\omega_2 = \alpha_2(\theta_1, \theta_2, \omega_1, \omega_2)$$

with the angular accelerations

$$\alpha_1 = \frac{-g(2m_1+m_2)\sin\theta_1 - m_2 g\sin(\theta_1-2\theta_2) - 2\sin(\theta_1-\theta_2)\,m_2\left(\omega_2^2 L_2 + \omega_1^2 L_1\cos(\theta_1-\theta_2)\right)}{L_1\left(2m_1+m_2-m_2\cos(2\theta_1-2\theta_2)\right)}$$

$$\alpha_2 = \frac{2\sin(\theta_1-\theta_2)\left(\omega_1^2 L_1(m_1+m_2) + g(m_1+m_2)\cos\theta_1 + \omega_2^2 L_2 m_2\cos(\theta_1-\theta_2)\right)}{L_2\left(2m_1+m_2-m_2\cos(2\theta_1-2\theta_2)\right)}$$

These are the equations in the form given on [myPhysicsLab](https://www.myphysicslab.com/pendulum/double-pendulum).

## Numerical method

Each frame, the classical RK4 scheme advances the state $S$ by a time step $\Delta t$:

$$S_{n+1} = S_n + \frac{\Delta t}{6}\left(k_1 + 2k_2 + 2k_3 + k_4\right)$$

where $k_1$ to $k_4$ are the time derivatives evaluated at the start, twice at the midpoint, and at the end of the step. RK4 is fourth-order accurate, so it stays stable far longer than a simple Euler step, where errors quickly pile up in a chaotic system like this one.

## How it came about

I started by modelling the forces on each bob on paper (F = ma) and integrating them directly with a simple time step. That first version became unstable within seconds: the pendulum shook itself apart, as the early commits in this repository show. Rewriting the problem in terms of the two angles and integrating it with RK4 fixed it.

## Limitations and next steps

- The time step is the frame time, so the motion depends slightly on the frame rate. A fixed time step with several RK4 sub-steps per frame would decouple the physics from the rendering.
- Total energy is not tracked. Plotting it over time would show how well the integrator conserves it.

## References

- [myPhysicsLab: Double Pendulum](https://www.myphysicslab.com/pendulum/double-pendulum), for the form of the equations of motion
- [macroquad documentation](https://docs.rs/macroquad), for the window and drawing
- [nalgebra documentation](https://docs.rs/nalgebra), for the vector types

## A note on tools

I wrote the simulation myself, working from the documentation of the libraries above. Afterwards, I used Claude (Anthropic's AI assistant) to write this README.
