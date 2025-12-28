1. Make subcommand helpers more consistent, right now some print, while some return data for run() to print, etc. I'd prefer a single pattern
2. handle file corruption edge cases
On read_cmd_file_contents need to handle:
    1. If file is empty
    2. If json is invalid
3. Add some sort of rollback on file update failure