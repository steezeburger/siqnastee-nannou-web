import('../pkg/index.js').then(module => {
  let width = window.innerWidth
  let height = window.innerHeight

  module.main_web(width, height)
})
