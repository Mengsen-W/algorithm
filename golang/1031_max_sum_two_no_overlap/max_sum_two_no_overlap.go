package main

func maxSumTwoNoOverlap(nums []int, firstLen int, secondLen int) int {
	max := func(a int, b int) int {
		if a > b {
			return a
		}
		return b
	}
	help := func(nums []int, firstLen int, secondLen int) int {
		suml := 0
		for i := 0; i < firstLen; i++ {
			suml += nums[i]
		}
		maxSumL := suml
		sumr := 0
		for i := firstLen; i < firstLen+secondLen; i++ {
			sumr += nums[i]
		}
		res := maxSumL + sumr
		for i, j := firstLen+secondLen, firstLen; i < len(nums); i, j = i+1, j+1 {
			suml += nums[j] - nums[j-firstLen]
			maxSumL = max(maxSumL, suml)
			sumr += nums[i] - nums[i-secondLen]
			res = max(res, maxSumL+sumr)
		}
		return res
	}
	return max(help(nums, firstLen, secondLen), help(nums, secondLen, firstLen))
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{0, 6, 5, 2, 2, 5, 1, 9, 4}
		firstLen := 1
		secondLen := 2
		ans := 20
		assert(maxSumTwoNoOverlap(nums, firstLen, secondLen) == ans)
	}

	{
		nums := []int{3, 8, 1, 3, 2, 1, 8, 9, 0}
		firstLen := 3
		secondLen := 2
		ans := 29
		assert(maxSumTwoNoOverlap(nums, firstLen, secondLen) == ans)
	}

	{
		nums := []int{2, 1, 5, 6, 0, 9, 5, 0, 3, 8}
		firstLen := 4
		secondLen := 3
		ans := 31
		assert(maxSumTwoNoOverlap(nums, firstLen, secondLen) == ans)
	}
}
