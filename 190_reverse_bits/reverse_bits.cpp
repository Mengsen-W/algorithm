/*
 * @Date: 2021-03-29 08:34:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-29 08:45:13
 */

#include <cassert>
#include <cstdint>

uint32_t reverse_bits(uint32_t n) {
  uint32_t result = 0;
  for (int i = 0; i < 32; ++i) result = (result << 1) + (n >> i & 1);
  return result;
}

int main() {
  assert(reverse_bits(0b00000010100101000001111010011100) ==
         0b00111001011110000010100101000000);
  assert(reverse_bits(0b00000010100101000001111010011100) == 964176192);
  assert(reverse_bits(0b11111111111111111111111111111101) ==
         0b10111111111111111111111111111111);
  assert(reverse_bits(0b11111111111111111111111111111101) == 3221225471);
}
