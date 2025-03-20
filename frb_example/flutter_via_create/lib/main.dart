import 'package:flutter/material.dart';
import 'package:flutter_via_create/src/rust/frb_api.dart';
import 'package:flutter_via_create/src/rust/frb_generated.dart';

MyParentStruct? myStruct;
Future<void> main() async {
  await RustLib.init();
  myStruct = MyParentStruct();
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
                        'My Struct is disposed: ${myStruct?.field0.isDisposed}');
                    print(
                        'My Struct counter is: ${myStruct?.getData()} after printJson');
                    printJson(myStruct!);
                    print(
                        'My Struct counter is: ${myStruct?.getData()} after printJson');
                    print(
                        'My Struct is disposed: ${myStruct?.field0.isDisposed} after printJson');
                    print(
                        'Two structs are same: ${myStruct?.compareMyStruct(other: MyParentStruct())} ');
                    print(
                        'Two structs are same: ${myStruct?.compareMyStruct(other: MyParentStruct())} ');
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

String? printJson(MyParentStruct struct) {
  try {
    print(struct.toJson());
  } catch (e) {
    print(e);
    return null;
  }
}
