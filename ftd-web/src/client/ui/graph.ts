import * as d3 from 'd3';
import { BaseType, Simulation, SimulationNodeDatum } from 'd3';
import {
    Account,
    getAccountConfirmedIcon,
    getAccountDisplay,
    GraphData,
    TransferVolume,
} from '../model/ftd-model';
import { formatNumber, truncateAddress } from '../util/format';
import { Polkadot } from '../util/constants';
import { polkadotIcon } from '@polkadot/ui-shared';

const LINK_DISTANCE = 400;
const LINK_ARROW_SIZE = 10;
const LINK_SEPARATION_OFFSET = 12;
const ACCOUNT_RADIUS = 90;
const BALANCE_DENOMINATOR = BigInt(10_000_000);

type SVG_SVG_SELECTION = d3.Selection<SVGSVGElement, unknown, HTMLElement, any>;
type SVG_CIRCLE_SELECTION = d3.Selection<BaseType | SVGCircleElement, unknown, SVGGElement, any>;
type SVG_BASE_SELECTION = d3.Selection<BaseType, unknown, SVGGElement, any>;
type SVG_GROUP_SELECTION = d3.Selection<SVGGElement, unknown, HTMLElement, any>;
type SVG_TEXT_SELECTION = d3.Selection<BaseType | SVGTextElement, unknown, SVGGElement, any>;
type SVG_SIMULATION = Simulation<SimulationNodeDatum, undefined>;

enum LinkPosition {
    Left,
    Middle,
    Right,
}

let balanceStrokeScale: d3.ScaleLinear<number, number>;
let balanceColorScale: d3.ScaleLinear<string, string>;
let balanceOpacityScale: d3.ScaleLinear<number, number>;

let transferStrokeScale: d3.ScaleLinear<number, number>;
let transferColorScale: d3.ScaleLinear<string, string>;
let transferOpacityScale: d3.ScaleLinear<number, number>;

function getIdenticon(address: string): string {
    const circles = polkadotIcon(address, { isAlternative: false })
        .map(
            ({ cx, cy, fill, r }) =>
                `<circle class="identicon" cx=${cx} cy=${cy} fill="${fill}" r=${r} />`,
        )
        .join('');
    return `${circles}`;
}

function appendSVG(): SVG_SVG_SELECTION {
    const width = window.innerWidth;
    const height = window.innerHeight;
    return d3
        .select('.graph-container')
        .append('svg')
        .attr('width', width)
        .attr('height', height)
        .attr("viewBox", [-width / 2, -height / 2, width, height])
        .attr('style', 'max-width: 100%; max-height: 100%;');
}

function appendSVGMarkerDefs(svg: SVG_SVG_SELECTION) {
    svg.append('defs')
        .selectAll('marker')
        .data(['transfer'])
        .enter()
        .append('marker')
        .attr('id', (d) => d)
        .attr('markerWidth', LINK_ARROW_SIZE)
        .attr('markerHeight', LINK_ARROW_SIZE)
        .attr('refX', ACCOUNT_RADIUS + LINK_ARROW_SIZE)
        .attr('refY', LINK_ARROW_SIZE / 2)
        .attr('orient', 'auto')
        .attr('markerUnits', 'userSpaceOnUse')
        .append('path')
        .attr('d', `M0,0L0,${LINK_ARROW_SIZE}L${LINK_ARROW_SIZE},${LINK_ARROW_SIZE / 2}z`);
}

