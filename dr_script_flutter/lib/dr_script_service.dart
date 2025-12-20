import 'dart:async';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';
import 'package:ffi/ffi.dart';

// --- ТИПЫ FFI ---
typedef PrintCallbackC = Void Function(Pointer<Utf8>);
typedef InputRequestCallbackC = Void Function(Pointer<Utf8>);
typedef SubmitInputC = Void Function(Pointer<Utf8>);
typedef SubmitInputDart = void Function(Pointer<Utf8>);

typedef RunDrScriptC = Void Function(
    Pointer<Utf8>,
    Pointer<NativeFunction<PrintCallbackC>>,
    Pointer<NativeFunction<InputRequestCallbackC>>);

class DrScriptService {
  static SendPort? _isolateSendPort;
  static Isolate? _activeIsolate;

  // Храним ссылку на функцию отправки ввода в ГЛАВНОМ изоляте
  static SubmitInputDart? _mainThreadSubmitInputFn;

  // Контроллер для передачи сообщений в UI
  static final StreamController<dynamic> _controller = StreamController.broadcast();
  static Stream<dynamic> get events => _controller.stream;

  /// Загрузка библиотеки (общая логика)
  static DynamicLibrary _loadLibrary() {
    if (Platform.isLinux) {
      // Пытаемся найти библиотеку в разных местах для Linux
      final exePath = File(Platform.resolvedExecutable).parent.path;
      if (File("$exePath/lib/libdr_script.so").existsSync()) return DynamicLibrary.open("$exePath/lib/libdr_script.so");
      if (File("$exePath/libdr_script.so").existsSync()) return DynamicLibrary.open("$exePath/libdr_script.so");
      return DynamicLibrary.open('./libdr_script.so');
    } else if (Platform.isWindows) {
      return DynamicLibrary.open('dr_script.dll');
    } else {
      return DynamicLibrary.open('libdr_script.dylib');
    }
  }

  /// Запуск кода
  static Future<void> runCode(String code) async {
    // 1. Инициализируем FFI в главном потоке для ввода, если еще не сделали
    if (_mainThreadSubmitInputFn == null) {
      try {
        final dylib = _loadLibrary();
        _mainThreadSubmitInputFn = dylib.lookupFunction<SubmitInputC, SubmitInputDart>('submit_input');
      } catch (e) {
        _controller.add({'type': 'print', 'data': 'Error loading lib in Main: $e\n'});
        return;
      }
    }

    // 2. Убиваем старый изолят
    if (_activeIsolate != null) {
      _activeIsolate!.kill(priority: Isolate.immediate);
      _activeIsolate = null;
      _isolateSendPort = null;
    }

    // 3. Запускаем новый
    await _spawnIsolate();

    // 4. Отправляем код на выполнение
    _isolateSendPort!.send({'type': 'run', 'code': code});
  }

  /// Отправка ввода пользователя (ВЫЗЫВАЕТСЯ ПРЯМО ИЗ UI)
  static void sendInput(String text) {
    if (_mainThreadSubmitInputFn != null) {
      final textPtr = text.toNativeUtf8();
      // Вызываем C-функцию напрямую из главного потока.
      // Так как в Rust используется Mutex/LazyStatic, это безопасно и разблокирует изолят.
      _mainThreadSubmitInputFn!(textPtr);
      malloc.free(textPtr);
    }
  }

  /// Инициализация изолята
  static Future<void> _spawnIsolate() async {
    final receivePort = ReceivePort();
    final completer = Completer<SendPort>();

    receivePort.listen((message) {
      if (message is SendPort) {
        if (!completer.isCompleted) completer.complete(message);
      } else {
        _controller.add(message);
      }
    });

    _activeIsolate = await Isolate.spawn(_isolateEntry, receivePort.sendPort);
    _isolateSendPort = await completer.future;
  }

  // --- КОД ВНУТРИ ИЗОЛЯТА ---

  static SendPort? _globalMainSendPort;

  static void _isolateEntry(SendPort mainSendPort) {
    _globalMainSendPort = mainSendPort;
    final isolateReceivePort = ReceivePort();
    mainSendPort.send(isolateReceivePort.sendPort);

    // 1. Загрузка библиотеки внутри изолята
    DynamicLibrary dylib;
    try {
      dylib = _loadLibrary();
    } catch (e) {
      mainSendPort.send({'type': 'print', 'data': 'Isolate Load Error: $e\n'});
      mainSendPort.send({'type': 'finish'});
      return;
    }

    // 2. Поиск функции запуска
    final runFn = dylib.lookupFunction<RunDrScriptC, void Function(
      Pointer<Utf8>,
      Pointer<NativeFunction<PrintCallbackC>>,
      Pointer<NativeFunction<InputRequestCallbackC>>
    )>('run_dr_script');

    // 3. Слушаем команды от Main (только 'run', т.к. 'input' теперь идет напрямую)
    isolateReceivePort.listen((message) {
      if (message is! Map) return;

      if (message['type'] == 'run') {
        final code = message['code'] as String;
        final codePtr = code.toNativeUtf8();

        try {
          // Это блокирующий вызов!
          runFn(
            codePtr,
            Pointer.fromFunction<PrintCallbackC>(_onPrint),
            Pointer.fromFunction<InputRequestCallbackC>(_onInputReq),
          );
        } catch (e) {
          mainSendPort.send({'type': 'print', 'data': '\nError: $e\n'});
        } finally {
          malloc.free(codePtr);
          mainSendPort.send({'type': 'finish'});
        }
      }
    });
  }

  static void _onPrint(Pointer<Utf8> msg) {
    final str = msg.toDartString();
    _globalMainSendPort?.send({'type': 'print', 'data': str});
  }

  static void _onInputReq(Pointer<Utf8> prompt) {
    final str = prompt.toDartString();
    _globalMainSendPort?.send({'type': 'input_req', 'prompt': str});
  }
}