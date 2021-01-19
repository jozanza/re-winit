#![allow(non_snake_case, dead_code, unused_imports)]
use ocaml::{CamlError, Error, FromValue, Pointer, ToValue, Value};
use std::{
  convert::{TryFrom, TryInto},
  fmt::Debug,
  marker::Sized,
  time::{Duration, Instant},
};
use winit::{
  dpi::{PhysicalPosition, PhysicalSize},
  event::{
    AxisId, DeviceEvent, DeviceId, ElementState, Event, Force, KeyboardInput, ModifiersState,
    MouseButton, MouseScrollDelta, StartCause, Touch, TouchPhase, VirtualKeyCode, WindowEvent,
  },
  event_loop::{ControlFlow, EventLoop},
  window::{Theme, Window, WindowBuilder, WindowId},
};

fn isize_from_instant(i: Instant) -> isize {
  let now = Instant::now();
  // Calculate a millisecond delta from now
  if now > i {
    now.duration_since(i).as_millis() as i64 as isize
  } else {
    -(i.duration_since(now).as_millis() as i64 as isize)
  }
}

fn now_plus_millis(n: u64) -> Instant {
  Instant::now()
    .checked_add(Duration::from_millis(n))
    .unwrap()
}

// Exception
//------------------------------------------------------------------------------

fn Val_Exn(err: String) -> Error {
  Error::Caml(CamlError::Exception(Value::string(format!("{}", err))))
}

// window::WindowId
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue)]
struct WindowId_(Pointer<WindowId>);

impl Debug for WindowId_ {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self.0.as_ref())
  }
}

#[allow(non_snake_case)]
unsafe extern "C" fn finalize_WindowId(v: Value) {
  let ptr: Pointer<WindowId> = Pointer::from_value(v);
  ptr.drop_in_place()
}

fn Val_WindowId(val: WindowId) -> WindowId_ {
  let ptr = Pointer::alloc_final(val, Some(finalize_WindowId), None);
  WindowId_(ptr)
}

fn WindowId_val<'a>(val: &'a WindowId_) -> &'a WindowId {
  val.0.as_ref()
}

// window::Window
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue)]
struct Window_(Pointer<Window>);

impl Debug for Window_ {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self.0.as_ref())
  }
}

#[allow(non_snake_case)]
unsafe extern "C" fn finalize_Window(v: Value) {
  let ptr: Pointer<Window> = Pointer::from_value(v);
  ptr.drop_in_place()
}

fn Val_Window(val: Window) -> Window_ {
  let ptr = Pointer::alloc_final(val, Some(finalize_Window), None);
  Window_(ptr)
}

fn Window_val<'a>(val: &'a Window_) -> &'a Window {
  val.0.as_ref()
}

// window::Window::new
#[ocaml::func]
pub fn re_winit_window_Window_new(event_loop: EventLoop_) -> Result<Window_, Error> {
  if let Some(event_loop) = event_loop.0.as_ref() {
    return match Window::new(event_loop) {
      Ok(window) => Ok(Val_Window(window)),
      Err(err) => Err(Val_Exn(err.to_string())),
    };
  }
  Err(Val_Exn("No EventLoop".to_string()))
}

// window::WindowBuilder
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue)]
struct WindowBuilder_(Pointer<WindowBuilder>);

impl Debug for WindowBuilder_ {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self.0.as_ref())
  }
}

#[allow(non_snake_case)]
unsafe extern "C" fn finalize_WindowBuilder(v: Value) {
  let ptr: Pointer<WindowBuilder> = Pointer::from_value(v);
  ptr.drop_in_place()
}

fn Val_WindowBuilder(val: WindowBuilder) -> WindowBuilder_ {
  let ptr = Pointer::alloc_final(val, Some(finalize_WindowBuilder), None);
  WindowBuilder_(ptr)
}

fn WindowBuilder_val<'a>(val: &'a WindowBuilder_) -> &'a WindowBuilder {
  val.0.as_ref()
}

// window::WindowBuilder::new
#[ocaml::func]
pub fn re_winit_event_loop_WindowBuilder_new() -> WindowBuilder_ {
  Val_WindowBuilder(WindowBuilder::new())
}

