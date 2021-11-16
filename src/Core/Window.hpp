#pragma once

#include "frpch.hpp"

#include "Events/Event.hpp"

namespace Frameio {

struct WindowProps {
  std::string Title;
  int Width;
  int Height;

  WindowProps(const std::string &title = "Texturia", unsigned int width = 1280,
              unsigned int height = 720)
      : Title(title), Width(width), Height(height) {}
};

class Window {
public:
  virtual ~Window() = default;

  virtual void OnUpdate() = 0;

  virtual unsigned int GetWidth() const = 0;
  virtual unsigned int GetHeight() const = 0;

  virtual void *GetNativeWindow() const = 0;

  using EventCallbackFunction = std::function<void(Event &)>;

  // Window attributes
  virtual void SetEventCallback(const EventCallbackFunction &callback) = 0;
  virtual void SetVSync(bool enabled) = 0;
  virtual bool IsVSync() const = 0;

  static Window *Create(const WindowProps &props = WindowProps());
};

} // namespace Frameio