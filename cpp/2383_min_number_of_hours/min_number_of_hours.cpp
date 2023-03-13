/*
 * @Date: 2023-03-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-13
 * @FilePath: /algorithm/cpp/2383_min_number_of_hours/min_number_of_hours.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minNumberOfHours(int initialEnergy, int initialExperience, vector<int>& energy, vector<int>& experience) {
    int sum = 0;
    for (int e : energy) {
      sum += e;
    }
    int trainingHours = initialEnergy > sum ? 0 : sum + 1 - initialEnergy;
    for (int e : experience) {
      if (initialExperience <= e) {
        trainingHours += 1 + (e - initialExperience);
        initialExperience = (e + 1) + e;  // ( e + 1 )当前经验 + e 击败所获取的经验
      } else {
        initialExperience += e;
      }
    }
    return trainingHours;
  }
};

int main() {
  {
    int initialEnergy = 5;
    int initialExperience = 3;
    vector<int> energy{1, 4, 3, 2};
    vector<int> experience{2, 6, 3, 1};
    int ans = 8;
    assert(Solution().minNumberOfHours(initialEnergy, initialExperience, energy, experience) == ans);
  }

  {
    int initialEnergy = 2;
    int initialExperience = 4;
    vector<int> energy{1};
    vector<int> experience{3};
    int ans = 0;
    assert(Solution().minNumberOfHours(initialEnergy, initialExperience, energy, experience) == ans);
  }
}
