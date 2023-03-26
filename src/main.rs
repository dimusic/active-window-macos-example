use std::thread;

use active_win_pos_rs::get_active_window;
use cocoa::appkit::NSApplication;
use cocoa::base::nil;
use objc::declare::ClassDecl;
use objc::rc::StrongPtr;
use objc::runtime::{Class, Object, Sel};
use objc::*;

extern "C" {
    static NSWorkspaceDidActivateApplicationNotification: *mut Object;
}

extern "C" fn application_did_finish_launching(this: &mut Object, _sel: Sel, _notif: *mut Object) {
    println!("Application did finish launching");

    unsafe {
        let workspace: *mut Object = msg_send![class!(NSWorkspace), sharedWorkspace];
        let notification_center: *mut Object = msg_send![workspace, notificationCenter];

        let _: *mut Object = msg_send![notification_center, addObserver:this as *mut Object
                    selector:sel!(workspace_app_activated:)
                        name:NSWorkspaceDidActivateApplicationNotification
                      object:nil];
    };

    thread::spawn(move || loop {
        match get_active_window() {
            Ok(window) => {
                println!("Active window: {:?}", window);
            }
            Err(_e) => {
                println!("No active window");
            }
        };

        thread::sleep(std::time::Duration::from_secs(1));
    });
}

extern "C" fn handle_workspace_app_activated(_this: &mut Object, _sel: Sel, _notif: *mut Object) {
    // Could be empty; Only needed to subscribe to the workspace_app_activated event

    println!("App activated");
}

fn init_app_delegate_class() -> &'static Class {
    let mut decl = ClassDecl::new("AppDelegate", class!(NSObject)).unwrap();

    unsafe {
        decl.add_method(
            sel!(applicationDidFinishLaunching:),
            application_did_finish_launching as extern "C" fn(&mut Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(workspace_app_activated:),
            handle_workspace_app_activated as extern "C" fn(&mut Object, Sel, *mut Object),
        );

        decl.register()
    }
}

fn main() {
    unsafe {
        let cls = init_app_delegate_class();
        let app = NSApplication::sharedApplication(nil);

        let delegate: *mut Object = msg_send![cls, alloc];
        let delegate: *mut Object = msg_send![delegate, init];
        let delegate = StrongPtr::new(delegate);

        let _: () = msg_send![app, setDelegate: delegate];
        let _: () = msg_send![app, run];

        app.run();
    }
}
