import * as TWEEN from '@tweenjs/tween.js';
import { EventBus } from '../event/event-bus';
import { Graph } from './graph';
import { SearchBar } from './search-bar';
import { Network } from '../model/substrate/network';
import { Account } from '../model/ftd-model';
import { hide, show } from '../util/ui-util';
import { API } from '../api/api';

class UI {
    private readonly root: HTMLElement;
    private readonly background: HTMLDivElement;
    private readonly content: HTMLDivElement;
    private readonly eventBus = EventBus.getInstance();
    private readonly graph;
    private readonly searchBar: SearchBar;
    private readonly help: HTMLDivElement;
    private readonly showHelpButton: HTMLElement;
    private readonly hideHelpButton: HTMLElement;
    private readonly loading: HTMLDivElement;
    private readonly api: API;
    private readonly initialAddresses = [
        '1wpTXaBGoyLNTDF9bosbJS3zh8V8D2ta7JKacveCkuCm7s6',
        '1EpEiYpWRAWmte4oPLtR5B1TZFxcBShBdjK4X9wWnq2KfLK',
        '15fTH34bbKGMUjF1bLmTqxPYgpg481imThwhWcQfCyktyBzL',
        '13JJDv1yBfMtP1E66pHvm1ysreAXqkZHxY5jqFR4yKPfL2iB',
        '1eUsBZgJuvpmVNBrBSRQ9gjPTuH6QMAnQrdwQ1ZXwa5FEvo',
    ];

    constructor(network: Network) {
        this.root = <HTMLElement>document.getElementById('root');
        this.background = <HTMLDivElement>document.getElementById('background');
        this.content = <HTMLDivElement>document.getElementById('content');
        this.help = <HTMLDivElement>document.getElementById('help');
        this.showHelpButton = <HTMLElement>document.getElementById('show-help-button');
        this.hideHelpButton = <HTMLElement>document.getElementById('hide-help-button');
        this.showHelpButton.addEventListener('click', (_event) => {
            show(this.help);
        });
        this.hideHelpButton.addEventListener('click', (_event) => {
            hide(this.help);
        });
        this.searchBar = new SearchBar(network, (account: Account) => {
            this.loadAccountGraph(account.address);
        });
        this.graph = new Graph(
            (address: string) => {
                this.expandGraph(address);
            },
            (address: string) => {
                this.loadAccountGraph(address);
            },
        );
        this.loading = <HTMLDivElement>document.getElementById('loading-container');
        this.api = new API(network.apiHost, network.apiPort);
    }

    async init() {
        this.animate();
        const initialAddress =
            this.initialAddresses[Math.floor(Math.random() * this.initialAddresses.length)];
        const data = await this.api.getAccountGraph(initialAddress);
        this.graph.appendData(initialAddress, data);
        hide(this.loading);
    }

    private animate() {
        requestAnimationFrame(() => {
            this.animate();
        });
        TWEEN.update();
    }

    private async loadAccountGraph(address: string) {
        this.graph.reset();
        this.searchBar.disable();
        show(this.loading);
        try {
            const data = await this.api.getAccountGraph(address);
            hide(this.loading);
            this.graph.appendData(address, data);
            this.searchBar.enable();
        } catch (error) {
            hide(this.loading);
            this.searchBar.enable();
            alert(`Error while getting account graph: ${error}`);
        }
    }

    private async expandGraph(address: string) {
        show(this.loading);
        try {
            const data = await this.api.getAccountGraph(address);
            hide(this.loading);
            this.graph.appendData(address, data);
        } catch (error) {
            hide(this.loading);
            alert(`Error while getting account graph: ${error}`);
        }
    }
}

export { UI };
