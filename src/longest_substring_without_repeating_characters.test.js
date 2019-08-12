const test = require("ava");
const { lengthOfLongestSubstring } = require("./longest_substring_without_repeating_characters");

test("test1", t => {
  const input = "abcabcbb";
  const output = 3;
  t.is(lengthOfLongestSubstring(input), output); // abc
});

test("test2", t => {
  const input = "bbbbb";
  const output = 1;
  t.is(lengthOfLongestSubstring(input), output); // b
});

test("test3", t => {
  const input = "pwwkew";
  const output = 3;
  t.is(lengthOfLongestSubstring(input), output); // wke
});

test("test4", t => {
  const input = " ";
  const output = 1;
  t.is(lengthOfLongestSubstring(input), output);
});

test("test5", t => {
  const input = "aa";
  const output = 1;
  t.is(lengthOfLongestSubstring(input), output);
});

test("test6", t => {
  const input = "au";
  const output = 2;
  t.is(lengthOfLongestSubstring(input), output);
});
