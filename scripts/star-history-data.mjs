const API_ROOT = "https://api.github.com";
const PAGE_SIZE = 100;
const MAX_PAGES = 400;

export function normalizeSeries(value, expectedRepository = "") {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("star history must be a JSON object");
  }
  const repository = value.repository;
  if (typeof repository !== "string" || repository.length === 0) {
    throw new Error("star history repository is required");
  }
  if (expectedRepository && repository !== expectedRepository) {
    throw new Error(
      `star history belongs to ${repository}, expected ${expectedRepository}`,
    );
  }
  if (!Array.isArray(value.points)) {
    throw new Error("star history points must be an array");
  }

  const byDate = new Map();
  for (const point of value.points) {
    if (
      !Array.isArray(point) ||
      point.length !== 2 ||
      !/^\d{4}-\d{2}-\d{2}$/.test(point[0]) ||
      !Number.isInteger(point[1]) ||
      point[1] < 0
    ) {
      throw new Error(`invalid star history point: ${JSON.stringify(point)}`);
    }
    byDate.set(point[0], point[1]);
  }

  return {
    repository,
    points: [...byDate.entries()].sort(([left], [right]) =>
      left.localeCompare(right),
    ),
  };
}

export function seriesFromStarredAt(repository, timestamps) {
  const totalsByDate = new Map();
  for (const timestamp of [...timestamps].sort()) {
    if (typeof timestamp !== "string" || timestamp.length < 10) {
      throw new Error(`invalid starred_at timestamp: ${timestamp}`);
    }
    const date = timestamp.slice(0, 10);
    totalsByDate.set(date, (totalsByDate.get(date) ?? 0) + 1);
  }

  let total = 0;
  const points = [];
  for (const [date, count] of totalsByDate) {
    total += count;
    points.push([date, total]);
  }
  return { repository, points };
}

export function appendSnapshot(series, date, count) {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(date)) {
    throw new Error(`invalid snapshot date: ${date}`);
  }
  if (!Number.isInteger(count) || count < 0) {
    throw new Error(`invalid snapshot count: ${count}`);
  }
  return normalizeSeries({
    repository: series.repository,
    points: [...series.points.filter(([pointDate]) => pointDate !== date), [date, count]],
  });
}

export async function fetchStarredAt(repository, token, fetchImpl = fetch) {
  if (!token) {
    throw new Error("a token is required for exact stargazer history");
  }

  const timestamps = [];
  for (let page = 1; page <= MAX_PAGES; page += 1) {
    const url =
      `${API_ROOT}/repos/${repository}/stargazers` +
      `?per_page=${PAGE_SIZE}&page=${page}`;
    const response = await fetchImpl(url, {
      headers: githubHeaders(token, "application/vnd.github.star+json"),
    });
    const payload = await readResponse(response, url);
    if (!Array.isArray(payload)) {
      throw new Error(`${url} returned a non-array response`);
    }
    for (const entry of payload) {
      if (typeof entry?.starred_at === "string") {
        timestamps.push(entry.starred_at);
      }
    }
    if (payload.length < PAGE_SIZE) {
      return timestamps;
    }
  }
  throw new Error(`stargazer history exceeded ${MAX_PAGES * PAGE_SIZE} entries`);
}

export async function fetchStarCount(repository, token = "", fetchImpl = fetch) {
  const url = `${API_ROOT}/repos/${repository}`;
  const response = await fetchImpl(url, {
    headers: githubHeaders(token, "application/vnd.github+json"),
  });
  const payload = await readResponse(response, url);
  if (!Number.isInteger(payload?.stargazers_count)) {
    throw new Error(`${url} returned no stargazers_count`);
  }
  return payload.stargazers_count;
}

function githubHeaders(token, accept) {
  const headers = {
    Accept: accept,
    "User-Agent": "fennara-star-history",
    "X-GitHub-Api-Version": "2022-11-28",
  };
  if (token) {
    headers.Authorization = `Bearer ${token}`;
  }
  return headers;
}

async function readResponse(response, url) {
  const body = await response.text();
  if (!response.ok) {
    const detail = body.slice(0, 500).replace(/\s+/g, " ");
    throw new Error(`${url} returned HTTP ${response.status}: ${detail}`);
  }
  try {
    return JSON.parse(body);
  } catch {
    throw new Error(`${url} returned invalid JSON`);
  }
}
