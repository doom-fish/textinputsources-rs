import Foundation

public typealias KeyboardSelectionContextFunction =
  @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class KeyboardSelectionCallback {
  private let context: UnsafeMutableRawPointer
  private let callback: KeyboardSelectionContextFunction
  private let retainContext: KeyboardSelectionContextFunction
  private let releaseContext: KeyboardSelectionContextFunction

  init(
    context: UnsafeMutableRawPointer,
    callback: @escaping KeyboardSelectionContextFunction,
    retainContext: @escaping KeyboardSelectionContextFunction,
    releaseContext: @escaping KeyboardSelectionContextFunction
  ) {
    self.context = context
    self.callback = callback
    self.retainContext = retainContext
    self.releaseContext = releaseContext
    retainContext(context)
  }

  deinit {
    releaseContext(context)
  }

  func call() {
    retainContext(context)
    defer { releaseContext(context) }
    callback(context)
  }
}

private final class RegisteredKeyboardSelectionObserver {
  private let center: NotificationCenter
  private let token: NSObjectProtocol

  init(
    callback: @escaping KeyboardSelectionContextFunction,
    context: UnsafeMutableRawPointer,
    retainContext: @escaping KeyboardSelectionContextFunction,
    releaseContext: @escaping KeyboardSelectionContextFunction
  ) {
    center = NotificationCenter.default
    let retainedCallback = KeyboardSelectionCallback(
      context: context,
      callback: callback,
      retainContext: retainContext,
      releaseContext: releaseContext
    )
    token = center.addObserver(
      forName: Notification.Name("NSTextInputContextKeyboardSelectionDidChangeNotification"),
      object: nil,
      queue: nil
    ) { _ in
      retainedCallback.call()
    }
  }

  deinit {
    center.removeObserver(token)
  }
}

@_cdecl("textinputsources_observer_new")
public func textinputsourcesObserverNew(
  _ callback: @escaping KeyboardSelectionContextFunction,
  _ context: UnsafeMutableRawPointer,
  _ retainContext: @escaping KeyboardSelectionContextFunction,
  _ releaseContext: @escaping KeyboardSelectionContextFunction
) -> UnsafeMutableRawPointer {
  Unmanaged.passRetained(
    RegisteredKeyboardSelectionObserver(
      callback: callback,
      context: context,
      retainContext: retainContext,
      releaseContext: releaseContext
    )
  ).toOpaque()
}

@_cdecl("textinputsources_observer_release")
public func textinputsourcesObserverRelease(_ handle: UnsafeMutableRawPointer) {
  Unmanaged<RegisteredKeyboardSelectionObserver>.fromOpaque(handle).release()
}
