use std::collections::HashMap;
use wayland_client::backend::ObjectId;

use wayland_client::protocol::wl_registry;
use wayland_client::{event_created_child, Connection, Dispatch, EventQueue, Proxy, QueueHandle};
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_v1::{
    Event as ConfigurationEvent, ZwlrOutputConfigurationV1,
};
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_head_v1::{
    self, Event as HeadEvent, ZwlrOutputHeadV1,
};
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_manager_v1::{
    self, Event as ManagerEvent, ZwlrOutputManagerV1,
};
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_mode_v1::ZwlrOutputModeV1;

#[derive(Debug)]
pub struct HeadInfo {
    pub head: ZwlrOutputHeadV1,
    pub name: Option<String>,
    pub description: Option<String>,
    pub serial: Option<String>,
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
}

impl HeadInfo {
    pub fn new(head: ZwlrOutputHeadV1) -> Self {
        HeadInfo {
            description: None,
            name: None,
            serial: None,
            head,
        }
    }
}

static OUTPUT_MANAGER_INTERFACE_NAME: &str = "zwlr_output_manager_v1";

/**
 * This method subscribes to the Global events. The global events advertise the capabilities of the system.
 * Once we encounter event for interface of type "output_manager" we bind to it
**/
impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        _state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<AppData>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version: _,
        } = event
        {
            if interface == OUTPUT_MANAGER_INTERFACE_NAME {
                registry.bind::<ZwlrOutputManagerV1, _, _>(name, 4, qh, ());
            }
        }
    }
}

impl Dispatch<ZwlrOutputConfigurationV1, ()> for AppData {
    fn event(
        state: &mut Self,
        _configuration: &ZwlrOutputConfigurationV1,
        event: ConfigurationEvent,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        match event {
            ConfigurationEvent::Succeeded => state.config_result = Some(ConfigResult::Succeeded),
            ConfigurationEvent::Failed => state.config_result = Some(ConfigResult::Failed),
            ConfigurationEvent::Cancelled => state.config_result = Some(ConfigResult::Cancelled),
            _ => {}
        }
    }
}

impl Dispatch<ZwlrOutputManagerV1, ()> for AppData {
    fn event(
        state: &mut Self,
        manager: &ZwlrOutputManagerV1,
        event: ManagerEvent,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        state.manager = Some(manager.clone());
        match event {
            ManagerEvent::Head { head } => {
                state.heads.insert(head.id(), HeadInfo::new(head.clone()));
            }
            ManagerEvent::Done { serial } => {
                state.initial_done = true;
                state.config_serial = Some(serial)
            }
            _ => {}
        }
    }

    event_created_child!(AppData, ZwlrOutputManagerV1, [
        zwlr_output_manager_v1::EVT_HEAD_OPCODE => (ZwlrOutputHeadV1, ()),
    ]);
}

impl Dispatch<ZwlrOutputHeadV1, ()> for AppData {
    fn event(
        state: &mut Self,
        head: &ZwlrOutputHeadV1,
        event: HeadEvent,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        let current_head = state.heads.get_mut(&head.id()).unwrap();

        match event {
            HeadEvent::Name { name } => current_head.name = Some(name),
            HeadEvent::SerialNumber { serial_number } => current_head.serial = Some(serial_number),
            HeadEvent::Description { description } => current_head.description = Some(description),
            _ => {}
        }
    }

    event_created_child!(AppData, ZwlrOutputHeadV1, [
        zwlr_output_head_v1::EVT_MODE_OPCODE => (ZwlrOutputModeV1, ()),
    ]);
}

impl Dispatch<ZwlrOutputModeV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _mode: &ZwlrOutputModeV1,
        _event: <ZwlrOutputModeV1 as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
    }
}