// window::WindowBuilder::build
#[ocaml::func]
pub fn re_winit_event_loop_WindowBuilder_build(
  builder: WindowBuilder_,
  event_loop: EventLoop_,
) -> Result<Window_, Error> {
  let builder = builder.0.as_ref().clone();
  if let Some(event_loop) = event_loop.0.as_ref() {
    return match builder.build(event_loop) {
      Ok(window) => Ok(Val_Window(window)),
      Err(err) => Err(Val_Exn(err.to_string())),
    };
  }
  Err(Val_Exn("No EventLoop".to_string()))
}

// window::Theme
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum Theme_ {
  Light,
  Dark,
}

fn Val_Theme(value: Theme) -> Theme_ {
  use Theme::*;
  match value {
    Light => Theme_::Light,
    Dark => Theme_::Dark,
  }
}

fn Theme_val(value: Value) -> Theme {
  use Theme::*;
  match Theme_::from_value(value) {
    Theme_::Light => Light,
    Theme_::Dark => Dark,
  }
}

// event::AxisId
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue)]
struct AxisId_(Pointer<AxisId>);

impl Debug for AxisId_ {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self.0.as_ref())
  }
}

#[allow(non_snake_case)]
unsafe extern "C" fn finalize_AxisId(v: Value) {
  let ptr: Pointer<AxisId> = Pointer::from_value(v);
  ptr.drop_in_place()
}

fn Val_AxisId(val: AxisId) -> AxisId_ {
  let ptr = Pointer::alloc_final(val, Some(finalize_AxisId), None);
  AxisId_(ptr)
}

fn AxisId_val<'a>(val: &'a AxisId_) -> &'a AxisId {
  val.0.as_ref()
}

// event::DeviceId
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue)]
struct DeviceId_(Pointer<DeviceId>);

impl Debug for DeviceId_ {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self.0.as_ref())
  }
}

#[allow(non_snake_case)]
unsafe extern "C" fn finalize_DeviceId(v: Value) {
  let ptr: Pointer<DeviceId> = Pointer::from_value(v);
  ptr.drop_in_place()
}

fn Val_DeviceId(val: DeviceId) -> DeviceId_ {
  let ptr = Pointer::alloc_final(val, Some(finalize_DeviceId), None);
  DeviceId_(ptr)
}

fn DeviceId_val<'a>(val: &'a DeviceId_) -> &'a DeviceId {
  val.0.as_ref()
}

// event::VirtualKeyCode
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum VirtualKeyCode_ {
  Key1,
  Key2,
  Key3,
  Key4,
  Key5,
  Key6,
  Key7,
  Key8,
  Key9,
  Key0,
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  Escape,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  F13,
  F14,
  F15,
  F16,
  F17,
  F18,
  F19,
  F20,
  F21,
  F22,
  F23,
  F24,
  Snapshot,
  Scroll,
  Pause,
  Insert,
  Home,
  Delete,
  End,
  PageDown,
  PageUp,
  Left,
  Up,
  Right,
  Down,
  Back,
  Return,
  Space,
  Compose,
  Caret,
  Numlock,
  Numpad0,
  Numpad1,
  Numpad2,
  Numpad3,
  Numpad4,
  Numpad5,
  Numpad6,
  Numpad7,
  Numpad8,
  Numpad9,
  NumpadAdd,
  NumpadDivide,
  NumpadDecimal,
  NumpadComma,
  NumpadEnter,
  NumpadEquals,
  NumpadMultiply,
  NumpadSubtract,
  AbntC1,
  AbntC2,
  Apostrophe,
  Apps,
  Asterisk,
  At,
  Ax,
  Backslash,
  Calculator,
  Capital,
  Colon,
  Comma,
  Convert,
  Equals,
  Grave,
  Kana,
  Kanji,
  LAlt,
  LBracket,
  LControl,
  LShift,
  LWin,
  Mail,
  MediaSelect,
  MediaStop,
  Minus,
  Mute,
  MyComputer,
  NavigateForward,
  NavigateBackward,
  NextTrack,
  NoConvert,
  OEM102,
  Period,
  PlayPause,
  Plus,
  Power,
  PrevTrack,
  RAlt,
  RBracket,
  RControl,
  RShift,
  RWin,
  Semicolon,
  Slash,
  Sleep,
  Stop,
  Sysrq,
  Tab,
  Underline,
  Unlabeled,
  VolumeDown,
  VolumeUp,
  Wake,
  WebBack,
  WebFavorites,
  WebForward,
  WebHome,
  WebRefresh,
  WebSearch,
  WebStop,
  Yen,
  Copy,
  Paste,
  Cut,
}

