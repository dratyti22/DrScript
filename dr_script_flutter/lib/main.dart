import 'dart:io';
import 'package:flutter/material.dart';
import 'package:code_text_field/code_text_field.dart';
import 'package:flutter/services.dart';
import 'package:google_fonts/google_fonts.dart';
import 'dr_script_service.dart';
import "languages/dr_script.dart";

void main() {
  runApp(const DrScriptIDE());
}

// --- ЦВЕТОВАЯ ПАЛИТРА ---
class AppColors {
  static const bgDark = Color(0xFF1E1E1E);
  static const bgPanel = Color(0xFF252526);
  static const bgHeader = Color(0xFF333333);
  static const accent = Color(0xFF007ACC);
  static const runBtn = Color(0xFF4CAF50);
  static const textMain = Color(0xFFCCCCCC);
  static const textDim = Color(0xFF858585);
  static const border = Color(0xFF3E3E42);
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
  List<EditorTab> tabs = [];
  int _activeTabIndex = -1;

  // ТЕКУЩАЯ РАБОЧАЯ ПАПКА (откуда запущено приложение)
  final Directory currentDir = Directory.current;
  List<FileSystemEntity> filesInDir = [];

  String output = "";
  bool isRunning = false;
  bool isExplorerVisible = true;
  bool isWaitingForInput = false;
  String inputPrompt = "";
  final TextEditingController _inputController = TextEditingController();
  final FocusNode _inputFocusNode = FocusNode();

