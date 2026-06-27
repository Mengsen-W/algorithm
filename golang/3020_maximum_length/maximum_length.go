// Package main ...
package main

import "fmt"

func maximumLength(nums []int) int {
	cnt := map[int64]int{}
	for _, num := range nums {
		cnt[int64(num)]++
	}

	// ans 至少是 1 的数量，向下取奇数
	oneCnt := cnt[1]
	ans := oneCnt
	if oneCnt%2 == 0 {
		ans--
	}

	delete(cnt, 1)

	for num := range cnt {
		res := 0
		x := num

		for cnt[x] > 1 {
			res += 2
			x *= x
		}

		if _, ok := cnt[x]; ok {
			ans = max(ans, res+1)
		} else {
			ans = max(ans, res-1)
		}
	}

	return ans
}

func main() {
	tests := []struct {
		nums   []int
		except int
	}{
		{[]int{5, 4, 1, 2, 2}, 3},
		{[]int{1, 3, 2, 4}, 1},
	}

	for _, test := range tests {
		result := maximumLength(test.nums)
		if result != test.except {
			fmt.Println("test failed", test.nums, result, test.except)
		}
	}
}
