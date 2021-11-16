#include "frpch.hpp"

#include "Core/LayerStack.hpp"

namespace Frameio {

LayerStack::LayerStack() {}

LayerStack::~LayerStack() {
  for (Layer *layer : m_Layers)
    delete layer;
}

void LayerStack::PushLayer(Layer *layer) {
  m_Layers.emplace(m_Layers.begin() + m_layerInsertIndex, layer);
  m_layerInsertIndex++;
  layer->OnAttach();
}

void LayerStack::PushOverlay(Layer *overlay) {
  m_Layers.emplace_back(overlay);
  overlay->OnAttach();
}

void LayerStack::PopLayer(Layer *layer) {
  auto it = std::find(m_Layers.begin(), m_Layers.end(), layer);
  if (it != m_Layers.end()) {
    m_Layers.erase(it);
    m_layerInsertIndex--;
    layer->OnDetach();
  };
}

void LayerStack::PopOverlay(Layer *overlay) {
  auto it = std::find(m_Layers.begin(), m_Layers.end(), overlay);
  if (it != m_Layers.end()) {
    m_Layers.erase(it);
    overlay->OnDetach();
  };
}

} // namespace Frameio