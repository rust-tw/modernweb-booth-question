import 'xterm/dist/xterm.css'

import { Terminal } from 'xterm'

const term = new Terminal()
window.term = term

const $root = document.querySelector('#root')
term.open($root)
term.focus()

import('./game')
  .then(({ start }) => {
    start(term)
  })
  .catch(console.error)
