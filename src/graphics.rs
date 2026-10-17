//! Windows draws through Direct3D 12, including WARP when available.

use eframe::egui_wgpu::{WgpuConfiguration, WgpuSetup, WgpuSetupCreateNew};
use eframe::wgpu;

pub fn configuration(software: bool) -> WgpuConfiguration {
    let mut setup = WgpuSetupCreateNew::without_display_handle();
    // Override WGPU_BACKEND as well: even a stale environment variable must
    // not reintroduce an OpenGL requirement into the Windows build.
    setup.instance_descriptor.backends = wgpu::Backends::DX12;
    if software {
        setup.native_adapter_selector = Some(std::sync::Arc::new(|adapters, surface| {
            adapters
                .iter()
                .find(|adapter| {
                    adapter.get_info().device_type == wgpu::DeviceType::Cpu
                        && surface.is_none_or(|surface| adapter.is_surface_supported(surface))
                })
                .cloned()
                .ok_or_else(|| "No surface-compatible software adapter is available".to_owned())
        }));
    }
    WgpuConfiguration {
        wgpu_setup: WgpuSetup::CreateNew(setup),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_never_enables_opengl() {
        for software in [false, true] {
            let WgpuSetup::CreateNew(setup) = configuration(software).wgpu_setup else {
                panic!("a new graphics instance is required");
            };
            assert!(
                !setup
                    .instance_descriptor
                    .backends
                    .contains(wgpu::Backends::GL)
            );
            assert_eq!(setup.instance_descriptor.backends, wgpu::Backends::DX12);
            assert_eq!(setup.native_adapter_selector.is_some(), software);
        }
    }

    #[test]
    fn missing_software_adapter_is_an_error_instead_of_a_gpu_fallback() {
        let WgpuSetup::CreateNew(setup) = configuration(true).wgpu_setup else {
            panic!("a new graphics instance is required");
        };
        let select = setup.native_adapter_selector.expect("software selector");
        assert!(select(&[], None).is_err());
    }
}