function transformAccountLabel(d: any, scale: number): string {
    const groupSelector = `#account-label-${d.address}`;
    const group = d3.select(groupSelector);
    // @ts-ignore
    const groupWidth = group.node()!.getBoundingClientRect().width;

    // set balance label position
    const balanceLabelSelector = `#account-balance-label-${d.address}`;
    const balanceLabel = d3.select(balanceLabelSelector);
    // @ts-ignore
    const balanceLabelWidth = balanceLabel.node()!.getBoundingClientRect().width;
    balanceLabel.attr(
        'transform',
        `translate(${(groupWidth - balanceLabelWidth) / scale / 2}, 24)`,
    );

    // set identicon position & size
    const identiconSelector = `#account-identicon-${d.address}`;
    const identicon = d3.select(identiconSelector);
    if (!identicon) {
        return 'translate(0,0)';
    }
    // @ts-ignore
    const identiconWidth = identicon.node()!.getBoundingClientRect().width;
    const x = (groupWidth - identiconWidth) / 2 / scale;
    identicon.attr('transform', `translate(${x}, -46) scale(0.4, 0.4)`);

    /*
    const LABEL_FORCE_PADDING = ACCOUNT_RADIUS + 10;
    const width = window.innerWidth;
    const height = window.innerHeight;
    d.x = d.x <= LABEL_FORCE_PADDING ? LABEL_FORCE_PADDING : d.x >= width - LABEL_FORCE_PADDING ? width - LABEL_FORCE_PADDING : d.x;
    d.y = d.y <= LABEL_FORCE_PADDING ? LABEL_FORCE_PADDING : d.y >= height - LABEL_FORCE_PADDING ? height - LABEL_FORCE_PADDING : d.y;
     */
    return 'translate(' + (d.x - groupWidth / scale / 2) + ',' + d.y + ')';
}

function getAccountStrokeWidth(account: Account): number {
    return balanceStrokeScale(Number((account.balance.free / BALANCE_DENOMINATOR).valueOf()));
}

function getAccountStrokeColor(account: Account): string {
    return balanceColorScale(Number((account.balance.free / BALANCE_DENOMINATOR).valueOf()));
}

function getAccountStrokeOpacity(account: Account): number {
    return balanceOpacityScale(Number((account.balance.free / BALANCE_DENOMINATOR).valueOf()));
}

function getTransferStrokeWidth(transfer: TransferVolume): number {
    return transferStrokeScale(Number((transfer.volume / BALANCE_DENOMINATOR).valueOf()));
}

function getTransferStrokeColor(transfer: TransferVolume): string {
    return transferColorScale(Number((transfer.volume / BALANCE_DENOMINATOR).valueOf()));
}

function getTransferStrokeOpacity(transfer: TransferVolume): number {
    return transferOpacityScale(Number((transfer.volume / BALANCE_DENOMINATOR).valueOf()));
}

class Graph {
    private readonly svg;
    private accountGroup: SVG_GROUP_SELECTION;
    private transferGroup: SVG_GROUP_SELECTION;
    private simulation: SVG_SIMULATION;
    private scale = 1;
    private accounts: any[] = [];
    private transferVolumes: any[] = [];

    constructor() {
        this.svg = appendSVG();
        this.transferGroup = this.svg.append('g');
        this.accountGroup = this.svg.append('g');
        this.simulation = d3
            .forceSimulation()
            .force(
                'link',
                d3
                    .forceLink()
                    // @ts-ignore
                    .id((d) => d.address)
                    .strength(0.8)
                    .distance(LINK_DISTANCE),
            )
            .force('charge', d3.forceManyBody().strength(-10000))
            .force('y', d3.forceY())
            .force('x', d3.forceX())
            //.force('center', d3.forceCenter(0 , 0).strength(0.1));

        //.force('x', d3.forceX(window.innerWidth / 2))
        //.force('y', d3.forceY(window.innerHeight / 2));
        appendSVGMarkerDefs(this.svg);
        this.svg
            .call(
                // @ts-ignore
                d3
                    .zoom()
                    .extent([
                        [0, 0],
                        [window.innerWidth, window.innerHeight],
                    ])
                    .scaleExtent([0.2, 8])
                    .on('zoom', (e) => {
                        this.scale = e.transform.k;
                        this.transferGroup.attr('transform', e.transform);
                        this.accountGroup.attr('transform', e.transform);
                    }),
            )
            .on('dblclick.zoom', null);
    }