fn Val_VirtualKeyCode(value: VirtualKeyCode) -> VirtualKeyCode_ {
  use VirtualKeyCode::*;
  match value {
    Key1 => VirtualKeyCode_::Key1,
    Key2 => VirtualKeyCode_::Key2,
    Key3 => VirtualKeyCode_::Key3,
    Key4 => VirtualKeyCode_::Key4,
    Key5 => VirtualKeyCode_::Key5,
    Key6 => VirtualKeyCode_::Key6,
    Key7 => VirtualKeyCode_::Key7,
    Key8 => VirtualKeyCode_::Key8,
    Key9 => VirtualKeyCode_::Key9,
    Key0 => VirtualKeyCode_::Key0,
    A => VirtualKeyCode_::A,
    B => VirtualKeyCode_::B,
    C => VirtualKeyCode_::C,
    D => VirtualKeyCode_::D,
    E => VirtualKeyCode_::E,
    F => VirtualKeyCode_::F,
    G => VirtualKeyCode_::G,
    H => VirtualKeyCode_::H,
    I => VirtualKeyCode_::I,
    J => VirtualKeyCode_::J,
    K => VirtualKeyCode_::K,
    L => VirtualKeyCode_::L,
    M => VirtualKeyCode_::M,
    N => VirtualKeyCode_::N,
    O => VirtualKeyCode_::O,
    P => VirtualKeyCode_::P,
    Q => VirtualKeyCode_::Q,
    R => VirtualKeyCode_::R,
    S => VirtualKeyCode_::S,
    T => VirtualKeyCode_::T,
    U => VirtualKeyCode_::U,
    V => VirtualKeyCode_::V,
    W => VirtualKeyCode_::W,
    X => VirtualKeyCode_::X,
    Y => VirtualKeyCode_::Y,
    Z => VirtualKeyCode_::Z,
    Escape => VirtualKeyCode_::Escape,
    F1 => VirtualKeyCode_::F1,
    F2 => VirtualKeyCode_::F2,
    F3 => VirtualKeyCode_::F3,
    F4 => VirtualKeyCode_::F4,
    F5 => VirtualKeyCode_::F5,
    F6 => VirtualKeyCode_::F6,
    F7 => VirtualKeyCode_::F7,
    F8 => VirtualKeyCode_::F8,
    F9 => VirtualKeyCode_::F9,
    F10 => VirtualKeyCode_::F10,
    F11 => VirtualKeyCode_::F11,
    F12 => VirtualKeyCode_::F12,
    F13 => VirtualKeyCode_::F13,
    F14 => VirtualKeyCode_::F14,
    F15 => VirtualKeyCode_::F15,
    F16 => VirtualKeyCode_::F16,
    F17 => VirtualKeyCode_::F17,
    F18 => VirtualKeyCode_::F18,
    F19 => VirtualKeyCode_::F19,
    F20 => VirtualKeyCode_::F20,
    F21 => VirtualKeyCode_::F21,
    F22 => VirtualKeyCode_::F22,
    F23 => VirtualKeyCode_::F23,
    F24 => VirtualKeyCode_::F24,
    Snapshot => VirtualKeyCode_::Snapshot,
    Scroll => VirtualKeyCode_::Scroll,
    Pause => VirtualKeyCode_::Pause,
    Insert => VirtualKeyCode_::Insert,
    Home => VirtualKeyCode_::Home,
    Delete => VirtualKeyCode_::Delete,
    End => VirtualKeyCode_::End,
    PageDown => VirtualKeyCode_::PageDown,
    PageUp => VirtualKeyCode_::PageUp,
    Left => VirtualKeyCode_::Left,
    Up => VirtualKeyCode_::Up,
    Right => VirtualKeyCode_::Right,
    Down => VirtualKeyCode_::Down,
    Back => VirtualKeyCode_::Back,
    Return => VirtualKeyCode_::Return,
    Space => VirtualKeyCode_::Space,
    Compose => VirtualKeyCode_::Compose,
    Caret => VirtualKeyCode_::Caret,
    Numlock => VirtualKeyCode_::Numlock,
    Numpad0 => VirtualKeyCode_::Numpad0,
    Numpad1 => VirtualKeyCode_::Numpad1,
    Numpad2 => VirtualKeyCode_::Numpad2,
    Numpad3 => VirtualKeyCode_::Numpad3,
    Numpad4 => VirtualKeyCode_::Numpad4,
    Numpad5 => VirtualKeyCode_::Numpad5,
    Numpad6 => VirtualKeyCode_::Numpad6,
    Numpad7 => VirtualKeyCode_::Numpad7,
    Numpad8 => VirtualKeyCode_::Numpad8,
    Numpad9 => VirtualKeyCode_::Numpad9,
    NumpadAdd => VirtualKeyCode_::NumpadAdd,
    NumpadDivide => VirtualKeyCode_::NumpadDivide,
    NumpadDecimal => VirtualKeyCode_::NumpadDecimal,
    NumpadComma => VirtualKeyCode_::NumpadComma,
    NumpadEnter => VirtualKeyCode_::NumpadEnter,
    NumpadEquals => VirtualKeyCode_::NumpadEquals,
    NumpadMultiply => VirtualKeyCode_::NumpadMultiply,
    NumpadSubtract => VirtualKeyCode_::NumpadSubtract,
    AbntC1 => VirtualKeyCode_::AbntC1,
    AbntC2 => VirtualKeyCode_::AbntC2,
    Apostrophe => VirtualKeyCode_::Apostrophe,
    Apps => VirtualKeyCode_::Apps,
    Asterisk => VirtualKeyCode_::Asterisk,
    At => VirtualKeyCode_::At,
    Ax => VirtualKeyCode_::Ax,
    Backslash => VirtualKeyCode_::Backslash,
    Calculator => VirtualKeyCode_::Calculator,
    Capital => VirtualKeyCode_::Capital,
    Colon => VirtualKeyCode_::Colon,
    Comma => VirtualKeyCode_::Comma,
    Convert => VirtualKeyCode_::Convert,
    Equals => VirtualKeyCode_::Equals,
    Grave => VirtualKeyCode_::Grave,
    Kana => VirtualKeyCode_::Kana,
    Kanji => VirtualKeyCode_::Kanji,
    LAlt => VirtualKeyCode_::LAlt,
    LBracket => VirtualKeyCode_::LBracket,
    LControl => VirtualKeyCode_::LControl,
    LShift => VirtualKeyCode_::LShift,
    LWin => VirtualKeyCode_::LWin,
    Mail => VirtualKeyCode_::Mail,
    MediaSelect => VirtualKeyCode_::MediaSelect,
    MediaStop => VirtualKeyCode_::MediaStop,
    Minus => VirtualKeyCode_::Minus,
    Mute => VirtualKeyCode_::Mute,
    MyComputer => VirtualKeyCode_::MyComputer,
    NavigateForward => VirtualKeyCode_::NavigateForward,
    NavigateBackward => VirtualKeyCode_::NavigateBackward,
    NextTrack => VirtualKeyCode_::NextTrack,
    NoConvert => VirtualKeyCode_::NoConvert,
    OEM102 => VirtualKeyCode_::OEM102,
    Period => VirtualKeyCode_::Period,
    PlayPause => VirtualKeyCode_::PlayPause,
    Plus => VirtualKeyCode_::Plus,
    Power => VirtualKeyCode_::Power,
    PrevTrack => VirtualKeyCode_::PrevTrack,
    RAlt => VirtualKeyCode_::RAlt,
    RBracket => VirtualKeyCode_::RBracket,
    RControl => VirtualKeyCode_::RControl,
    RShift => VirtualKeyCode_::RShift,
    RWin => VirtualKeyCode_::RWin,
    Semicolon => VirtualKeyCode_::Semicolon,
    Slash => VirtualKeyCode_::Slash,
    Sleep => VirtualKeyCode_::Sleep,
    Stop => VirtualKeyCode_::Stop,
    Sysrq => VirtualKeyCode_::Sysrq,
    Tab => VirtualKeyCode_::Tab,
    Underline => VirtualKeyCode_::Underline,
    Unlabeled => VirtualKeyCode_::Unlabeled,
    VolumeDown => VirtualKeyCode_::VolumeDown,
    VolumeUp => VirtualKeyCode_::VolumeUp,
    Wake => VirtualKeyCode_::Wake,
    WebBack => VirtualKeyCode_::WebBack,
    WebFavorites => VirtualKeyCode_::WebFavorites,
    WebForward => VirtualKeyCode_::WebForward,
    WebHome => VirtualKeyCode_::WebHome,
    WebRefresh => VirtualKeyCode_::WebRefresh,
    WebSearch => VirtualKeyCode_::WebSearch,
    WebStop => VirtualKeyCode_::WebStop,
    Yen => VirtualKeyCode_::Yen,
    Copy => VirtualKeyCode_::Copy,
    Paste => VirtualKeyCode_::Paste,
    Cut => VirtualKeyCode_::Cut,
  }
}

