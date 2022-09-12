import 'regenerator-runtime/runtime'
import React from 'react'

import './assets/css/global.css'

import {login, logout, get_habits, add_habit} from './assets/js/near/utils'
import getConfig from './assets/js/near/config'


export default function App() {
  // use React Hooks to store habit in component state
  const [habit, setHabits] = React.useState()

  // when the user has not yet interacted with the form, disable the button
  const [buttonDisabled, setButtonDisabled] = React.useState(true)

  // after submitting the form, we want to show Notification
  const [showNotification, setShowNotification] = React.useState(false)

  // The useEffect hook can be used to fire side-effects during render
  // Learn more: https://reactjs.org/docs/hooks-intro.html
  React.useEffect(
    () => {
      // get_greeting is in near/utils.js
      get_habits()
        .then(habitsFromContract => {
          setHabits(habitsFromContract)
        })
    },

    // The second argument to useEffect tells React when to re-run the effect
    // Use an empty array to specify "only run on first render"
    // This works because signing into NEAR Wallet reloads the page
    []
  )

  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) {
    return (
      <main>
        <h1>
          Welcome to Sticky Habits!
        </h1>
        <p>
         Would you like to build new habit or get rid of the old one ?
        </p>
        <p>
        Habits are built when we keep going for at least 21 days. I will remember all habits you set.
            Link your friend and make yourself accountable.
        </p>
        <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
          <button onClick={login}>Sign in</button>
        </p>
      </main>
    )
  }

  return (
    // use React Fragment, <>, to avoid wrapping elements in unnecessary divs
    <>
      <button className="link" style={{ float: 'right' }} onClick={logout}>
        Sign out
      </button>
      <main>
        <h1>
          Hello {window.accountId}
        </h1>

        <p>
          What are you up to ?
        </p>
          <table style={{ width: '150%' }}>
              <tr>
                  <th>Habit</th>
                  <th>Deadline</th>
                  <th>Penalty</th>
                  <th>Give to</th>
              </tr>
              <tr>
                  <td>Eat no cake this week</td>
                  <td>June 31st</td>
                  <td>$100</td>
                  <td>neighbor.test.near</td>
              </tr>
              <tr>
                  <td>Help my mother trash the trash</td>
                  <td>June 28th</td>
                  <td>$150</td>
                  <td>mother.test.near</td>
              </tr>
          </table>
          <form onSubmit={async event => {
              event.preventDefault()

              // get elements from the form using their id attribute
              const { fieldset, habit, deadline, penalty, beneficiary } = event.target.elements

              // hold onto new user-entered values from React's SynthenticEvent for use after `await` call
              const newHabit = habit.value
              const newDeadline = deadline.value
              const newPenalty = penalty.value
              const newBeneficiary = beneficiary.value

              // disable the form while the value gets updated on-chain
              fieldset.disabled = true

              try {
                  // make an update call to the smart contract
                  // pass the value that the user entered in the greeting field
                  await add_habit(newHabit, newDeadline, newPenalty, newBeneficiary)
              } catch (e) {
                  alert(
                      'Something went wrong! ' +
                      'Maybe you need to sign out and back in? ' +
                      'Check your browser console for more info.'
                  )
                  throw e
              } finally {
                  // re-enable the form, whether the call succeeded or failed
                  fieldset.disabled = false
              }

              // update local `greeting` variable to match persisted value
              setHabits(newHabit)

              // show Notification
              setShowNotification(true)

              // remove Notification again after css animation completes
              // this allows it to be shown again next time the form is submitted
              setTimeout(() => {
                  setShowNotification(false)
              }, 11000)
          }}>
              <fieldset id="fieldset">
                  <label
                      htmlFor="greeting"
                      style={{
                          display: 'block',
                          color: 'var(--gray)',
                          marginBottom: '0.5em'
                      }}
                  >
                  </label>
                  <div style={{ display: 'flex' }}>
                      <input
                          autoComplete="off"
                          defaultValue="Go to gym "
                          id="habit"
                          onChange={e => setButtonDisabled(e.target.value === habit)}
                          style={{ flex: 1 }}
                      />
                      <input
                          autoComplete="off"
                          defaultValue="January 1st"
                          id="deadline"
                          onChange={e => setButtonDisabled(e.target.value === deadline)}
                          style={{ flex: 1 }}
                      />
                      <input
                          autoComplete="off"
                          defaultValue="$100"
                          id="penalty"
                          onChange={e => setButtonDisabled(e.target.value === penalty)}
                          style={{ flex: 1 }}
                      />
                      <input
                          autoComplete="off"
                          defaultValue="neighbor.near"
                          id="beneficiary"
                          onChange={e => setButtonDisabled(e.target.value === beneficiary)}
                          style={{ flex: 1 }}
                      />
                      <button
                          disabled={buttonDisabled}
                          style={{ borderRadius: '0 5px 5px 0' }}
                      >
                          Add New Habit
                      </button>
                  </div>
              </fieldset>
          </form>
          <hr />
          <label
              htmlFor="greeting"
              style={{
                  display: 'block',
                  color: 'var(--gray)',
                  marginBottom: '0.5em'
              }}
          >
              Given away total: 1000$
              Success rate: 80%
          </label>

      </main>
      {showNotification && <Notification />}
    </>
  )
}

// this component gets rendered by App after the form is submitted
function Notification() {
  const { networkId } = getConfig(process.env.NODE_ENV || 'development')
  const urlPrefix = `https://explorer.${networkId}.near.org/accounts`

  return (
    <aside>
      <a target="_blank" rel="noreferrer" href={`${urlPrefix}/${window.accountId}`}>
        {window.accountId}
      </a>
      {' '/* React trims whitespace around tags; insert literal space character when needed */}
      called method: 'set_greeting' in contract:
      {' '}
      <a target="_blank" rel="noreferrer" href={`${urlPrefix}/${window.contract.contractId}`}>
        {window.contract.contractId}
      </a>
      <footer>
        <div>✔ Succeeded</div>
        <div>Just now</div>
      </footer>
    </aside>
  )
}
