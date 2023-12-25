type Node<T> = {
    value: T;
    next?: Node<T>;
};

export default class Queue<T> {
    public length: number;
    private head?: Node<T>;
    private tail?: Node<T>;

    constructor() {
        this.head = this.tail = undefined;
        this.length = 0;
    }

    // S -> O(1)
    // T -> O(1)
    enqueue(item: T): void {
        this.length += 1;

        if (!this.head) {
            this.head = this.tail = { value: item } as Node<T>;
            return;
        }

        const node = { value: item } as Node<T>;
        this.tail!.next = node;
        this.tail = node;
    }
    // S -> O(1)
    // T -> O(1)
    deque(): T | undefined {
        if (!this.head) {
            return;
        }

        this.length -= 1;

        const head = this.head;

        this.head = this.head.next;
        head.next = undefined;

        return head.value;
    }
    // S -> O(1)
    // T -> O(1)
    peek(): T | undefined {
        return this.head?.value;
    }
}
