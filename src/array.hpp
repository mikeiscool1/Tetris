#pragma once
#include <array>

template <typename T, size_t width, size_t height>
class array2d : public std::array<T, width * height> {
public:
  inline constexpr T* operator[](size_t row) {
    return this->data() + row * width;
  }

  inline constexpr const T* operator[](size_t row) const {
    return this->data() + row * width;
  }

  inline constexpr T& flat_index(size_t index) {
    return *(this->data() + index);
  }

  inline constexpr const T& flat_index(size_t index) const {
    return *(this->data() + index);
  }
};