use crate::sys::{self, Interface};
use std::sync::OnceLock;

/// Initialize COM as single-threaded.
pub unsafe fn init_com_single_threaded() {
    unsafe {
        sys::CoInitializeEx(
            None,
            sys::COINIT_APARTMENTTHREADED | sys::COINIT_DISABLE_OLE1DDE,
        )
        .unwrap();
    }
}

type InternalApis = (
    /* GetResizeDCompositionSynchronizationObject */
    unsafe extern "system" fn(sys::HWND, *mut sys::HANDLE),
    /* EnableResizeLayoutSynchronization */
    unsafe extern "system" fn(sys::HWND, sys::BOOL),
);

fn get_apis() -> &'static InternalApis {
    static APIS: OnceLock<InternalApis> = OnceLock::new();
    APIS.get_or_init(|| unsafe {
        let dll = sys::GetModuleHandleW(sys::w!("user32"))
            .or_else(|_| sys::LoadLibraryExW(sys::w!("user32"), None, sys::LOAD_LIBRARY_FLAGS(0)))
            .unwrap();
        let [p1, p2] = [2614, 2615]
            .map(|ordinal| sys::GetProcAddress(dll, sys::PCSTR(ordinal as *const u8)).unwrap());
        (std::mem::transmute(p1), std::mem::transmute(p2))
    })
}

/// When the window is marked as eligible for resize synchronization by calling
/// [`make_window_eligible_for_dcomp_resize_sync`], the OS will create a DComp
/// synchronization handle during `WM_SIZE`, which can be retrieved by calling this.
pub unsafe fn get_resize_dcomp_sync_object(hwnd: sys::HWND) -> sys::HANDLE {
    let mut handle: sys::HANDLE = Default::default();
    let (__synchronization_object, _) = get_apis();
    unsafe { __synchronization_object(hwnd, &mut handle) };
    handle
}

#[sys::interface("3CBCF1BF-2F76-4E9C-96AB-E84B37972554")]
unsafe trait IDesktopWindowXamlSourceNative: sys::IUnknown {
    unsafe fn attach_to_window(&self, parentwnd: sys::HWND) -> sys::HRESULT;
    unsafe fn window_handle(&self, hwnd: *mut sys::HWND) -> sys::HRESULT;
}
unsafe impl Send for IDesktopWindowXamlSourceNative {}
unsafe impl Sync for IDesktopWindowXamlSourceNative {}

/// Trick the OS into making the window eligible for resize synchronization.
/// This requires a manifest stating support for >=Win10 1903 to work!
pub unsafe fn make_window_eligible_for_dcomp_resize_sync(hwnd: sys::HWND) {
    static DWXS: OnceLock<IDesktopWindowXamlSourceNative> = OnceLock::new();
    let dwxs_native = DWXS.get_or_init(|| {
        let class_name = sys::h!("Windows.UI.Xaml.Hosting.DesktopWindowXamlSource");
        let inspectable = unsafe { sys::RoActivateInstance(class_name) }.unwrap();
        inspectable
            .cast::<IDesktopWindowXamlSourceNative>()
            .unwrap()
    });
    unsafe { dwxs_native.attach_to_window(hwnd) }.unwrap();

    let (_, __layout_synchronization) = get_apis();
    unsafe { __layout_synchronization(hwnd, sys::TRUE) };
}

/// Signal to the compositor that resize has been completed, so that it can finally
/// start composing a new frame for the window.
pub unsafe fn signal_dcomp_resize_completion() {
    sys::CoreWindowResizeManager::GetForCurrentView()
        .unwrap()
        .NotifyLayoutCompleted()
        .unwrap();
}

// could ai have written this for me
#[sys::interface("E01EB649-787E-4560-B398-0DE7A2065D8B")]
pub unsafe trait IDCompositionDesktopDevicePartner6: sys::IUnknown {
    unsafe fn __commit(&self);
    unsafe fn __wait_for_commit_completion(&self);
    unsafe fn __get_frame_statistics(&self);
    unsafe fn __create_visual(&self);
    unsafe fn __create_surface_factory(&self);
    unsafe fn __create_surface(&self);
    unsafe fn __create_virtual_surface(&self);
    unsafe fn __create_translate_transform(&self);
    unsafe fn __create_scale_transform(&self);
    unsafe fn __create_rotate_transform(&self);
    unsafe fn __create_skew_transform(&self);
    unsafe fn __create_matrix_transform(&self);
    unsafe fn __create_transform_group(&self);
    unsafe fn __create_translate_transform3_d(&self);
    unsafe fn __create_scale_transform3_d(&self);
    unsafe fn __create_rotate_transform3_d(&self);
    unsafe fn __create_matrix_transform3_d(&self);
    unsafe fn __create_transform3_dgroup(&self);
    unsafe fn __create_effect_group(&self);
    unsafe fn __create_rectangle_clip(&self);
    unsafe fn __create_animation(&self);
    unsafe fn __create_target_for_hwnd(&self);
    unsafe fn __create_surface_from_handle(&self);
    unsafe fn __create_surface_from_hwnd(&self);
    unsafe fn __create_shared_resource(&self);
    unsafe fn __open_shared_resource_handle(&self);
    unsafe fn __open_shared_resource_1(&self);
    unsafe fn __create_animation_trigger(&self);
    unsafe fn __create_gradient_surface(&self);
    unsafe fn __enable_white_pixel_optimization(&self);
    unsafe fn __offer_surface_resources(&self);
    unsafe fn __reclaim_surface_resources(&self);
    unsafe fn __disable_d2_dstate_preservation(&self);
    unsafe fn __flush(&self);
    unsafe fn __create_surface_from_visual_snapshot(&self);
    unsafe fn __create_interaction(&self);
    unsafe fn __create_touch_interaction_configuration(&self);
    unsafe fn __create_touchpad_interaction_configuration(&self);
    unsafe fn __create_pen_interaction_configuration(&self);
    unsafe fn __create_mouse_interaction_configuration(&self);
    unsafe fn __open_shared_resource_2(&self);
    unsafe fn __create_desktop_tree(&self);
    unsafe fn __get_current_batch_id(&self);
    unsafe fn __get_last_confirmed_batch_id(&self);
    unsafe fn __create_gaussian_blur_effect(&self);
    unsafe fn __create_manipulation_transform(&self);
    unsafe fn __create_brightness_effect(&self);
    unsafe fn __create_color_matrix_effect(&self);
    unsafe fn __create_shadow_effect(&self);
    unsafe fn __create_hue_rotation_effect(&self);
    unsafe fn __create_saturation_effect(&self);
    unsafe fn __create_turbulence_effect(&self);
    unsafe fn __create_linear_transfer_effect(&self);
    unsafe fn __create_table_transfer_effect(&self);
    unsafe fn __register_callback_thread(&self);
    unsafe fn __create_composite_effect(&self);
    unsafe fn __snap_animation_read_back_time(&self);
    unsafe fn __get_max_texture_size(&self);
    unsafe fn __create_blend_effect(&self);
    unsafe fn __create_arithmetic_composite_effect(&self);
    unsafe fn __create_affine_transform2_deffect(&self);
    unsafe fn __set_composition_primitive_group_renderer_enabled(&self);
    unsafe fn __create_render_target(&self);
    unsafe fn __create_ddisplay_render_target(&self);
    unsafe fn __create_cursor_visual(&self);
    unsafe fn __create_mousewheel_interaction_configuration(&self);
    pub unsafe fn synchronized_commit(&self, hobject: sys::HANDLE) -> sys::HRESULT;
    unsafe fn __create_visual_reference_controller(&self);
    unsafe fn __create_window_target(&self);
}
