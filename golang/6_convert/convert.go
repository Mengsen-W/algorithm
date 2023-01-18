/*
 * @Date: 2022-03-01 00:06:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-01 00:37:59
 * @FilePath: /algorithm/6_convert/convert.go
 */

package main

import (
	"bytes"
	"reflect"
)

func convert1(s string, numRows int) string {
	n, r := len(s), numRows
	if r == 1 || r >= n {
		return s
	}
	t := r*2 - 2
	c := (n + t - 1) / t * (r - 1)
	mat := make([][]byte, r)
	for i := range mat {
		mat[i] = make([]byte, c)
	}
	x, y := 0, 0
	for i, ch := range s {
		mat[x][y] = byte(ch)
		if i%t < r-1 {
			x++ // 向下移动
		} else {
			x--
			y++ // 向右上移动
		}
	}
	ans := make([]byte, 0, n)
	for _, row := range mat {
		for _, ch := range row {
			if ch > 0 {
				ans = append(ans, ch)
			}
		}
	}
	return string(ans)
}

func convert2(s string, numRows int) string {
	r := numRows
	if r == 1 || r >= len(s) {
		return s
	}
	mat := make([][]byte, r)
	t, x := r*2-2, 0
	for i, ch := range s {
		mat[x] = append(mat[x], byte(ch))
		if i%t < r-1 {
			x++
		} else {
			x--
		}
	}
	return string(bytes.Join(mat, nil))
}

func convert3(s string, numRows int) string {
	n, r := len(s), numRows
	if r == 1 || r >= n {
		return s
	}
	t := r*2 - 2
	ans := make([]byte, 0, n)
	for i := 0; i < r; i++ { // 枚举矩阵的行
		for j := 0; j+i < n; j += t { // 枚举每个周期的起始下标
			ans = append(ans, s[j+i]) // 当前周期的第一个字符
			if 0 < i && i < r-1 && j+t-i < n {
				ans = append(ans, s[j+t-i]) // 当前周期的第二个字符
			}
		}
	}
	return string(ans)
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(convert1("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR")
	assert(convert2("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR")
	assert(convert3("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR")

	assert(convert1("PAYPALISHIRING", 4), "PINALSIGYAHRPI")
	assert(convert2("PAYPALISHIRING", 4), "PINALSIGYAHRPI")
	assert(convert3("PAYPALISHIRING", 4), "PINALSIGYAHRPI")

	assert(convert1("A", 1), "A")
	assert(convert2("A", 1), "A")
	assert(convert3("A", 1), "A")
}
