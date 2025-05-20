use super::{ImageFit, ImageRendering, Item, ItemConsts, ItemRc, RenderingResult};
use crate::graphics::Image;
use crate::input::{FocusEvent, FocusEventResult, InputEventFilterResult, InputEventResult, KeyEvent, KeyEventResult, MouseEvent};
use crate::item_rendering::{CachedRenderingData, RenderImage};
use crate::layout::{LayoutInfo, Orientation};
use crate::lengths::{LogicalLength, LogicalRect, LogicalSize};
#[cfg(feature = "rtti")]
use crate::rtti::*;
use crate::window::WindowAdapter;
use crate::{Brush, Coord, Property, SharedString};
use alloc::rc::Rc;
use const_field_offset::FieldOffsets;
use core::cell::RefCell;
use core::pin::Pin;
use i_slint_core_macros::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct LottieFile {
    assets: Option<Vec<Asset>>, 
}

#[derive(Deserialize)]
struct Asset {
    p: Option<SharedString>,
}

#[repr(C)]
#[derive(FieldOffsets, SlintElement)]
#[pin]
pub struct LottieAnimation {
    pub source: Property<SharedString>,
    pub progress: Property<f32>,
    pub width: Property<LogicalLength>,
    pub height: Property<LogicalLength>,
    pub image_fit: Property<ImageFit>,
    pub image_rendering: Property<ImageRendering>,
    pub colorize: Property<Brush>,
    pub cached_rendering_data: CachedRenderingData,

    frames: RefCell<Vec<Image>>,
}

impl Default for LottieAnimation {
    fn default() -> Self {
        Self {
            source: Property::new(SharedString::default()),
            progress: Property::new(0.),
            width: Property::new(LogicalLength::new(0.)),
            height: Property::new(LogicalLength::new(0.)),
            image_fit: Property::new(Default::default()),
            image_rendering: Property::new(Default::default()),
            colorize: Property::new(Default::default()),
            cached_rendering_data: Default::default(),
            frames: RefCell::new(Vec::new()),
        }
    }
}

impl LottieAnimation {
    fn load(&self) {
        let path = self.source.get();
        if path.is_empty() {
            return;
        }
        if let Ok(data) = std::fs::read(path.as_str()) {
            if let Ok(lottie) = serde_json::from_slice::<LottieFile>(&data) {
                if let Some(assets) = lottie.assets {
                    let mut frames = self.frames.borrow_mut();
                    frames.clear();
                    for asset in assets.into_iter().flatten() {
                        if let Some(p) = asset.p {
                            if let Ok(img) = Image::load_from_path(std::path::Path::new(p.as_str())) {
                                frames.push(img);
                            }
                        }
                    }
                }
            }
        }
    }

    fn current_image(&self) -> Option<Image> {
        let frames = self.frames.borrow();
        if frames.is_empty() {
            return None;
        }
        let idx = ((self.progress.get().clamp(0.,1.) * frames.len() as f32) as usize).min(frames.len()-1);
        Some(frames[idx].clone())
    }
}

impl Item for LottieAnimation {
    fn init(self: Pin<&Self>, _self_rc: &ItemRc) {
        self.load();
    }

    fn layout_info(
        self: Pin<&Self>,
        orientation: Orientation,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
    ) -> LayoutInfo {
        LayoutInfo {
            preferred: match orientation {
                _ if self.width().get() == 0. || self.height().get() == 0. => 0 as Coord,
                Orientation::Horizontal => self.width().get(),
                Orientation::Vertical => self.height().get(),
            },
            ..Default::default()
        }
    }

    fn input_event_filter_before_children(
        self: Pin<&Self>,
        _: MouseEvent,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
    ) -> InputEventFilterResult {
        InputEventFilterResult::ForwardAndIgnore
    }

    fn input_event(
        self: Pin<&Self>,
        _: MouseEvent,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
    ) -> InputEventResult {
        InputEventResult::EventIgnored
    }

    fn key_event(
        self: Pin<&Self>,
        _: &KeyEvent,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
    ) -> KeyEventResult {
        KeyEventResult::EventIgnored
    }

    fn focus_event(
        self: Pin<&Self>,
        _: &FocusEvent,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
    ) -> FocusEventResult {
        FocusEventResult::FocusIgnored
    }

    fn render(
        self: Pin<&Self>,
        backend: &mut &mut dyn crate::item_rendering::ItemRenderer,
        _self_rc: &ItemRc,
        _size: LogicalSize,
    ) -> RenderingResult {
        if let Some(img) = self.current_image() {
            backend.draw_image_direct(img);
        }
        RenderingResult::ContinueRenderingChildren
    }

    fn bounding_rect(
        self: core::pin::Pin<&Self>,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
        geometry: LogicalRect,
    ) -> LogicalRect {
        geometry
    }

    fn clips_children(self: core::pin::Pin<&Self>) -> bool {
        false
    }
}

impl RenderImage for LottieAnimation {
    fn target_size(self: Pin<&Self>) -> LogicalSize {
        LogicalSize::from_lengths(self.width(), self.height())
    }

    fn source(self: Pin<&Self>) -> Image {
        self.current_image().unwrap_or_default()
    }

    fn source_clip(self: Pin<&Self>) -> Option<crate::graphics::IntRect> { None }

    fn image_fit(self: Pin<&Self>) -> ImageFit { self.image_fit() }

    fn rendering(self: Pin<&Self>) -> ImageRendering { self.image_rendering() }

    fn colorize(self: Pin<&Self>) -> Brush { self.colorize() }

    fn alignment(self: Pin<&Self>) -> (super::ImageHorizontalAlignment, super::ImageVerticalAlignment) {
        Default::default()
    }

    fn tiling(self: Pin<&Self>) -> (super::ImageTiling, super::ImageTiling) {
        Default::default()
    }
}

impl ItemConsts for LottieAnimation {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<LottieAnimation, CachedRenderingData> =
        LottieAnimation::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

