# agile_cli
kanban board command line application

Users can create multiple boards; custom column layouts (pending); add, remove and block cards (pending)

## Commands:

- **list** - list all boards
- **select** - select board
- **view** - display the details of a card
- **new** - create new board, column or card
- **block** - block a card
- **delete** - delect a board, column of card
- **help** - prints this information or the help of a given command

For full usage details, see [Commands](docs/commands.md). 

For details on error and warning messages see [Errors and Warnings](docs/errors-and-warnings.md)


## Dependencies

- **clap** - command line parser
- **serde, serde_json** - JSON serializer, use to save the state of the boards

