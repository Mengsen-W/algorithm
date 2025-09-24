// Package main ...
package main

func zeroFilledSubarray(nums []int) (ans int64) {
	cnt0 := 0
	for _, x := range nums {
		if x != 0 {
			cnt0 = 0
		} else {
			cnt0++ // 右端点为 i 的全 0 子数组比右端点为 i-1 的全 0 子数组多一个
			ans += int64(cnt0)
		}
	}
	return
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{1, 3, 0, 0, 2, 0, 0, 4}, 6},
		{[]int{0, 0, 0, 2, 0, 0}, 9},
		{[]int{2, 10, 2019}, 0},
	}

	for _, test := range tests {
		ans := zeroFilledSubarray(test.nums)
		if ans != test.ans {
			panic("error")
		}
	}
}
