import('../pkg/index.js').then(module => {
  let width = window.innerWidth
  let height = window.innerHeight

  module.main_web(width, height)

  // window.addEventListener('resize', function () {
  //   let width = window.innerWidth
  //   let height = window.innerHeight
  //   module.print_wh(width, height)
  // })
})
