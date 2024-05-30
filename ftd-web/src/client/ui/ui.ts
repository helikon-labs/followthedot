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
    private readonly loading: HTMLDivElement;
    private readonly api: API;
    private readonly initialAddress = '13JJDv1yBfMtP1E66pHvm1ysreAXqkZHxY5jqFR4yKPfL2iB';

    constructor(network: Network) {
        this.root = <HTMLElement>document.getElementById('root');
        this.background = <HTMLDivElement>document.getElementById('background');
        this.content = <HTMLDivElement>document.getElementById('content');
        this.searchBar = new SearchBar(network, (account: Account) => {
            this.loadAccountGraph(account.address);
        });
        this.graph = new Graph(
            (address: string) => {
                this.expandGraph(address);
            },
            (address: string) => {
                this.loadAccountGraph(address);
            }
        )
        this.loading = <HTMLDivElement>document.getElementById('loading-container');
        this.api = new API(network.apiHost, network.apiPort);
    }

    async init() {
        this.animate();
        const data = await this.api.getAccountGraph(this.initialAddress);
        this.graph.appendData(this.initialAddress, data);
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
