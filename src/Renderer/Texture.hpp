#pragma once

#include "frpch.hpp"

namespace Frameio {

class Texture {
public:
  virtual ~Texture() = default;

  virtual void Bind(uint32_t slot = 0) const = 0;
  virtual uint32_t GetWidth() = 0;
  virtual uint32_t GetHeight() = 0;
};

class Texture2D : public Texture {
public:
  static Ref<Texture2D> Create(const std::string& filePath);
};

} // namespace Frameio