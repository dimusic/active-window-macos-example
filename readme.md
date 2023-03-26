A small example of an MacOS app that prints currently active window in a loop once every second.

It just starts NSApplication and subscribes to the [NSWorkspaceDidActivateApplicationNotification](https://developer.apple.com/documentation/appkit/nsworkspacedidactivateapplicationnotification)
to receive updates.