fn VirtualKeyCode_val(_: Value) -> VirtualKeyCode {
  unimplemented!("VirtualKeyCode_val")
}

// event::ElementState
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum ElementState_ {
  Pressed,
  Released,
}

fn Val_ElementState(value: ElementState) -> ElementState_ {
  use ElementState::*;
  match value {
    Pressed => ElementState_::Pressed,
    Released => ElementState_::Released,
  }
}

fn ElementState_val(_: Value) -> ElementState {
  unimplemented!("ElementState_val")
}

// event::TouchPhase
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum TouchPhase_ {
  Started,
  Moved,
  Ended,
  Cancelled,
}

fn Val_TouchPhase(value: TouchPhase) -> TouchPhase_ {
  use TouchPhase::*;
  match value {
    Started => TouchPhase_::Started,
    Moved => TouchPhase_::Moved,
    Ended => TouchPhase_::Ended,
    Cancelled => TouchPhase_::Cancelled,
  }
}

fn TouchPhase_val(_: Value) -> TouchPhase {
  unimplemented!("TouchPhase_val")
}

// event::MouseButton
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum MouseButton_ {
  Left,
  Right,
  Middle,
  Other(u16),
}

fn Val_MouseButton(value: MouseButton) -> MouseButton_ {
  use MouseButton::*;
  match value {
    Left => MouseButton_::Left,
    Right => MouseButton_::Right,
    Middle => MouseButton_::Middle,
    Other(size) => MouseButton_::Other(size),
  }
}

