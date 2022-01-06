/*
 * @Date: 2022-01-06 01:59:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-06 02:07:50
 */

package main

import "strings"

func simplifyPath(path string) string {
	stack := []string{}
	for _, name := range strings.Split(path, "/") {
		if name == ".." {
			if len(stack) > 0 {
				stack = stack[:len(stack)-1]
			}
		} else if name != "" && name != "." {
			stack = append(stack, name)
		}
	}
	return "/" + strings.Join(stack, "/")
}

func main() {
	assert := func(a, b string) {
		if a != b {
			panic("Not Passed")
		}
	}

	assert(simplifyPath("/home/"), "/home")
	assert(simplifyPath("/../"), "/")
	assert(simplifyPath("/home//foo/"), "/home/foo")
	assert(simplifyPath("/a/./b/../../c/"), "/c")
}
