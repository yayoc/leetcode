const test = require("ava");
const { longestPalindrome } = require("./longest_palindromic_substring");

test("test1", t => {
    const input = "babad";
    const output = "bab";
    t.is(longestPalindrome(input), output);
});

test("test2", t => {
    const input = "cbbd";
    const output = "bb";
    t.is(longestPalindrome(input), output);
});
