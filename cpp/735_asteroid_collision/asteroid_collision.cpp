/*
 * @Date: 2022-07-13
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-13
 * @FilePath: /algorithm/735_asteroid_collision/asteroid_collision.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> asteroidCollision(vector<int>& asteroids) {
    vector<int> st;
    for (auto aster : asteroids) {
      bool alive = true;
      while (alive && aster < 0 && !st.empty() && st.back() > 0) {
        alive = st.back() < -aster;  // aster 是否存在
        if (st.back() <= -aster) {   // 栈顶行星爆炸
          st.pop_back();
        }
      }
      if (alive) {
        st.push_back(aster);
      }
    }
    return st;
  }
};

int main() {
  {
    vector<int> asteroids{5, 10, -5};
    vector<int> ans{5, 10};
    assert(Solution().asteroidCollision(asteroids) == ans);
  }

  {
    vector<int> asteroids{8, -8};
    vector<int> ans{};
    assert(Solution().asteroidCollision(asteroids) == ans);
  }

  {
    vector<int> asteroids{10, 2, -5};
    vector<int> ans{10};
    assert(Solution().asteroidCollision(asteroids) == ans);
  }
}