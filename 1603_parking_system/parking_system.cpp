/*
 * @Date: 2021-03-19 08:21:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-19 08:24:35
 */

#include <cassert>

class ParkingSystem {
 public:
  ParkingSystem(int big, int medium, int small) : park_{0, 0, 0} {
    park_[0] = big;
    park_[1] = medium;
    park_[2] = small;
  }

  bool addCar(int carType) {
    if (park_[carType - 1] == 0)
      return false;
    else
      --park_[carType - 1];
    return true;
  }

 private:
  // vector<int> park_;
  int park_[3];
};

int main() {
  ParkingSystem* obj = new ParkingSystem(1, 1, 0);
  assert(obj->addCar(1));
  assert(obj->addCar(2));
  assert(!obj->addCar(3));
  assert(!obj->addCar(1));
  return 0;
}