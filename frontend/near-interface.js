/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */

export class StickyHabits {
    constructor({ contractId, walletToUse }) {
        this.contractId = contractId;
        this.wallet = walletToUse;

    }

    async getHabits() {
        const min = 0;
        const limit = 7;
        let habits = await this.wallet.viewMethod({ contractId: this.contractId, method: 'get_habits',
            args:{ user: this.wallet.getAccountId(), from_index: min.toString(), limit_to: limit.toString() }});

        return habits
    }

    async addHabit(description, deadline_extension, beneficiary) {
        return await this.wallet.callMethod({ contractId: this.contractId, method: 'add_habit',
            args:{ description: description, deadline_extension: deadline_extension.toString(), beneficiary: beneficiary }});

    }



}