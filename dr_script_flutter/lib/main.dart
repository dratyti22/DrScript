import 'package:flutter/material.dart';
import 'package:code_text_field/code_text_field.dart';
import 'package:google_fonts/google_fonts.dart'; // Импорт шрифтов
import 'dr_script_service.dart';
import "languages/dr_script.dart";

void main() {
  runApp(const DrScriptIDE());
}

class DrScriptIDE extends StatelessWidget {
  const DrScriptIDE({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Dr Script IDE',
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color(0xFF1E1E1E),
        // Основной фон VS Code
        appBarTheme: const AppBarTheme(
          backgroundColor: Color(0xFF333333),
          elevation: 0,
        ),
        colorScheme: const ColorScheme.dark(
          primary: Color(0xFF007ACC), // Синий акцент
          secondary: Color(0xFF4CAF50), // Зеленый акцент
          surface: Color(0xFF252526), // Цвет панелей
        ),
        textTheme: GoogleFonts.interTextTheme(ThemeData.dark().textTheme),
      ),
      home: const HomePage(),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  late CodeController _codeController;
  String output = "";
  bool isRunning = false;

  @override
  void initState() {
    super.initState();
    _codeController = CodeController(
      text: '''print("Привет, Dr Script!");
var x = 10;
var y = 5;
print("Сумма: " + (x + y));
for (i=0;i<3;i++) {
  print("Цикл " + i);
}''',
      language: drScript,
    );
  }

  void runCode() async {
    final code = _codeController.text;
    final result =
        DrScriptService.runCode(code); // Получаем результат или отчет об ошибке

    setState(() {
      isRunning = false;

      // Проверяем, начинается ли вывод с маркера REPORT_START:
      if (result.startsWith("REPORT_START:")) {
        // Сохраняем весь отчет (включая маркер)
        output = result;
      } else {
        // Иначе это обычный вывод или Runtime Error (который выводится пока в стандартном формате)
        output = result.isEmpty ? "Программа завершена без вывода." : result;
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: _buildAppBar(),
      body: Column(
        children: [
          Expanded(
            child: Row(
              children: [
                // Левая панель: Файлы
                const SizedBox(
                  width: 250,
                  child: FileExplorerPanel(),
                ),
                // Разделитель
                Container(width: 1, color: Colors.black),
                // Центральная панель: Редактор
                Expanded(
                  child: CodeEditorPanel(controller: _codeController),
                ),
                // Разделитель
                Container(width: 1, color: Colors.black),
                // Правая панель: Консоль
                SizedBox(
                  width: 350,
                  child: ConsolePanel(output: output, isRunning: isRunning),
                ),
              ],
            ),
          ),
          // Нижний статус-бар
          const StatusBar(),
        ],
      ),
    );
  }

  PreferredSizeWidget _buildAppBar() {
    return AppBar(
      titleSpacing: 20,
      title: Row(
        children: [
          const Icon(Icons.code, color: Color(0xFF007ACC)),
          const SizedBox(width: 10),
          Text(
            "Dr Script IDE",
            style: GoogleFonts.jetBrainsMono(
                fontWeight: FontWeight.bold, fontSize: 16),
          ),
        ],
      ),
      actions: [
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
          child: ElevatedButton.icon(
            onPressed: isRunning ? null : runCode,
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF4CAF50),
              foregroundColor: Colors.white,
              elevation: 4,
              shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(8)),
              padding: const EdgeInsets.symmetric(horizontal: 20),
            ),
            icon: isRunning
                ? const SizedBox(
                    width: 16,
                    height: 16,
                    child: CircularProgressIndicator(
                        color: Colors.white, strokeWidth: 2))
                : const Icon(Icons.play_arrow, size: 20),
            label: Text(isRunning ? "Запуск..." : "RUN",
                style: GoogleFonts.jetBrainsMono(fontWeight: FontWeight.bold)),
          ),
        ),
      ],
    );
  }
}

// --- ВИДЖЕТ: ПРОВОДНИК ФАЙЛОВ ---
class FileExplorerPanel extends StatelessWidget {
  const FileExplorerPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      color: const Color(0xFF252526),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.all(10.0),
            child: Text("EXPLORER",
                style: GoogleFonts.inter(
                    fontSize: 11,
                    fontWeight: FontWeight.bold,
                    color: Colors.grey)),
          ),
          // Оставляем только основной файл
          _buildFileItem("main.dr", isActive: true),
          // Вы можете добавить сюда кнопку "Добавить файл" позже
        ],
      ),
    );
  }

  Widget _buildFileItem(String name, {required bool isActive}) {
    return Container(
      color: isActive ? const Color(0xFF37373D) : Colors.transparent,
      child: ListTile(
        dense: true,
        visualDensity: const VisualDensity(vertical: -4),
        leading: Icon(
          Icons.description,
          color: const Color(0xFF4CAF50), // Зеленый для .dr
          size: 18,
        ),
        title: Text(name,
            style: GoogleFonts.inter(
                color: isActive ? Colors.white : Colors.grey[400],
                fontSize: 13)),
        // Убрали onTap, так как сейчас переключения нет
      ),
    );
  }
}