fn MouseButton_val(_: Value) -> MouseButton {
  unimplemented!("MouseButton_val")
}

// event::MouseScrollDelta
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum MouseScrollDelta_ {
  LineDelta(f32, f32),
  PixelDelta(PhysicalPosition_<f64>),
}

fn Val_MouseScrollDelta(value: MouseScrollDelta) -> MouseScrollDelta_ {
  use MouseScrollDelta::*;
  match value {
    LineDelta(x, y) => MouseScrollDelta_::LineDelta(x, y),
    PixelDelta(position) => {
      let position = Val_PhysicalPosition(position);
      MouseScrollDelta_::PixelDelta(position)
    }
  }
}

fn MouseScrollDelta_val(_: Value) -> MouseScrollDelta {
  unimplemented!("MouseScrollDelta_val")
}

// event::Force
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum Force_ {
  Calibrated {
    force: f64,
    max_possible_force: f64,
    altitude_angle: Option<f64>,
  },
  Normalized(f64),
}

fn Val_Force(value: Force) -> Force_ {
  use Force::*;
  match value {
    Calibrated {
      force,
      max_possible_force,
      altitude_angle,
    } => Force_::Calibrated {
      force,
      max_possible_force,
      altitude_angle,
    },
    Normalized(amount) => Force_::Normalized(amount),
  }
}

fn Force_val(_: Value) -> Force {
  unimplemented!("Force_val")
}

// event::Touch
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
struct Touch_ {
  device_id: DeviceId_,
  phase: TouchPhase_,
  location: PhysicalPosition_<f64>,
  force: Option<Force_>,
  id: isize,
}

fn Val_Touch(value: Touch) -> Touch_ {
  Touch_ {
    device_id: Val_DeviceId(value.device_id),
    phase: Val_TouchPhase(value.phase),
    location: Val_PhysicalPosition(value.location),
    force: value.force.map(Val_Force),
    id: value.id as isize,
  }
}

fn Touch_val(_: Value) -> Touch {
  unimplemented!("Touch_val")
}

// event::ModifiersState
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
struct ModifiersState_ {
  shift: bool,
  ctrl: bool,
  alt: bool,
  logo: bool,
}

fn Val_ModifiersState(value: ModifiersState) -> ModifiersState_ {
  ModifiersState_ {
    shift: value.shift(),
    ctrl: value.ctrl(),
    alt: value.alt(),
    logo: value.logo(),
  }
}

