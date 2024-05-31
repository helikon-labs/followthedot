import { hide, show } from '../util/ui-util';
import { Network } from '../model/substrate/network';
import { API } from '../api/api';
import { Account, getAccountConfirmedIcon, getAccountDisplay } from '../model/ftd-model';
import { generateIdenticonSVGHTML } from '../util/identicon';

interface UI {
    input: HTMLInputElement;
    animation: HTMLDivElement;
    resultContainer: HTMLDivElement;
}

class SearchBar {
    private readonly onSelectAccount: (account: Account) => void;
    private readonly ui: UI;
    private inputDebounceTimeout?: NodeJS.Timeout = undefined;
    private api: API;
    private accounts: Account[] = [];

    constructor(network: Network, onSelectAccount: (account: Account) => void) {
        this.onSelectAccount = onSelectAccount;
        this.api = new API(network.apiHost, network.apiPort);
        this.ui = {
            input: <HTMLInputElement>document.getElementById('search-input'),
            animation: <HTMLDivElement>document.getElementById('search-animation'),
            resultContainer: <HTMLDivElement>document.getElementById('search-result-container'),
        };
        hide(this.ui.animation);
        hide(this.ui.resultContainer);
        this.ui.input.oninput = () => {
            clearTimeout(this.inputDebounceTimeout);
            this.clearAccounts();
            this.inputDebounceTimeout = setTimeout(() => {
                this.search();
            }, 300);
        };
        document.onkeydown = (event) => {
            if (event.key.toLowerCase() === 'escape') {
                hide(this.ui.resultContainer);
            }
        };
    }

    private sortAccounts() {
        this.accounts.sort((a, b) => {
            if (a.identity?.display || a.superIdentity?.display) {
                if (b.identity?.display || b.superIdentity?.display) {
                    return getAccountDisplay(a).localeCompare(getAccountDisplay(b));
                } else {
                    return -1;
                }
            } else {
                if (b.identity?.display || b.superIdentity?.display) {
                    return 1;
                } else {
                    return getAccountDisplay(a).localeCompare(getAccountDisplay(b));
                }
            }
        });
    }

    private async search() {
        const query = this.ui.input.value.toLocaleLowerCase().replace('_', '').trim();
        if (query.length == 0) {
            return;
        }
        show(this.ui.animation);
        try {
            this.accounts = await this.api.searchAccount(query);
            hide(this.ui.animation);
            if (this.accounts.length > 0) {
                this.sortAccounts();
                this.displayAccounts();
            }
        } catch (error) {
            hide(this.ui.animation);
            alert(`Error while searching accounts: ${error}`);
        }
    }

    private clearAccounts() {
        for (const account of this.accounts) {
            const accountDiv = document.getElementById(`search-result-${account.address}`);
            accountDiv?.remove();
        }
        hide(this.ui.resultContainer);
    }

    private displayAccounts() {
        let html = '';
        this.accounts.forEach((account) => {
            const identiconHTML = generateIdenticonSVGHTML(account.address, 20);
            const display = getAccountDisplay(account);
            let confirmedIconHTML = '';
            const confirmedIcon = getAccountConfirmedIcon(account);
            if (confirmedIcon) {
                confirmedIconHTML = `<img src=${confirmedIcon} alt="${display}">`;
            }
            html += `<div class="search-result" id="search-result-${account.address}">${identiconHTML}${confirmedIconHTML}<span>${display}</span></div>`;
            setTimeout(() => {
                const accountDiv = document.getElementById(`search-result-${account.address}`);
                accountDiv?.addEventListener('click', (_event) => {
                    this.ui.input.value = '';
                    this.clearAccounts();
                    this.onSelectAccount(account);
                });
            }, 500);
        });
        this.ui.resultContainer.innerHTML = html;
        show(this.ui.resultContainer);
    }

    disable() {
        this.ui.input.disabled = true;
    }

    enable() {
        this.ui.input.disabled = false;
    }
}

export { SearchBar };
