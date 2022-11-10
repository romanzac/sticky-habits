/* Talking with a contract encapsulated into a class */

import { utils } from "near-api-js";

export class StickyHabits {
    constructor({ contractId, walletToUse }) {
        this.contractId = contractId;
        this.wallet = walletToUse;

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

    async updateEvidence(at_index, evidence) {
        return await this.wallet.callMethod({ contractId: this.contractId, method: 'update_evidence',
            args:{ at_index: at_index.toString(), evidence: evidence } });
    }

    async approveHabit(user, at_index) {
        return await this.wallet.callMethod({ contractId: this.contractId, method: 'approve_habit',
            args:{ user: user, at_index: at_index.toString() } });
    }

    async unlockDeposit(user, at_index) {
        return await this.wallet.callMethod({ contractId: this.contractId, method: 'unlock_deposit',
            args:{ user: user, at_index: at_index.toString() } });
    }


}