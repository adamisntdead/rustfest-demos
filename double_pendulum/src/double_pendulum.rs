pub struct DoublePendulumLagrangian {
    /// Gravitational Constant.
    pub g: f64,
    /// Mass of the first bob.
    pub m1: f64,
    /// Mass of the second bob.
    pub m2: f64,
    /// Initial angle of the first bob.
    pub t1: f64,
    /// Initial angle of the second bob.
    pub t2: f64,
    /// Angular velocity of the first bob.
    pub dt1: f64,
    /// Angular velocity of the second bob.
    pub dt2: f64,
    /// Length of the rod for the first bob.
    pub l1: f64,
    /// Length of the rod for the second bob.
    pub l2: f64,
}

impl DoublePendulumLagrangian {
    /// Advances one time step using RK4 (classical Runge-Kutta
    /// method).
    pub fn time_step(&mut self, dt: f64) {
      let g = self.g;
      let m1 = self.m1;
      let m2 = self.m2;
      let t1 = self.t1;
      let t2 = self.t2;
      let l1 = self.l1;
      let l2 = self.l2;
      let dt1 = self.dt1;
      let dt2 = self.dt2;

      let mu = 1.0 + m1 / m2;

      let d2t1 = (g * (t2.sin() * (t1 - t2).cos() - mu * t1.sin()) - (l2 * dt2 * dt2 + l1 * dt1 * dt1 * (t1 - t2).cos()) * (t1 - t2).sin()) / (l1 * (mu - (t1 - t2).cos() * (t1 - t2).cos()));
      let d2t2 = (mu * g * (t1.sin() * (t1 - t2).cos() - t2.sin()) - t2.sin() + (mu * l1 * dt1 * dt1 + l2 * dt2 * dt2 * (t1 - t2).cos()) * (t1 - t2).sin()) / (l2 * (mu - (t1 - t2).cos() * (t1-t2).cos()));
      self.dt1 += d2t1 * dt;
      self.dt2 += d2t2 * dt;

      self.t1 += self.dt1 * dt;
      self.t2 += self.dt2 * dt;
    }
}
