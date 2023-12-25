export default class ArrayList<T> {
    public length: number;
    private capacity: number;
    private array: T[];
    private defaultCapacity: number;

    constructor(capacity: number) {
        this.length = 0;
        this.defaultCapacity = 5;
        this.capacity = capacity;
        this.array = new Array(this.capacity);
    }

    prepend(item: T): void {
        this.length += 1;

        if (this.length > this.capacity) {
            // Increasing the capacity
            this.increaseCapacity();
        }

        // Moving all items by one
        for (let i = this.length; i > 0; i--) {
            this.array[i] = this.array[i - 1];
        }

        this.array[0] = item;
    }
    insertAt(item: T, idx: number): void {
        if (idx > this.length || idx < 0) {
            return;
        } else if (idx === this.length) {
            return this.append(item);
        }

        this.array[idx] = item;
    }
    append(item: T): void {
        if (this.length + 1 > this.capacity) {
            // Increasing the capacity
            this.increaseCapacity();
        }

        this.array[this.length] = item;
        this.length += 1;
    }
    remove(item: T): T | undefined {
        // Find index of element
        let idx = 0;

        while (this.array[idx] !== item) idx += 1;

        // There is no element in array
        if (idx >= this.length) return;

        const itemToRemove = this.array[idx];

        for (let i = idx; i < this.length; i++) {
            this.array[i] = this.array[i + 1];
        }

        this.length -= 1;

        return itemToRemove;
    }
    get(idx: number): T | undefined {
        if (idx > this.length || idx < 0) {
            return;
        }

        return this.array[idx];
    }
    removeAt(idx: number): T | undefined {
        if (idx > this.length || idx < 0) {
            return;
        }

        const itemToRemove = this.array[idx];

        for (let i = idx; i < this.length; i++) {
            this.array[i] = this.array[i + 1];
        }

        return itemToRemove;
    }
    private increaseCapacity(): void {
        const newCapacity = this.length + this.defaultCapacity;
        const newArray = new Array(newCapacity);

        // Copying prev array
        for (let i = 0; i < this.length; i++) {
            newArray[i] = this.array[i];
        }

        this.capacity = newCapacity;
        this.array = newArray;
    }
}
