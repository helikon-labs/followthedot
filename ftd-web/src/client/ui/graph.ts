import * as d3 from 'd3';
import { Account, GraphData } from '../model/ftd-model';
import { formatNumber, truncateAddress } from '../util/format';
import { BaseType } from 'd3';

const LINK_DISTANCE = 400;
const LINK_ARROW_SIZE = 8;
const LINK_SEPARATION_OFFSET = 12;
const ACCOUNT_RADIUS = 90;

type SVG = d3.Selection<SVGSVGElement, unknown, HTMLElement, any>;
type SVG_CIRCLE = d3.Selection<BaseType | SVGCircleElement, unknown, SVGGElement, any>;
type SVG_TEXT = d3.Selection<BaseType | SVGTextElement, unknown, SVGGElement, any>;
type SVG_GROUP = d3.Selection<BaseType | SVGGElement, unknown, SVGGElement, any>;
type SVG_PATH = d3.Selection<BaseType | SVGTextElement, unknown, SVGGElement, any>;

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
        return account.identity.display;
    }
    if (account.superIdentity?.display && account.subIdentity?.subDisplay) {
        return `${account.subIdentity.subDisplay} / ${account.superIdentity.display}`;
    }
    return truncateAddress(account.address);
}

function getAccountConfirmedIcon(account: Account): string | undefined {
    if (account.identity) {
        if (account.identity.isInvalid) {
            return './img/icon/id-invalid-icon.svg';
        }
        if (account.identity.isConfirmed) {
            return './img/icon/id-confirmed-icon.svg';
        }
        if (!account.identity.isConfirmed) {
            return './img/icon/id-unconfirmed-icon.svg';
        }
    }
    if (account.superIdentity) {
        if (account.superIdentity.isInvalid) {
            return './img/icon/parent-id-invalid-icon.svg';
        }
        if (account.superIdentity.isConfirmed) {
            return './img/icon/parent-id-confirmed-icon.svg';
        }
        if (!account.superIdentity.isConfirmed) {
            return './img/icon/parent-id-unconfirmed-icon.svg';
        }
    }
    return undefined;
}

class Graph {
    private readonly svg;

    constructor() {
        this.svg = appendSVG();
        appendSVGMarkerDefs(this.svg);
    }

    private radius = (account: Account) => ACCOUNT_RADIUS;

