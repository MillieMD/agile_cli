# agile-cli Commands

Full explaination of the usage for all commands

**Contents:**
- [list](#list)
- [select](#select)
- [view](#view-not-implemented)
- [new](#new)
- [block](#block)
- [delete](#delete)

---

### List

USAGE: list

Lists saved boards from "temp.txt"

### Select

USAGE: select < board_name >

Sets given board as the active board. 

### View

USAGE: view < card_name >

Displays all info for given board

### New

USAGE: new < subcommand >

Creates a new board, column or card

#### New Board

USAGE: new board < board_name >

#### New Column

USAGE new column < column_name > < location > [ default ] --wip [ wip_limit ]

#### New Card

USAGE: new card < card_name > [ location ]If the list command is showing no results, either temp.txt does not exist, or it exists but it is empty - in either case there are no boards to display

### Block

USAGE: block < card_name >

Sets a cards status as blocked

### Delete

USAGE: delete < subcommand >

#### Delete Board

#### Delete Column

#### Delete Card