fn ModifiersState_val(_: Value) -> ModifiersState {
  unimplemented!("ModifiersState_val")
}

// event::KeyboardInput
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
struct KeyboardInput_ {
  scancode: isize,
  state: ElementState_,
  virtual_keycode: Option<VirtualKeyCode_>,
}

fn Val_KeyboardInput(value: KeyboardInput) -> KeyboardInput_ {
  KeyboardInput_ {
    scancode: value.scancode as isize,
    state: Val_ElementState(value.state),
    virtual_keycode: value.virtual_keycode.map(Val_VirtualKeyCode),
  }
}

fn KeyboardInput_val(_: Value) -> KeyboardInput {
  unimplemented!("KeyboardInput_val")
}

// dpi::PhysicalSize
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
struct PhysicalSize_<T> {
  width: T,
  height: T,
}

fn Val_PhysicalSize<T>(value: PhysicalSize<T>) -> PhysicalSize_<T> {
  PhysicalSize_ {
    width: value.width,
    height: value.height,
  }
}

fn PhysicalSize_val<T>(_: Value) -> PhysicalSize<T> {
  unimplemented!("PhysicalSize_val")
}

// dpi::PhysicalPosition
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
struct PhysicalPosition_<T> {
  x: T,
  y: T,
}

fn Val_PhysicalPosition<T>(value: PhysicalPosition<T>) -> PhysicalPosition_<T> {
  PhysicalPosition_ {
    x: value.x,
    y: value.y,
  }
}

fn PhysicalPosition_val<T>(_: Value) -> PhysicalPosition<T> {
  unimplemented!("PhysicalPosition_val")
}

// event::StartCause
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum StartCause_ {
  ResumeTimeReached {
    start: isize,
    requested_resume: isize,
  },
  WaitCancelled {
    start: isize,
    requested_resume: Option<isize>,
  },
  Poll,
  Init,
}

fn Val_StartCause(value: StartCause) -> StartCause_ {
  use StartCause::*;
  match value {
    ResumeTimeReached {
      start,
      requested_resume,
    } => StartCause_::ResumeTimeReached {
      start: isize_from_instant(start),
      requested_resume: isize_from_instant(requested_resume),
    },
    WaitCancelled {
      start,
      requested_resume,
    } => StartCause_::WaitCancelled {
      start: isize_from_instant(start),
      requested_resume: requested_resume.map(isize_from_instant),
    },
    Poll => StartCause_::Poll,
    Init => StartCause_::Init,
  }
}

fn StartCause_val(_val: StartCause_) -> StartCause {
  unimplemented!("StartCause_val")
}

// event::WindowEvent
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum WindowEvent_ {
  Resized(PhysicalSize_<i16>),
  Moved(PhysicalPosition_<i16>),
  CloseRequested,
  Destroyed,
  DroppedFile(String),
  HoveredFile(String),
  HoveredFileCancelled,
  ReceivedCharacter(String),
  Focused(bool),
  KeyboardInput {
    device_id: DeviceId_,
    input: KeyboardInput_,
    is_synthetic: bool,
  },
  ModifiersChanged(ModifiersState_),
  CursorMoved {
    device_id: DeviceId_,
    position: PhysicalPosition_<f64>,
  },
  CursorEntered {
    device_id: DeviceId_,
  },
  CursorLeft {
    device_id: DeviceId_,
  },
  MouseWheel {
    device_id: DeviceId_,
    delta: MouseScrollDelta_,
    phase: TouchPhase_,
  },
  MouseInput {
    device_id: DeviceId_,
    state: ElementState_,
    button: MouseButton_,
  },
  TouchpadPressure {
    device_id: DeviceId_,
    pressure: f32,
    stage: isize,
  },
  AxisMotion {
    device_id: DeviceId_,
    axis: AxisId_,
    value: f64,
  },
  Touch(Touch_),
  ScaleFactorChanged {
    scale_factor: f64,
    // TODO: handle mutation side-effect
    new_inner_size: PhysicalSize_<i16>,
  },
  ThemeChanged(Theme_),
}

