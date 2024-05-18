import * as d3 from 'd3';
import { Account, Data } from '../data/data';
import { formatNumber, truncateAddress } from '../util/format';

const LINK_DISTANCE = 300;
const LINK_ARROW_SIZE = 8;
const LINK_SEPARATION_OFFSET = 12;
const ACCOUNT_RADIUS = 60;

class Graph {
    private readonly accounts: any[];
    private readonly transferVolumes: any[];

    private readonly radius = (account: Account) => ACCOUNT_RADIUS;
    private readonly width = window.innerWidth;
    private readonly height = window.innerHeight;

    start() {
        const simulation = d3
            .forceSimulation(this.accounts)
            .force(
                'link',
                d3
                    .forceLink()
                    // @ts-ignore
                    .id((account) => account.address)
                    .distance(LINK_DISTANCE)
                    .links(this.transferVolumes),
            )
            // .force('charge', d3.forceManyBody())
            .force('center', d3.forceCenter(this.width / 2, this.height / 2));
        //.force('x', d3.forceX(this.width / 2))
        //.force('y', d3.forceY(this.height / 2))

        const svg = d3
            .select('#chart-container')
            .append('svg')
            .attr('width', this.width)
            .attr('height', this.height)
            .attr('viewBox', [0, 0, this.width, this.height])
            .attr('style', 'max-width: 100%; max-height: 100%;');

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

        const linkGroup = svg.append('g');
        const link = linkGroup
            .selectAll('path')
            .data(this.transferVolumes)
            .enter()
            .append('path')
            .attr('id', (d, i) => `link-${i}`)
            .attr('stroke', (d) => `#666`)
            .attr('stroke-width', (d) => 0.75)
            .attr('marker-end', (d) => `url(#transfer)`);
        const linkLabel = linkGroup
            .selectAll('text')
            .data(this.transferVolumes)
            .enter()
            .append('text')
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

        const accountGroup = svg.append('g');
        const account = accountGroup
            .selectAll('circle')
            .data(this.accounts)
            .join('circle')
            .attr('fill', '#DDD')
            .attr('stroke', '#00000033')
            .attr('stroke-width', 5.0)
            .attr('r', this.radius)
            .on('mouseover', function () {
                d3.select(this).attr('fill', '#EFEFEF');
                d3.select(this).attr('cursor', 'pointer');
            })
            .on('mouseout', function () {
                d3.select(this).attr('fill', '#DDD');
            })
            .on('dblclick', function (e, d) {
                alert(d.address);
                return false;
            });

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
            .selectAll('text')
            .data(this.accounts)
            .enter()
            .append('text')
            .attr('class', 'account-label')
            .attr('x', (d) => '0')
            //.attr('x', (d) => '.91em')
            .attr('y', '.31em')
            .attr('text-anchor', 'middle')
            .text((account: Account) => truncateAddress(account.address))
            .style('pointer-events', 'none');
        account.append('title').text((d) => truncateAddress(d.address));

        const identityIconGroup = svg.append('g');
        const identityIcon = identityIconGroup
            .selectAll('image')
            .data(this.accounts)
            .enter()
            .append('svg:image')
            .attr('xlink:href', function (d) {
                return '/img/icon/id-confirmed-icon.svg';
            })
            .attr('x', -44)
            .attr('y', -8)
            .attr('width', 16)
            .attr('height', 16)
            .attr('opacity', (d) => 1.0);

        svg.call(
            // @ts-ignore
            d3
                .zoom()
                .extent([
                    [0, 0],
                    [this.width, this.height],
                ])
                .scaleExtent([0.2, 8])
                .on('zoom', (e) => {
                    linkGroup.attr('transform', e.transform);
                    accountGroup.attr('transform', e.transform);
                    identityIconGroup.attr('transform', e.transform);
                }),
        ).on('dblclick.zoom', null);

        simulation.on('tick', () => {
            account.attr('cx', (d) => d.x).attr('cy', (d) => d.y);
            accountLabel.attr('transform', transform);
            identityIcon.attr('transform', transform);
            link.attr('d', (d) => `M${d.source.x},${d.source.y} L${d.target.x},${d.target.y}`).attr(
                'transform',
                (d) => {
                    const translation = getTransferVolumeTranslation(
                        d.targetDistance * LINK_SEPARATION_OFFSET,
                        d.source,
                        d.target,
                    );
                    d.offsetX = translation.dx;
                    d.offsetY = translation.dy;
                    return `translate (${d.offsetX}, ${d.offsetY})`;
                },
            );
            linkLabel.attr('transform', (d) => {
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
        });

        function getTransferVolumeTranslation(targetDistance: any, point0: any, point1: any) {
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

        const transform = (d: any) => {
            d.x = d.x <= 5 ? 5 : d.x >= this.width - 5 ? this.width - 5 : d.x;
            d.y = d.y <= 5 ? 5 : d.y >= this.height - 5 ? this.height - 5 : d.y;
            return 'translate(' + d.x + ',' + d.y + ')';
        };
    }

    constructor(data: Data) {
        this.accounts = data.accounts.map((d) => ({ ...d }));
        this.transferVolumes = data.transferVolumes.map((d) => ({
            id: d.id,
            source: d.from,
            target: d.to,
            count: d.count,
            volume: d.volume,
        }));

        for (let i = 0; i < this.transferVolumes.length; i++) {
            if (this.transferVolumes[i].targetDistance === -1) continue;
            this.transferVolumes[i].targetDistance = 0;
            for (let j = i + 1; j < this.transferVolumes.length; j++) {
                if (this.transferVolumes[j].targetDistance === -1) continue;
                if (
                    this.transferVolumes[i].target === this.transferVolumes[j].source &&
                    this.transferVolumes[i].source === this.transferVolumes[j].target
                ) {
                    this.transferVolumes[i].targetDistance = 1;
                    this.transferVolumes[j].targetDistance = -1;
                }
            }
        }
    }
}

export { Graph };
