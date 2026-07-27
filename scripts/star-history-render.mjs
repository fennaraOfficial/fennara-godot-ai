const WIDTH = 700;
const HEIGHT = 360;
const PLOT = { left: 62, right: 664, top: 76, bottom: 292 };

export function niceScale(maximum) {
  const safeMaximum = Math.max(1, maximum);
  const roughStep = safeMaximum / 5;
  const magnitude = 10 ** Math.floor(Math.log10(roughStep));
  const normalized = roughStep / magnitude;
  const factor = [1, 2, 2.5, 5, 10].find((candidate) => candidate >= normalized);
  const step = factor * magnitude;
  return {
    step,
    ceiling: Math.ceil(safeMaximum / step) * step,
  };
}

export function renderStarHistorySvg(series, updatedDate) {
  if (series.points.length < 2) {
    throw new Error("at least two star history points are required");
  }

  const points = series.points.map(([date, count]) => ({
    date,
    time: Date.parse(`${date}T00:00:00Z`),
    count,
  }));
  const first = points[0];
  const last = points.at(-1);
  const scale = niceScale(Math.max(...points.map(({ count }) => count)));
  const x = (time) =>
    PLOT.left +
    ((time - first.time) / Math.max(last.time - first.time, 1)) *
      (PLOT.right - PLOT.left);
  const y = (count) =>
    PLOT.bottom -
    (count / scale.ceiling) * (PLOT.bottom - PLOT.top);

  const linePath = points
    .map(
      (point, index) =>
        `${index === 0 ? "M" : "L"} ${decimal(x(point.time))} ${decimal(y(point.count))}`,
    )
    .join(" ");
  const areaPath =
    `${linePath} L ${PLOT.right} ${PLOT.bottom}` +
    ` L ${PLOT.left} ${PLOT.bottom} Z`;

  const yTicks = [];
  for (let value = 0; value <= scale.ceiling; value += scale.step) {
    const position = decimal(y(value));
    yTicks.push(
      `    <line x1="${PLOT.left}" x2="${PLOT.right}" y1="${position}" y2="${position}"/>`,
      `    <text x="${PLOT.left - 10}" y="${decimal(y(value) + 3)}" text-anchor="end">${formatCount(value)}</text>`,
    );
  }

  const monthTicks = createMonthTicks(first.time, last.time).map(
    ({ time, label, anchor }) =>
      `    <text x="${decimal(x(time))}" y="${PLOT.bottom + 22}" text-anchor="${anchor}">${label}</text>`,
  );
  const endX = x(last.time);
  const endY = y(last.count);
  const repository = escapeXml(series.repository);
  const firstStars = `${formatCount(first.count)} ${first.count === 1 ? "star" : "stars"}`;
  const lastStars = `${formatCount(last.count)} ${last.count === 1 ? "star" : "stars"}`;

  return `<svg xmlns="http://www.w3.org/2000/svg" width="${WIDTH}" height="${HEIGHT}" viewBox="0 0 ${WIDTH} ${HEIGHT}" role="img" aria-labelledby="title desc">
  <title id="title">GitHub star history for ${repository}</title>
  <desc id="desc">${repository} grew from ${firstStars} on ${first.date} to ${lastStars} on ${last.date}.</desc>
  <style>
    :root {
      --bg: #ffffff;
      --text: #1f2328;
      --muted: #59636e;
      --border: #d1d9e0;
      --grid: #d8dee4;
      --accent: #8250df;
    }
    @media (prefers-color-scheme: dark) {
      :root {
        --bg: #0d1117;
        --text: #f0f6fc;
        --muted: #9198a1;
        --border: #30363d;
        --grid: #21262d;
        --accent: #a371f7;
      }
    }
    text { fill: var(--text); font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; font-weight: 400; }
    .surface { fill: var(--bg); stroke: var(--border); }
    .repo { font-size: 12px; font-weight: 500; }
    .descriptor, .updated, .axes text { fill: var(--muted); }
    .descriptor, .updated { font-size: 11px; }
    .axes text { font-size: 10px; }
    .axes line { stroke: var(--grid); stroke-width: 1; }
    .area { fill: var(--accent); opacity: 0.07; }
    .curve { fill: none; stroke: var(--accent); stroke-width: 2.5; stroke-linecap: round; stroke-linejoin: round; }
    .endpoint { fill: var(--accent); }
    .current { font-size: 12px; font-weight: 500; }
  </style>
  <rect class="surface" x="1" y="1" width="698" height="358" rx="14"/>
  <text class="repo" x="36" y="42">${repository}</text>
  <text class="descriptor" x="664" y="42" text-anchor="end">GitHub stars over time</text>
  <g class="axes">
${yTicks.join("\n")}
${monthTicks.join("\n")}
  </g>
  <path class="area" d="${areaPath}"/>
  <path class="curve" d="${linePath}"/>
  <circle class="endpoint" cx="${decimal(endX)}" cy="${decimal(endY)}" r="4"/>
  <text class="current" x="${decimal(endX - 10)}" y="${decimal(endY - 10)}" text-anchor="end">${formatCount(last.count)} stars</text>
  <text class="updated" x="664" y="334" text-anchor="end">updated ${escapeXml(updatedDate)}</text>
</svg>
`;
}

function createMonthTicks(firstTime, lastTime) {
  const cursor = new Date(firstTime);
  cursor.setUTCDate(1);
  const months = [];
  while (cursor.getTime() <= lastTime) {
    if (cursor.getTime() >= firstTime) {
      months.push(new Date(cursor));
    }
    cursor.setUTCMonth(cursor.getUTCMonth() + 1);
  }
  if (months.length === 0 || months[0].getTime() !== firstTime) {
    months.unshift(new Date(firstTime));
  }

  const stride = Math.max(1, Math.ceil(months.length / 4));
  const visible = months.filter(
    (_, index) => index % stride === 0 || index === months.length - 1,
  );
  return visible.map((date, index) => ({
    time: date.getTime(),
    label:
      index === 0
        ? date.toLocaleDateString("en-US", {
            month: "short",
            year: "numeric",
            timeZone: "UTC",
          })
        : date.toLocaleDateString("en-US", {
            month: "short",
            timeZone: "UTC",
          }),
    anchor:
      index === 0
        ? "start"
        : date.getTime() === lastTime
          ? "end"
          : "middle",
  }));
}

function formatCount(value) {
  return new Intl.NumberFormat("en-US").format(value);
}

function decimal(value) {
  return Number(value.toFixed(1));
}

function escapeXml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&apos;");
}