fn Val_WindowEvent(value: WindowEvent) -> WindowEvent_ {
  use WindowEvent::*;
  match value {
    Resized(size) => {
      let size = Val_PhysicalSize(size.cast());
      WindowEvent_::Resized(size)
    }
    Moved(position) => {
      let position = Val_PhysicalPosition(position.cast());
      WindowEvent_::Moved(position)
    }
    CloseRequested => WindowEvent_::CloseRequested,
    Destroyed => WindowEvent_::Destroyed,
    Focused(focused) => WindowEvent_::Focused(focused),
    DroppedFile(path) => {
      let path = path.to_string_lossy().into_owned();
      WindowEvent_::DroppedFile(path)
    }
    HoveredFile(path) => {
      let path = path.to_string_lossy().into_owned();
      WindowEvent_::HoveredFile(path)
    }
    HoveredFileCancelled => WindowEvent_::HoveredFileCancelled,
    ReceivedCharacter(char) => {
      let char = char.to_string();
      WindowEvent_::ReceivedCharacter(char)
    }
    KeyboardInput {
      device_id,
      input,
      is_synthetic,
    } => WindowEvent_::KeyboardInput {
      device_id: Val_DeviceId(device_id),
      input: Val_KeyboardInput(input),
      is_synthetic,
    },
    ModifiersChanged(state) => {
      let state = Val_ModifiersState(state);
      WindowEvent_::ModifiersChanged(state)
    }
    CursorMoved {
      device_id,
      position,
      ..
    } => WindowEvent_::CursorMoved {
      device_id: Val_DeviceId(device_id),
      position: Val_PhysicalPosition(position),
    },
    CursorEntered { device_id } => WindowEvent_::CursorEntered {
      device_id: Val_DeviceId(device_id),
    },
    CursorLeft { device_id } => WindowEvent_::CursorLeft {
      device_id: Val_DeviceId(device_id),
    },
    MouseWheel {
      device_id,
      delta,
      phase,
      ..
    } => WindowEvent_::MouseWheel {
      device_id: Val_DeviceId(device_id),
      delta: Val_MouseScrollDelta(delta),
      phase: Val_TouchPhase(phase),
    },
    MouseInput {
      device_id,
      state,
      button,
      ..
    } => WindowEvent_::MouseInput {
      device_id: Val_DeviceId(device_id),
      state: Val_ElementState(state),
      button: Val_MouseButton(button),
    },
    TouchpadPressure {
      device_id,
      pressure,
      stage,
      ..
    } => WindowEvent_::TouchpadPressure {
      device_id: Val_DeviceId(device_id),
      pressure,
      stage: stage as isize,
    },
    AxisMotion {
      device_id,
      axis,
      value,
      ..
    } => WindowEvent_::AxisMotion {
      device_id: Val_DeviceId(device_id),
      axis: Val_AxisId(axis),
      value,
    },
    Touch(touch) => {
      let touch = Val_Touch(touch);
      WindowEvent_::Touch(touch)
    }
    ScaleFactorChanged {
      scale_factor,
      new_inner_size,
    } => WindowEvent_::ScaleFactorChanged {
      scale_factor,
      new_inner_size: Val_PhysicalSize(new_inner_size.cast()),
    },
    ThemeChanged(theme) => {
      let theme = Val_Theme(theme);
      WindowEvent_::ThemeChanged(theme)
    }
  }
}

fn WindowEvent_val<'a>(_val: WindowEvent_) -> WindowEvent<'a> {
  unimplemented!("WindowEvent_val")
}

// event::DeviceEvent
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum DeviceEvent_ {
  Added,
  Removed,
  MouseMotion { delta: (f64, f64) },
  MouseWheel { delta: (f64, f64) },
  Motion { axis: isize, value: f64 },
  Button { button: isize, state: bool },
  Key(bool),
  Text { codepoint: String },
}

fn Val_DeviceEvent(value: DeviceEvent) -> DeviceEvent_ {
  use DeviceEvent::*;
  match value {
    Added => DeviceEvent_::Added,
    Removed => DeviceEvent_::Removed,
    MouseMotion { delta } => DeviceEvent_::MouseMotion { delta },
    MouseWheel { delta: _ } => DeviceEvent_::MouseWheel { delta: (0., 0.) },
    Motion { axis: _, value } => DeviceEvent_::Motion { axis: 0, value },
    Button {
      button: _,
      state: _,
    } => DeviceEvent_::Button {
      button: 0,
      state: false,
    },
    Key(_) => DeviceEvent_::Key(false),
    Text { codepoint } => DeviceEvent_::Text {
      codepoint: format!("{}", codepoint),
    },
  }
}

