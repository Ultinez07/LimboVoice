@echo off
echo Installing dependencies...
pip install -r requirements.txt

echo.
echo Installing PyInstaller...
pip install pyinstaller

echo.
echo Building Limbo Voice.exe...
pyinstaller --onefile --windowed --name "LimboVoice" --icon=NONE limbo_voice.py

echo.
echo Done! Your .exe is in the 'dist' folder
echo Run dist\LimboVoice.exe to test!
pause
