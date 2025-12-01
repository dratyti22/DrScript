import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

typedef RunDrScriptC = Pointer<Utf8> Function(Pointer<Utf8>);
typedef RunDrScript = Pointer<Utf8> Function(Pointer<Utf8>);

typedef FreeDrStringC = Void Function(Pointer<Utf8>);
typedef FreeDrString = void Function(Pointer<Utf8>);

class DrScriptService {
  static DynamicLibrary? _dylib;
  static RunDrScript? _runScript;
  static FreeDrString? _freeString;

  static bool _initialize() {
    if (_dylib != null) return true;

    try {
      if (Platform.isWindows) {
        _dylib = DynamicLibrary.open('dr_script.dll');
      } else if (Platform.isLinux) {
        _dylib = DynamicLibrary.open('./libdr_script.so');
      } else if (Platform.isMacOS) {
        _dylib = DynamicLibrary.open('./libdr_script.dylib');
      } else {
        return false;
      }

      _runScript = _dylib!.lookupFunction<RunDrScriptC, RunDrScript>('run_dr_script');
      _freeString = _dylib!.lookupFunction<FreeDrStringC, FreeDrString>('free_dr_string');
      
      return true;
    } catch (e) {
      print('Failed to load Dr Script library: $e');
      return false;
    }
  }

  static String runCode(String code) {
    if (!_initialize()) {
      return 'Error: Could not load Dr Script library';
    }

    final codePtr = code.toNativeUtf8();
    
    try {
      final resultPtr = _runScript!(codePtr);
      
      if (resultPtr == nullptr) {
        return 'Error: Failed to execute code';
      }
      
      final result = resultPtr.toDartString();
      _freeString!(resultPtr);
      
      return result;
    } finally {
      malloc.free(codePtr);
    }
  }
}
