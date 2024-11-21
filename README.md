# Rust Texture Mapping with wgpu

This project is an **educational exercise** to explore the fundamentals of **Rust** and the **wgpu** graphics library, focusing on **texture mapping** in GPU programming.

The goal of this project is to create a simple rendering application that:
- Loads and maps a texture onto a basic 2D plane.
- Implements rotation and transformation logic for the texture.
- Utilizes `wgpu` concepts (like buffers, pipelines, bind groups, and shaders).

By working on this project, the intention was to gain hands-on experience with:
- Writing and understanding **WGSL shaders**.
- Configuring **GPU resources** (e.g., textures, samplers, and bind groups).
- Creating and managing **render pipelines**.
- Experimenting with **matrix transformations** for basic object rotation.

---

## Project Structure

- **`src/lib.rs`**: Contains the main logic for initializing the `wgpu` instance, setting up the render pipeline, and implementing texture mapping with bind groups.
- **`src/main.rs`**: Entry point of the application. It initializes the window and event loop using `winit`.
- **`src/shader.wgsl`**: WGSL shader code used for vertex and fragment operations, including texture sampling.
- **`assets/scenary.png`**: The texture image used for mapping in the application.
- **`Cargo.toml`**: Rust's manifest file, specifying dependencies such as `wgpu`, `winit`, and `image`.
- **`LICENSE`**: License information for the project.

---

## Learning Objectives

1. Understand the basics of **GPU programming** using `wgpu` in Rust.
2. Learn how to load and bind textures with samplers and shaders.
3. Implement a simple transformation pipeline with rotation and scaling.
4. Explore the structure and requirements of a rendering application.

---

## Dependencies

The project uses the following crates:
- **`wgpu`**: For GPU abstraction and rendering.
- **`winit`**: For window and event handling.
- **`image`**: For loading texture data (e.g., PNG files).
- **`bytemuck`**: For safe casting of data types to raw byte slices.

Ensure you have a Rust environment installed. Use the following command to add the necessary crates:

```bash
cargo build
```

---

## Usage

To run the project, use the following:

   ```bash
   git clone https://github.com/evanwmart/wgpu-texture-mapping.git
   cd wgpu-texture-mapping
   cargo run
   ```

---

## References and Inspiration

- [wgpu Documentation](https://docs.rs/wgpu)
- [Learn wgpu](https://sotrh.github.io/learn-wgpu/)
- [Unsplash Image](https://unsplash.com/photos/waves-of-body-of-water-splashing-on-sand-mBQIfKlvowM): The source for the texture used in this project.

---

## License

This project is distributed under the MIT license. See the [LICENSE](LICENSE) file for more information.

To be honest, I'm just slapping a license on this 🤷‍♂️. Use it how you like, reference it if you feel nice enough.

---

Shoutout wgpu and Rust people. 🫡