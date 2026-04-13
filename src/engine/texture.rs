// FIXED: Removed the unused `use wgpu::util::DeviceExt;` warning!

pub fn create_texture_from_gltf(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    image_data: gltf::image::Data,
    label: &str,
    is_srgb: bool,
) -> wgpu::TextureView {
    let mut rgba_pixels = Vec::new();
    if image_data.format == gltf::image::Format::R8G8B8 {
        for chunk in image_data.pixels.chunks_exact(3) {
            rgba_pixels.extend_from_slice(chunk);
            rgba_pixels.push(255);
        }
    } else {
        rgba_pixels = image_data.pixels;
    }

    let texture_size = wgpu::Extent3d { width: image_data.width, height: image_data.height, depth_or_array_layers: 1 };
    let format = if is_srgb { wgpu::TextureFormat::Rgba8UnormSrgb } else { wgpu::TextureFormat::Rgba8Unorm };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_size, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
        format, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some(label), view_formats: &[],
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo { texture: &texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
        &rgba_pixels,
        wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4 * image_data.width), rows_per_image: Some(image_data.height) },
        texture_size,
    );
    
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}