/*
 * @Date: 2022-05-12 09:30:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-12 09:41:26
 * @FilePath: /algorithm/944_min_deletion_size/min_deletion_size.go
 */

package main

func minDeletionSize(strs []string) (ans int) {
	for j := range strs[0] {
		for i := 1; i < len(strs); i++ {
			if strs[i-1][j] > strs[i][j] {
				ans++
				break
			}
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(minDeletionSize([]string{"cba", "daf", "ghi"}) == 1)
	assert(minDeletionSize([]string{"a", "b"}) == 0)
	assert(minDeletionSize([]string{"zyx", "wvu", "tsr"}) == 3)
}
