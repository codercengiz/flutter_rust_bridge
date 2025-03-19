import 'package:flutter/material.dart';
import 'package:flutter_via_create/src/rust/api/simple.dart';
import 'package:flutter_via_create/src/rust/frb_generated.dart';

MyStruct? myStruct;
Future<void> main() async {
  await RustLib.init();
  myStruct = MyStruct();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Column(
            children: [
              ElevatedButton(
                  onPressed: () {
                    print(
                        'My Struct is disposed: ${myStruct?.data.isDisposed}');
                    printJson(myStruct!);
                    print(
                        'My Struct is disposed: ${myStruct?.data.isDisposed} after printJson');
                    printJson(myStruct!);
                  },
                  child: Text('Test')),
            ],
          ),
        ),
      ),
    );
  }
}

String? printJson(MyStruct struct) {
  try {
    print(struct.toJson());
  } catch (e) {
    print(e);
    return null;
  }
}
