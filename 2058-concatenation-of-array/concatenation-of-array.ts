function getConcatenation(nums: number[]): number[] {
    const n: number = nums.length;
    var result: number[] = new Array(n * 2);
    for(let i = 0; i < n; i++){
        result[i] = nums[i];
        result[i + n] = nums[i];
    }
    return result;
};