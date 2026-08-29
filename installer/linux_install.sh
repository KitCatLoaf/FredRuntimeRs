#!/usr/bin/env bash

set -e

BIN_DIR="$HOME/.local/bin"
MIME_DIR="$HOME/.local/share/mime/packages"
APP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/128x128/apps"

echo "Installing Fred Runtime for Linux..."

mkdir -p "$BIN_DIR" "$MIME_DIR" "$APP_DIR" "$ICON_DIR"

cp ./fred "$BIN_DIR/fred"
chmod +x "$BIN_DIR/fred"

if [ -f "./src/icon/fred.png" ]; then
    cp ./src/icon/fred.png "$ICON_DIR/fred-runtime.png"
elif [ -f "./fred.png" ]; then
    cp ./fred.png "$ICON_DIR/fred-runtime.png"
fi

cat << 'EOF' > "$MIME_DIR/fred-runtime.xml"
<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
  <mime-type type="application/x-fred-script">
    <comment>Fred Runtime Script</comment>
    <glob pattern="*.frd"/>
    <icon name="fred-runtime"/>
  </mime-type>
</mime-info>
EOF

cat << EOF > "$APP_DIR/fred-runtime.desktop"
[Desktop Entry]
Type=Application
Name=Fred Runtime
Comment=Lua Scripting Environment
Exec=$BIN_DIR/fred %f
Icon=fred-runtime
Terminal=true
MimeType=application/x-fred-script;
Categories=Development;
EOF

echo "Updating desktop and icon caches..."
update-mime-database "$HOME/.local/share/mime" > /dev/null 2>&1 || true
update-desktop-database "$APP_DIR" > /dev/null 2>&1 || true
gtk-update-icon-cache -f -t "$HOME/.local/share/icons/hicolor" > /dev/null 2>&1 || true

echo "Installation complete! .frd files are now associated with Fred Runtime."