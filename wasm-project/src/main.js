const { DoublePendulumLagrangian } = wasm_bindgen;

function runApp() {
  const elem = document.getElementById('container')
  const two = new Two({ fullscreen: true }).appendTo(elem)

  const g = 9.8
  const m1 = 2.0
  const m2 = 2.0
  const t1 = 2.0
  const t2 = 1.5
  const l1 = 100
  const l2 = 100

  const pendulum = DoublePendulumLagrangian.new(g, m1, m2, t1, t2, l1, l2)
  drawBobs(two, pendulum)
}

function drawBobs(two, pendulum) {
  two.bind('update', frameCount => {
    pendulum.time_step(0.1)

    two.clear()

    const x1 = pendulum.l1 * Math.sin(pendulum.t1)
    const y1 = pendulum.l1 * Math.cos(pendulum.t1)

    const x2 = x1 + (pendulum.l2 * Math.sin(pendulum.t2))
    const y2 = y1 + (pendulum.l2 * Math.cos(pendulum.t2))

    const bob1Line =  two.makeLine(0, 0, x1, y1)
    const bob2Line =  two.makeLine(x1, y1, x2, y2)

    const bob1Circle = two.makeCircle(x1, y1, pendulum.m1 * 5)
    const bob2Circle = two.makeCircle(x2, y2, pendulum.m2 * 5)

    bob1Circle.fill = '#000000'
    bob2Circle.fill = '#000000'

    const group = two.makeGroup(bob1Circle, bob2Circle, bob1Line, bob2Line)
    group.translation.set(two.width / 2, two.height / 2)
    group.scale = 1
    group.linewidth = 5
  }).play()
}

wasm_bindgen('../out/main_bg.wasm').then(runApp).catch(console.error)