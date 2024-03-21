/*
 * @Date: 2024-03-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-21
 * @FilePath: /algorithm/cpp/2671_FrequencyTracker/FrequencyTracker.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class FrequencyTracker {
 public:
  FrequencyTracker() : freq(N), freq_cnt(N) {}

  void add(int number) {
    --freq_cnt[freq[number]];
    ++freq[number];
    ++freq_cnt[freq[number]];
  }

  void deleteOne(int number) {
    if (freq[number] == 0) {
      return;
    }
    --freq_cnt[freq[number]];
    --freq[number];
    ++freq_cnt[freq[number]];
  }

  bool hasFrequency(int frequency) { return freq_cnt[frequency]; }

 private:
  static constexpr int N = 100001;
  vector<int> freq;
  vector<int> freq_cnt;
};

int main() {
  {
    FrequencyTracker* frequencyTracker = new FrequencyTracker();
    frequencyTracker->add(3);                           // 数据结构现在包含 [3]
    frequencyTracker->add(3);                           // 数据结构现在包含 [3, 3]
    assert(frequencyTracker->hasFrequency(2) == true);  // 返回 true ，因为 3 出现 2 次
  }

  {
    FrequencyTracker* frequencyTracker = new FrequencyTracker();
    frequencyTracker->add(1);                            // 数据结构现在包含 [1]
    frequencyTracker->deleteOne(1);                      // 数据结构现在为空 []
    assert(frequencyTracker->hasFrequency(1) == false);  // 返回 false ，因为数据结构为空
  }

  {
    FrequencyTracker* frequencyTracker = new FrequencyTracker();
    frequencyTracker->hasFrequency(2);                  // 返回 false ，因为数据结构为空
    frequencyTracker->add(3);                           // 数据结构现在包含 [3]
    assert(frequencyTracker->hasFrequency(1) == true);  // 返回 true ，因为 3 出现 1 次
  }
}