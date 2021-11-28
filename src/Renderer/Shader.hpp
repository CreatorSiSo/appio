#pragma once

#include "frpch.hpp"

#include <glm/glm.hpp>

namespace Frameio {

class Shader {
public:
  Shader(const std::string vertexShaderSrc, const std::string fragmentShaderSrc);
  ~Shader();

  void Bind() const;
  void Unbind() const;

  void UploadUniformFloat4(const std::string& name, const glm::vec4& value);
  void UploadUniformMat4(const std::string& name, const glm::mat4& value);

private:
  uint32_t m_RendererID;
};

} // namespace Frameio
