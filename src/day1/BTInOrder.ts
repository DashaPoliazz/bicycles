import { BinaryNode } from "global";

export default function in_order_search(head: BinaryNode<number>): number[] {
    const result: number[] = [];

    function traverse(node: BinaryNode<number>) {
        if (!node) return;

        node.left && traverse(node.left);
        result.push(node.value);
        node.right && traverse(node.right);
    }

    traverse(head);

    return result;
}
