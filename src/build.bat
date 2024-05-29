cd c:\rbhome\rust\rblib\src
xcopy c:\cprogs\rblib-rs\src\*.rs   . /D /V /C /F /Y
cd ..
xcopy c:\cprogs\rblib-rs\cargo.toml . /D /V /C /F /Y
cargo build
pause
