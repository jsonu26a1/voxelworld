# Notes
Just putting some links for documentation and articles that are relevant.

- ["How to write a renderer for modern graphics APIs" by Jasper St. Pierre](https://blog.mecheye.net/2023/09/how-to-write-a-renderer-for-modern-apis/)
- ["The Missing Guide to Modern Graphics APIs – 1. Intro" by Jasper St. Pierre](https://blog.mecheye.net/2020/06/modern-graphics-apis-1-intro/)
- ["An Opinionated Post on Modern Rendering Abstraction Layers" by Alex Tardif](https://alextardif.com/RenderingAbstractionLayers.html)
- ["Robust pipeline cache serialization" by Arseny Kapoulkine](https://zeux.io/2019/07/17/serializing-pipeline-cache/)

- ["Vulkan-Tutorial.com"](https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Base_code)
- ["API without Secrets: Introduction to Vulkan" by Pawel Lapinski at Intel](https://www.intel.com/content/www/us/en/developer/articles/training/api-without-secrets-introduction-to-vulkan-part-1.html)

# Update 2023-12-04
I decided to use a rust wrapper around bgfx instead of trying to learn Vulkan. Currently, the
main.rs has a hello_winit example, that I adapted for newer version of winit. Next, I need to
work on getting a custom shader, rendering a 3d terrain like in the SimonDev videos, and
eventually getting a camera and input-movement system so I can explore the world.

Here are the new resources I'll be using now that I'm using bgfx-rs.

- ["3D World Generation" by SimonDev on YouTube](https://www.youtube.com/playlist?list=PLRL3Z3lpLmH3PNGZuDNf2WXnLTHpN9hXy)
- ["3D Game Shaders For Beginners" on GitHub](https://github.com/lettier/3d-game-shaders-for-beginners)

