# HtwCleanCoderInRust
- [x] ## Main.java
  - [x] MessageReceiver_part
- [ ] ## HtwFactory.java
  - it's for creating game instances
  - probably not necessary
  - will find a suitable way for rust
- [x] ## HuntTheWumpusGame.java
  - [x] class Connection
  - [x] member variables
  - [x] functions
    - [x] HuntTheWumpus functions
      - [ ] figure out how to implement GameCommand
      - [x] cannot mutate Game's members from Command, or use functions; figure out a different way
    - [x] game unique functions (2 more to go[move_wumpus, randomly_transport_player])
- [x] ## HtwMessageReceiver.java
- [x] ## HuntTheWumpus.java
  - [x] imple
  - [x] it's strange it depends on HuntTheWumpusGame
    - [x] program worked without depending on HuntTheWumpusGame
      ```java
        HuntTheWumpus.Command makeRestCommand();
        HuntTheWumpus.Command makeShootCommand(Direction direction);
        HuntTheWumpus.Command makeMoveCommand(Direction direction);
      ```
- [x] ## Move functions to lib.rs
# Todos
- [ ] Rustのモジュール管理勉強
- [ ] HTWのファイルわけ
- [x] HTWのテストコード
- [x] HTWをrust向けにリファクタ
- [ ] learn how to mock functions in tests
- [ ] learn the difference between String::from and to_string
- [x] Define types for caverns & others
