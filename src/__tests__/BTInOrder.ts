import bt_in_order from "../day1/BTInOrder";
import { tree } from "./tree";

test("In order", function () {
    const res = bt_in_order(tree);
    console.log("RESULT", res);
    expect(bt_in_order(tree)).toEqual([5, 7, 10, 15, 20, 29, 30, 45, 50, 100]);
});
