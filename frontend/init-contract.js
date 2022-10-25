import 'regenerator-runtime/runtime';
import { Wallet } from './near-wallet';
import { StickyHabits } from './near-interface';

// When creating the wallet you can optionally ask to create an access key
// Having the key enables to call non-payable methods without interrupting the user to sign
const wallet = new Wallet({ createAccessKeyFor: process.env.CONTRACT_NAME })

// Abstract the logic of interacting with the contract to simplify your flow
const stickyHabits = new StickyHabits({ contractId: process.env.CONTRACT_NAME, walletToUse: wallet });

// Setup on page load
window.onload = async () => {
  let isSignedIn = await wallet.startUp();

  if (isSignedIn) {
    signedInFlow();
  } else {
    signedOutFlow();
  }

};

// Button clicks
document.querySelector('form').onsubmit = doUserAction;
document.querySelector('#sign-in-button').onclick = () => { wallet.signIn(); };
document.querySelector('#sign-out-button').onclick = () => { wallet.signOut(); };

// Take the new habit and send it to the contract
async function doUserAction(event) {
  event.preventDefault();
  const { owner, dev_fee, habit_acquisition_period, approval_grace_period } = event.target.elements;

  try {
    await stickyHabits.initContract(owner.value, dev_fee.value, habit_acquisition_period.value,
        approval_grace_period.value);
  } catch (e) {
    alert(
        'Something went wrong! ' +
        'Maybe you need to sign out and back in? ' +
        'Check your browser console for more info.'
    )
    throw e
  }

}


// Display the signed-out-flow container
function signedOutFlow() {
  document.querySelector('#signed-in-flow').style.display = 'none';
  document.querySelector('#signed-out-flow').style.display = 'block';
}

// Displaying the signed in flow container and fill in account-specific data
function signedInFlow() {
  document.querySelector('#signed-out-flow').style.display = 'none';
  document.querySelector('#signed-in-flow').style.display = 'block';
  document.querySelectorAll('[data-behavior=account-id]').forEach(el => {
    el.innerText = wallet.accountId;
  });
}