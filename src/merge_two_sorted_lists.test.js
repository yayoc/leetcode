const test = require("ava");
const { LinkNode, mergeTwoLists } = require("./merge_two_sorted_lists");

function createNodeList(nums) {
  if (!nums.length) return null;
  const nodes = nums.map(num => new LinkNode(num));
  nodes.forEach((node, i) => {
    node.next = nodes[i + 1] || null;
  });
  return nodes[0];
}

test("createNodeList", t => {
  const l = createNodeList([1, 2, 3]);
  const node1 = new LinkNode(1);
  const node2 = new LinkNode(2);
  const node3 = new LinkNode(3);
  node1.next = node2;
  node2.next = node3;
  t.deepEqual(l, node1);
});

test("mergeTwoLists", t => {
  const l1 = createNodeList([1, 2]);
  const l2 = createNodeList([1, 2]);
  const merged = mergeTwoLists(l1, l2);
  console.log("merged", merged);
  t.deepEqual(merged, createNodeList([1, 1, 2, 2]));
});
