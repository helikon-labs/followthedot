import * as TWEEN from '@tweenjs/tween.js';
import { EventBus } from '../event/event-bus';
import { Graph } from './graph';
import { DATA, DATA_PT2 } from '../data/data';

class UI {
    private readonly root: HTMLElement;
    private readonly background: HTMLDivElement;
    private readonly content: HTMLDivElement;
    private readonly eventBus = EventBus.getInstance();
    private readonly graph = new Graph();

    constructor() {
        this.root = <HTMLElement>document.getElementById('root');
        this.background = <HTMLDivElement>document.getElementById('background');
        this.content = <HTMLDivElement>document.getElementById('content');
    }

    init() {
        this.animate();
        this.graph.appendData(DATA);
        setTimeout(() => {
            this.graph.appendData(DATA_PT2);
        }, 2000);
    }

    animate() {
        requestAnimationFrame(() => {
            this.animate();
        });
        TWEEN.update();
    }
}

export { UI };