    private getLinkTranslation(linkPosition: LinkPosition, point0: any, point1: any) {
        const x1_x0 = point1.x - point0.x,
            y1_y0 = point1.y - point0.y;
        let targetDistance = 0;
        switch (linkPosition) {
            case LinkPosition.Left:
                targetDistance = -1 * LINK_SEPARATION_OFFSET;
                break;
            case LinkPosition.Right:
                targetDistance = LINK_SEPARATION_OFFSET;
                break;
        }
        let x2_x0, y2_y0;
        if (y1_y0 === 0) {
            x2_x0 = 0;
            y2_y0 = targetDistance;
        } else {
            const angle = Math.atan(x1_x0 / y1_y0);
            x2_x0 = -targetDistance * Math.cos(angle);
            y2_y0 = targetDistance * Math.sin(angle);
        }
        return {
            dx: x2_x0,
            dy: y2_y0,
        };
    }

    private tick(
        accounts: SVG_CIRCLE_SELECTION,
        accountLabels: SVG_BASE_SELECTION,
        transfers: SVG_TEXT_SELECTION,
        transferLabels: SVG_BASE_SELECTION,
    ) {
        accounts.attr('cx', (d: any) => d.x).attr('cy', (d: any) => d.y);
        accountLabels.attr('transform', (d: any) => transformAccountLabel(d, this.scale));
        transfers
            .attr('d', (d: any) => `M${d.source.x},${d.source.y} L${d.target.x},${d.target.y}`)
            .attr('transform', (d: any) => {
                const translation = this.getLinkTranslation(d.linkPosition, d.source, d.target);
                d.offsetX = translation.dx;
                d.offsetY = translation.dy;
                return `translate (${d.offsetX}, ${d.offsetY})`;
            });
        transferLabels.attr('transform', (d: any) => {
            if (d.target.x < d.source.x) {
                return (
                    'rotate(180,' +
                    ((d.source.x + d.target.x) / 2 + d.offsetX) +
                    ',' +
                    ((d.source.y + d.target.y) / 2 + d.offsetY) +
                    ')'
                );
            } else {
                return 'rotate(0)';
            }
        });
    }

    private resetTransferVolumeLinkPositions() {
        for (let i = 0; i < this.transferVolumes.length; i++) {
            this.transferVolumes[i].linkPosition = LinkPosition.Middle;
        }
        for (let i = 0; i < this.transferVolumes.length; i++) {
            if (this.transferVolumes[i].linkPosition === LinkPosition.Left) continue;
            this.transferVolumes[i].linkPosition = LinkPosition.Middle;
            for (let j = i + 1; j < this.transferVolumes.length; j++) {
                if (this.transferVolumes[j].linkPosition === LinkPosition.Left) continue;
                if (
                    this.transferVolumes[i].target === this.transferVolumes[j].source &&
                    this.transferVolumes[i].source === this.transferVolumes[j].target
                ) {
                    this.transferVolumes[i].linkPosition = LinkPosition.Right;
                    this.transferVolumes[j].linkPosition = LinkPosition.Left;
                }
            }
        }
    }

    private resetScales() {
        const maxBalance = this.accounts.reduce((acc, account) => {
            const balance = account.balance.free / BALANCE_DENOMINATOR;
            return acc > balance ? acc : balance;
        }, 0);
        balanceStrokeScale = d3.scaleLinear([0n, maxBalance].map(Number), [1, 10]);
        balanceColorScale = d3.scaleLinear([0n, maxBalance].map(Number), ['gray', 'blue']);
        balanceOpacityScale = d3.scaleLinear([0n, maxBalance].map(Number), [0.75, 0.4]);

        const maxTransferVolume = this.transferVolumes.reduce((acc, transferVolume) => {
            const volume = transferVolume.volume / BALANCE_DENOMINATOR;
            return acc > volume ? acc : volume;
        }, 0);
        transferStrokeScale = d3.scaleLinear([0n, maxTransferVolume].map(Number), [0.5, 5]);
        transferColorScale = d3.scaleLinear([0n, maxTransferVolume].map(Number), ['gray', 'red']);
        transferOpacityScale = d3.scaleLinear([0n, maxTransferVolume].map(Number), [1.0, 0.5]);
    }

