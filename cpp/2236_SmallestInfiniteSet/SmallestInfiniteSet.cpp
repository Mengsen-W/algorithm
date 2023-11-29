/*
 * @Date: 2023-11-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-29
 * @FilePath: /algorithm/cpp/2236_SmallestInfiniteSet/SmallestInfiniteSet.cpp
 */

#include <cassert>
#include <set>

using namespace std;

class SmallestInfiniteSet {
 public:
  SmallestInfiniteSet() {}

  int popSmallest() {
    if (s.empty()) {
      int ans = thres;
      ++thres;
      return ans;
    }
    int ans = *s.begin();
    s.erase(s.begin());
    return ans;
  }

  void addBack(int num) {
    if (num < thres) {
      s.insert(num);
    }
  }

 private:
  int thres = 1;
  set<int> s;
};

int main() {
  SmallestInfiniteSet smallestInfiniteSet = SmallestInfiniteSet();
  smallestInfiniteSet.addBack(2);                  // 2 已经在集合中，所以不做任何变更。
  assert(smallestInfiniteSet.popSmallest() == 1);  // 返回 1 ，因为 1 是最小的整数，并将其从集合中移除。
  assert(smallestInfiniteSet.popSmallest() == 2);  // 返回 2 ，并将其从集合中移除。
  assert(smallestInfiniteSet.popSmallest() == 3);  // 返回 3 ，并将其从集合中移除。
  smallestInfiniteSet.addBack(1);                  // 将 1 添加到该集合中。
  assert(smallestInfiniteSet.popSmallest() == 1);  // 返回 1 ，因为 1 在上一步中被添加到集合中，
                                                   // 且 1 是最小的整数，并将其从集合中移除。
  assert(smallestInfiniteSet.popSmallest() == 4);  // 返回 4 ，并将其从集合中移除。
  assert(smallestInfiniteSet.popSmallest() == 5);  // 返回 5 ，并将其从集合中移除。
}