fn DeviceEvent_val(_val: DeviceEvent_) -> DeviceEvent {
  unimplemented!("DeviceEvent_val")
}

// event::Event
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum Event_ {
  NewEvents(StartCause_),
  WindowEvent {
    window_id: WindowId_,
    event: WindowEvent_,
  },
  DeviceEvent {
    device_id: DeviceId_,
    event: DeviceEvent_,
  },
  UserEvent(Value),
  Suspended,
  Resumed,
  MainEventsCleared,
  RedrawRequested(WindowId_),
  RedrawEventsCleared,
  LoopDestroyed,
}

fn Val_Event<'a, T>(value: Event<'a, T>) -> Event_ {
  use Event::*;
  match value {
    NewEvents(cause) => Event_::NewEvents(Val_StartCause(cause)),
    WindowEvent { window_id, event } => Event_::WindowEvent {
      window_id: Val_WindowId(window_id),
      event: Val_WindowEvent(event),
    },
    DeviceEvent { device_id, event } => Event_::DeviceEvent {
      device_id: Val_DeviceId(device_id),
      event: Val_DeviceEvent(event),
    },
    UserEvent(_) => unimplemented!("UserEvent"),
    Suspended => Event_::Suspended,
    Resumed => Event_::Resumed,
    MainEventsCleared => Event_::MainEventsCleared,
    RedrawRequested(window_id) => Event_::RedrawRequested(Val_WindowId(window_id)),
    RedrawEventsCleared => Event_::RedrawEventsCleared,
    LoopDestroyed => Event_::LoopDestroyed,
  }
}

fn Event_val<'a, T>(_val: Event_) -> Event<'a, T> {
  unimplemented!("Event_val")
}

// event_loop::ControlFlow
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue, Debug)]
enum ControlFlow_ {
  Poll,
  Wait,
  WaitUntil(isize),
  Exit,
}

fn Val_ControlFlow(value: ControlFlow) -> ControlFlow_ {
  use ControlFlow::*;
  match value {
    Poll => ControlFlow_::Poll,
    Wait => ControlFlow_::Wait,
    WaitUntil(instant) => {
      let millis = isize_from_instant(instant);
      ControlFlow_::WaitUntil(millis)
    }
    Exit => ControlFlow_::Exit,
  }
}

fn ControlFlow_val(value: Value) -> ControlFlow {
  use ControlFlow::*;
  match ControlFlow_::from_value(value) {
    ControlFlow_::Poll => Poll,
    ControlFlow_::Wait => Wait,
    ControlFlow_::WaitUntil(millis) => {
      let millis = now_plus_millis(millis as u64);
      WaitUntil(millis)
    }
    ControlFlow_::Exit => Exit,
  }
}

// event_loop::EventLoop<T>
//------------------------------------------------------------------------------

#[derive(ToValue, FromValue)]
struct EventLoop_(Pointer<Option<EventLoop<()>>>);

impl Debug for EventLoop_ {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self.0.as_ref())
  }
}

unsafe extern "C" fn finalize_EventLoop(v: Value) {
  let ptr: Pointer<EventLoop_> = Pointer::from_value(v);
  ptr.drop_in_place()
}

fn Val_EventLoop(val: EventLoop<()>) -> EventLoop_ {
  let ptr = Pointer::alloc_final(Some(val), Some(finalize_EventLoop), None);
  EventLoop_(ptr)
}

// event_loop::EventLoop<T>::new
#[ocaml::func]
pub fn re_winit_event_loop_EventLoop_new() -> EventLoop_ {
  Val_EventLoop(EventLoop::new())
}

// event_loop::EventLoop<T>::run
#[ocaml::func]
pub unsafe fn re_winit_event_loop_EventLoop_run(cb: ocaml::Value, mut event_loop: EventLoop_) {
  let cb = std::sync::Arc::new(cb);
  let event_loop = event_loop.0.as_mut();
  if let Some(event_loop) = event_loop.take() {
    event_loop.run(move |event, _event_loop_window_target, control_flow| {
      let val_event = Val_Event(event);
      let val_target = ();
      let val_control_flow = Val_ControlFlow(*control_flow);
      let result = cb.call3(val_event, val_target, val_control_flow);
      *control_flow = match result {
        Err(_err) => unimplemented!(),
        Ok(value) => ControlFlow_val(value),
      }
    });
  }
}
