import 'package:flutter/material.dart';
import 'package:code_text_field/code_text_field.dart';
import 'package:google_fonts/google_fonts.dart';
import 'dr_script_service.dart';
import "languages/dr_script.dart";

void main() {
  runApp(const DrScriptIDE());
}

// --- ЦВЕТОВАЯ ПАЛИТРА (VS Code Dark Theme Inspired) ---
class AppColors {
  static const bgDark = Color(0xFF1E1E1E); // Основной фон редактора
  static const bgPanel = Color(0xFF252526); // Фон панелей (Explorer/Terminal)
  static const bgHeader = Color(0xFF333333); // Заголовки вкладок
  static const accent = Color(0xFF007ACC); // Акцентный синий
  static const runBtn = Color(0xFF4CAF50); // Зеленый Run
  static const textMain = Color(0xFFCCCCCC); // Основной текст
  static const textDim = Color(0xFF858585); // Тусклый текст
  static const border = Color(0xFF3E3E42); // Границы
}

class DrScriptIDE extends StatelessWidget {
  const DrScriptIDE({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Dr Script IDE',
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: AppColors.bgDark,
        textTheme: GoogleFonts.interTextTheme(ThemeData.dark().textTheme),
        dividerColor: AppColors.border,
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

  // Состояние видимости панелей
  bool isExplorerVisible = true; // <--- НОВАЯ ПЕРЕМЕННАЯ

  // Переменные для ввода
  bool isWaitingForInput = false;
  String inputPrompt = "";
  final TextEditingController _inputController = TextEditingController();
  final FocusNode _inputFocusNode = FocusNode();

  @override
  void initState() {
    super.initState();
    _codeController = CodeController(
      text: '''ver a = input("Как тебя зовут? ");
print("Привет, " + a);
print("Длина имени: " + len(a));
print("Готово!");''',
      language: drScript,
    );

    DrScriptService.events.listen((event) {
      if (!mounted) return;
      setState(() {
        if (event['type'] == 'print') {
          output += event['data'];
        } else if (event['type'] == 'input_req') {
          isWaitingForInput = true;
          inputPrompt = event['prompt'];
          Future.delayed(const Duration(milliseconds: 100), () {
            if (mounted) _inputFocusNode.requestFocus();
          });
        } else if (event['type'] == 'finish') {
          isRunning = false;
          isWaitingForInput = false;
        }
      });
    });
  }

  void runCode() async {
    setState(() {
      isRunning = true;
      output = "";
      isWaitingForInput = false;
    });
    await DrScriptService.runCode(_codeController.text);
  }

  void _submitInput() {
    if (!isWaitingForInput) return;
    final text = _inputController.text;
    setState(() {
      _inputController.clear();
      isWaitingForInput = false;
    });
    DrScriptService.sendInput(text);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          // Верхняя панель (Toolbar)
          _buildToolbar(),

          Expanded(
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // 1. Activity Bar (Узкая полоска слева)
                ActivityBar(
                  isActive: isExplorerVisible,
                  onToggle: () {
                    setState(() {
                      isExplorerVisible = !isExplorerVisible;
                    });
                  },
                ),

                // 2. Проводник (Показываем только если isExplorerVisible == true)
                if (isExplorerVisible) ...[
                  const SizedBox(width: 220, child: FileExplorerPanel()),
                  const VerticalDivider(width: 1),
                ],

                // 3. Редактор кода (Занимает все свободное место)
                Expanded(child: CodeEditorPanel(controller: _codeController)),
                const VerticalDivider(width: 1),

                // 4. Терминал
                SizedBox(
                  width: 350,
                  child: ConsolePanel(
                    output: output,
                    isRunning: isRunning,
                    isWaitingForInput: isWaitingForInput,
                    inputPrompt: inputPrompt,
                    inputController: _inputController,
                    onInputSubmit: _submitInput,
                    focusNode: _inputFocusNode,
                  ),
                ),
              ],
            ),
          ),

          const StatusBar(),
        ],
      ),
    );
  }

  Widget _buildToolbar() {
    return Container(
      height: 50,
      color: const Color(0xFF3C3C3C),
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Row(
        children: [
          // Логотип сместим немного, так как слева теперь Activity Bar
          const SizedBox(width: 40),
          const Icon(Icons.code, color: AppColors.accent),
          const SizedBox(width: 10),
          Text("DrScript IDE",
              style:
                  GoogleFonts.inter(fontWeight: FontWeight.bold, fontSize: 16)),
          const Spacer(),
          ElevatedButton.icon(
            onPressed: isRunning ? null : runCode,
            style: ElevatedButton.styleFrom(
              backgroundColor: AppColors.runBtn,
              foregroundColor: Colors.white,
              padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
              shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(4)),
            ),
            icon: isRunning
                ? const SizedBox(
                    width: 16,
                    height: 16,
                    child: CircularProgressIndicator(
                        color: Colors.white, strokeWidth: 2))
                : const Icon(Icons.play_arrow, size: 18),
            label: Text(isRunning ? "Running..." : "Run Script",
                style: const TextStyle(fontWeight: FontWeight.w600)),
          ),
        ],
      ),
    );
  }
}

