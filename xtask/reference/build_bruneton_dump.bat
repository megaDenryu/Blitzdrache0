@echo off
rem Builds the Bruneton reference dumper with MSVC.
rem Args: 1=precomputed_atmospheric_scattering working copy, 2=Blitzdrache0 root, 3=output exe.
rem Comments are ASCII because cmd.exe reads batch files in the console code page.
rem /utf-8 is required: the dumper has Japanese comments and CP932 would mis-read them.
rem The Japanese explanation lives in xtask/src/gen_atmosphere_reference.rs.
setlocal
set VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe
if not exist "%VSWHERE%" (
  echo vswhere.exe not found 1>&2
  exit /b 1
)
for /f "usebackq tokens=*" %%i in (`"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do set VSPATH=%%i
if "%VSPATH%"=="" (
  echo Visual Studio with the C++ toolset not found 1>&2
  exit /b 1
)
call "%VSPATH%\VC\Auxiliary\Build\vcvars64.bat" 1>&2
if errorlevel 1 exit /b 1
cl /nologo /EHsc /O2 /std:c++17 /utf-8 /I "%~1" /I "%~1\external\dimensional_types" /I "%~2" "%~2\xtask\reference\bruneton_dump.cc" "%~1\atmosphere\reference\functions.cc" /Fe:"%~3" /Fo:"%~dp3\" 1>&2
exit /b %errorlevel%
