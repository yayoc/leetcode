var maxArea = function(height) {
    let maxArea = 0;
    let l = 0;
    let r = height.length - 1;
    while(l < r) {
        maxArea = Math.max(maxArea, Math.min(height[l], height[r]) * (r - l));
        if (height[l] <= height[r]) {
            l++;
        } else {
            r--;
        }
    }
    return maxArea;
};

module.exports = {
    maxArea: maxArea
};