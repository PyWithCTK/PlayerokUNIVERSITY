#define AppName "PlayerokUniversity"
#ifndef AppVersion
  #define AppVersion "2.0.0"
#endif

[Setup]
AppId={{6B1D7C52-4A0E-4F0B-9E57-5A1B2C3D4E5F}
AppName={#AppName}
AppVersion={#AppVersion}
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputDir=..\dist
OutputBaseFilename=PlayerokUniversity-Setup-{#AppVersion}
Compression=lzma2
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64compatible
UninstallDisplayIcon={app}\PlayerokUniversity.exe
WizardStyle=modern

[Languages]
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"

[Tasks]
Name: "desktopicon"; Description: "Создать ярлык на рабочем столе"; Flags: unchecked

[Files]
Source: "..\target\release\playerok-university.exe"; DestDir: "{app}"; DestName: "PlayerokUniversity.exe"; Flags: ignoreversion

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\PlayerokUniversity.exe"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\PlayerokUniversity.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\PlayerokUniversity.exe"; Description: "Запустить {#AppName}"; Flags: nowait postinstall skipifsilent
