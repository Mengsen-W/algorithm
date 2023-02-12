/*
 * @Date: 2023-02-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-12
 * @FilePath: /algorithm/golang/1138_alphabet_board_path/alphabet_board_path.go
 */

package main

func alphabetBoardPath(target string) string {
	cx, cy := 0, 0
	res := []byte{}
	for _, c := range target {
		nx := int(c-'a') / 5
		ny := int(c-'a') % 5
		if nx < cx {
			for j := 0; j < cx-nx; j++ {
				res = append(res, 'U')
			}
		}
		if ny < cy {
			for j := 0; j < cy-ny; j++ {
				res = append(res, 'L')
			}
		}
		if nx > cx {
			for j := 0; j < nx-cx; j++ {
				res = append(res, 'D')
			}
		}
		if ny > cy {
			for j := 0; j < ny-cy; j++ {
				res = append(res, 'R')
			}
		}
		res = append(res, '!')
		cx = nx
		cy = ny
	}
	return string(res)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		target := "leet"
		ans := "DDR!UURRR!!DDD!"
		assert(alphabetBoardPath(target) == ans)
	}

	{
		target := "code"
		ans := "RR!DDRR!UUL!R!"
		assert(alphabetBoardPath(target) == ans)
	}
}