// --- ВИДЖЕТ: РЕДАКТОР КОДА ---
class CodeEditorPanel extends StatelessWidget {
  final CodeController controller;

  const CodeEditorPanel({super.key, required this.controller});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // Tab bar (имитация)
        Container(
          height: 35,
          color: const Color(0xFF1E1E1E),
          child: Row(
            children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 15),
                color: const Color(0xFF1E1E1E), // Активный таб
                alignment: Alignment.center,
                child: Row(
                  children: [
                    const Icon(Icons.description,
                        color: Color(0xFF4CAF50), size: 14),
                    const SizedBox(width: 8),
                    Text("main.dr",
                        style: GoogleFonts.inter(
                            color: Colors.white, fontSize: 13)),
                    const SizedBox(width: 8),
                    const Icon(Icons.close, color: Colors.grey, size: 14),
                  ],
                ),
              ),
              // Верхняя граница таба
              Expanded(child: Container(color: const Color(0xFF252526))),
            ],
          ),
        ),
        // Сам редактор
        Expanded(
          child: Container(
            color: const Color(0xFF1E1E1E),
            child: CodeTheme(
              data: CodeThemeData(styles: {
                'root': TextStyle(
                  fontFamily: GoogleFonts.jetBrainsMono().fontFamily,
                  fontSize: 15,
                  backgroundColor: const Color(0xFF1E1E1E),
                  color: const Color(0xFFD4D4D4),
                ),
                'punctuation': const TextStyle(color: Color(0xFFAFAFAF)),
                'operator': const TextStyle(color: Color(0xFFC586C0)),
                'keyword': const TextStyle(
                    color: Color(0xFF569CD6), fontWeight: FontWeight.bold),
                'function': const TextStyle(color: Color(0xFFDCDCAA)),
                'string': const TextStyle(color: Color(0xFFCE9178)),
                'number': const TextStyle(color: Color(0xFFB5CEA8)),
                'comment': const TextStyle(color: Color(0xFF6A9955)),
                'doctag': const TextStyle(
                    color: Colors.lightBlue, fontStyle: FontStyle.italic),
              }),
              child: SingleChildScrollView(
                child: CodeField(
                  controller: controller,
                  // **ОШИБОЧНЫЙ БЛОК УДАЛЕН**
                  textStyle:
                      GoogleFonts.jetBrainsMono(fontSize: 15, height: 1.3),
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}

// --- ВИДЖЕТ: КОНСОЛЬ ---
class ConsolePanel extends StatelessWidget {
  final String output;
  final bool isRunning;

  const ConsolePanel(
      {super.key, required this.output, required this.isRunning});

  @override
  Widget build(BuildContext context) {
    final isParseError = output.startsWith("REPORT_START:");

    // Определяем текст, который будет отображаться
    final displayText = isParseError
        ? output.replaceFirst("REPORT_START:\n", "") // Удаляем маркер
        : (output.isEmpty ? "Ready to run..." : output);
    return Container(
      color: const Color(0xFF1E1E1E),
      child: Column(
        children: [
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
            decoration: const BoxDecoration(
              border: Border(bottom: BorderSide(color: Colors.black)),
              color: Color(0xFF252526),
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text("TERMINAL",
                    style: GoogleFonts.inter(
                        fontSize: 11,
                        fontWeight: FontWeight.bold,
                        color: Colors.grey)),
                if (isRunning)
                  const SizedBox(
                    width: 10,
                    height: 10,
                    child: CircularProgressIndicator(strokeWidth: 2),
                  ),
              ],
            ),
          ),
          Expanded(
            child: Container(
              width: double.infinity,
              padding: const EdgeInsets.all(12),
              color: const Color(0xFF1E1E1E),
              child: SingleChildScrollView(
                child: SelectableText(
                  displayText, // Отображаем текст без маркера
                  style: GoogleFonts.jetBrainsMono(
                    // Цвет: Красный для ParseError, желтый для Runtime, или белый для вывода
                    color: isParseError
                        ? Colors
                            .redAccent // Красный для отформатированного ParseError
                        : output.startsWith("RUNTIME ERROR:")
                            ? Colors
                                .yellowAccent // Если вы оставили Runtime Error в старом формате
                            : const Color(0xFFCCCCCC),
                    fontSize: 13,
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

// --- ВИДЖЕТ: СТАТУС БАР ---
class StatusBar extends StatelessWidget {
  const StatusBar({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 24,
      color: const Color(0xFF007ACC), // Стандартный синий цвет VS Code
      padding: const EdgeInsets.symmetric(horizontal: 10),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Row(
            children: [
              const Icon(Icons.code_off, size: 12, color: Colors.white),
              const SizedBox(width: 5),
              Text("main.dr",
                  style: GoogleFonts.inter(color: Colors.white, fontSize: 11)),
            ],
          ),
          Row(
            children: [
              Text("UTF-8",
                  style: GoogleFonts.inter(color: Colors.white, fontSize: 11)),
              const SizedBox(width: 15),
              const Icon(Icons.notifications_none,
                  size: 12, color: Colors.white),
            ],
          )
        ],
      ),
    );
  }
}
