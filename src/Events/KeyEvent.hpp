#pragma once

#include "frpch.hpp"

#include "Event.hpp"

namespace Frameio {

class KeyEvent : public Event {
public:
  inline int GetKeyCode() const { return m_KeyCode; }

  EVENT_CLASS_CATEGORY(EventCategoryKeyboard | EventCategoryInput)

protected:
  KeyEvent(int keycode) : m_KeyCode(keycode) {}

  int m_KeyCode;
};

class KeyDownEvent : public KeyEvent {
public:
  KeyDownEvent(int keycode, int repeatCount)
      : KeyEvent(keycode), m_RepeatCount(repeatCount) {}

  inline int GetRepeatCount() const { return m_RepeatCount; }

  std::string ToString() const override {
    std::stringstream ss;
    ss << "KeyDownEvent: " << m_KeyCode << " {" << m_RepeatCount << " repeats}";
    return ss.str();
  }

  EVENT_CLASS_TYPE(KeyDown)

private:
  int m_RepeatCount;
};

class KeyUpEvent : public KeyEvent {
public:
  KeyUpEvent(int keycode) : KeyEvent(keycode) {}

  std::string ToString() const override {
    std::stringstream ss;
    ss << "KeyUpEvent: " << m_KeyCode;
    return ss.str();
  }

  EVENT_CLASS_TYPE(KeyUp)
};

class KeyTypedEvent : public KeyEvent {
public:
  KeyTypedEvent(int keycode) : KeyEvent(keycode) {}

  std::string ToString() const override {
    std::stringstream ss;
    ss << "KeyTypedEvent: " << m_KeyCode;
    return ss.str();
  }

  EVENT_CLASS_TYPE(KeyTyped)
};

} // namespace Frameio