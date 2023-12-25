import SinglyLinkedList from "../day1/SinglyLinkedList";
import { test_list } from "./ListTest";

test("linked-list", function () {
    const list = new SinglyLinkedList<number>();
    test_list(list);
});
