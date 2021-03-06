open Winit;

Printexc.record_backtrace(true);

let logEvent: Event.t(unit) => unit =
  fun
  | Event.NewEvents(WaitCancelled(_)) => ()
  | DeviceEvent(_) => ()
  | Suspended => ()
  | Resumed => ()
  | MainEventsCleared => ()
  | RedrawRequested(_) => ()
  | RedrawEventsCleared => ()
  | event => {
      event |> Event.show((_, _) => ()) |> print_endline;
    };

let eventLoop = EventLoop.new_();

let window = Window.new_(eventLoop);

window |> Window.set_title("Ayyyyyy!");

window |> Window.set_resizable(false);

let a = window |> Window.id;

let b = window |> Window.id;

if (Window.WindowId.(a != b)) {
  failwith("WindowId.t equality failed");
};

let handleEvents = (event, _, _controlFlow): ControlFlow.t => {
  event |> logEvent;
  switch (event) {
  | WindowEvent({window_id: _, event}) =>
    switch (event) {
    | KeyboardInput({
        input: {state: Released, virtual_keycode: Some(Escape), _},
        _,
      })
    | CloseRequested => Exit
    | _ => Wait
    }
  | _ => Wait
  };
};

eventLoop |> EventLoop.run(handleEvents);
