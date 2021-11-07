#pragma once

#include "frpch.hpp"

#include "Events/Event.hpp"

namespace Framio {

class Input {
public:
  Input() {}
  ~Input() {}

  inline static bool IsKeyDown(int keycode) {
    return s_Instance->IsKeyDownImpl(keycode);
  }

  inline static bool IsMouseButtonDown(int button) {
    return s_Instance->IsMouseButtonDownImpl(button);
  }

  inline static std::pair<float, float> GetMousePos() {
    return s_Instance->GetMousePosImpl();
  }
  inline static float GetMouseX() { return s_Instance->GetMouseXImpl(); }
  inline static float GetMouseY() { return s_Instance->GetMouseYImpl(); }

protected:
  virtual bool IsKeyDownImpl(int keycode) = 0;
  virtual bool IsMouseButtonDownImpl(int button) = 0;
  virtual std::pair<float, float> GetMousePosImpl() = 0;
  virtual float GetMouseXImpl() = 0;
  virtual float GetMouseYImpl() = 0;

private:
  static Input *s_Instance;
};

} // namespace Framio
