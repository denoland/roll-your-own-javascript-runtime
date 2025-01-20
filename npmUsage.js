import get from 'lodash/get'

const o = { alpha: 42 }

console.log(`alpha: ${get(o, 'alpha')}`)
