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
procedure CurStepChanged(CurStep: TSetupStep);
var
  OldPath, NewPath, UserHome, CargoPath: string;
begin
  if CurStep = ssInstall then
  begin
    // Safely retrieve the user profile path using GetEnv
    UserHome := GetEnv('USERPROFILE');
    if UserHome <> '' then
    begin
      CargoPath := UserHome + '\.cargo\bin\fred.exe';
      if FileExists(CargoPath) then
      begin
        DeleteFile(CargoPath);
      end;
    end;
  end;

  if (CurStep = ssPostInstall) and IsTaskSelected('envPath') then
  begin
    if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OldPath) then
    begin
      if Pos(ExpandConstant('{app}'), OldPath) = 0 then
      begin
        NewPath := ExpandConstant('{app}') + ';' + OldPath;
        RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', NewPath);
      end;
    end;
  end;
end;