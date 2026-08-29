[Setup]
AppName=Fred Runtime
AppVersion=2.0-ALPHA
AppPublisher=Fred Runtime Project
DefaultDirName={autopf}\FredRuntime
DefaultGroupName=Fred Runtime
OutputBaseFilename=fredsetup
Compression=lzma2
SolidCompression=yes
; Tells Windows Explorer to refresh file icons immediately upon finishing install
ChangesAssociations=yes
; Optional installer executable icon
SetupIconFile=src\icon\fred.ico

PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "addtopath"; Description: "Add Fred Runtime to System PATH"; Flags: unchecked
Name: "vscodeext"; Description: "Install Fred Runtime VS Code Extension"; Flags: checkedonce

[Files]
; Copy the main binary (with embedded icon)
Source: "target\release\fred.exe"; DestDir: "{app}"; Flags: ignoreversion
; Copy VS Code extension package if built
Source: "vscode-extension\fred-runtime-support-1.0.0.vsix"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist

[Registry]
; 1. Register .frd File Extension
Root: HKCU; Subkey: "Software\Classes\.frd"; ValueType: string; ValueData: "FredRuntimeScript"; Flags: uninsdeletevalue

; 2. Define ProgID details
Root: HKCU; Subkey: "Software\Classes\FredRuntimeScript"; ValueType: string; ValueData: "Fred Runtime Script"; Flags: uninsdeletekey

; 3. Use Icon Index 0 from fred.exe for .frd files
Root: HKCU; Subkey: "Software\Classes\FredRuntimeScript\DefaultIcon"; ValueType: string; ValueData: "{app}\fred.exe,0"

; 4. Set Double-Click execution action
Root: HKCU; Subkey: "Software\Classes\FredRuntimeScript\shell\open\command"; ValueType: string; ValueData: """{app}\fred.exe"" ""%1"""

; Optional: Add {app} to User PATH
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Tasks: addtopath; Check: NeedsAddPath('{app}')

[Run]
; Auto-install VS Code extension if selected and VS Code is installed on user machine
[Run]
Filename: "code"; Parameters: "--install-extension ""{app}\fred-runtime-support-1.0.0.vsix"" --force"; Tasks: vscodeext; Flags: runhidden skipifdoesntexist; StatusMsg: "Installing VS Code Extension..."

[Code]
// Helper function to check if directory is already in user PATH
function NeedsAddPath(Param: string): Boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OrigPath) then
  begin
    Result := True;
    Exit;
  end;
  Result := Pos(';' + UpperCase(Param) + ';', ';' + UpperCase(OrigPath) + ';') = 0;
end;