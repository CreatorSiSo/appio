- [ ] Renderer::BeginScene(cameradata)

- [ ] Shader::UpdloadUniformMat4(mat4)
  - [ ] use this to make Renderer::BeginScene() actually upload the camera

- [ ] CameraGeneral:
  - [ ] use glm internal ortho and perspective mats for calculation at init time
    - [ ] calculate viewProjMat at init
    - [ ] use viewProjMat via a uniform in the vertex shader
  - [ ] Set/GetPos(vec3)
  - [ ] Set/GetProjMat4
  - [ ] Set/GetViewMat4
  - [ ] ReCalcViewMat()
    - [ ] apply calculation from video
    - [ ] invert calculated mat
  - [ ] holds:
    - [ ] projMat4
    - [ ] viewMat4
    - [ ] viewProjMat4
    - [ ] pos
    - [ ] rot

- [ ] OrthoCamera:
  - [ ] init(top, right, bottom, left)
  - [ ] Set/GetRot(float)

- [ ] PerspectiveCamera:
  - [ ] init(FOV, near, far)
  - [ ] Set/GetRot(vec3?/quat?/euler?)

- [x] abstract shader binding away into Renderer::Submit()

- [ ] Initialize member variables correctly!!! glm does not do that for you

- Think about loading shaders and passing them to the renderer:
  - crosscompiling?
  - what shader languages to support?
  - watching shader files
    - only recompile shaders that changed
    - how to handle caching?