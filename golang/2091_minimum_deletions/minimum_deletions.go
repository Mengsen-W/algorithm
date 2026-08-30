// Package main ...
package main

func minimumDeletions(nums []int) int {
	n := len(nums)

	// 找到最小值和最大值的下标
	minidx, maxidx := 0, 0
	for i := 0; i < n; i++ {
		if nums[i] < nums[minidx] {
			minidx = i
		}
		if nums[i] > nums[maxidx] {
			maxidx = i
		}
	}
	l := min(minidx, maxidx) // 最值下标中的较小值
	r := max(minidx, maxidx) // 最值下标中的较大值

	// 计算三种情况下删除次数的最小值
	return min(min(r+1, n-l), l+1+n-r)
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 10, 7, 5, 4, 1, 8, 6}, 5},
		{[]int{0, -4, 19, 1, 8, -2, -3, 5}, 3},
		{[]int{101}, 1},
	}

	for _, test := range tests {
		if ans := minimumDeletions(test.nums); ans != test.ans {
			println("error:", test.nums, ans, test.ans)
		} else {
			println("success:", test.nums, ans)
		}
	}
}
