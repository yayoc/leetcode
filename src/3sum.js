var threeSum = function (nums) {
    let ret = [];
    const sortedNums = nums.sort((a, b) => a - b);
    for (let i = 0; i < sortedNums.length; i++) {
        if (sortedNums[i] === sortedNums[i - 1]) {
            continue;
        }
        const target = -sortedNums[i];
        let l = i + 1;
        let r = sortedNums.length - 1;
        while (l < r) {
            const sum = sortedNums[l] + sortedNums[r];
            if (target === sum) {
                ret.push([sortedNums[i], sortedNums[l], sortedNums[r]]);
                while (sortedNums[l] === sortedNums[l + 1]) {
                    l++;
                }
                l++;
                while (sortedNums[r] === sortedNums[r - 1]) {
                    r--;
                }
                r--;
            } else if (target > sum) {
                l++;
            } else {
                r--;
            }
        }
    }
    return ret;
};

module.exports = {
    threeSum: threeSum
};