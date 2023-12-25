type Node<T> = {
    value: T;
    next?: Node<T>;
};

export default class Stack<T> {
    public length: number;
    private head?: Node<T>;

    constructor() {
        this.length = 0;
    }

    // T -> O(1)
    // S -> O(1)
    push(item: T): void {
        const node = { value: item } as Node<T>;
        this.length += 1;

        if (!this.head) {
            this.head = node;
            return;
        }

        node.next = this.head;
        this.head = node;
    }
    // T -> O(1)
    // S -> O(1)
    pop(): T | undefined {
        if (!this.head) {
            return;
        }

        const node = this.head;

        this.head = this.head.next;
        node.next = undefined;
        this.length -= 1;

        return node.value;
    }
    // T -> O(1)
    // S -> O(1)
    peek(): T | undefined {
        return this.head?.value;
    }
}