    reset() {
        this.accounts = [];
        this.transferVolumes = [];
        this.display();
    }

    removeAccount(address: string) {
        this.accounts = this.accounts.filter((a) => a.address != address);
        this.transferVolumes = this.transferVolumes.filter(
            (t) => t.source.address != address && t.target.address != address,
        );
        this.resetTransferVolumeLinkPositions();
        this.resetScales();
        this.display();
    }

    appendData(data: GraphData) {
        for (const account of data.accounts) {
            if (this.accounts.findIndex((a) => a.address === account.address) === -1) {
                this.accounts.push({ ...account });
            }
        }
        for (const transfer of data.transferVolumes) {
            if (this.transferVolumes.findIndex((t) => t.id === transfer.id) === -1) {
                // check that all accounts exist
                const fromIndex = this.accounts.findIndex((a) => transfer.from === a.address);
                const toIndex = this.accounts.findIndex((a) => transfer.to === a.address);
                if (fromIndex >= 0 && toIndex >= 0) {
                    this.transferVolumes.push({
                        id: transfer.id,
                        source: transfer.from,
                        target: transfer.to,
                        count: transfer.count,
                        volume: transfer.volume,
                    });
                } else {
                    if (fromIndex < 0) {
                        console.error(
                            `Transfer #${transfer.id} sender account ${transfer.from} not found.`,
                        );
                    }
                    if (toIndex < 0) {
                        console.error(
                            `Transfer #${transfer.id} receipient account ${transfer.to} not found.`,
                        );
                    }
                }
            }
        }
        this.resetTransferVolumeLinkPositions();
        this.resetScales();
        this.display();
    }

    private displayTransfers(): SVG_BASE_SELECTION {
        return this.transferGroup
            .selectAll('path.transfer')
            .data(this.transferVolumes, (d: any) => d.id)
            .join('path')
            .attr('id', (d) => `link-${d.id}`)
            .attr('class', 'transfer')
            .attr('stroke', (transfer: TransferVolume) => getTransferStrokeColor(transfer))
            .attr('stroke-width', (transfer: TransferVolume) => getTransferStrokeWidth(transfer))
            .attr('stroke-opacity', (transfer: TransferVolume) =>
                getTransferStrokeOpacity(transfer),
            )
            .attr('marker-end', 'url(#transfer)');
    }

    private displayTransferLabels(): SVG_BASE_SELECTION {
        return this.transferGroup
            .selectAll('text.transfer-label')
            .data(this.transferVolumes, (d: any) => d.id)
            .join(
                (enter) => {
                    const transferLabels = enter
                        .append('text')
                        .attr('class', 'transfer-label')
                        .attr('text-anchor', 'middle')
                        //.attr('dy', '0.31em');
                        .attr('dy', '-0.25em');
                    transferLabels
                        .append('textPath')
                        .attr('href', (d) => `#link-${d.id}`)
                        .attr('startOffset', '50%')
                        .text((d) => formatNumber(d.volume, Polkadot.DECIMAL_COUNT, 2, 'DOT'))
                        //.style('pointer-events', 'none')
                        .on('mouseover', function () {
                            d3.select(this).attr('cursor', 'pointer');
                        })
                        .on('mouseout', function () {});
                    return transferLabels;
                },
                undefined,
                (exit) => exit.remove(),
            );
    }

