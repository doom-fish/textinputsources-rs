import Carbon
import Foundation

private func inputSource(_ handle: UnsafeMutableRawPointer) -> TISInputSource {
  Unmanaged<TISInputSource>.fromOpaque(handle).takeUnretainedValue()
}

private func sourceProperty(_ handle: UnsafeMutableRawPointer, _ key: CFString) -> CFTypeRef? {
  guard let value = TISGetInputSourceProperty(inputSource(handle), key) else {
    return nil
  }
  return Unmanaged<CFTypeRef>.fromOpaque(value).takeUnretainedValue()
}

private func layoutData(_ handle: UnsafeMutableRawPointer) -> CFData? {
  guard
    let value = sourceProperty(handle, kTISPropertyUnicodeKeyLayoutData),
    CFGetTypeID(value) == CFDataGetTypeID()
  else {
    return nil
  }
  return unsafeBitCast(value, to: CFData.self)
}

@_cdecl("textinputsources_current_keyboard_layout")
public func textinputsourcesCurrentKeyboardLayout() -> UnsafeMutableRawPointer? {
  guard let source = TISCopyCurrentKeyboardLayoutInputSource()?.takeRetainedValue() else {
    return nil
  }
  return Unmanaged.passRetained(source).toOpaque()
}

@_cdecl("textinputsources_current_keyboard_input_source")
public func textinputsourcesCurrentKeyboardInputSource() -> UnsafeMutableRawPointer? {
  guard let source = TISCopyCurrentKeyboardInputSource()?.takeRetainedValue() else {
    return nil
  }
  return Unmanaged.passRetained(source).toOpaque()
}

@_cdecl("textinputsources_current_ascii_capable_keyboard_layout")
public func textinputsourcesCurrentASCIICapableKeyboardLayout() -> UnsafeMutableRawPointer? {
  guard let source = TISCopyCurrentASCIICapableKeyboardLayoutInputSource()?.takeRetainedValue()
  else {
    return nil
  }
  return Unmanaged.passRetained(source).toOpaque()
}

@_cdecl("textinputsources_source_release")
public func textinputsourcesSourceRelease(_ handle: UnsafeMutableRawPointer) {
  Unmanaged<TISInputSource>.fromOpaque(handle).release()
}

@_cdecl("textinputsources_source_string")
public func textinputsourcesSourceString(
  _ handle: UnsafeMutableRawPointer,
  _ property: UInt32
) -> UnsafeMutablePointer<CChar>? {
  let key: CFString
  switch property {
  case 0: key = kTISPropertyInputSourceID
  case 1: key = kTISPropertyLocalizedName
  case 2: key = kTISPropertyInputSourceType
  case 3: key = kTISPropertyInputSourceCategory
  default: return nil
  }
  guard let value = sourceProperty(handle, key),
    CFGetTypeID(value) == CFStringGetTypeID()
  else {
    return nil
  }
  let string = unsafeBitCast(value, to: CFString.self)
  let capacity =
    CFStringGetMaximumSizeForEncoding(
      CFStringGetLength(string),
      CFStringBuiltInEncodings.UTF8.rawValue
    ) + 1
  guard capacity > 0, capacity <= 1_048_576,
    let result = malloc(capacity)?.assumingMemoryBound(to: CChar.self)
  else {
    return nil
  }
  guard CFStringGetCString(string, result, capacity, CFStringBuiltInEncodings.UTF8.rawValue) else {
    free(result)
    return nil
  }
  return result
}

@_cdecl("textinputsources_string_free")
public func textinputsourcesStringFree(_ string: UnsafeMutablePointer<CChar>?) {
  free(string)
}

@_cdecl("textinputsources_source_is_ascii_capable")
public func textinputsourcesSourceIsASCIICapable(_ handle: UnsafeMutableRawPointer) -> Bool {
  guard
    let value = sourceProperty(handle, kTISPropertyInputSourceIsASCIICapable),
    CFGetTypeID(value) == CFBooleanGetTypeID()
  else {
    return false
  }
  return CFBooleanGetValue(unsafeBitCast(value, to: CFBoolean.self))
}

@_cdecl("textinputsources_source_is_keyboard_input_mode")
public func textinputsourcesSourceIsKeyboardInputMode(_ handle: UnsafeMutableRawPointer) -> Bool {
  guard
    let value = sourceProperty(handle, kTISPropertyInputSourceType)
  else {
    return false
  }
  return CFEqual(value, kTISTypeKeyboardInputMode)
}

@_cdecl("textinputsources_source_layout_data_length")
public func textinputsourcesSourceLayoutDataLength(_ handle: UnsafeMutableRawPointer) -> Int {
  guard let data = layoutData(handle) else {
    return -1
  }
  return CFDataGetLength(data)
}

@_cdecl("textinputsources_source_copy_layout_data")
public func textinputsourcesSourceCopyLayoutData(
  _ handle: UnsafeMutableRawPointer,
  _ destination: UnsafeMutablePointer<UInt8>,
  _ length: Int
) -> Bool {
  guard let data = layoutData(handle), CFDataGetLength(data) == length,
    let bytes = CFDataGetBytePtr(data)
  else {
    return false
  }
  destination.update(from: bytes, count: length)
  return true
}

@_cdecl("textinputsources_keyboard_type")
public func textinputsourcesKeyboardType() -> UInt32 {
  UInt32(LMGetKbdType())
}

@_cdecl("textinputsources_translate")
public func textinputsourcesTranslate(
  _ handle: UnsafeMutableRawPointer,
  _ keyCode: UInt16,
  _ modifierState: UInt32,
  _ keyboardType: UInt32,
  _ options: UInt32,
  _ deadKeyState: UnsafeMutablePointer<UInt32>,
  _ output: UnsafeMutablePointer<UInt16>,
  _ capacity: Int,
  _ length: UnsafeMutablePointer<Int>
) -> Int32 {
  guard let data = layoutData(handle) else {
    return 1
  }
  guard CFDataGetLength(data) >= MemoryLayout<UCKeyboardLayout>.size,
    let bytes = CFDataGetBytePtr(data)
  else {
    return 2
  }
  guard capacity > 0 else {
    return 3
  }
  var actualLength = 0
  let status = bytes.withMemoryRebound(to: UCKeyboardLayout.self, capacity: 1) { layout in
    UCKeyTranslate(
      layout,
      keyCode,
      UInt16(kUCKeyActionDown),
      modifierState,
      keyboardType,
      options,
      deadKeyState,
      capacity,
      &actualLength,
      output
    )
  }
  guard status == noErr else {
    return status
  }
  guard actualLength >= 0, actualLength < capacity else {
    return 3
  }
  length.pointee = actualLength
  return 0
}
