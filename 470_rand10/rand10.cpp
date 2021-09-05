/*
 * @Date: 2021-09-05 10:07:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-05 10:11:11
 */

class Solution {
 public:
  int rand7() {}
  int rand10() {
    int row, col, idx;
    do {
      row = rand7();
      col = rand7();
      idx = col + (row - 1) * 7;
    } while (idx > 40);
    return 1 + (idx - 1) % 10;
  }
};
