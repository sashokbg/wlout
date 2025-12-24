use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use wayland_client::backend::ObjectId;
use wayland_client::protocol::wl_output::Transform;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_head_v1::{
    AdaptiveSyncState, ZwlrOutputHeadV1,
};
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_manager_v1::ZwlrOutputManagerV1;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_mode_v1::ZwlrOutputModeV1;

#[derive(Clone, Debug)]
pub struct HeadModeInput {
    pub width: i32,
    pub height: i32,
    pub rate: i32,
}

impl PartialEq<Self> for HeadModeInput {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.rate == other.rate
    }
}

impl Display for HeadModeInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}@{}", self.width, self.height, self.rate)
    }
}

#[derive(Debug, Clone)]
pub struct HeadInfo {
    pub head: ZwlrOutputHeadV1,
    pub name: Option<String>,
    pub description: Option<String>,
    pub serial: Option<String>,
    pub modes: HashMap<ObjectId, HeadMode>,
    pub position_x: Option<i32>,
    pub position_y: Option<i32>,
    pub model: Option<String>,
    pub make: Option<String>,
    pub physical_width: Option<i32>,
    pub physical_height: Option<i32>,
    pub transform: Option<Transform>,
    pub scale: Option<f64>,
    pub adaptive_sync: Option<AdaptiveSyncState>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Hash)]
pub struct HeadMode {
    pub mode: Option<ZwlrOutputModeV1>,
    pub height: i32,
    pub width: i32,
    pub rate: i32,
    pub is_preferred: bool,
    pub is_current: bool,
}

impl Eq for HeadMode {}

impl PartialEq for HeadMode {
    fn eq(&self, other: &Self) -> bool {
        self.rate == other.rate && self.height == other.height && self.width == other.width
    }
}

impl HeadMode {
    fn has_same_dimensions(&self, other: &HeadMode) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl Display for HeadMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}@{}", self.width, self.height, self.rate)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigResult {
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Debug)]
pub struct AppData {
    pub initial_done: bool,
    pub heads: HashMap<ObjectId, HeadInfo>,
    pub manager: Option<ZwlrOutputManagerV1>,
    pub config_result: Option<ConfigResult>,
    pub config_serial: Option<u32>,
    pub output_manager_found: bool
}

impl AppData {
    pub(crate) fn get_head(&self, name: &str) -> HeadInfo {
        self.heads
            .values()
            .find(|head_info| head_info.name.as_deref() == Some(name))
            .expect(&*format!("Display \"{}\" not found", name))
            .clone()
    }

    #[allow(dead_code)]
    pub(crate) fn try_get_head(&self, name: &str) -> Option<&HeadInfo> {
        self.heads
            .values()
            .find(|head_info| head_info.name.as_deref() == Some(name))
            .clone()
    }
}

impl HeadInfo {
    pub fn new(head: ZwlrOutputHeadV1) -> Self {
        HeadInfo {
            description: None,
            name: None,
            serial: None,
            head,
            modes: HashMap::new(),
            position_x: None,
            position_y: None,
            adaptive_sync: None,
            transform: None,
            model: None,
            scale: None,
            physical_height: None,
            physical_width: None,
            make: None,
            enabled: false,
        }
    }

    pub fn get_current_mode(&self) -> Option<&HeadMode> {
        self.modes.values().find(|m| m.is_current)
    }

    pub fn find_mode(&self, width: i32, height: i32, rate: i32) -> Option<&HeadMode> {
        self.modes
            .values()
            .find(|m| m.width == width && m.height == height && m.rate == rate)
    }
}

pub fn get_common_modes(
    head_info: &HeadInfo,
    other_info: &HeadInfo,
) -> (Vec<HeadMode>, Vec<HeadMode>) {
    let mut result1: HashSet<HeadMode> = HashSet::new();
    let mut result2: HashSet<HeadMode> = HashSet::new();

    for el in head_info.modes.values() {
        let corresponding_elements = other_info
            .modes
            .values()
            .filter(|m| m.has_same_dimensions(&el))
            .map(|el| el.clone())
            .collect::<Vec<HeadMode>>();

        if !corresponding_elements.is_empty() {
            result1.insert(el.clone());
        }

        result2.extend(corresponding_elements);
    }

    (result1.into_iter().collect(), result2.into_iter().collect())
}

fn compare_modes(mode: &HeadMode, other: &HeadMode) -> Ordering {
    other
        .height
        .cmp(&mode.height)
        .then(other.width.cmp(&mode.width))
        .then(other.rate.cmp(&mode.rate))
}

pub fn get_best_display_modes(
    mut modes1: Vec<HeadMode>,
    mut modes2: Vec<HeadMode>,
) -> (HeadMode, HeadMode) {
    modes1.sort_by(compare_modes);
    let max_1 = modes1.first().unwrap().to_owned();
    modes2.sort_by(compare_modes);
    let max_2 = modes2.first().unwrap().to_owned();

    (max_1, max_2)
}
