/*
 * @Date: 2023-03-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-07
 * @FilePath: /algorithm/golang/1096_brace_expansion_ii/brace_expansion_ii.go
 */

package main

import (
	"reflect"
	"sort"
)

func braceExpansionII(expression string) []string {
	n := len(expression)
	string2add := func(set map[string]bool, str string) map[string]bool {
		if len(str) == 0 {
			return set
		}
		if len(set) == 0 {
			set[str] = true
			return set
		}
		ret := map[string]bool{}
		for k := range set {
			ret[k+str] = true
		}
		return ret
	}
	add := func(set, set1 map[string]bool) map[string]bool {
		if len(set) == 0 {
			return set1
		}
		for k := range set1 {
			set[k] = true
		}
		return set
	}
	mul := func(set, set1 map[string]bool) map[string]bool {
		if len(set1) == 0 {
			return set
		}
		//if len(set) == 0 { set = set1 }
		if len(set) == 0 {
			set = add(set, set1)
			return set
		}
		ret := map[string]bool{}
		for k := range set {
			for t := range set1 {
				ret[k+t] = true
			}
		}
		return ret
	}
	var dfs func(start, end int) map[string]bool // 去重
	dfs = func(start, end int) (res map[string]bool) {
		//fmt.Println(expression[start:end+1])
		// res 表示当前层的集合
		tmp := []byte{}        // 缓存当前层当前段处理的字符，注 , 为分割符号
		s := map[string]bool{} // 缓存当前层当前段的集合 注 , 为分割符号
		i := start
		for i <= end {
			switch expression[i] {
			case ',':
				res = add(res, string2add(s, string(tmp)))
				s, tmp = map[string]bool{}, []byte{} //重置
			case '{':
				i++
				t, cnt := i, 1
				for cnt > 0 && i <= end { // 找 } 注意花括号可嵌套
					if expression[i] == '{' {
						cnt++
					}
					if expression[i] == '}' {
						cnt--
					}
					i++
				}
				s = string2add(s, string(tmp))
				tmp = []byte{}
				s = mul(s, dfs(t, i-2)) // i-2 排除掉 }
				i--
			default:
				tmp = append(tmp, expression[i])
			}
			i++
		}
		res = add(res, string2add(s, string(tmp)))
		return res
	}
	ans := []string{}
	for k := range dfs(0, n-1) {
		ans = append(ans, k)
	}
	sort.Strings(ans) // 题目要求排序
	return ans
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		expression := "{a,b}{c,{d,e}}"
		ans := []string{"ac", "ad", "ae", "bc", "bd", "be"}
		assert(braceExpansionII(expression), ans)
	}

	{
		expression := "{{a,z},a{b,c},{ab,z}}"
		ans := []string{"a", "ab", "ac", "z"}
		assert(braceExpansionII(expression), ans)
	}
}
