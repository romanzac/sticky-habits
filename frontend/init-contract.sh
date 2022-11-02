#!/bin/sh

CONTRACT_DIRECTORY=../contract
DEV_ACCOUNT_FILE="${CONTRACT_DIRECTORY}/neardev/dev-account.env"
CONTRACT_ACCOUNT=$(grep 'CONTRACT_NAME=' ${DEV_ACCOUNT_FILE} | cut -d= -f2)

source ~/near/init-local-near-env.sh

# 10000000000 = 10 seconds

init () {
  echo ">> Initializing contract at $CONTRACT_ACCOUNT"
  local_near call ${CONTRACT_ACCOUNT} init '{
      "owner": "zajda.test.near",
      "dev_fee": "5",
      "habit_acquisition_period": "60000000000",
      "approval_grace_period": "60000000000"
  }' --accountId ${CONTRACT_ACCOUNT}

}

alert () {
  echo "======================================================"
  echo "It looks like you forgot to deploy your contract"
  echo ">> Run ${GREEN}'npm run deploy'${NC} from the 'root' directory"
  echo "======================================================"
}

if [ -f "$DEV_ACCOUNT_FILE" ]; then
  init
else
  alert
fi


