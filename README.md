Doing sotrh's Learn WGPU-rs tutorial https://sotrh.github.io/learn-wgpu/

## Questions for later

https://sotrh.github.io/learn-wgpu/beginner/tutorial2-swapchain/

Why is `winit::dpi::PhysicalSize<u32>` an int? What if i happen to have noninteger pixels per centimetre?

Why does the program crash, unable to obtain a graphics a `wgpu::Adapter`? oh wait NixOS and libraries https://github.com/gfx-rs/wgpu-rs/issues/332#issuecomment-655672840

..can i use this to get Julia to open a GLFW.jl window now? (currently it reports `ERROR: GLFWError (API_UNAVAILABLE): GLX: No GLXFBConfigs returned` presumably because a driver's not getting linked in?)

What did kvark mean here by "Given no *good* way to generate SPIR-V" https://github.com/gfx-rs/gfx/issues/2999 -- what's wrong with each of {[shaderc-rs](https://crates.io/crates/shaderc), [glslang](https://github.com/KhronosGroup/glslang), [glsl-to-spirv](https://crates.io/crates/glsl-to-spirv)}?


