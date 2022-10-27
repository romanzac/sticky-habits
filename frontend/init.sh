#!/bin/sh

CONTRACT_DIRECTORY=../contract
DEV_ACCOUNT_FILE="${CONTRACT_DIRECTORY}/neardev/dev-account.env"

echo ">> Initializing contract"

source ~/near/init-local-near-env.sh
# You have to call the fn signing the TX with the account that the contract is deployed to.
# eg: near local_call mycontract.testnet '{"init": "beneficiary": "flmel.testnet"}' --accountId mycontract.testnet

#    async initContract(owner, dev_fee, habit_acquisition_period, approval_grace_period) {
#        const THIRTY_TGAS = '30000000000000';
#        return await this.wallet.callMethod({ contractId: this.contractId, method: 'init',
#            args:{ owner: owner, dev_fee: dev_fee, habit_acquisition_period: habit_acquisition_period,
#                approval_grace_period: approval_grace_period }, gas: THIRTY_TGAS });
#
#    }



start () {
  echo The app is starting!
  env-cmd -f $DEV_ACCOUNT_FILE parcel index.html --open
}

alert () {
  echo "======================================================"
  echo "It looks like you forgot to deploy your contract"
  echo ">> Run ${GREEN}'npm run deploy'${NC} from the 'root' directory"
  echo "======================================================"
}

if [ -f "$DEV_ACCOUNT_FILE" ]; then
  start
else
  alert
fi


