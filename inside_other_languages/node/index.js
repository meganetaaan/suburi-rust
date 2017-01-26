const ffi = require('ffi')

const lib = ffi.Library('../embed/target/release/libembed', {
  'process': ['void', []]
})

lib.process()

console.log('done!')
