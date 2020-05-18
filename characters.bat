tasklist /nh /fi "WINDOWTITLE eq devwarrior" | find /i "eqgame.exe" > nul ||(start "devwarrior" /d "C:\games\Everquest\TGCALTS\everquest_rof2" "C:\games\Everquest\TGCALTS\everquest_rof2\eqgame.exe" patchme -h /login:devwarrior)
tasklist /nh /fi "WINDOWTITLE eq devcleric" | find /i "eqgame.exe" > nul ||(start "devcleric" /d "C:\games\Everquest\TGCMAIN\everquest_rof2" "C:\games\Everquest\TGCMAIN\everquest_rof2\eqgame.exe" patchme -h /login:devcleric)
tasklist /nh /fi "WINDOWTITLE eq devshaman" | find /i "eqgame.exe" > nul ||(start "devshaman" /d "C:\games\Everquest\TGCALTS\everquest_rof2" "C:\games\Everquest\TGCALTS\everquest_rof2\eqgame.exe" patchme -h /login:devshaman)
tasklist /nh /fi "WINDOWTITLE eq devenchanter" | find /i "eqgame.exe" > nul ||(start "devenchanter" /d "C:\games\Everquest\TGCMAIN\everquest_rof2" "C:\games\Everquest\TGCMAIN\everquest_rof2\eqgame.exe" patchme -h /login:devenchanter)
tasklist /nh /fi "WINDOWTITLE eq devbard" | find /i "eqgame.exe" > nul ||(start "devbard" /d "C:\games\Everquest\TGCALTS\everquest_rof2" "C:\games\Everquest\TGCALTS\everquest_rof2\eqgame.exe" patchme -h /login:devbard)
PING localhost -n 5 >NUL
tasklist /nh /fi "WINDOWTITLE eq devsk" | find /i "eqgame.exe" > nul ||(start "devsk" /d "C:\games\Everquest\TGCALTS\everquest_rof2" "C:\games\Everquest\TGCALTS\everquest_rof2\eqgame.exe" patchme -h /login:devsk)
tasklist /nh /fi "WINDOWTITLE eq devpaladin" | find /i "eqgame.exe" > nul ||(start "devpaladin" /d "C:\games\Everquest\TGCMAIN\everquest_rof2" "C:\games\Everquest\TGCMAIN\everquest_rof2\eqgame.exe" patchme -h /login:devpaladin)
tasklist /nh /fi "WINDOWTITLE eq devdruid" | find /i "eqgame.exe" > nul ||(start "devdruid" /d "C:\games\Everquest\TGCALTS\everquest_rof2" "C:\games\Everquest\TGCALTS\everquest_rof2\eqgame.exe" patchme -h /login:devdruid)
tasklist /nh /fi "WINDOWTITLE eq devmage" | find /i "eqgame.exe" > nul ||(start "devmage" /d "C:\games\Everquest\TGCALTS\everquest_rof2" "C:\games\Everquest\TGCALTS\everquest_rof2\eqgame.exe" patchme -h /login:devmage)
tasklist /nh /fi "WINDOWTITLE eq devwiz" | find /i "eqgame.exe" > nul ||(start "devwiz" /d "C:\games\Everquest\TGCALTS\everquest_rof2" "C:\games\Everquest\TGCALTS\everquest_rof2\eqgame.exe" patchme -h /login:devwiz)
PING localhost -n 5 >NUL
tasklist /nh /fi "WINDOWTITLE eq devmonk" | find /i "eqgame.exe" > nul ||(start "devmonk" /d "C:\games\Everquest\TGCALTS\everquest_rof2" "C:\games\Everquest\TGCALTS\everquest_rof2\eqgame.exe" patchme -h /login:devmonk)
tasklist /nh /fi "WINDOWTITLE eq devnecro" | find /i "eqgame.exe" > nul ||(start "devnecro" /d "C:\games\Everquest\TGCALTS\everquest_rof2" "C:\games\Everquest\TGCALTS\everquest_rof2\eqgame.exe" patchme -h /login:devnecro)
PING localhost -n 10 >NUL
powershell "$process=GET-PROCESS eqgame; foreach ($i in $process) {$i.ProcessorAffinity=16777215}"
exit
