func getConcatenation(nums []int) []int {
    var n int = len(nums)
    var result = make([]int, n*2)
    for i:=0;i<n;i++{
        result[i] = nums[i]
        result[i + n] = nums[i]
    }
    return result
}