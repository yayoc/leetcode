const test = require("ava");
const { threeSum } = require("./3sum");

test("test1", t => {
    const input = [-1, 0, 1, 2, -1, -4];
    t.deepEqual([[-1, -1, 2], [-1, 0, 1]], threeSum(input));
});

