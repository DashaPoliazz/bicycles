import { BinaryNode } from "global";

export default function post_order_search(head: BinaryNode<number>): number[] {
    const path: number[] = [];

    function traverse(node: BinaryNode<number>) {
        if (!node) return;

        node.left && traverse(node.left);
        node.right && traverse(node.right);

        path.push(node.value);
    }

    traverse(head);

    return path;
}
