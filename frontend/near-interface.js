/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */

import { utils } from "near-api-js";

export class StickyHabits {
    constructor({ contractId, walletToUse }) {
        this.contractId = contractId;
        this.wallet = walletToUse;

    }

    async initContract(owner, dev_fee, habit_acquisition_period, approval_grace_period) {
        const THIRTY_TGAS = '30000000000000';
        return await this.wallet.callMethod({ contractId: this.contractId, method: 'init',
            args:{ owner: owner, dev_fee: dev_fee, habit_acquisition_period: habit_acquisition_period,
                approval_grace_period: approval_grace_period }, gas: THIRTY_TGAS });

    }

    async getHabits() {
        const min = 0;
        const limit = 7;
        return await this.wallet.viewMethod({ contractId: this.contractId, method: 'get_habits',
             args:{ user: this.wallet.accountId, from_index: min.toString(), limit_to: limit.toString() }});

    }

    async addHabit(description, deadline_extension, deposit, beneficiary) {
        const depositInYocto = utils.format.parseNearAmount(deposit);
        const THIRTY_TGAS = '30000000000000';

        return await this.wallet.callMethod({ contractId: this.contractId, method: 'add_habit',
            args:{ description: description, deadline_extension: deadline_extension.toString(),
                beneficiary: beneficiary }, gas: THIRTY_TGAS, deposit: depositInYocto });

    }
}