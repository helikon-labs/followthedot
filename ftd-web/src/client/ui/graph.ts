import * as d3 from 'd3';
import { Account, GraphData, Identity, TransferVolume } from '../model/ftd-model';
import { formatNumber, trimText, truncateAddress } from '../util/format';
import { BaseType } from 'd3';
import { Constants, Polkadot } from '../util/constants';
import { polkadotIcon } from '@polkadot/ui-shared';

const LINK_DISTANCE = 400;
const LINK_ARROW_SIZE = 8;
const LINK_SEPARATION_OFFSET = 12;
const ACCOUNT_RADIUS = 90;

type SVG = d3.Selection<SVGSVGElement, unknown, HTMLElement, any>;
type SVG_CIRCLE = d3.Selection<BaseType | SVGCircleElement, unknown, SVGGElement, any>;
type SVG_TEXT = d3.Selection<BaseType, unknown, SVGGElement, any>;
type SVG_GROUP = d3.Selection<BaseType, unknown, SVGGElement, any>;
type SVG_PATH = d3.Selection<BaseType | SVGTextElement, unknown, SVGGElement, any>;

enum LinkPosition {
    Left,
    Middle,
    Right,
}

const balanceStrokeScale = d3.scaleLinear([0n, 50_000_000_000n].map(Number), [1, 10]);
const balanceColorScale = d3.scaleLinear([0n, 50_000_000_000n].map(Number), ['gray', 'blue']);
const balanceOpacityScale = d3.scaleLinear([0n, 50_000_000_000n].map(Number), [0.75, 0.4]);

const transferStrokeScale = d3.scaleLinear([0n, 50_000_000n].map(Number), [0.5, 5]);
const transferColorScale = d3.scaleLinear([0n, 50_000_000n].map(Number), ['gray', 'red']);
const transferOpacityScale = d3.scaleLinear([0n, 50_000_000n].map(Number), [1.0, 0.25]);

function getIdenticon(address: string): string {
    const circles = polkadotIcon(address, { isAlternative: false })
        .map(({ cx, cy, fill, r }) => `<circle cx=${cx} cy=${cy} fill="${fill}" r=${r} />`)
        .join('');
    return `${circles}`;
    // return `<svg style="width; ${size}; height: ${size};" viewBox='0 0 64 64'>${circles}</svg>`;
}

function appendSVG(): SVG {
    const width = window.innerWidth;
    const height = window.innerHeight;
    return d3
        .select('#chart-container')
        .append('svg')
        .attr('width', width)
        .attr('height', height)
        .attr('viewBox', [0, 0, width, height])
        .attr('style', 'max-width: 100%; max-height: 100%;');
}

