initSidebarItems({"enum":[["ClipboardContent","Types of Clipboard contents"],["ClipboardEvent","The event clipboard type"],["MouseButton","Defines Mouse buttons"],["MouseWheel","Event direction with Mousewheel event"],["Scheme","Set the app scheme"]],"fn":[["abi_version","Gets FLTK ABI version"],["add_handler","Adds a custom handler for unhandled events"],["add_idle","Add an idle callback to run within the event loop. Calls to `WidgetExt::redraw` within the callback require an explicit sleep"],["add_idle2","Add an idle callback to run within the event loop. Calls to `WidgetExt::redraw` within the callback require an explicit sleep"],["add_system_handler","Add a system handler"],["add_timeout","Adds a one-shot timeout callback. The timeout duration `tm` is indicated in seconds Example:"],["add_timeout2","Adds a one-shot timeout callback. The timeout duration `tm` is indicated in seconds Example:"],["api_version","Gets FLTK API version"],["awake","Trigger event loop handling in the main thread"],["awake_callback","Registers a function that will be called by the main thread during the next message handling cycle"],["awake_msg","Sends a custom message"],["background","Set the background color"],["background2","Set the background color for input and text widgets"],["belowmouse","Gets the widget that is below the mouse cursor. This returns an Option which can be specified in the function call"],["channel","Creates a channel returning a Sender and Receiver structs (mpsc)"],["check","Calling this during a big calculation will keep the screen up to date and the interface responsive."],["clipboard_contains","Check the contents of the clipboard"],["close_display","Close the current display"],["crate_version","Gets FLTK crate version"],["damage","Returns whether any of the widgets were damaged"],["delete_widget","Deletes widgets and their children."],["display","Gets the display global variable, `fl_display`. `_XDisplay` on X11, `HINSTANCE` on Windows."],["dnd","Initiate dnd action"],["draw_frame_active","Determines if the currently drawn box is active or inactive"],["enable_locks","Enable locks. This is called automatically in the beginning of the app initialization"],["event","Returns the latest captured event"],["event_button","Returns the captured button event. 1 for left key, 2 for middle, 3 for right"],["event_clicks","Returns false for a single click and true for more"],["event_clicks_num","Returns the number of clicks - 1"],["event_clipboard","Get the clipboard content type"],["event_clipboard_image","Get the clipboard content if it’s an image"],["event_coords","Returns the x and y coordinates of the captured event"],["event_dispatch","The event dispatch function is called after native events are converted to FLTK events, but before they are handled by FLTK. If the dispatch function handler is set, it is up to the dispatch function to call `app::handle2(Event, WindowPtr)` or to ignore the event."],["event_dx","Returns the current horizontal mouse scrolling associated with the Mousewheel event. Returns `MouseWheel::None`, `Right` or `Left`"],["event_dy","Returns the current horizontal mouse scrolling associated with the Mousewheel event. Returns `MouseWheel::None`, `Up` or `Down`. Doesn’t indicate scrolling direction which depends on system preferences"],["event_inside","Returns whether an event occured within a region"],["event_inside_widget","Returns whether an event occured within a widget"],["event_is_click","Determines whether an event was a click"],["event_key","Returns the presed key"],["event_key_down","Returns whether the  key is pressed or held down during the last event"],["event_length","Returns the duration of an event"],["event_mouse_button","Returns the captured button event"],["event_original_key","Returns the original key"],["event_state","Returns the state of the event"],["event_text","Returns a textual representation of the latest event"],["event_x","Gets the x coordinate of the mouse in the window"],["event_x_root","Gets the x coordinate of the mouse in the screen"],["event_y","Gets the y coordinate of the mouse in the window"],["event_y_root","Gets the y coordinate of the mouse in the screen"],["first_window","Returns the first window of the application"],["flush","Causes all the windows that need it to be redrawn and graphics forced out through the pipes."],["focus","Gets the widget which has focus"],["font_count","Gets the number of loaded fonts"],["font_index","Finds the index of a font through its name"],["font_name","Gets the name of a font through its index"],["fonts","Gets a Vector of loaded fonts"],["foreground","Set the foreground color"],["frame_border_radius_max","Get the max border radius for frame types"],["frame_color","Fl::box_color. Gets the current frame color within box/frame drawing mode"],["frame_shadow_width","Get the shadwo width for frames types with shadows"],["frame_type","Get the app’s frame type"],["get_font","Get the font’s name"],["get_font_name","Get the font’s name"],["get_font_names","Returns a list of available fonts to the application"],["get_font_sizes","Get a font’s sizes"],["get_mouse","Gets the mouse coordinates relative to the screen"],["get_system_colors","Gets the system colors"],["grab","Get the grabbed window"],["graphics_context","Get the graphics context, `fl_gc`"],["handle","Send a signal to a window. Integral values from 0 to 30 are reserved. Returns Ok(true) if the event was handled. Returns Ok(false) if the event was not handled. Returns Err on error or in use of one of the reserved values."],["handle_main","Send a signal to the main window. Integral values from 0 to 30 are reserved. Returns Ok(true) if the event was handled. Returns Ok(false) if the event was not handled."],["handle_raw","Send a signal to a window pointer from event_dispatch. Returns true if the event was handled. Returns false if the event was not handled or ignored."],["has_idle","Checks whether an idle function is installed"],["has_idle2","Checks whether an idle function is installed"],["has_timeout","Check whether a timeout is installed"],["has_timeout2","Check whether a timeout is installed"],["init_all","Inits all styles, fonts and images available to FLTK. Also initializes global locking"],["is_event_alt","Returns whether the event is a alt key press"],["is_event_command","Returns whether the event is a command key press"],["is_event_ctrl","Returns whether the event is a control key press"],["is_event_shift","Returns whether the event is a shift press"],["lock","Locks the main UI thread"],["menu_linespacing","Get the default menu linespacing"],["next_window","Returns the next window in order"],["open_display","Open the current display"],["own_colormap","Makes FLTK use its own colormap. This may make FLTK display better"],["paste","Pastes content from the clipboard"],["paste_image","Pastes image content from the clipboard"],["paste_text","Pastes textual content from the clipboard"],["program_should_quit","Determines whether a program should quit"],["pushed","Gets the widget which was pushed"],["quit","Quit the app"],["ready","This is similar to app::check() except this does not call app::flush() or any callbacks, which is useful if your program is in a state where such callbacks are illegal."],["redraw","Redraws everything"],["release","Unset the currently grabbed window"],["reload_scheme","Reload the app scheme"],["remove_idle","Remove an idle function"],["remove_idle2","Remove an idle function"],["remove_system_handler","Add a system handler"],["remove_timeout","Removes a timeout callback"],["remove_timeout2","Removes a timeout callback"],["repeat_timeout","Repeats a timeout callback from the expiration of the previous timeout. You may only call this method inside a timeout callback. The timeout duration `tm` is indicated in seconds Example:"],["repeat_timeout2","Repeats a timeout callback from the expiration of the previous timeout. You may only call this method inside a timeout callback. The timeout duration `tm` is indicated in seconds Example:"],["run","Runs the event loop"],["scheme","Gets the scheme of the application"],["screen_coords","Returns a pair of the x & y coords of the screen"],["screen_count","Get the screen count"],["screen_dpi","Get a screen’s dpi resolution"],["screen_num","Get the screen number based on its coordinates"],["screen_scale","Get the screen scale"],["screen_scaling_supported","Return whether scaling the screen is supported"],["screen_size","Returns a pair of the width and height of the screen"],["screen_work_area","Get a screen’s working area"],["screen_xywh","Get a screen’s xywh"],["scrollbar_size","Get the app’s scrollbar size"],["set_background2_color","Set the background color for input and text widgets"],["set_background_color","Set the background color"],["set_callback","Sets the callback of a widget"],["set_color","Swap a color with a custom RGB value"],["set_damage","Sets the damage to true or false, illiciting a redraw by the application"],["set_focus","Sets the widget which has focus at the start of the application"],["set_font","Set the app’s font"],["set_font_size","Set the app’s font size"],["set_fonts","Initializes loaded fonts of a certain pattern `name`"],["set_foreground_color","Set the foreground color"],["set_frame_border_radius_max","Set the max border radius for frame types"],["set_frame_color","Fl::set_box_color. Sets the current frame color within box/frame drawing mode"],["set_frame_shadow_width","Set the shadwo width for frames types with shadows"],["set_frame_type","Set the app’s default frame type"],["set_frame_type2","Set the app’s default frame type without storing the old type"],["set_frame_type_cb","Set the app’s default frame type"],["set_grab","Set the current grab"],["set_inactive_color","Sets the app’s default selection color"],["set_menu_linespacing","Set the menu linespacing"],["set_raw_callback","Set a widget callback using a C style API"],["set_scheme","sets the scheme of the application"],["set_screen_scale","Set the screen scale"],["set_scrollbar_size","Set the application’s scrollbar size"],["set_selection_color","Sets the app’s default selection color"],["set_visible_focus","Show focus around widgets"],["set_visual","Sets the visual mode of the application"],["should_program_quit","Returns whether a quit signal was sent"],["sleep","Put the thread to sleep for `dur` seconds"],["swap_frame_type","Swap the default frame type with a new frame type"],["thread_msg","Receives a custom message"],["unlock","Unlocks the main UI thread"],["version","Gets FLTK version"],["visible_focus","Return whether visible focus is shown"],["wait","Starts waiting for events. Calls to redraw within wait require an explicit sleep"],["wait_for","Waits a maximum of `dur` seconds or until “something happens”. Returns true if an event happened (always true on windows). Returns false if nothing happened."],["windows","Returns the apps windows."]],"struct":[["App","Basic Application struct, used to instatiate, set the scheme and run the event loop"],["Receiver","Creates a receiver struct"],["Sender","Creates a sender struct"]],"type":[["AppScheme","Alias Scheme to `AppScheme`"],["Display","The display global variable, `fl_display`. `_XDisplay` on X11, `HINSTANCE` on Windows."],["GraphicsContext","The current graphics context of the app, `fl_gc`. `*mut c_void` to `HDC` on Windows, `CGContextRef` on macOS, `_XGC` on X11"],["WidgetPtr","Alias Widget ptr"],["WindowPtr","Alias Window ptr"]]});