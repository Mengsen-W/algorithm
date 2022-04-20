/*
 * @Date: 2022-04-20 09:17:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-20 09:29:40
 * @FilePath: /algorithm/388_length_longest_path/length_longest_path.go
 */

package main

func lengthLongestPath(input string) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	st := []int{}
	for i, n := 0, len(input); i < n; {
		// 检测当前文件的深度
		depth := 1
		for ; i < n && input[i] == '\t'; i++ {
			depth++
		}

		// 统计当前文件名的长度
		length, isFile := 0, false
		for ; i < n && input[i] != '\n'; i++ {
			if input[i] == '.' {
				isFile = true
			}
			length++
		}
		i++ // 跳过换行符

		for len(st) >= depth {
			st = st[:len(st)-1]
		}
		if len(st) > 0 {
			length += st[len(st)-1] + 1
		}
		if isFile {
			ans = max(ans, length)
		} else {
			st = append(st, length)
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

	assert(lengthLongestPath("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext") == 20)
	assert(lengthLongestPath("a") == 0)
	assert(lengthLongestPath("file1.txt\nfile2.txt\nlongfile.txt") == 12)
}
