import { Constants } from '../util/constants';
import { createTween } from '../util/tween';
import { show } from '../util/ui-util';
import * as TWEEN from '@tweenjs/tween.js';
import { EventBus } from '../event/event-bus';
import { Graph } from './graph';
import { DATA } from '../data/data';

class UI {
    private readonly root: HTMLElement;
    private readonly background: HTMLDivElement;
    private readonly content: HTMLDivElement;
    private readonly eventBus = EventBus.getInstance();
    private readonly graph = new Graph(DATA);

    constructor() {
        this.root = <HTMLElement>document.getElementById('root');
        this.background = <HTMLDivElement>document.getElementById('background');
        this.content = <HTMLDivElement>document.getElementById('content');
    }

    init() {
        this.animate();
        this.graph.start();
    }

    animate() {
        requestAnimationFrame(() => {
            this.animate();
        });
        TWEEN.update();
    }

    private fadeInBackground(onComplete: () => void) {
        this.background.style.opacity = '0%';
        this.background.classList.remove('hidden');
        show(this.background);
        const opacity = { opacity: 0 };
        createTween(
            opacity,
            { opacity: 80 },
            TWEEN.Easing.Exponential.InOut,
            Constants.CONTENT_FADE_ANIM_DURATION_MS,
            undefined,
            () => {
                this.background.style.opacity = `${opacity.opacity}%`;
            },
            onComplete,
        ).start();
    }

    private fadeInContent(onComplete?: () => void) {
        this.content.style.opacity = '0%';
        this.content.classList.remove('hidden');
        show(this.content);
        const opacity = { opacity: 0 };
        createTween(
            opacity,
            { opacity: 100 },
            TWEEN.Easing.Exponential.InOut,
            Constants.CONTENT_FADE_ANIM_DURATION_MS,
            undefined,
            () => {
                this.content.style.opacity = `${opacity.opacity}%`;
            },
            onComplete,
        ).start();
    }
}

export { UI };