  @override
  void initState() {
    super.initState();
    _refreshFileList(); // Загружаем список файлов при старте

    // Слушатель событий от Rust
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

  // --- РАБОТА С ФАЙЛОВОЙ СИСТЕМОЙ ---

  void _refreshFileList() {
    setState(() {
      try {
        // Получаем список файлов и сортируем: папки сверху, файлы снизу
        filesInDir = currentDir.listSync();
        filesInDir.sort((a, b) {
          return a.path.compareTo(b.path);
        });
      } catch (e) {
        print("Error reading directory: $e");
      }
    });
  }

  // Создание нового файла через диалог
  Future<void> _showCreateFileDialog() async {
    String newFileName = "";
    await showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text("Новый файл"),
        backgroundColor: AppColors.bgPanel,
        content: TextField(
          autofocus: true,
          decoration: const InputDecoration(
            hintText: "example.dr",
            enabledBorder: UnderlineInputBorder(
                borderSide: BorderSide(color: AppColors.accent)),
          ),
          onChanged: (value) => newFileName = value,
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text("Отмена"),
          ),
          ElevatedButton(
            style: ElevatedButton.styleFrom(backgroundColor: AppColors.accent),
            onPressed: () {
              Navigator.pop(context);
              if (newFileName.isNotEmpty) {
                _createNewFile(newFileName);
              }
            },
            child: const Text("Создать"),
          ),
        ],
      ),
    );
  }

  void _createNewFile(String name) async {
    final file = File('${currentDir.path}/$name');
    if (!await file.exists()) {
      await file.writeAsString(""); // Создаем пустой файл
      _refreshFileList(); // Обновляем проводник
      _openFile(file.path, name); // Сразу открываем его
    } else {
      // Если файл есть, просто открываем
      _openFile(file.path, name);
    }
  }

  // Открытие существующего файла
  Future<void> _openFile(String path, String name) async {
    // 1. Проверяем, не открыт ли уже
    int existingIndex = tabs.indexWhere((tab) => tab.filePath == path);
    if (existingIndex != -1) {
      setState(() => _activeTabIndex = existingIndex);
      return;
    }

    // 2. Читаем контент
    final file = File(path);
    String content = "";
    if (await file.exists()) {
      content = await file.readAsString();
    }

    // 3. Создаем вкладку
    final controller = CodeController(
      text: content,
      language: drScript,
    );

    controller.addListener(() {
      final idx = tabs.indexWhere((t) => t.filePath == path);
      if (idx != -1 && !tabs[idx].isDirty) {
        setState(() => tabs[idx].isDirty = true);
      }
    });

    setState(() {
      tabs.add(
          EditorTab(filePath: path, fileName: name, controller: controller));
      _activeTabIndex = tabs.length - 1;
    });
  }

  void _closeTab(int index) {
    setState(() {
      tabs[index].controller.dispose();
      tabs.removeAt(index);
      if (_activeTabIndex >= index) {
        _activeTabIndex = _activeTabIndex - 1;
        if (_activeTabIndex < 0 && tabs.isNotEmpty) _activeTabIndex = 0;
      }
    });
  }

  // Обновленный метод сохранения
  Future<void> _saveCurrentFile({bool showNotification = false}) async {
    if (_activeTabIndex == -1) return;

    final tab = tabs[_activeTabIndex];
    final file = File(tab.filePath);

    try {
      await file.writeAsString(tab.controller.text);

      setState(() {
        tab.isDirty = false;
      });

      if (showNotification) {
        ScaffoldMessenger.of(context).clearSnackBars(); // Убираем старые
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text("Saved: ${tab.fileName}"),
            duration: const Duration(milliseconds: 800),
            backgroundColor: AppColors.accent,
            behavior: SnackBarBehavior.floating,
            width: 200,
          ),
        );
      }
      print("Saved: ${tab.filePath}");
    } catch (e) {
      print("Error saving file: $e");
    }
  }

  void runCode() async {
    if (_activeTabIndex == -1) return;
    await _saveCurrentFile();
    setState(() {
      isRunning = true;
      output = "";
      isWaitingForInput = false;
    });
    // Rust получит реальный путь к файлу, поэтому import сработает!
    await DrScriptService.runCode(tabs[_activeTabIndex].controller.text);
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
    // 1. Определяем комбинации клавиш
    return CallbackShortcuts(
      bindings: {
        // Ctrl + S (Windows/Linux)
        const SingleActivator(LogicalKeyboardKey.keyS, control: true): () {
          _saveCurrentFile(showNotification: true);
        },
        // Cmd + S (macOS)
        const SingleActivator(LogicalKeyboardKey.keyS, meta: true): () {
          _saveCurrentFile(showNotification: true);
        },
      },
      // 2. Focus нужен, чтобы перехватывать нажатия глобально
      child: Focus(
        autofocus: true,
        child: Scaffold(
          body: Column(
            children: [
              _buildToolbar(),
              Expanded(
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    // Внутри метода build -> Column -> Expanded -> Row:

                    ActivityBar(
                      isActive: isExplorerVisible,
                      onToggle: () => setState(
                          () => isExplorerVisible = !isExplorerVisible),
                      onCreateFile:
                          _showCreateFileDialog, // <--- ДОБАВЬ ЭТУ СТРОКУ
                    ),
                    if (isExplorerVisible) ...[
                      SizedBox(
                        width: 220,
                        child: FileExplorerPanel(
                          // Передаем текущие файлы
                          files: filesInDir,
                          // <--- Убедись, что переменная filesInDir доступна (мы её добавляли в прошлом шаге)
                          onFileTap: (file) {
                            String name =
                                file.path.split(Platform.pathSeparator).last;
                            _openFile(file.path, name);
                          },
                        ),
                      ),
                      const VerticalDivider(width: 1),
                    ],
                    Expanded(
                      child: tabs.isEmpty
                          ? const Center(
                              child: Text("Нет открытых файлов",
                                  style: TextStyle(color: Colors.grey)))
                          : CodeEditorPanel(
                              tabs: tabs,
                              activeIndex: _activeTabIndex,
                              onTabSwitch: (index) =>
                                  setState(() => _activeTabIndex = index),
                              onTabClose: (index) => _closeTab(index),
                            ),
                    ),
                    const VerticalDivider(width: 1),
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
        ),
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
          const SizedBox(width: 40),
          const Icon(Icons.code, color: AppColors.accent),
          const SizedBox(width: 10),
          Text("DrScript IDE",
              style:
                  GoogleFonts.inter(fontWeight: FontWeight.bold, fontSize: 16)),
          const Spacer(),
          // Показываем путь активного файла (для отладки)
          if (_activeTabIndex != -1)
            Text(tabs[_activeTabIndex].filePath,
                style: const TextStyle(color: Colors.grey, fontSize: 10)),
          const SizedBox(width: 20),
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

// --- НОВЫЙ ACTIVITY BAR С КНОПКОЙ СОЗДАНИЯ ---
class ActivityBar extends StatelessWidget {
  final bool isActive;
  final VoidCallback onToggle;
  final VoidCallback onCreateFile; // Новый колбэк

  const ActivityBar({
    super.key,
    required this.isActive,
    required this.onToggle,
    required this.onCreateFile,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 50,
      color: const Color(0xFF333333),
      child: Column(
        children: [
          const SizedBox(height: 10),
          // Кнопка переключения проводника
          _buildIconButton(
              icon: Icons.copy_all_outlined,
              isActive: isActive,
              onTap: onToggle),
          const SizedBox(height: 15),

          // Кнопка "Создать файл" (+)
          _buildIconButton(
              icon: Icons.add, isActive: false, onTap: onCreateFile),

          const Spacer(),
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
          border: isActive
              ? const Border(left: BorderSide(color: Colors.white, width: 2))
              : null,
        ),
        child: Icon(icon,
            size: 24, color: isActive ? Colors.white : const Color(0xFF858585)),
      ),
    );
  }
}

// --- УМНЫЙ ПРОВОДНИК (FILE EXPLORER) ---
class FileExplorerPanel extends StatelessWidget {
  final List<FileSystemEntity> files;
  final Function(FileSystemEntity) onFileTap;

  const FileExplorerPanel({
    super.key,
    required this.files,
    required this.onFileTap,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      color: AppColors.bgPanel,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Container(
            padding: const EdgeInsets.all(12),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text("EXPLORER",
                    style: GoogleFonts.inter(
                        fontSize: 11,
                        fontWeight: FontWeight.bold,
                        color: AppColors.textDim)),
                // Можно добавить кнопку обновления списка
                const Icon(Icons.refresh, size: 14, color: Colors.grey),
              ],
            ),
          ),
          Expanded(
            child: ListView.builder(
              itemCount: files.length,
              itemBuilder: (context, index) {
                final file = files[index];
                // Получаем только имя файла
                final name = file.path.split(Platform.pathSeparator).last;
                // Пропускаем скрытые файлы (начинаются с точки)
                if (name.startsWith('.')) return const SizedBox.shrink();

                final isDir = FileSystemEntity.isDirectorySync(file.path);

                return InkWell(
                  onTap: () {
                    if (!isDir) onFileTap(file);
                  },
                  child: Container(
                    padding:
                        const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
                    child: Row(
                      children: [
                        Icon(
                          isDir ? Icons.folder : Icons.description,
                          size: 16,
                          color: isDir
                              ? AppColors.textMain
                              : const Color(0xFFE8D18D),
                        ),
                        const SizedBox(width: 8),
                        Expanded(
                          child: Text(
                            name,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: GoogleFonts.inter(
                                color: AppColors.textMain, fontSize: 13),
                          ),
                        ),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

// ... Остальные классы (CodeEditorPanel, ConsolePanel, StatusBar, EditorTab) без изменений ...
// Скопируй их из прошлого файла, они не меняются.
// Если нужно, я могу скинуть их полным файлом.
class CodeEditorPanel extends StatelessWidget {
  final List<EditorTab> tabs;
  final int activeIndex;
  final Function(int) onTabSwitch;
  final Function(int) onTabClose;

  const CodeEditorPanel({
    super.key,
    required this.tabs,
    required this.activeIndex,
    required this.onTabSwitch,
    required this.onTabClose,
  });

  @override
  Widget build(BuildContext context) {
    if (tabs.isEmpty || activeIndex == -1)
      return Container(color: AppColors.bgDark);
    final codeFont = GoogleFonts.jetBrainsMono(fontSize: 14, height: 1.4);
    final activeTab = tabs[activeIndex];

    return Column(
      children: [
        Container(
          height: 35,
          color: const Color(0xFF252526),
          child: ListView.builder(
            scrollDirection: Axis.horizontal,
            itemCount: tabs.length,
            itemBuilder: (context, index) {
              final tab = tabs[index];
              final isActive = index == activeIndex;
              return GestureDetector(
                onTap: () => onTabSwitch(index),
                child: Container(
                  constraints: const BoxConstraints(minWidth: 120),
                  decoration: BoxDecoration(
                    color: isActive
                        ? const Color(0xFF1E1E1E)
                        : const Color(0xFF2D2D2D),
                    border: Border(
                      top: isActive
                          ? const BorderSide(color: Color(0xFF007ACC), width: 2)
                          : BorderSide.none,
                      right: const BorderSide(color: Colors.black12, width: 1),
                    ),
                  ),
                  padding: const EdgeInsets.symmetric(horizontal: 10),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(Icons.description,
                          size: 14,
                          color:
                              isActive ? const Color(0xFFE8D18D) : Colors.grey),
                      const SizedBox(width: 8),
                      Text(
                        "${tab.fileName}${tab.isDirty ? ' ●' : ''}",
                        style: GoogleFonts.inter(
                            color: isActive ? Colors.white : Colors.grey,
                            fontSize: 13),
                      ),
                      const SizedBox(width: 8),
                      InkWell(
                        onTap: () => onTabClose(index),
                        hoverColor: Colors.white10,
                        borderRadius: BorderRadius.circular(4),
                        child: const Icon(Icons.close,
                            size: 14, color: Colors.white70),
                      ),
                    ],
                  ),
                ),
              );
            },
          ),
        ),
        Expanded(
          child: Container(
            color: const Color(0xFF1E1E1E),
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
                  key: ValueKey(activeTab.filePath),
                  controller: activeTab.controller,
                  textStyle: codeFont,
                  cursorColor: const Color(0xFF007ACC),
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
          Expanded(
            child: Container(
              width: double.infinity,
              padding: const EdgeInsets.all(12),
              color: const Color(0xFF181818),
              child: SingleChildScrollView(
                reverse: true,
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(output,
                        style: GoogleFonts.jetBrainsMono(
                            color: AppColors.textMain, fontSize: 13)),
                    if (isWaitingForInput)
                      Padding(
                        padding: const EdgeInsets.only(top: 8.0),
                        child: Row(
                          children: [
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
              Text("Ready",
                  style: GoogleFonts.inter(
                      color: Colors.white,
                      fontSize: 11,
                      fontWeight: FontWeight.w500)),
            ],
          ),
          Row(
            children: [
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

class EditorTab {
  final String filePath;
  final String fileName;
  final CodeController controller;
  bool isDirty;

  EditorTab({
    required this.filePath,
    required this.fileName,
    required this.controller,
    this.isDirty = false,
  });
}
