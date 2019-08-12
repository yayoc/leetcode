var lengthOfLongestSubstring = function(s) {
  if (!s) return 0;
  if (s.length === 1) return 1;

  let maxLen = 1;
  let curLen = 1;
  let visited = {};
  visited[s[0]] = 0;
  for (let i = 1; i < s.length; i++) {
    const char = s[i];
    if (visited[char] === undefined || i - curLen > visited[char]) {
      curLen += 1;
    } else {
      if (curLen > maxLen) {
        maxLen = curLen;
      }
      curLen = i - visited[char];
    }
    if (curLen > maxLen) {
      maxLen = curLen;
    }
    visited[char] = i;
  }
  return maxLen;
};

module.exports = {
    lengthOfLongestSubstring: lengthOfLongestSubstring,
};