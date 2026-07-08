// Package main ...
package main

import (
	"fmt"
	"reflect"
)

const (
	MOD   int64 = 1000000007
	MAX_N       = 100001
)

var pow10 []int64

// init 对于所有测试用例只运行一次
func init() {
	pow10 = make([]int64, MAX_N)
	pow10[0] = 1
	for i := 1; i < MAX_N; i++ {
		pow10[i] = (pow10[i-1] * 10) % MOD
	}
}

func sumAndMultiply(s string, queries [][]int) []int {
	n := len(s)
	sum := make([]int, n+1)
	x := make([]int64, n+1)
	cnt := make([]int, n+1)
	for i := 0; i < n; i++ {
		d := int(s[i] - '0')
		sum[i+1] = sum[i] + d
		if d > 0 {
			x[i+1] = (x[i]*10 + int64(d)) % MOD
			cnt[i+1] = cnt[i] + 1
		} else {
			x[i+1] = x[i]
			cnt[i+1] = cnt[i]
		}
	}
	m := len(queries)
	res := make([]int, m)
	for i := 0; i < m; i++ {
		l := queries[i][0]
		r := queries[i][1] + 1
		length := cnt[r] - cnt[l]
		val_x := (x[r] - x[l]*pow10[length]%MOD + MOD) % MOD
		val_sum := int64(sum[r] - sum[l])
		res[i] = int((val_x * val_sum) % MOD)
	}
	return res
}

func main() {
	tests := []struct {
		s       string
		queries [][]int
		ans     []int
	}{
		{"10203004", [][]int{{0, 7}, {1, 3}, {4, 6}}, []int{12340, 4, 9}},
		{"1000", [][]int{{0, 3}, {1, 1}}, []int{1, 0}},
		{"9876543210", [][]int{{0, 9}}, []int{444444137}},
	}

	for _, test := range tests {
		res := sumAndMultiply(test.s, test.queries)
		if !reflect.DeepEqual(res, test.ans) {
			fmt.Println("Test failed:", test.s, test.queries, res, test.ans)
		}
	}
}
