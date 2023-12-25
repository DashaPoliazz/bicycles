class Node<T> {
    value: T;
    prev: Node<T> | undefined;
    next: Node<T> | undefined;

    constructor(value: T) {
        this.value = value;
        this.prev = undefined;
        this.next = undefined;
    }
}

export default class DoublyLinkedList<T> {
    public length: number;
    private head: Node<T> | undefined;
    private tail: Node<T> | undefined;

    constructor() {
        this.length = 0;
        this.head = undefined;
        this.tail = undefined;
    }

    prepend(item: T): void {
        const node = new Node<T>(item);

        this.length += 1;
        if (!this.head) {
            this.head = this.tail = node;
            return;
        }

        node.next = this.head;
        this.head!.prev = node;
        this.head = node;
    }

    insertAt(item: T, idx: number): void {
        if (idx < 0 || idx > this.length) return;
        if (idx === 0) return this.prepend(item);
        if (idx === this.length) return this.append(item);

        let curr = this.getAt(idx);

        const node = new Node(item);
        const prev = curr!.prev;

        prev!.next = node;
        node.prev = prev;
        node.next = curr;
        curr!.prev = node;
        this.length += 1;
    }
    append(item: T): void {
        if (this.length === 0) return this.prepend(item);

        const node = new Node(item);

        node.prev = this.tail;
        this.tail!.next = node;
        this.tail = node;
        this.length += 1;
    }
    remove(item: T): T | undefined {
        let curr = this.head;

        while (curr && curr.value !== item) curr = curr.next;

        if (!curr) return;

        if (curr === this.head) {
            if (this.head === this.tail) this.head = this.tail = undefined;

            this.head!.next!.prev = undefined;
            this.head = this.head!.next;
            this.head!.next = undefined;
        } else if (curr === this.tail) {
            this.tail!.prev!.next = undefined;
            this.tail = this.tail.prev;
            curr.prev = undefined;
        } else {
            curr.prev!.next = curr.next;
            curr.next!.prev = curr.prev;
            curr.next = undefined;
            curr.prev = undefined;
        }

        this.length -= 1;

        return item;
    }
    get(idx: number): T | undefined {
        return this.getAt(idx)?.value;
    }
    removeAt(idx: number): T | undefined {
        const node = this.getAt(idx);
        if (!node) return;

        this.length -= 1;

        if (this.length === 0) {
            const out = node.value;
            this.head = this.tail = undefined;
            return out;
        }

        if (node.prev) {
            node.prev.next = node.next;
        }
        if (node.next) {
            node.next.prev = node.prev;
        }

        if (node === this.head) {
            this.head = node.next;
        }
        if (node === this.tail) {
            this.tail = node.prev;
        }

        const out = node.value;

        node.prev = node.next = undefined;

        return out;
    }

    private getAt(index: number): Node<T> | undefined {
        if (!this.length) return;
        if (index < 0 || index >= this.length) return;

        let curr = this.head;

        for (let i = 0; curr && i < index; i++) curr = curr.next;

        return curr;
    }
    debug() {
        console.log(`LENGTH: ${this.length}`);
    }
}
