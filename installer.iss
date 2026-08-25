[Setup]
AppName=Fred Runtime
AppVersion=2.0-ALPHA
DefaultDirName={autopf}\FredRuntime
DefaultGroupName=Fred Runtime
UninstallDisplayIcon={app}\fred.exe
Compression=lzma2
SolidCompression=yes
OutputBaseFilename=FredSetup

[Files]
; Path to your compiled Rust binary
Source: "target\release\fred.exe"; DestDir: "{app}"; Flags: ignoreversion

[Tasks]
Name: envPath; Description: "Add Fred Runtime to system PATH"; Flags: unchecked

[Code]
// Helper function to append the installation directory to the user's PATH environment variable
procedure CurStepChanged(CurStep: TSetupStep);
var
  OldPath: string;
  NewPath: string;
begin
  if (CurStep = ssPostInstall) and IsTaskSelected('envPath') then
  begin
    if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OldPath) then
    begin
      if Pos(ExpandConstant('{app}'), OldPath) = 0 then
      begin
        NewPath := OldPath + ';' + ExpandConstant('{app}');
        RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', NewPath);
      end;
    end;
  end;
end;