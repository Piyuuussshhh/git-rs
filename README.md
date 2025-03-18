# gitrs - Git rewritten in Rust. (WIP)
### Steps to create a UTF-8 encoded text file on Windows for testing purposes.
1. Run this in powershell: `"<Your content>" | Out-File -FilePath <Your file path>.txt -Encoding utf8`.
2. You need to click the 'UTF-8 with BOM' tile in the bottom right corner of VSCode and click 'Save with Encoding'. Select utf8.
3. Now you can test gitrs.