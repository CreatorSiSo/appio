#pragma once

#include "frpch.hpp"

namespace Frameio {

class Shader {
public:
  Shader(const std::string vertexShaderSrc,
         const std::string fragmentShaderSrc);
  ~Shader();

  void Bind() const;
  void Unbind() const;

private:
  uint32_t m_RendererID;
};

} // namespace Frameio