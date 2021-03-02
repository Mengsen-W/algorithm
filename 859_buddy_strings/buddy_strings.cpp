/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-02 15:56:24
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-02 16:03:49
 */

#include <algorithm>
#include <cassert>
#include <string>

using namespace std;

bool buddy_strings(string A, string B) {
  int aLength = A.size();
  int bLength = B.size();
  // 长度不相等, 肯定不是亲密字符串
  // 长度1是特殊情况
  if (aLength != bLength || aLength == 1) {
    return false;
  }
  // 两个字符串相等
  if (A == B) {
    // 排序, 看看字符串中是否有字符个数大于1
    sort(A.begin(), A.end());
    auto ptr = unique(A.begin(), A.end());
    if (ptr == A.end()) {
      // 所有字符都互不相同, 不满足交换两个字符位置后 两个字符串还相等
      return false;
    }
    return true;
  }

  int firstPos = -1;
  int secondPos = -1;
  // 找到两个字符串中不相等的位置
  for (auto i = 0; i < aLength; ++i) {
    if (A[i] == B[i]) continue;

    if (firstPos == -1) {
      firstPos = i;
      continue;
    }

    if (secondPos == -1) {
      secondPos = i;
      continue;
    }

    // 有3个以上不相等的位置, 直接返回
    return false;
  }

  // 小于两个不相等的位置, 不能交换
  if (firstPos == -1 || secondPos == -1) {
    return false;
  }

  // 满足题目给出的条件
  if (A[firstPos] == B[secondPos] && A[secondPos] == B[firstPos]) {
    return true;
  }

  // 其他情况
  return false;
}

int main() {
  assert(buddy_strings("ab", "ba"));
  assert(!buddy_strings("ab", "ab"));
  assert(buddy_strings("aa", "aa"));
  assert(buddy_strings("aaaaaaabc", "aaaaaaacb"));
  return 0;
}