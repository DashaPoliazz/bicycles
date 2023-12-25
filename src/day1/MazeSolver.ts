import { Point } from "global";

const dirs = [
    [-1, 0],
    [0, 1],
    [1, 0],
    [0, -1],
];

function walk(
    maze: string[],
    wall: string,
    curr: Point,
    end: Point,
    seen: Boolean[][],
    path: Point[],
): boolean {
    // Base case:
    // 1. It's a wall
    if (maze[curr.y][curr.x] === wall) return false;
    // 2. Off the map
    if (curr.y >= maze.length || curr.x >= maze[0].length) return false;
    // 3. It's the end
    if (curr.y === end.y && curr.x === end.x) {
        path.push(end);
        return true;
    }
    // 4. If we have seen it
    if (seen[curr.y][curr.x]) return false;

    // Mark current point as seen
    seen[curr.y][curr.x] = true;
    path.push(curr);

    for (const [dX, dY] of dirs) {
        if (
            walk(
                maze,
                wall,
                { x: curr.x + dX, y: curr.y + dY },
                end,
                seen,
                path,
            )
        ) {
            return true;
        }
    }

    path.pop();

    return false;
}

export default function solve(
    maze: string[],
    wall: string,
    start: Point,
    end: Point,
): Point[] {
    const seen: Boolean[][] = new Array(maze.length)
        .fill(false)
        .map((_) => new Array(maze[0].length).fill(false));
    const path: Point[] = [];

    // filling the path with walk fn
    walk(maze, wall, start, end, seen, path);

    return path;
}