// --- НОВЫЙ ВИДЖЕТ: ACTIVITY BAR (Узкая полоска слева) ---
class ActivityBar extends StatelessWidget {
  final bool isActive;
  final VoidCallback onToggle;

  const ActivityBar({
    super.key,
    required this.isActive,
    required this.onToggle,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 50,
      color: const Color(0xFF333333), // Цвет Activity Bar (как в VS Code)
      child: Column(
        children: [
          const SizedBox(height: 10),
          // Кнопка "Проводник"
          _buildIconButton(
            icon: Icons.copy_all_outlined, // Иконка "файлы"
            isActive: isActive,
            onTap: onToggle,
          ),
          const SizedBox(height: 15),
          // Заглушка для поиска (для красоты)
          _buildIconButton(icon: Icons.search, isActive: false, onTap: () {}),
          const SizedBox(height: 15),
          // Заглушка для git (для красоты)
          _buildIconButton(
              icon: Icons.source_outlined, isActive: false, onTap: () {}),

          const Spacer(),
          // Иконка настроек внизу
          _buildIconButton(
              icon: Icons.settings_outlined, isActive: false, onTap: () {}),
          const SizedBox(height: 10),
        ],
      ),
    );
  }

  Widget _buildIconButton(
      {required IconData icon,
      required bool isActive,
      required VoidCallback onTap}) {
    return InkWell(
      onTap: onTap,
      child: Container(
        width: 50,
        height: 50,
        decoration: BoxDecoration(
          // Слева белая полоска, если активно
          border: isActive
              ? const Border(left: BorderSide(color: Colors.white, width: 2))
              : null,
        ),
        child: Icon(
          icon,
          size: 24,
          // Если активно - белый цвет, если нет - серый
          color: isActive ? Colors.white : const Color(0xFF858585),
        ),
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
    // Тот же шрифт для идеальной синхронизации
    final codeFont = GoogleFonts.jetBrainsMono(fontSize: 14, height: 1.4);

    return Column(
      children: [
        // --- ВЕРХНЯЯ ПАНЕЛЬ ВКЛАДОК ---
        Container(
          height: 35,
          color: const Color(0xFF252526),
          // Темный фон заголовка (как панель Explorer)
          child: Row(
            children: [
              // Активная вкладка "main.dr"
              Container(
                width: 150,
                decoration: const BoxDecoration(
                  color: Color(0xFF1E1E1E),
                  // Цвет фона такой же, как у редактора (сливается)
                  border: Border(
                    top: BorderSide(color: Color(0xFF007ACC), width: 2),
                    // Синяя полоска активности
                    right: BorderSide(
                        color: Color(0xFF252526),
                        width: 1), // Разделитель справа
                  ),
                ),
                padding: const EdgeInsets.symmetric(horizontal: 10),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Row(
                      children: [
                        // Иконка файла (желтоватая для кода)
                        const Icon(Icons.description,
                            size: 14, color: Color(0xFFE8D18D)),
                        const SizedBox(width: 8),
                        Text(
                          "main.dr",
                          style: GoogleFonts.inter(
                            color: Colors.white,
                            // Яркий белый текст (активный файл)
                            fontSize: 13,
                          ),
                        ),
                      ],
                    ),
                    // Кнопка закрытия (крестик) с эффектом при наведении
                    InkWell(
                      onTap: () {
                        // Тут можно добавить логику закрытия в будущем
                        print("Close tab pressed");
                      },
                      hoverColor: Colors.white.withOpacity(0.1),
                      borderRadius: BorderRadius.circular(4),
                      child: const Padding(
                        padding: EdgeInsets.all(2.0),
                        child:
                            Icon(Icons.close, size: 14, color: Colors.white70),
                      ),
                    ),
                  ],
                ),
              ),
              // Пустое место справа от вкладок
            ],
          ),
        ),

        // --- САМ РЕДАКТОР ---
        Expanded(
          child: Container(
            color: const Color(0xFF1E1E1E), // Основной фон редактора
            alignment: Alignment.topLeft, // Код прижат к левому верхнему углу
            child: CodeTheme(
              data: CodeThemeData(styles: {
                'root': codeFont.copyWith(color: const Color(0xFFD4D4D4)),
                'keyword': codeFont.copyWith(color: const Color(0xFFC586C0)),
                'built_in': codeFont.copyWith(color: const Color(0xFFDCDCAA)),
                'literal': codeFont.copyWith(color: const Color(0xFF569CD6)),
                'string': codeFont.copyWith(color: const Color(0xFFCE9178)),
                'comment': codeFont.copyWith(color: const Color(0xFF6A9955)),
                'punctuation':
                    codeFont.copyWith(color: const Color(0xFFFFD700)),
                'operator': codeFont.copyWith(color: const Color(0xFFD4D4D4)),
              }),
              child: SingleChildScrollView(
                child: CodeField(
                  controller: controller,
                  textStyle: codeFont,
                  cursorColor: const Color(0xFF007ACC), // Синий курсор
                  lineNumberStyle: LineNumberStyle(
                    textStyle:
                        codeFont.copyWith(color: const Color(0xFF858585)),
                    width: 45,
                    margin: 20,
                  ),
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}

// --- ВИДЖЕТ: ПРОВОДНИК (EXPLORER) ---
class FileExplorerPanel extends StatelessWidget {
  const FileExplorerPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      color: AppColors.bgPanel,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Container(
            padding: const EdgeInsets.all(12),
            child: Text("EXPLORER",
                style: GoogleFonts.inter(
                    fontSize: 11,
                    fontWeight: FontWeight.bold,
                    color: AppColors.textDim)),
          ),
          _buildItem("main.dr", isSelected: true),
        ],
      ),
    );
  }

  Widget _buildItem(String name,
      {bool isSelected = false, bool isFolder = false}) {
    return Container(
      color: isSelected ? const Color(0xFF37373D) : Colors.transparent,
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
      child: Row(
        children: [
          Icon(isFolder ? Icons.folder : Icons.description,
              size: 16,
              color: isFolder ? AppColors.textDim : const Color(0xFFE8D18D)),
          const SizedBox(width: 8),
          Text(name,
              style: GoogleFonts.inter(
                  color: isSelected ? Colors.white : AppColors.textMain,
                  fontSize: 13)),
        ],
      ),
    );
  }
}

// --- ВИДЖЕТ: ТЕРМИНАЛ (CONSOLE) ---
class ConsolePanel extends StatelessWidget {
  final String output;
  final bool isRunning;
  final bool isWaitingForInput;
  final String inputPrompt;
  final TextEditingController inputController;
  final VoidCallback onInputSubmit;
  final FocusNode focusNode;

  const ConsolePanel({
    super.key,
    required this.output,
    required this.isRunning,
    required this.isWaitingForInput,
    required this.inputPrompt,
    required this.inputController,
    required this.onInputSubmit,
    required this.focusNode,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      color: AppColors.bgDark,
      child: Column(
        children: [
          // Заголовок терминала
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
            color: AppColors.bgPanel,
            width: double.infinity,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text("TERMINAL",
                    style: GoogleFonts.inter(
                        fontSize: 11,
                        fontWeight: FontWeight.bold,
                        color: Colors.white)),
                if (isRunning)
                  const SizedBox(
                      width: 12,
                      height: 12,
                      child: CircularProgressIndicator(
                          strokeWidth: 2, color: AppColors.accent)),
              ],
            ),
          ),
          const Divider(height: 1),

          // Поле вывода
          Expanded(
            child: Container(
              width: double.infinity,
              padding: const EdgeInsets.all(12),
              color: const Color(0xFF181818), // Чуть темнее для контраста
              child: SingleChildScrollView(
                reverse: true, // Автоскролл вниз
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(output,
                        style: GoogleFonts.jetBrainsMono(
                            color: AppColors.textMain, fontSize: 13)),

                    // Поле ввода (появляется только когда нужно)
                    if (isWaitingForInput)
                      Padding(
                        padding: const EdgeInsets.only(top: 8.0),
                        child: Row(
                          children: [
                            // Промпт (текст перед вводом)
                            Text(inputPrompt,
                                style: GoogleFonts.jetBrainsMono(
                                    color: AppColors.accent,
                                    fontWeight: FontWeight.bold,
                                    fontSize: 13)),
                            const SizedBox(width: 8),
                            Expanded(
                              child: TextField(
                                controller: inputController,
                                focusNode: focusNode,
                                style: GoogleFonts.jetBrainsMono(
                                    color: Colors.white,
                                    fontWeight: FontWeight.bold,
                                    fontSize: 13),
                                cursorColor: Colors.white,
                                decoration: const InputDecoration.collapsed(
                                    hintText: ""),
                                onSubmitted: (_) => onInputSubmit(),
                              ),
                            ),
                          ],
                        ),
                      ),
                  ],
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

// --- ВИДЖЕТ: СТАТУС БАР (Внизу) ---
class StatusBar extends StatelessWidget {
  const StatusBar({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 24,
      color: AppColors.accent,
      padding: const EdgeInsets.symmetric(horizontal: 12),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Row(
            children: [
              const Icon(Icons.code, size: 12, color: Colors.white),
              const SizedBox(width: 6),
              Text("main.dr",
                  style: GoogleFonts.inter(
                      color: Colors.white,
                      fontSize: 11,
                      fontWeight: FontWeight.w500)),
            ],
          ),
          Row(
            children: [
              Text("Ln 1, Col 1",
                  style: GoogleFonts.inter(color: Colors.white, fontSize: 11)),
              const SizedBox(width: 15),
              Text("UTF-8",
                  style: GoogleFonts.inter(color: Colors.white, fontSize: 11)),
              const SizedBox(width: 15),
              const Icon(Icons.check, size: 12, color: Colors.white),
            ],
          )
        ],
      ),
    );
  }
}
