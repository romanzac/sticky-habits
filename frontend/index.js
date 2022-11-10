import 'regenerator-runtime/runtime';
import { Wallet } from './near-wallet';
import { StickyHabits } from './near-interface';
import { utils } from "near-api-js";

// Wallet instance
const wallet = new Wallet({ createAccessKeyFor: process.env.CONTRACT_NAME })

// Logic for interacting with the contract 
const stickyHabits = new StickyHabits({ contractId: process.env.CONTRACT_NAME, walletToUse: wallet });

// Setup on page load
window.onload = async () => {
  let isSignedIn = await wallet.startUp();

  if (isSignedIn) {
    signedInFlow();
    await fetchHabits();
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
  const { description, deadline_extension, deposit, beneficiary } = event.target.elements;

  // document.querySelector('#signed-in-flow main')
  //   .classList.add('please-wait');

  try {
    await stickyHabits.addHabit(description.value, deadline_extension.value, deposit.value, beneficiary.value);
  } catch (e) {
    alert(
        'Something went wrong! ' +
        'Maybe you need to sign out and back in? ' +
        'Check your browser console for more info.'
    )
    throw e
  }

 //  ===== Fetch the data from the blockchain =====
 //await fetchHabits();
 //  document.querySelector('#signed-in-flow main')
 //    .classList.remove('please-wait');
}

// Get habits from the contract on chain
async function fetchHabits() {
  const wipHabits = await stickyHabits.getHabits();
  console.log(wipHabits);

  document.getElementById('habits-table').innerHTML = ''

  wipHabits.forEach(elem => {
    const depositinNear = utils.format.formatNearAmount(elem.deposit)
    const date = new Date(elem.deadline / 1000000);
    let tr = document.createElement('tr')
    tr.innerHTML = `
      <tr>
        <th scope="row">${elem.description}</th>
        <td>${date}</td>
        <td>${depositinNear}</td>
        <td>${elem.beneficiary}</td>
        <td>${elem.evidence}</td>
        <td>${elem.approved}</td>
      </tr>
    `
    document.getElementById('habits-table').appendChild(tr)
  });

  // document.querySelectorAll('[data-behavior=habits]').forEach(el => {
  //   el.innerText = currentHabits.description;
  //   el.value = currentHabits.deposit;
  // });
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