CLI tool to manage todo's in project and global

`todo add "description"` - adds a todo
`-d "today"` - add due date `[today | tomorrow | in x days (where x is a whole number) | weekday (where weekday is Mo or Mon or Monday etc.)]`
`-t "12:00"` - add due time 

`todo ls` - to list

`todo open` - to get absolute path to todo file to open it like `todo open | nvim`

`todo finish` - list all todo's with index
`todo finish {index}` - finishes todo

`todo delete` - list all todo's with index
`todo delete {index}` - deletes todo

`todo config` - list current config
`todo config name` - list current name
`todo config name "new_name"` - sets current name
`todo config home "home_path"` - sets new home path
`todo config deleted` - list current deleted config
`todo config deleted {in_file/delete}` - sets current deleted config. in_file: save deleted always to `.todo.deleted`. `delete`: deletes the todo

`todo clear` - asks and clears all `.todo.*` files

`todo create` - creates `.todo.*` files with default config
`todo create -c "config_file"` copies the config file
`todo create -c` ask questions to create config

`-g` access global todo's (configs) in home directory

if no todo config exists in current folder, it walks down the dir-tree until it hits the root. Then it looks into home. If none exists, one will be created in home.

files this program uses:
`.todo.todo` - todo's stored
`.todo.finished` - finished todo's stored
`.todo.deleted` - deleted todo's stored
`.todo.config` - deleted todo's stored

