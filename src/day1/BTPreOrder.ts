import { BinaryNode } from "global";

export default function pre_order_search(head: BinaryNode<number>): number[] {
    const path: number[] = [];

    function traverse(node: BinaryNode<number>) {
        if (!node) return;

        path.push(node.value);
        node.left && traverse(node.left);
        node.right && traverse(node.right);
    }

    traverse(head);

    return path;
}
