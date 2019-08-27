import { Game } from '../pkg/index.js'
import question from './question.json'

export function start(term) {
  let ended = false
  const game = new Game(question.length)
  console.log(game)
  game.next_question(question.shift())
  term.write(game.render())
  let buf = ''
  term.on('data', data => {
    if (ended) {
      return
    }
    if (data === '\b') {
      buf = buf.slice(0, -1)
    } else {
      buf += data
    }
    term.write(data)
    console.log(JSON.stringify(data))
    if (!buf.endsWith('\r')) {
      return
    }

    const { correct, message } = game.input(buf)
    console.log(correct)
    term.write('\n' + message)
    buf = ''
    if (correct) {
      const subject = question.shift()
      if (subject) {
        game.next_question(subject)
        term.write(game.render())
      } else {
        term.write(game.end())
        ended = true
      }
    } else {
      term.write(game.render())
    }
  })
}
