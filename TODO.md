1. Make subcommand helpers more consistent, right now some print, some return data, etc.
2. handle file corruption edge cases
On read_cmd_file_contents handle:
    1. If file is empty
    2. If json is invali
3. Add some sort of rollback on file update failure