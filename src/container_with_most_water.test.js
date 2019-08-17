const test = require("ava");
const { maxArea } = require("./container_with_most_water");

test("test1", t => {
    const input = [1,8,6,2,5,4,8,3,7];
    t.is(maxArea(input), 49);
});
