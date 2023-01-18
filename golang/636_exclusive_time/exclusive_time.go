/*
 * @Date: 2022-08-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-07
 * @FilePath: /algorithm/636_exclusive_time/exclusive_time.go
 */

package main

import (
	"reflect"
	"strconv"
	"strings"
)

func exclusiveTime(n int, logs []string) []int {
	ans := make([]int, n)
	type pair struct{ idx, timestamp int }
	st := []pair{}
	for _, log := range logs {
		sp := strings.Split(log, ":")
		idx, _ := strconv.Atoi(sp[0])
		timestamp, _ := strconv.Atoi(sp[2])
		if sp[1][0] == 's' {
			if len(st) > 0 {
				ans[st[len(st)-1].idx] += timestamp - st[len(st)-1].timestamp
				st[len(st)-1].timestamp = timestamp
			}
			st = append(st, pair{idx, timestamp})
		} else {
			p := st[len(st)-1]
			st = st[:len(st)-1]
			ans[p.idx] += timestamp - p.timestamp + 1
			if len(st) > 0 {
				st[len(st)-1].timestamp = timestamp + 1
			}
		}
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		n := 2
		logs := []string{"0:start:0", "1:start:2", "1:end:5", "0:end:6"}
		ans := []int{3, 4}
		assert(exclusiveTime(n, logs), ans)
	}
	{
		n := 1
		logs := []string{"0:start:0", "0:start:2", "0:end:5", "0:start:6", "0:end:6", "0:end:7"}
		ans := []int{8}
		assert(exclusiveTime(n, logs), ans)
	}
	{
		n := 2
		logs := []string{"0:start:0", "0:start:2", "0:end:5", "1:start:6", "1:end:6", "0:end:7"}
		ans := []int{7, 1}
		assert(exclusiveTime(n, logs), ans)
	}
	{
		n := 2
		logs := []string{"0:start:0", "0:start:2", "0:end:5", "1:start:7", "1:end:7", "0:end:8"}
		ans := []int{8, 1}
		assert(exclusiveTime(n, logs), ans)
	}
	{
		n := 1
		logs := []string{"0:start:0", "0:end:0"}
		ans := []int{1}
		assert(exclusiveTime(n, logs), ans)
	}
}