    private getTransferVolumeTranslation(targetDistance: number, point0: any, point1: any) {
        const x1_x0 = point1.x - point0.x,
            y1_y0 = point1.y - point0.y;
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

    private transform = (d: any) => {
        const idSelector = `#account-${d.address}-label`;
        const element = d3.select(idSelector).node();
        // @ts-ignore
        const elementWidth = element!.getBoundingClientRect().width;
        const width = window.innerWidth;
        const height = window.innerHeight;
        d.x = d.x <= 5 ? 5 : d.x >= width - 5 ? width - 5 : d.x;
        d.y = d.y <= 5 ? 5 : d.y >= height - 5 ? height - 5 : d.y;
        return 'translate(' + (d.x - elementWidth / 2) + ',' + d.y + ')';
    };

    private tick(
        account: SVG_CIRCLE,
        accountLabel: SVG_GROUP,
        link: SVG_PATH,
        linkLabel: SVG_TEXT,
    ) {
        account.attr('cx', (d: any) => d.x).attr('cy', (d: any) => d.y);
        accountLabel.attr('transform', this.transform);
        link.attr(
            'd',
            (d: any) => `M${d.source.x},${d.source.y} L${d.target.x},${d.target.y}`,
        ).attr('transform', (d: any) => {
            const translation = this.getTransferVolumeTranslation(
                d.targetDistance * LINK_SEPARATION_OFFSET,
                d.source,
                d.target,
            );
            d.offsetX = translation.dx;
            d.offsetY = translation.dy;
            return `translate (${d.offsetX}, ${d.offsetY})`;
        });
        linkLabel.attr('transform', (d: any) => {
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

    update(data: GraphData) {
        const width = window.innerWidth;
        const height = window.innerHeight;

        const accounts: any[] = data.accounts.map((d) => ({ ...d }));
        const transferVolumes: any[] = data.transferVolumes.map((d) => ({
            id: d.id,
            source: d.from,
            target: d.to,
            count: d.count,
            volume: d.volume,
        }));

        for (let i = 0; i < transferVolumes.length; i++) {
            if (transferVolumes[i].targetDistance === -1) continue;
            transferVolumes[i].targetDistance = 0;
            for (let j = i + 1; j < transferVolumes.length; j++) {
                if (transferVolumes[j].targetDistance === -1) continue;
                if (
                    transferVolumes[i].target === transferVolumes[j].source &&
                    transferVolumes[i].source === transferVolumes[j].target
                ) {
                    transferVolumes[i].targetDistance = 1;
                    transferVolumes[j].targetDistance = -1;
                }
            }
        }

        const linkGroup = this.svg.append('g');
        const link = linkGroup
            .selectAll('path')
            .data(transferVolumes, (d: any) => d.id)
            .join('path')
            .attr('id', (d, i) => `link-${i}`)
            .attr('stroke', (d) => `#666`)
            .attr('stroke-width', (d) => 0.75)
            .attr('marker-end', (d) => `url(#transfer)`);
        const linkLabel = linkGroup
            .selectAll('text')
            .data(transferVolumes, function (d: any) {
                return d.id;
            })
            .join('text')
            .attr('class', 'link-label')
            .attr('text-anchor', 'middle')
            //.attr('dy', '0.31em');
            .attr('dy', '-0.25em');
        linkLabel
            .append('textPath')
            .attr('href', (d, i) => `#link-${i}`)
            .attr('startOffset', '50%')
            .text((d) => formatNumber(d.volume, 10, 2, 'DOT'))
            //.style('pointer-events', 'none')
            .on('mouseover', function () {
                d3.select(this).attr('cursor', 'pointer');
            })
            .on('mouseout', function () {});

        const accountGroup = this.svg.append('g');
        const account = accountGroup
            .selectAll('circle')
            .data(accounts)
            .join('circle')
            //.attr('fill', '#DDD')
            .attr('fill', '#FFF')
            .attr('stroke', '#00000033')
            .attr('stroke-width', 5.0)
            .attr('r', this.radius)
            .on('mouseover', function () {
                d3.select(this).attr('fill', '#EFEFEF');
                d3.select(this).attr('cursor', 'pointer');
            })
            .on('mouseout', function () {
                d3.select(this).attr('fill', '#FFF');
            })
            .on('dblclick', function (e, d) {
                alert(d.address);
                return false;
            });
        account.append('title').text((d) => truncateAddress(d.address));
        account.call(
            // @ts-ignore
            d3
                .drag()
                .on('start', (event) => {
                    if (!event.active) simulation.alphaTarget(0.3).restart();
                    event.subject.fx = event.subject.x;
                    event.subject.fy = event.subject.y;
                })
                .on('drag', (event) => {
                    event.subject.fx = event.x;
                    event.subject.fy = event.y;
                })
                .on('end', (event) => {
                    if (!event.active) simulation.alphaTarget(0);
                    event.subject.fx = null;
                    event.subject.fy = null;
                }),
        );

        const accountLabel = accountGroup
            .selectAll('g')
            .data(accounts, (a: any) => a.address)
            .join('g')
            .attr('id', (account: Account) => `account-${account.address}-label`);
        accountLabel
            .append('text')
            .attr('class', 'account-label')
            .attr('x', (account: Account) => getAccountConfirmedIcon(account) ? '16px' : '0')
            .attr('y', '.31em')
            // .attr('text-anchor', 'middle')
            .text((account: Account) => getAccountDisplay(account))
            .style('pointer-events', 'none');
        accountLabel
            .append('svg:image')
            .attr('xlink:href', (account: Account) => getAccountConfirmedIcon(account) ?? '')
            // .attr('x', -44)
            .attr('y', -6)
            .attr('width', 12)
            .attr('height', 12)
            .attr('opacity', (account: Account) => getAccountConfirmedIcon(account) ? 1.0 : 0);

        this.svg.call(
            // @ts-ignore
            d3
                .zoom()
                .extent([
                    [0, 0],
                    [width, height],
                ])
                .scaleExtent([0.2, 8])
                .on('zoom', (e) => {
                    linkGroup.attr('transform', e.transform);
                    accountGroup.attr('transform', e.transform);
                }),
        ).on('dblclick.zoom', null);

        const simulation = d3
            .forceSimulation(accounts)
            .force(
                'link',
                d3
                    .forceLink()
                    // @ts-ignore
                    .id((d) => d.address)
                    .distance(LINK_DISTANCE)
                    .links(transferVolumes),
            )
            .force('center', d3.forceCenter(width / 2, height / 2));
        simulation.on('tick', () => {
            this.tick(account, accountLabel, link, linkLabel);
        });
    }
}

export { Graph };
