#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool asteroidsDestroyed(int mass, vector<int>& asteroids) {
    sort(asteroids.begin(), asteroids.end());  // 按照质量升序排序
    long long mass1 = mass;
    for (const int asteroid : asteroids) {
      // 按顺序遍历小行星，尝试摧毁并更新质量或者返回结果
      if (mass1 < asteroid) {
        return false;
      }
      mass1 += asteroid;
    }
    return true;  // 成功摧毁所有小行星
  }
};

int main() {
  vector<tuple<int, vector<int>, bool>> tests{
      {10, {3, 9, 19, 5, 21}, true},
      {5, {4, 9, 23, 4}, false},
  };

  for (auto& [mass, asteroids, expect] : tests) {
    assert(Solution().asteroidsDestroyed(mass, asteroids) == expect);
  }
}
