import { BinaryNode } from "global";

export default function bfs(head: BinaryNode<number>, needle: number): boolean {
    if (!head) return false;

    const q = [];
    q.push(head);

    while (q.length) {
        const len = q.length;

        for (let i = 0; i < len; i++) {
            const node = q.shift();

            if (node && node.value === needle) return true;

            if (node && node.left) q.push(node.left);
            if (node && node.right) q.push(node.right);
        }
    }

    return false;
}
