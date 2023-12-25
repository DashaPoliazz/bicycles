export default class SinglyLinkedList<T> {
    public length: number;

    constructor() {}

    prepend(item: T): void {}
    append(item: T): void {}
    insertAt(item: T, idx: number): void {}
    remove(item: T): T | undefined {}
    get(idx: number): T | undefined {}
    removeAt(idx: number): T | undefined {}
}
