module rec ControlFlow: {
  [@deriving show({with_path: false})]
  type t =
    | Poll
    | Wait
    | WaitUntil(int)
    | Exit;
} = {
  [@deriving show({with_path: false})]
  type t =
    | Poll
    | Wait
    | WaitUntil(int)
    | Exit;
}
and Dpi: {
  module PhysicalSize: {
    [@deriving show({with_path: false})]
    type t('a) = {
      width: 'a,
      height: 'a,
    };
  };
  module LogicalSize: {
    [@deriving show({with_path: false})]
    type t('a) = {
      width: 'a,
      height: 'a,
    };
  };
  module PhysicalPosition: {
    [@deriving show({with_path: false})]
    type t('a) = {
      x: 'a,
      y: 'a,
    };
  };
  module LogicalPosition: {
    [@deriving show({with_path: false})]
    type t('a) = {
      x: 'a,
      y: 'a,
    };
  };
} = {
  module PhysicalSize = {
    [@deriving show({with_path: false})]
    type t('a) = {
      width: 'a,
      height: 'a,
    };
  };
  module LogicalSize = {
    [@deriving show({with_path: false})]
    type t('a) = {
      width: 'a,
      height: 'a,
    };
  };
  module PhysicalPosition = {
    [@deriving show({with_path: false})]
    type t('a) = {
      x: 'a,
      y: 'a,
    };
  };
  module LogicalPosition = {
    [@deriving show({with_path: false})]
    type t('a) = {
      x: 'a,
      y: 'a,
    };
  };
}
and Event: {
  module DeviceId: {type t;};
  module AxisId: {type t;};
  module ButtonId: {type t;};
  module VirtualKeyCode: {
    [@deriving show({with_path: false})]
    type t =
      | Key1
      | Key2
      | Key3
      | Key4
      | Key5
      | Key6
      | Key7
      | Key8
      | Key9
      | Key0
      | A
      | B
      | C
      | D
      | E
      | F
      | G
      | H
      | I
      | J
      | K
      | L
      | M
      | N
      | O
      | P
      | Q
      | R
      | S
      | T
      | U
      | V
      | W
      | X
      | Y
      | Z
      | Escape
      | F1
      | F2
      | F3
      | F4
      | F5
      | F6
      | F7
      | F8
      | F9
      | F10
      | F11
      | F12
      | F13
      | F14
      | F15
      | F16
      | F17
      | F18
      | F19
      | F20
      | F21
      | F22
      | F23
      | F24
      | Snapshot
      | Scroll
      | Pause
      | Insert
      | Home
      | Delete
      | End
      | PageDown
      | PageUp
      | Left
      | Up
      | Right
      | Down
      | Back
      | Return
      | Space
      | Compose
      | Caret
      | Numlock
      | Numpad0
      | Numpad1
      | Numpad2
      | Numpad3
      | Numpad4
      | Numpad5
      | Numpad6
      | Numpad7
      | Numpad8
      | Numpad9
      | NumpadAdd
      | NumpadDivide
      | NumpadDecimal
      | NumpadComma
      | NumpadEnter
      | NumpadEquals
      | NumpadMultiply
      | NumpadSubtract
      | AbntC1
      | AbntC2
      | Apostrophe
      | Apps
      | Asterisk
      | At
      | Ax
      | Backslash
      | Calculator
      | Capital
      | Colon
      | Comma
      | Convert
      | Equals
      | Grave
      | Kana
      | Kanji
      | LAlt
      | LBracket
      | LControl
      | LShift
      | LWin
      | Mail
      | MediaSelect
      | MediaStop
      | Minus
      | Mute
      | MyComputer
      | NavigateForward
      | NavigateBackward
      | NextTrack
      | NoConvert
      | OEM102
      | Period
      | PlayPause
      | Plus
      | Power
      | PrevTrack
      | RAlt
      | RBracket
      | RControl
      | RShift
      | RWin
      | Semicolon
      | Slash
      | Sleep
      | Stop
      | Sysrq
      | Tab
      | Underline
      | Unlabeled
      | VolumeDown
      | VolumeUp
      | Wake
      | WebBack
      | WebFavorites
      | WebForward
      | WebHome
      | WebRefresh
      | WebSearch
      | WebStop
      | Yen
      | Copy
      | Paste
      | Cut;
  };
  module ElementState: {
    [@deriving show({with_path: false})]
    type t =
      | Pressed
      | Released;
  };
  module TouchPhase: {
    [@deriving show({with_path: false})]
    type t =
      | Started
      | Moved
      | Ended
      | Cancelled;
  };
  module MouseButton: {
    [@deriving show({with_path: false})]
    type t =
      | Left
      | Right
      | Middle
      | Other(int);
  };
  module MouseScrollDelta: {
    [@deriving show({with_path: false})]
    type t =
      | LineDelta(float, float)
      | PixelDelta(Dpi.PhysicalPosition.t(float));
  };
  module Force: {
    [@deriving show({with_path: false})]
    type t =
      | Calibrated({
          force: float,
          max_possible_force: float,
          altitude_angle: option(float),
        })
      | Normalize(float);
  };
  module Touch: {
    [@deriving show({with_path: false})]
    type t = {
      device_id: [@opaque] DeviceId.t,
      phase: TouchPhase.t,
      location: Dpi.PhysicalPosition.t(float),
      force: option(Force.t),
      id: int,
    };
  };
  module ModifiersState: {
    [@deriving show({with_path: false})]
    type t = {
      shift: bool,
      ctrl: bool,
      alt: bool,
      logo: bool,
    };
  };
  module KeyboardInput: {
    [@deriving show({with_path: false})]
    type t = {
      scancode: int,
      state: ElementState.t,
      virtual_keycode: option(VirtualKeyCode.t),
    };
  };
  module StartCause: {
    [@deriving show({with_path: false})]
    type t =
      | ResumeTimeReached({
          start: int,
          requested_resume: int,
        })
      | WaitCancelled({
          start: int,
          requested_resume: option(int),
        })
      | Poll
      | Init;
  };
  module WindowEvent: {
    [@deriving show({with_path: false})]
    type t =
      | Resized(Dpi.PhysicalSize.t(int))
      | Moved(Dpi.PhysicalPosition.t(int))
      | CloseRequested
      | Destroyed
      | DroppedFile(string)
      | HoveredFile(string)
      | HoveredFileCancelled
      | ReceivedCharacter(string)
      | Focused(bool)
      | KeyboardInput({
          device_id: [@opaque] DeviceId.t,
          input: KeyboardInput.t,
          is_synthetic: bool,
        })
      | ModifiersChanged(ModifiersState.t)
      | CursorMoved({
          device_id: [@opaque] DeviceId.t,
          position: Dpi.PhysicalPosition.t(float),
        })
      | CursorEntered({device_id: [@opaque] DeviceId.t})
      | CursorLeft({device_id: [@opaque] DeviceId.t})
      | MouseWheel({
          device_id: [@opaque] DeviceId.t,
          delta: MouseScrollDelta.t,
          phase: TouchPhase.t,
        })
      | MouseInput({
          device_id: [@opaque] DeviceId.t,
          state: ElementState.t,
          button: MouseButton.t,
        })
      | TouchpadPressure({
          device_id: [@opaque] DeviceId.t,
          pressure: float,
          stage: int,
        })
      | AxisMotion({
          device_id: [@opaque] DeviceId.t,
          axis: [@opaque] AxisId.t,
          value: float,
        })
      | Touch(Touch.t)
      | ScaleFactorChanged({
          scale_factor: float,
          mutable new_inner_size: Dpi.PhysicalSize.t(int),
        })
      | ThemeChanged(Window.Theme.t);
  };
  module DeviceEvent: {
    [@deriving show({with_path: false})]
    type t =
      | Added
      | Removed
      | MouseMotion({delta: (float, float)})
      | MouseWheel({delta: MouseScrollDelta.t})
      | Motion({
          axis: [@opaque] AxisId.t,
          value: float,
        })
      | Button({
          button: [@opaque] ButtonId.t,
          state: ElementState.t,
        })
      | Key(KeyboardInput.t)
      | Text({codepoint: string});
  };
  [@deriving show({with_path: false})]
  type t('a) =
    | NewEvents(StartCause.t)
    | WindowEvent({
        window_id: [@opaque] Window.WindowId.t,
        event: WindowEvent.t,
      })
    | DeviceEvent({
        device_id: [@opaque] DeviceId.t,
        event: DeviceEvent.t,
      })
    | UserEvent('a)
    | Suspended
    | Resumed
    | MainEventsCleared
    | RedrawRequested([@opaque] Window.WindowId.t)
    | RedrawEventsCleared
    | LoopDestroyed;
} = {
  module DeviceId = {
    type t;
  };
  module AxisId = {
    type t;
  };
  module ButtonId = {
    type t;
  };
  module VirtualKeyCode = {
    [@deriving show({with_path: false})]
    type t =
      | Key1
      | Key2
      | Key3
      | Key4
      | Key5
      | Key6
      | Key7
      | Key8
      | Key9
      | Key0
      | A
      | B
      | C
      | D
      | E
      | F
      | G
      | H
      | I
      | J
      | K
      | L
      | M
      | N
      | O
      | P
      | Q
      | R
      | S
      | T
      | U
      | V
      | W
      | X
      | Y
      | Z
      | Escape
      | F1
      | F2
      | F3
      | F4
      | F5
      | F6
      | F7
      | F8
      | F9
      | F10
      | F11
      | F12
      | F13
      | F14
      | F15
      | F16
      | F17
      | F18
      | F19
      | F20
      | F21
      | F22
      | F23
      | F24
      | Snapshot
      | Scroll
      | Pause
      | Insert
      | Home
      | Delete
      | End
      | PageDown
      | PageUp
      | Left
      | Up
      | Right
      | Down
      | Back
      | Return
      | Space
      | Compose
      | Caret
      | Numlock
      | Numpad0
      | Numpad1
      | Numpad2
      | Numpad3
      | Numpad4
      | Numpad5
      | Numpad6
      | Numpad7
      | Numpad8
      | Numpad9
      | NumpadAdd
      | NumpadDivide
      | NumpadDecimal
      | NumpadComma
      | NumpadEnter
      | NumpadEquals
      | NumpadMultiply
      | NumpadSubtract
      | AbntC1
      | AbntC2
      | Apostrophe
      | Apps
      | Asterisk
      | At
      | Ax
      | Backslash
      | Calculator
      | Capital
      | Colon
      | Comma
      | Convert
      | Equals
      | Grave
      | Kana
      | Kanji
      | LAlt
      | LBracket
      | LControl
      | LShift
      | LWin
      | Mail
      | MediaSelect
      | MediaStop
      | Minus
      | Mute
      | MyComputer
      | NavigateForward
      | NavigateBackward
      | NextTrack
      | NoConvert
      | OEM102
      | Period
      | PlayPause
      | Plus
      | Power
      | PrevTrack
      | RAlt
      | RBracket
      | RControl
      | RShift
      | RWin
      | Semicolon
      | Slash
      | Sleep
      | Stop
      | Sysrq
      | Tab
      | Underline
      | Unlabeled
      | VolumeDown
      | VolumeUp
      | Wake
      | WebBack
      | WebFavorites
      | WebForward
      | WebHome
      | WebRefresh
      | WebSearch
      | WebStop
      | Yen
      | Copy
      | Paste
      | Cut;
  };
  module ElementState = {
    [@deriving show({with_path: false})]
    type t =
      | Pressed
      | Released;
  };
  module TouchPhase = {
    [@deriving show({with_path: false})]
    type t =
      | Started
      | Moved
      | Ended
      | Cancelled;
  };
  module MouseButton = {
    [@deriving show({with_path: false})]
    type t =
      | Left
      | Right
      | Middle
      | Other(int);
  };
  module MouseScrollDelta = {
    [@deriving show({with_path: false})]
    type t =
      | LineDelta(float, float)
      | PixelDelta(Dpi.PhysicalPosition.t(float));
  };
  module Force = {
    [@deriving show({with_path: false})]
    type t =
      | Calibrated({
          force: float,
          max_possible_force: float,
          altitude_angle: option(float),
        })
      | Normalize(float);
  };
  module Touch = {
    [@deriving show({with_path: false})]
    type t = {
      device_id: [@opaque] DeviceId.t,
      phase: TouchPhase.t,
      location: Dpi.PhysicalPosition.t(float),
      force: option(Force.t),
      id: int,
    };
  };
  module ModifiersState = {
    [@deriving show({with_path: false})]
    type t = {
      shift: bool,
      ctrl: bool,
      alt: bool,
      logo: bool,
    };
  };
  module KeyboardInput = {
    [@deriving show({with_path: false})]
    type t = {
      scancode: int,
      state: ElementState.t,
      virtual_keycode: option(VirtualKeyCode.t),
    };
  };
  module StartCause = {
    [@deriving show({with_path: false})]
    type t =
      | ResumeTimeReached({
          start: int,
          requested_resume: int,
        })
      | WaitCancelled({
          start: int,
          requested_resume: option(int),
        })
      | Poll
      | Init;
  };
  module WindowEvent = {
    [@deriving show({with_path: false})]
    type t =
      | Resized(Dpi.PhysicalSize.t(int))
      | Moved(Dpi.PhysicalPosition.t(int))
      | CloseRequested
      | Destroyed
      | DroppedFile(string)
      | HoveredFile(string)
      | HoveredFileCancelled
      | ReceivedCharacter(string)
      | Focused(bool)
      | KeyboardInput({
          device_id: [@opaque] DeviceId.t,
          input: KeyboardInput.t,
          is_synthetic: bool,
        })
      | ModifiersChanged(ModifiersState.t)
      | CursorMoved({
          device_id: [@opaque] DeviceId.t,
          position: Dpi.PhysicalPosition.t(float),
        })
      | CursorEntered({device_id: [@opaque] DeviceId.t})
      | CursorLeft({device_id: [@opaque] DeviceId.t})
      | MouseWheel({
          device_id: [@opaque] DeviceId.t,
          delta: MouseScrollDelta.t,
          phase: TouchPhase.t,
        })
      | MouseInput({
          device_id: [@opaque] DeviceId.t,
          state: ElementState.t,
          button: MouseButton.t,
        })
      | TouchpadPressure({
          device_id: [@opaque] DeviceId.t,
          pressure: float,
          stage: int,
        })
      | AxisMotion({
          device_id: [@opaque] DeviceId.t,
          axis: [@opaque] AxisId.t,
          value: float,
        })
      | Touch(Touch.t)
      | ScaleFactorChanged({
          scale_factor: float,
          mutable new_inner_size: Dpi.PhysicalSize.t(int),
        })
      | ThemeChanged(Window.Theme.t);
  };
  module DeviceEvent = {
    [@deriving show({with_path: false})]
    type t =
      | Added
      | Removed
      | MouseMotion({delta: (float, float)})
      | MouseWheel({delta: MouseScrollDelta.t})
      | Motion({
          axis: [@opaque] AxisId.t,
          value: float,
        })
      | Button({
          button: [@opaque] ButtonId.t,
          state: ElementState.t,
        })
      | Key(KeyboardInput.t)
      | Text({codepoint: string});
  };
  [@deriving show({with_path: false})]
  type t('a) =
    | NewEvents(StartCause.t)
    | WindowEvent({
        window_id: [@opaque] Window.WindowId.t,
        event: WindowEvent.t,
      })
    | DeviceEvent({
        device_id: [@opaque] DeviceId.t,
        event: DeviceEvent.t,
      })
    | UserEvent('a)
    | Suspended
    | Resumed
    | MainEventsCleared
    | RedrawRequested([@opaque] Window.WindowId.t)
    | RedrawEventsCleared
    | LoopDestroyed;
}
and EventLoop: {
  type t('a);
  let new_: unit => t(unit);
  let run:
    ((Event.t('a), unit, ControlFlow.t) => ControlFlow.t, t('a)) => unit;
} = {
  type t('a);
  external new_: unit => t(unit) = "re_winit_event_loop_EventLoop_new";
  external run:
    ((Event.t('a), unit, ControlFlow.t) => ControlFlow.t, t('a)) => unit =
    "re_winit_event_loop_EventLoop_run";
}
and Window: {
  module WindowId: {
    type t;
    let (==): (t, t) => bool;
    let (!=): (t, t) => bool;
  };
  module Theme: {
    [@deriving show({with_path: false})]
    type t =
      | Light
      | Dark;
  };
  type t;
  let new_: EventLoop.t('a) => t;
  let id: t => WindowId.t;
  let set_always_on_top: (bool, t) => unit;
  let set_decorations: (bool, t) => unit;
  let set_minimized: (bool, t) => unit;
  let set_maximized: (bool, t) => unit;
  let set_resizable: (bool, t) => unit;
  let set_title: (string, t) => unit;
  let set_visible: (bool, t) => unit;
} = {
  module WindowId = {
    type t;
    external (==): (t, t) => bool = "re_winit_window_WindowId_eq";
    let (!=) = (a, b) => !(a == b);
  };
  module Theme = {
    [@deriving show({with_path: false})]
    type t =
      | Light
      | Dark;
  };
  type t;
  external new_: EventLoop.t('a) => t = "re_winit_window_Window_new";
  external id: t => WindowId.t = "re_winit_window_Window_id";
  external set_always_on_top: (bool, t) => unit =
    "re_winit_window_Window_set_always_on_top";
  external set_decorations: (bool, t) => unit =
    "re_winit_window_Window_set_decorations";
  external set_minimized: (bool, t) => unit =
    "re_winit_window_Window_set_minimized";
  external set_maximized: (bool, t) => unit =
    "re_winit_window_Window_set_maximized";
  external set_resizable: (bool, t) => unit =
    "re_winit_window_Window_set_resizable";
  external set_title: (string, t) => unit = "re_winit_window_Window_set_title";
  external set_visible: (bool, t) => unit =
    "re_winit_window_Window_set_visible";
};