function appendSVGMarkerDefs(svg: SVG) {
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

function getAccountDisplay(account: Account): string {
    if (account.identity?.display) {
        return trimText(account.identity.display, Constants.MAX_IDENTITY_DISPLAY_LENGTH);
    }
    if (account.superIdentity?.display && account.subIdentity?.subDisplay) {
        const display = `${account.subIdentity.subDisplay} / ${account.superIdentity.display}`;
        return trimText(display, Constants.MAX_IDENTITY_DISPLAY_LENGTH);
    }
    return truncateAddress(account.address);
}

function getAccountConfirmedIcon(account: Account): string | undefined {
    function getIdentityConfirmedIcon(identity: Identity, isParent: boolean): string | undefined {
        if (identity.isInvalid) {
            return `./img/icon/${isParent ? 'parent-' : ''}id-invalid-icon.svg`;
        }
        if (identity.isConfirmed) {
            return `./img/icon/${isParent ? 'parent-' : ''}id-confirmed-icon.svg`;
        }
        if (identity.isConfirmed) {
            return `./img/icon/${isParent ? 'parent-' : ''}id-unconfirmed-icon.svg`;
        }
    }

    if (account.superIdentity) {
        return getIdentityConfirmedIcon(account.superIdentity, true);
    }
    if (account.identity) {
        return getIdentityConfirmedIcon(account.identity, false);
    }
    return undefined;
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

    const width = window.innerWidth;
    const height = window.innerHeight;
    d.x = d.x <= 5 ? 5 : d.x >= width - 5 ? width - 5 : d.x;
    d.y = d.y <= 5 ? 5 : d.y >= height - 5 ? height - 5 : d.y;
    return 'translate(' + (d.x - groupWidth / scale / 2) + ',' + d.y + ')';
}

function getAccountStrokeWidth(account: Account): number {
    return balanceStrokeScale(Number((account.balance / BigInt(10_000_000)).valueOf()));
}

function getAccountStrokeColor(account: Account): string {
    return balanceColorScale(Number((account.balance / BigInt(10_000_000)).valueOf()));
}

function getAccountStrokeOpacity(account: Account): number {
    return balanceOpacityScale(Number((account.balance / BigInt(10_000_000)).valueOf()));
}

function getTransferStrokeWidth(transfer: TransferVolume): number {
    return transferStrokeScale(Number((transfer.volume / BigInt(10_000_000)).valueOf()));
}

function getTransferStrokeColor(transfer: TransferVolume): string {
    return transferColorScale(Number((transfer.volume / BigInt(10_000_000)).valueOf()));
}

function getTransferStrokeOpacity(transfer: TransferVolume): number {
    return transferOpacityScale(Number((transfer.volume / BigInt(10_000_000)).valueOf()));
}

class Graph {
    private readonly svg;
    private readonly accountGroup;
    private readonly transferGroup;
    private readonly simulation;
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
            .force('charge', d3.forceManyBody().strength(-5000))
            .force('center', d3.forceCenter(window.innerWidth / 2, window.innerHeight / 2).strength(0.5));
        appendSVGMarkerDefs(this.svg);
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
        account: SVG_CIRCLE,
        accountLabel: SVG_GROUP,
        transfer: SVG_PATH,
        transferLabel: SVG_TEXT,
    ) {
        account.attr('cx', (d: any) => d.x).attr('cy', (d: any) => d.y);
        accountLabel.attr('transform', (d: any) => transformAccountLabel(d, this.scale));
        transfer
            .attr('d', (d: any) => `M${d.source.x},${d.source.y} L${d.target.x},${d.target.y}`)
            .attr('transform', (d: any) => {
                const translation = this.getLinkTranslation(d.linkPosition, d.source, d.target);
                d.offsetX = translation.dx;
                d.offsetY = translation.dy;
                return `translate (${d.offsetX}, ${d.offsetY})`;
            });
        transferLabel.attr('transform', (d: any) => {
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

    remove(address: string) {
        console.log('remove', address);
        this.accounts = this.accounts.filter((a) => a.address != address);
        this.transferVolumes = this.transferVolumes.filter(
            (t) => t.source.address != address && t.target.address != address,
        );
        this.resetTransferVolumeLinkPositions();
        this.display();
    }

    append(data: GraphData) {
        for (const account of data.accounts) {
            if (this.accounts.findIndex((a) => a.address === account.address) === -1) {
                this.accounts.push({ ...account });
            }
        }
        for (const transferVolume of data.transferVolumes) {
            if (this.transferVolumes.findIndex((t) => t.id === transferVolume.id) === -1) {
                this.transferVolumes.push({
                    id: transferVolume.id,
                    source: transferVolume.from,
                    target: transferVolume.to,
                    count: transferVolume.count,
                    volume: transferVolume.volume,
                });
            }
        }
        this.resetTransferVolumeLinkPositions();
        this.display();
    }

    private display() {
        const transfer = this.transferGroup
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
        let transferLabel = this.transferGroup
            .selectAll('text')
            .data(this.transferVolumes, (d: any) => d.id);
        const transferLabelEnter = transferLabel
            .enter()
            .append('text')
            .attr('class', 'link-label')
            .attr('text-anchor', 'middle')
            //.attr('dy', '0.31em');
            .attr('dy', '-0.25em');
        transferLabelEnter
            .append('textPath')
            .attr('href', (d) => `#link-${d.id}`)
            .attr('startOffset', '50%')
            .text((d) => formatNumber(d.volume, Polkadot.DECIMAL_COUNT, 2, 'DOT'))
            //.style('pointer-events', 'none')
            .on('mouseover', function () {
                d3.select(this).attr('cursor', 'pointer');
            })
            .on('mouseout', function () {});
        transferLabel = this.transferGroup
            .selectAll('text')
            .data(this.transferVolumes, (d: any) => d.id);
        transferLabel.exit().remove();

        let account = this.accountGroup
            .selectAll('circle.account')
            .data(this.accounts, (d: any) => d.address);
        const accountEnter = account
            .enter()
            .append('circle')
            .attr('class', 'account')
            //.attr('fill', '#DDD')
            .attr('fill', '#FFF')
            .attr('stroke', (account: Account) => getAccountStrokeColor(account))
            .attr('stroke-width', (account: Account) => getAccountStrokeWidth(account))
            .attr('stroke-opacity', (account: Account) => getAccountStrokeOpacity(account))
            .attr('r', ACCOUNT_RADIUS)
            .on('mouseover', function (e, d) {
                d3.select(this).attr('fill', '#EFEFEF');
                d3.select(this).attr('cursor', 'pointer');
            })
            .on('mouseout', function (e, d) {
                d3.select(this).attr('fill', '#FFF');
            })
            .on('dblclick', (e, d) => {
                this.remove(d.address);
            });
        accountEnter.append('title').text((d) => truncateAddress(d.address));
        accountEnter.call(
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
        account.exit().remove();
        account = this.accountGroup
            .selectAll('circle.account')
            .data(this.accounts, (d: any) => d.address);

        let accountLabel = this.accountGroup
            .selectAll('g.account-label')
            .data(this.accounts, (a: any) => a.address);
        const accountLabelEnter = accountLabel
            .enter()
            .append('g')
            .attr('id', (account: Account) => `account-label-${account.address}`)
            .attr('class', 'account-label');
        accountLabelEnter
            .append('svg:image')
            .attr('xlink:href', (account: Account) => getAccountConfirmedIcon(account) ?? '')
            // .attr('x', -44)
            .attr('class', 'identity-icon')
            .attr('y', -7)
            .attr('opacity', (account: Account) => (getAccountConfirmedIcon(account) ? 1.0 : 0));
        accountLabelEnter
            .append('text')
            .attr('class', 'account-display-label')
            .attr('x', (account: Account) => (getAccountConfirmedIcon(account) ? '18px' : '0'))
            .attr('y', '.31em')
            //.attr('text-anchor', 'middle')
            .text((account: Account) => getAccountDisplay(account))
            .style('pointer-events', 'none');
        accountLabelEnter
            .append('text')
            .attr('id', (account: Account) => `account-balance-label-${account.address}`)
            .attr('class', 'account-balance-label')
            //.attr('text-anchor', 'middle')
            .text((account: Account) =>
                formatNumber(account.balance, Polkadot.DECIMAL_COUNT, 2, 'DOT'),
            )
            .style('pointer-events', 'none');
        accountLabelEnter
            .append('g')
            .attr('id', (d) => `account-identicon-${d.address}`)
            .html((d) => getIdenticon(d.address))
            .style('pointer-events', 'none');
        accountLabel.exit().remove();
        accountLabel = this.accountGroup.selectAll('g').data(this.accounts, (a: any) => a.address);

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

        this.simulation.nodes(this.accounts);
        // @ts-ignore
        this.simulation.force('link')!.links(this.transferVolumes);
        this.simulation.on('tick', () => {
            this.tick(account, accountLabel, transfer, transferLabel);
        });
    }
}

export { Graph };
