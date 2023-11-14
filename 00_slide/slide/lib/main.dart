import 'package:flutter/material.dart';
import 'package:flutter_math_fork/flutter_math.dart';
import 'package:provider/provider.dart';
import 'package:rinf/rinf.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) => MyAppState(),
      child: MaterialApp(
        title: 'Slide',
        theme: ThemeData(
          useMaterial3: true,
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepOrange),
        ),
        home: MyHomePage(),
      ),
    );
  }
}

class MyAppState extends ChangeNotifier {
  var selectedIndex = 0;
}

class MyHomePage extends StatefulWidget {
  @override
  State<MyHomePage> createState() => _MyHomePage();
}

class _MyHomePage extends State<MyHomePage> {
  Widget page = TitlePage();

  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();
    var selectedIndex = appState.selectedIndex;

    switch (selectedIndex) {
      case 0:
        page = TitlePage();
        break;
      case 1:
        page = WhatisRenderingPage();
        break;
      case 2:
        page = HowtoRenderingPage();
        break;
      case 3:
        page = SignedDistanceFunctionPage();
        break;
      case 4:
        page = WhatisRayMarchingPage();
        break;
      case 5:
        page = ExamplePage();
        break;
      case 6:
        page = ExplainPage();
        break;
      case 7:
        page = TorusFormulaPage();
        break;
    }

    return Scaffold(
      body: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          SafeArea(
              child: NavigationRail(
            extended: false,
            destinations: [
              NavigationRailDestination(
                icon: Icon(Icons.home),
                label: Text('Title'),
              ),
              NavigationRailDestination(
                  icon: Icon(Icons.abc), label: Text('What is rendering?')),
              NavigationRailDestination(
                  icon: Icon(Icons.arrow_forward_ios),
                  label: Text('How to rendering?')),
              NavigationRailDestination(
                  icon: Icon(Icons.arrow_forward_ios),
                  label: Text('Signed Distance Function')),
              NavigationRailDestination(
                  icon: Icon(Icons.arrow_forward_ios),
                  label: Text('What is ray marching?')),
              NavigationRailDestination(
                  icon: Icon(Icons.arrow_forward_ios), label: Text('Example')),
              NavigationRailDestination(
                  icon: Icon(Icons.arrow_forward_ios), label: Text('Explain')),
              NavigationRailDestination(
                  icon: Icon(Icons.arrow_forward_ios),
                  label: Text('Torus formula')),
            ],
            selectedIndex: selectedIndex,
            onDestinationSelected: (index) {
              setState(() {
                appState.selectedIndex = index;
              });
            },
          )),
          Expanded(
              child: Column(
            children: [
              Expanded(
                  child: Container(
                alignment: Alignment.center,
                child: page,
              )),
              Row(children: [
                IconButton(
                    onPressed: () {
                      setState(() {
                        if (appState.selectedIndex > 0) {
                          appState.selectedIndex--;
                        }
                      });
                    },
                    icon: Icon(Icons.arrow_back_ios)),
                Spacer(),
                IconButton(
                    onPressed: () {
                      setState(() {
                        if (appState.selectedIndex < 7) {
                          appState.selectedIndex++;
                        }
                      });
                    },
                    icon: Icon(Icons.arrow_forward_ios)),
              ]),
            ],
          ))
        ],
      ),
    );
  }
}

class TitlePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    // ↓ Add this.
    final style = theme.textTheme.displayLarge!.copyWith(
      color: theme.colorScheme.onPrimary,
    );

    return Container(
        child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
      Spacer(),
      Card(
        color: theme.colorScheme.primary,
        child: Padding(
          padding: const EdgeInsets.all(64.0),
          child: Text('Ray Marching 을 이용한 3D 렌더링', style: style),
        ),
      ),
      Spacer(),
      Row(
        mainAxisAlignment: MainAxisAlignment.end,
        children: [
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Column(
              children: [
                Text(
                  '한남대학교 수학과',
                  style: theme.textTheme.displaySmall,
                ),
                Text(
                  '20172581 김남훈',
                  style: theme.textTheme.displaySmall,
                ),
              ],
            ),
          ),
        ],
      )
    ]));
  }
}

class WhatisRenderingPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    // ↓ Add this.
    final style = theme.textTheme.displayMedium!.copyWith(
      color: theme.colorScheme.onPrimary,
    );

    return Container(
      child: Text('what is rendering?'),
    );
  }
}

class HowtoRenderingPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text('how to rendering'),
    );
  }
}

class SignedDistanceFunctionPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text('signed distance function'),
    );
  }
}

class WhatisRayMarchingPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text('what is ray marching?'),
    );
  }
}

class ExamplePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text('example'),
    );
  }
}

class ExplainPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text('explain'),
    );
  }
}

class TorusFormulaPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text('torus formula'),
    );
  }
}

class MarchingCodePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text('marching code'),
    );
  }
}

class RenderCodePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text('render code'),
    );
  }
}

class BorderStylePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text('border style'),
    );
  }
}

class DistanceStylePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text('distance style'),
    );
  }
}

//sdafasdsdaf