    private displayAccounts(): SVG_BASE_SELECTION {
        return this.accountGroup
            .selectAll('circle.account')
            .data(this.accounts, (d: any) => d.address)
            .join(
                (enter) => {
                    const accounts = enter
                        .append('circle')
                        .attr('class', 'account')
                        //.attr('fill', '#DDD')
                        .attr('fill', '#FFF')
                        .attr('stroke', (account: Account) => getAccountStrokeColor(account))
                        .attr('stroke-width', (account: Account) => getAccountStrokeWidth(account))
                        .attr('stroke-opacity', (account: Account) =>
                            getAccountStrokeOpacity(account),
                        )
                        .attr('r', ACCOUNT_RADIUS)
                        .on('mouseover', function () {
                            /*function (e, d) {*/
                            d3.select(this).attr('fill', '#EFEFEF');
                            d3.select(this).attr('cursor', 'pointer');
                        })
                        .on('mouseout', function () {
                            /*function (e, d) {*/
                            d3.select(this).attr('fill', '#FFF');
                        })
                        .on('dblclick', (e, d) => {
                            this.removeAccount(d.address);
                        });
                    accounts.append('title').text((d) => truncateAddress(d.address));
                    accounts.call(
                        // @ts-ignore
                        d3
                            .drag()
                            .on('start', (event) => {
                                if (!event.active) this.simulation.alphaTarget(0.3).restart();
                                event.subject.fx = event.subject.x;
                                event.subject.fy = event.subject.y;
                            })
                            .on('drag', (event) => {
                                event.subject.fx = event.x;
                                event.subject.fy = event.y;
                            })
                            .on('end', (event) => {
                                if (!event.active) this.simulation.alphaTarget(0);
                                event.subject.fx = null;
                                event.subject.fy = null;
                            }),
                    );
                    return accounts;
                },
                undefined,
                (exit) => exit.remove(),
            );
    }

    private displayAccountLabels(): SVG_BASE_SELECTION {
        return this.accountGroup
            .selectAll('g.account-label')
            .data(this.accounts, (a: any) => a.address)
            .join(
                (enter) => {
                    const accountLabel = enter
                        .append('g')
                        .attr('id', (account: Account) => `account-label-${account.address}`)
                        .attr('class', 'account-label');
                    accountLabel
                        .append('svg:image')
                        .attr(
                            'xlink:href',
                            (account: Account) => getAccountConfirmedIcon(account) ?? '',
                        )
                        // .attr('x', -44)
                        .attr('class', 'identity-icon')
                        .attr('y', -7)
                        .attr('opacity', (account: Account) =>
                            getAccountConfirmedIcon(account) ? 1.0 : 0,
                        );
                    accountLabel
                        .append('text')
                        .attr('class', 'account-display-label')
                        .attr('x', (account: Account) =>
                            getAccountConfirmedIcon(account) ? '18px' : '0',
                        )
                        .attr('y', '.31em')
                        //.attr('text-anchor', 'middle')
                        .text((account: Account) => getAccountDisplay(account))
                        .style('pointer-events', 'none');
                    accountLabel
                        .append('text')
                        .attr(
                            'id',
                            (account: Account) => `account-balance-label-${account.address}`,
                        )
                        .attr('class', 'account-balance-label')
                        //.attr('text-anchor', 'middle')
                        .text((account: Account) =>
                            formatNumber(account.balance.free, Polkadot.DECIMAL_COUNT, 2, 'DOT'),
                        )
                        .style('pointer-events', 'none');
                    accountLabel
                        .append('g')
                        .attr('id', (d) => `account-identicon-${d.address}`)
                        .html((d) => getIdenticon(d.address))
                        .style('pointer-events', 'none');
                    return accountLabel;
                },
                undefined,
                (exit) => exit.remove(),
            );
    }

    private display() {
        // update components
        const transfers = this.displayTransfers();
        const transferLabels = this.displayTransferLabels();
        const accounts = this.displayAccounts();
        const accountLabel = this.displayAccountLabels();

        // update simulation
        this.simulation.nodes(this.accounts);
        // @ts-ignore
        this.simulation.force('link')!.links(this.transferVolumes);
        this.simulation.on('tick', () => {
            this.tick(accounts, accountLabel, transfers, transferLabels);
        });
    }
}

export { Graph };
