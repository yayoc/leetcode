/**
 * Definition for singly-linked list.
 */
function LinkNode(val) {
  this.val = val;
  this.next = null;
}

/**
 * @param {Node} l1
 * @param {Node} l2
 * @return {Node}
 */
var mergeTwoLists = function(l1, l2) {
  if (!l1) return l2;
  if (!l2) return l1;

  if (l1.val < l2.val) {
    l1.next = mergeTwoLists(l1.next, l2);
    return l1;
  }
  l2.next = mergeTwoLists(l1, l2.next);
  return l2;
};

module.exports = {
  mergeTwoLists: mergeTwoLists,
  LinkNode: LinkNode
};
