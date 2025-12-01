import 'package:flutter/material.dart';
import 'package:code_text_field/code_text_field.dart';
import 'package:highlight/languages/javascript.dart';
import 'dr_script_service.dart';

void main() {
  runApp(const DrScriptIDE());
}

class DrScriptIDE extends StatelessWidget {
  const DrScriptIDE({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark(),
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

  @override
  void initState() {
    super.initState();
    _codeController = CodeController(
      text: '''print("Привет, Dr Script!");
var x = 10;
var y = 5;
print(x + y);''',
      language: javascript,
    );
  }

  void runCode() async {
    setState(() {
      output = "🚀 Запуск...\n";
    });

    await Future.delayed(const Duration(milliseconds: 100));
    
    final code = _codeController.text;
    final result = DrScriptService.runCode(code);
    
    setState(() {
      output = result.isEmpty ? "✅ Выполнено" : result;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xff1e1e1e),
      appBar: AppBar(
        backgroundColor: const Color(0xff252526),
        title: const Text("Dr Script IDE"),
        actions: [
          ElevatedButton.icon(
            onPressed: runCode,
            icon: const Icon(Icons.play_arrow),
            label: const Text('Запустить'),
            style: ElevatedButton.styleFrom(
              backgroundColor: Colors.green[700],
              foregroundColor: Colors.white,
            ),
          ),
          const SizedBox(width: 12),
        ],
      ),
      body: Row(
        children: [
          // Файлы
          Container(
            width: 200,
            color: const Color(0xff252526),
            child: const Column(
              children: [
                Padding(
                  padding: EdgeInsets.all(12),
                  child: Text("📁 ФАЙЛЫ", style: TextStyle(fontWeight: FontWeight.bold)),
                ),
                ListTile(
                  leading: Icon(Icons.description, color: Colors.blue),
                  title: Text("main.dr"),
                  dense: true,
                ),
              ],
            ),
          ),
          
          // Редактор
          Expanded(
            child: Container(
              padding: const EdgeInsets.all(8),
              child: CodeField(
                controller: _codeController,
                textStyle: const TextStyle(fontSize: 16),
              ),
            ),
          ),
          
          // Консоль
          Container(
            width: 300,
            color: const Color(0xff252526),
            child: Column(
              children: [
                Container(
                  padding: const EdgeInsets.all(12),
                  child: const Row(
                    children: [
                      Icon(Icons.terminal, size: 16),
                      SizedBox(width: 8),
                      Text("КОНСОЛЬ", style: TextStyle(fontWeight: FontWeight.bold)),
                    ],
                  ),
                ),
                Expanded(
                  child: Container(
                    padding: const EdgeInsets.all(12),
                    child: Text(
                      output.isEmpty ? "Нажмите 'Запустить'..." : output,
                      style: const TextStyle(fontFamily: 'monospace'),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}