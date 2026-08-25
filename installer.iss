[Setup]
AppName=Fred Runtime
AppVersion=2.0-ALPHA
DefaultDirName={autopf}\FredRuntime
DefaultGroupName=Fred Runtime
UninstallDisplayIcon={app}\fred.exe
Compression=lzma2
SolidCompression=yes
OutputBaseFilename=FredSetup
CloseApplications=yes
RestartApplications=no

[Files]
Source: "target\release\fred.exe"; DestDir: "{app}"; Flags: ignoreversion restartreplace

[Tasks]
Name: envPath; Description: "Add Fred Runtime to system PATH"; Flags: unchecked

[Code]
// Helper to safely delete conflicting files
procedure SafeDelete(FilePath: string);
begin
  if (FilePath <> '') and FileExists(FilePath) then
  begin
    DeleteFile(FilePath);
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
  OldPath, NewPath, UserHome, LocalAppData: string;
begin
  if CurStep = ssInstall then
  begin
    UserHome := GetEnv('USERPROFILE');
    LocalAppData := GetEnv('LOCALAPPDATA');

    // Clean up known legacy conflict locations
    if UserHome <> '' then
    begin
      SafeDelete(UserHome + '\.cargo\bin\fred.exe');
    end;
    if LocalAppData <> '' then
    begin
      SafeDelete(LocalAppData + '\Programs\Fred\fred.exe');
    end;
    SafeDelete('C:\FredRuntime\fred\build\fred.exe');
    SafeDelete('C:\Program Files (x86)\FredRuntime\fred.exe');
  end;

  if (CurStep = ssPostInstall) and IsTaskSelected('envPath') then
  begin
    if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OldPath) then
    begin
      if Pos(ExpandConstant('{app}'), OldPath) = 0 then
      begin
        // Prepend to PATH so Program Files takes priority over everything else
        NewPath := ExpandConstant('{app}') + ';' + OldPath;
        RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', NewPath);
      end;
    end;
  end;
end;