/**
 * @param {string} s
 * @return {string}
 */
var longestPalindrome = function(s) {
    if (!s || s.length <= 1) {
        return s;
    }
    let palindrome = s[0];
    for (let i = 0; i < s.length; i++) {
        let s1 = expand(s, i, i);
        let s2 = expand(s, i, i + 1);
        if (s1.length > palindrome.length) {
            palindrome = s1;
        }
        if (s2.length > palindrome.length) {
            palindrome = s2;
        }
    }
    return palindrome;
};

function expand(s, start, end) {
    while (start >= 0 && end <= s.length - 1 && s[start] === s[end]) {
        start--;
        end++;
    }
    return s.substring(start + 1, end);
}

module.exports = {
    longestPalindrome: longestPalindrome
};